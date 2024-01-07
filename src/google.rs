// ref: https://ai.google.dev/api/rest/v1beta/models/generateContent
pub struct Google<Model> {
    http: reqwest::Client,
    token: String,
    model: core::marker::PhantomData<Model>,
}

trait Model {
    const NAME: &'static str;
}

macro_rules! define_model {
    ($vis:vis $name:ident : $model:expr) => {
        $vis struct $name;

        impl Model for $name {
            const NAME: &'static str = $model;
        }
    };
}

define_model!(pub GeminiPro: "gemini-pro");

impl<Model> Google<Model> {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token.as_ref().to_owned(),
            model: core::marker::PhantomData,
        }
    }
}

impl<Model: self::Model + Send + Sync> super::Completion for Google<Model> {
    #[tracing::instrument(skip_all)]
    async fn next(
        &self,
        messages: &[super::Message],
    ) -> anyhow::Result<(super::Message, super::Metadata)> {
        let req = Request {
            contents: messages.iter().map(Into::into).collect(),
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            Model::NAME,
        );

        let raw = serde_json::to_vec(&req).map_err(|cause| {
            tracing::error!(?cause, "Failed to serialize request");
            anyhow::anyhow!("Failed to serialize request")
        })?;

        let body = reqwest::Body::from(raw);

        let res = self
            .http
            .post(url)
            .header(reqwest::header::HeaderName::from_static("x-goog-api-key"), &self.token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .map_err(|cause| {
                tracing::error!(?cause, "Failed to send request");
                anyhow::anyhow!("Failed to send request")
            })?;

        if res.status() != reqwest::StatusCode::OK {
            tracing::error!(res.status = ?res.status(), "Unexpected status code");
        }

        let res = res.bytes().await.map_err(|cause| {
            tracing::error!(?cause, "Failed to read response");
            anyhow::anyhow!("Failed to read response")
        })?;

        let res = serde_json::from_slice::<Response>(&res).map_err(|cause| {
            tracing::error!(?cause, "Failed to deserialize response");

            if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&res) {
                tracing::error!(%body, "Actual response (recognizable as JSON)");
            }

            anyhow::anyhow!("Failed to deserialize response")
        })?;

        let Some([candidate]) = res.candidates else {
            let reason = res.prompt_feedback.block_reason;
            anyhow::bail!("Prompt blocked! reason: {reason:?}");
        };

        if candidate.finish_reason != FinishReason::Stop {
            anyhow::bail!("Unexpected finish reason: {}", candidate.finish_reason);
        }

        let content = {
            let Content::Model { parts } = &candidate.content else {
                tracing::error!(?candidate.content, "Unexpected content");
                anyhow::bail!("Failed to deserialize response");
            };

            let [ref part] = parts[..] else {
                tracing::error!(?parts, "Unexpected number of parts");
                anyhow::bail!("Failed to deserialize response");
            };

            #[allow(irrefutable_let_patterns)]
            let Part::Text(text) = part
            else {
                tracing::error!(?part, "Unexpected part");
                anyhow::bail!("Failed to deserialize response");
            };

            text.clone().into_owned()
        };

        let contents = messages
            .iter()
            .map(Into::into)
            .chain([candidate.content])
            .collect();

        let metadata = super::Metadata {
            tokens: count_tokens::<Model>(&self.http, &self.token, contents).await?,
            price_yen: 0.0,
            by: Model::NAME,
        };

        Ok((super::Message::Model { content }, metadata))
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Request<'a> {
    contents: Vec<Content<'a>>,
    // tools: Vec<Tool>,
    // safety_settings: Vec<SafetySetting>,
    // generation_config: GenerationConfig,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "role")]
enum Content<'a> {
    User { parts: Vec<Part<'a>> },
    Model { parts: Vec<Part<'a>> },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum Part<'a> {
    // cannot use `&'a str`, futher infomation:  https://github.com/serde-rs/serde/issues/1413
    Text(alloc::borrow::Cow<'a, str>),
    // InlineData(Blob),
    // FunctionCall(FunctionCall),
    // FunctionResponse(FunctionResponse),
}

impl<'a> From<&'a super::Message> for Content<'a> {
    fn from(message: &'a super::Message) -> Self {
        match message {
            super::Message::User { content, .. } => Self::User {
                parts: vec![Part::Text(content.into())],
            },
            super::Message::Model { content, .. } => Self::Model {
                parts: vec![Part::Text(content.into())],
            },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response<'a> {
    // FACT: supported only `1`
    #[serde(borrow)]
    candidates: Option<[Candidate<'a>; 1]>,
    prompt_feedback: PromptFeedback,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PromptFeedback {
    block_reason: Option<BlockReason>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BlockReason {
    Safety,
    Other,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate<'a> {
    content: Content<'a>,
    finish_reason: FinishReason,
    // safety_ratings: Vec<SafetyRating>,
    // citation_metadata: CitationMetadata,
    // // FACT: doesn't exist in a response
    // token_count: usize,
    // grounding_attributions: Vec<GroundingAttribution>,
    // index: usize,
}

#[derive(Debug, PartialEq)]
enum FinishReason {
    Stop,
    Maxtokens,
    Safety,
    Recitation,
    Other,
}

#[rustfmt::skip]
impl core::fmt::Display for FinishReason {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Stop => {
                f.write_str("Natural stop point of the model or provided stop sequence")
            },
            Self::Maxtokens => {
                f.write_str("The maximum number of tokens as specified in the request was reached")
            },
            Self::Safety => {
                f.write_str("The candidate content was flagged for safety reasons")
            },
            Self::Recitation => {
                f.write_str("The candidate content was flagged for recitation reasons")
            },
            Self::Other => {
                f.write_str("Unknown reason")
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for FinishReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(FinishReasonVisitor)
    }
}

struct FinishReasonVisitor;

impl<'de> serde::de::Visitor<'de> for FinishReasonVisitor {
    type Value = FinishReason;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a finish reason")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "STOP" => Ok(FinishReason::Stop),
            "MAX_TOKENS" => Ok(FinishReason::Maxtokens),
            "SAFETY" => Ok(FinishReason::Safety),
            "RECITATION" => Ok(FinishReason::Recitation),
            "OTHER" => Ok(FinishReason::Other),
            _ => Err(E::custom(format!("unknown finish reason: {}", value))),
        }
    }
}

#[tracing::instrument(skip_all)]
async fn count_tokens<Model: self::Model + Send + Sync>(
    client: &reqwest::Client,
    token: &str,
    contents: Vec<Content<'_>>,
) -> anyhow::Result<usize> {
    #[derive(Debug, serde::Serialize)]
    struct Request<'a> {
        contents: Vec<Content<'a>>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        total_tokens: usize,
    }

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:countTokens",
        Model::NAME,
    );

    let req = Request { contents };
    let raw = serde_json::to_vec(&req).map_err(|cause| {
        tracing::error!(?cause, "Failed to serialize request");
        anyhow::anyhow!("Failed to serialize request")
    })?;
    let body = reqwest::Body::from(raw);

    let res = client
        .post(url)
        .header(
            reqwest::header::HeaderName::from_static("x-goog-api-key"),
            token,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .map_err(|cause| {
            tracing::error!(?cause, "Failed to send request");
            anyhow::anyhow!("Failed to send request")
        })?;

    if res.status() != reqwest::StatusCode::OK {
        tracing::error!(res.status = ?res.status(), "Unexpected status code");
    }

    let res = res.bytes().await.map_err(|cause| {
        tracing::error!(?cause, "Failed to read response");
        anyhow::anyhow!("Failed to read response")
    })?;

    let res = serde_json::from_slice::<Response>(&res).map_err(|cause| {
        tracing::error!(?cause, "Failed to deserialize response");

        if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&res) {
            tracing::error!(%body, "Actual response (recognizable as JSON)");
        }

        anyhow::anyhow!("Failed to deserialize response")
    })?;

    Ok(res.total_tokens)
}
