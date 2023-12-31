// ref: https://ai.google.dev/api/rest/v1beta/models/generateContent
const ENDPOINT: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent";

pub struct Gemini {
    http: reqwest::Client,
    token: String,
}

impl Gemini {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token.as_ref().to_owned(),
        }
    }
}

#[allow(clippy::declare_interior_mutable_const)]
const X_GOOG_API_KEY: reqwest::header::HeaderName =
    reqwest::header::HeaderName::from_static("x-goog-api-key");

impl super::Completion for Gemini {
    type Metadata = Metadata;

    async fn next<I: Send + Sync>(
        &self,
        messages: &[super::Message<I>],
    ) -> anyhow::Result<(super::Message<()>, Self::Metadata)> {
        let req = Request {
            contents: messages.iter().map(Into::into).collect(),
        };

        let res = self
            .http
            .post(ENDPOINT)
            .header(X_GOOG_API_KEY, &self.token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(reqwest::Body::from(serde_json::to_vec(&req)?))
            .send()
            .await?
            .error_for_status()?;

        if res.status() != reqwest::StatusCode::OK {
            anyhow::bail!("unexpected status code: {}", res.status());
        }

        let res = res.bytes().await?;
        let res = serde_json::from_slice::<Response>(&res)?;

        let [candidate] = res.candidates;

        if candidate.finish_reason != FinishReason::Stop {
            anyhow::bail!("unexpected finish reason: {}", candidate.finish_reason);
        }

        let content = {
            let Content::Model { parts } = &candidate.content else {
                anyhow::bail!("unexpected content: {:?}", candidate.content);
            };

            let [ref part] = parts[..] else {
                anyhow::bail!("unexpected number of parts: {}", parts.len());
            };

            #[allow(irrefutable_let_patterns)]
            let Part::Text(text) = part
            else {
                anyhow::bail!("unexpected part: {:?}", part);
            };

            text.clone().into_owned()
        };

        let contents = messages
            .iter()
            .map(Into::into)
            .chain([candidate.content])
            .collect();

        let metadata = Metadata {
            tokens: count_tokens(&self.http, &self.token, contents).await?,
        };

        Ok((super::Message::Model { id: (), content }, metadata))
    }
}

pub struct Metadata {
    pub tokens: usize,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Request<'a> {
    contents: Vec<Content<'a>>,
    // tools: Vec<Tool>,
    // safety_settings: Vec<SafetySetting>,
    // generation_config: GenerationConfig,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "role")]
enum Content<'a> {
    User {
        #[serde(borrow)]
        parts: Vec<Part<'a>>,
    },
    Model {
        #[serde(borrow)]
        parts: Vec<Part<'a>>,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Part<'a> {
    // cannot use `&'a str`, futher infomation:  https://github.com/serde-rs/serde/issues/1413
    Text(alloc::borrow::Cow<'a, str>),
    // InlineData(Blob),
    // FunctionCall(FunctionCall),
    // FunctionResponse(FunctionResponse),
}

impl<'a, I> From<&'a super::Message<I>> for Content<'a> {
    fn from(message: &'a super::Message<I>) -> Self {
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response<'a> {
    // FACT: supported only `1`
    #[serde(borrow)]
    candidates: [Candidate<'a>; 1],
    // prompt_feedback: PromptFeedback,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate<'a> {
    #[serde(borrow)]
    content: Content<'a>,
    finish_reason: FinishReason,
    // safety_ratings: Vec<SafetyRating>,
    // citation_metadata: CitationMetadata,
    // // FACT: doesn't exist in a response
    // token_count: usize,
    // grounding_attributions: Vec<GroundingAttribution>,
    // index: usize,
}

#[derive(PartialEq)]
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

async fn count_tokens(
    client: &reqwest::Client,
    token: &str,
    contents: Vec<Content<'_>>,
) -> anyhow::Result<usize> {
    #[derive(serde::Serialize)]
    struct Request<'a> {
        contents: Vec<Content<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        total_tokens: usize,
    }

    let res = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent")
        .header(X_GOOG_API_KEY, token)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(reqwest::Body::from(serde_json::to_vec(&Request {
            contents,
        })?))
        .send()
        .await?
        .error_for_status()?;

    if res.status() != reqwest::StatusCode::OK {
        anyhow::bail!("unexpected status code: {}", res.status());
    }

    let res = res.bytes().await?;
    let res = serde_json::from_slice::<Response>(&res)?;

    Ok(res.total_tokens)
}
