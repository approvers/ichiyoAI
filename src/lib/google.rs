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
    ) -> Result<(super::Message, super::CMetadata), alloc::borrow::Cow<'static, str>> {
        if messages.first().map(super::Message::is_user) != Some(true)
            || messages.last().map(super::Message::is_user) != Some(true)
        {
            return Err("First and last message must be from user".into());
        }

        let req = Request {
            contents: messages.iter().map(Into::into).collect(),
            ..Default::default()
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            Model::NAME,
        );

        let raw = serde_json::to_vec(&req).map_err(|cause| {
            tracing::error!(?cause, "Failed to serialize request");
            "Failed to serialize request"
        })?;

        let body = reqwest::Body::from(raw);

        let res = self
            .http
            .post(url)
            .header("X-Goog-API-Key", &self.token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .map_err(|cause| {
                tracing::error!(?cause, "Failed to send request");
                "Failed to send request"
            })?;

        if res.status() != reqwest::StatusCode::OK {
            tracing::error!(res.status = ?res.status(), "Unexpected status code");
        }

        let res = res.bytes().await.map_err(|cause| {
            tracing::error!(?cause, "Failed to read response");
            "Failed to read response"
        })?;

        let res = serde_json::from_slice::<Response>(&res).map_err(|cause| {
            tracing::error!(?cause, "Failed to deserialize response");

            if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&res) {
                tracing::error!(%body, "Actual response (recognizable as JSON)");
            }

            "Failed to deserialize response"
        })?;

        let Some([candidate]) = res.candidates else {
            let reason = res.prompt_feedback.block_reason;
            return Err(format!("Prompt blocked! reason: {reason:?}").into());
        };

        if candidate.finish_reason != FinishReason::Stop {
            return Err(format!("Unexpected finish reason: {}", candidate.finish_reason).into());
        }

        let content = {
            let Content::Model { parts } = &candidate.content else {
                tracing::error!(?candidate.content, "Unexpected content");
                return Err("Failed to deserialize response".into());
            };

            let [ref part] = parts[..] else {
                tracing::error!(?parts, "Unexpected number of parts");
                return Err("Failed to deserialize response".into());
            };

            #[allow(irrefutable_let_patterns)]
            let Part::Text(text) = part
            else {
                tracing::error!(?part, "Unexpected part");
                return Err("Failed to deserialize response".into());
            };

            text.clone().into_owned()
        };

        let contents = messages
            .iter()
            .map(Into::into)
            .chain([candidate.content])
            .collect();

        let metadata = super::CMetadata {
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
    safety_settings: Vec<SafetySetting>,
    generation_config: GenerationConfig,
}

impl<'a> Default for Request<'a> {
    fn default() -> Self {
        // NOTE: they are exhaustive
        #[rustfmt::skip]
        let safety_settings = [
            (HarmCategory::Harassment,       HarmBlockThreshold::None),
            (HarmCategory::HateSpeech,       HarmBlockThreshold::None),
            (HarmCategory::SexuallyExplicit, HarmBlockThreshold::None),
            (HarmCategory::DangerousContent, HarmBlockThreshold::None),
        ]
        .into_iter()
        .map(|(category, threshold)| SafetySetting {
            category,
            threshold,
        })
        .collect();

        Self {
            contents: vec![],
            // tools: vec![],
            safety_settings,
            generation_config: GenerationConfig::with_max_tokens(/* HACK */ 500),
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SafetySetting {
    category: HarmCategory,
    threshold: HarmBlockThreshold,
}

#[derive(Debug, serde::Serialize)]
enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    LowAndAbrove,
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    MediumAndAbove,
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    OnlyHigh,
    #[serde(rename = "BLOCK_NONE")]
    None,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
    // stop_sequences: Vec<&'a str>,
    // candidate_count: usize,
    max_output_tokens: usize,
    // temperature: f64,
    // top_p: f64,
    // top_k: usize,
}

impl GenerationConfig {
    fn with_max_tokens(max_output_tokens: usize) -> Self {
        Self { max_output_tokens }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BlockReason {
    Safety,
    Other,
}

#[derive(Debug, serde::Deserialize)]
struct SafetyRating {
    category: HarmCategory,
    probability: HarmProbability,
    // blocked: Option<bool>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    Derogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    Toxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    Violence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    Sexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    Medical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    Dangerous,
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    Harassment,
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HateSpeech,
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    SexuallyExplicit,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    DangerousContent,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
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

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FinishReason {
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
}

impl core::fmt::Display for FinishReason {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Stop => {
                f.write_str("Natural stop point of the model or provided stop sequence")
            },
            Self::MaxTokens => {
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

#[tracing::instrument(skip_all)]
async fn count_tokens<Model: self::Model + Send + Sync>(
    client: &reqwest::Client,
    token: &str,
    contents: Vec<Content<'_>>,
) -> Result<usize, alloc::borrow::Cow<'static, str>> {
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
        "Failed to serialize request"
    })?;
    let body = reqwest::Body::from(raw);

    let res = client
        .post(url)
        .header("X-Goog-API-Key", token)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await
        .map_err(|cause| {
            tracing::error!(?cause, "Failed to send request");
            "Failed to send request"
        })?;

    if res.status() != reqwest::StatusCode::OK {
        tracing::error!(res.status = ?res.status(), "Unexpected status code");
    }

    let res = res.bytes().await.map_err(|cause| {
        tracing::error!(?cause, "Failed to read response");
        "Failed to read response"
    })?;

    let res = serde_json::from_slice::<Response>(&res).map_err(|cause| {
        tracing::error!(?cause, "Failed to deserialize response");

        if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&res) {
            tracing::error!(%body, "Actual response (recognizable as JSON)");
        }

        "Failed to deserialize response"
    })?;

    Ok(res.total_tokens)
}
