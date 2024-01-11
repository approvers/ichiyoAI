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
            tracing::warn!(res.status = ?res.status(), "Unexpected status code");
        }

        let http_status = res.status().as_u16();

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

        let (candidates, prompt_feedback) = match res {
            Response::Success {
                candidates,
                prompt_feedback,
            } => (candidates, prompt_feedback),
            Response::Error { error } => {
                tracing::error!(?error, "Receive error response");

                let Error {
                    code,
                    message,
                    status,
                } = error;

                if code != http_status as usize {
                    tracing::warn!(error.code = %code, res.status = %http_status, "Unmatched error code");
                }

                let cr = code
                    .try_into()
                    .ok()
                    .and_then(|code| reqwest::StatusCode::from_u16(code).ok())
                    .and_then(|code| code.canonical_reason())
                    .unwrap_or("Unknown");

                let status = match status {
                    Either::Lhs(kind) => format!("{:?}", kind),
                    Either::Rhs(status) => format!("raw: `{status}`"),
                };

                let reason =
                    format!("Respond with \"{cr}\" ({code}), status = {status}):\n```{message}```");

                return Err(reason.into());
            }
        };

        let Some([candidate]) = candidates else {
            let reason = prompt_feedback
                .block_reason
                .map(|br| format!("{br:?}"))
                .unwrap_or("<None>".to_owned());

            let ratings = prompt_feedback
                .safety_ratings
                .iter()
                .map(|sr| format!("- **{}** {:?}", sr.category, sr.probability))
                .fold(String::new(), |c, n| c + &n + "\n");

            let reason =
                format!("Prompt blocked! reason = {reason}\n### Safety ratings:\n{ratings}");

            return Err(reason.into());
        };

        if candidate.finish_reason != FinishReason::Stop {
            tracing::warn!(?candidate.finish_reason, "Unexpected finish reason");
        }

        let Some(content) = &candidate.content else {
            let Candidate {
                finish_reason,
                safety_ratings,
                content: None,
            } = candidate
            else {
                unreachable!()
            };

            let ratings = safety_ratings
                .iter()
                .map(|sr| format!("- **{}** {:?}", sr.category, sr.probability))
                .fold(String::new(), |c, n| c + &n + "\n");

            let reason = format!(
                "Generation is stopped, reason = {finish_reason}.\n### Safety ratings:\n{ratings}"
            );

            return Err(reason.into());
        };

        let content = {
            let Content::Model { parts } = content else {
                tracing::error!(?candidate.content, "Unexpected content");
                return Err("Failed to deserialize response".into());
            };

            let [ref part] = parts[..] else {
                tracing::error!(?parts, "Unexpected number of parts");
                return Err("Failed to deserialize response".into());
            };

            let Part::Text(text) = part else {
                tracing::error!(?part, "Unexpected part");
                return Err("Failed to deserialize response".into());
            };

            text.clone().into_owned()
        };

        let contents = messages
            .iter()
            .map(Into::into)
            .chain([candidate.content.unwrap()])
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
#[serde(untagged)]
enum Response<'a> {
    #[serde(rename_all = "camelCase")]
    Success {
        // FACT: supported only `1`
        #[serde(borrow)]
        candidates: Option<[Candidate<'a>; 1]>,
        prompt_feedback: PromptFeedback,
    },
    #[serde(rename_all = "camelCase")]
    #[rustfmt::skip]
    Error {
        error: Error<'a>,
    },
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Error<'a> {
    code: usize,
    message: alloc::borrow::Cow<'a, str>,
    #[serde(borrow)]
    status: Either<ErrorKind, &'a str>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ErrorKind {
    InvalidArgument,
    Internal,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Lhs(L),
    Rhs(R),
}

impl<L, R> Either<L, R> {
    fn lhs(&self) -> Option<&L> {
        match self {
            Self::Lhs(l) => Some(l),
            Self::Rhs(_) => None,
        }
    }

    fn rhs(&self) -> Option<&R> {
        match self {
            Self::Lhs(_) => None,
            Self::Rhs(r) => Some(r),
        }
    }
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

impl core::fmt::Display for HarmCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Unspecified => {
                f.write_str("Category is unspecified")
            },
            Self::Derogatory => {
                f.write_str("Negative or harmful comments targeting identity and/or protected attribute")
            },
            Self::Toxicity => {
                f.write_str("Content that is rude, disrepspectful, or profane")
            },
            Self::Violence => {
                f.write_str("Describes scenarios depictng violence against an individual or group, or general descriptions of gore")
            },
            Self::Sexual => {
                f.write_str("Contains references to sexual acts or other lewd content")
            },
            Self::Medical => {
                f.write_str("Promotes unchecked medical advice")
            },
            Self::Dangerous => {
                f.write_str("Dangerous content that promotes, facilitates, or encourages harmful acts")
            },
            Self::Harassment => {
                f.write_str("Harasment content")
            },
            Self::HateSpeech => {
                f.write_str("Hate speech and content")
            },
            Self::SexuallyExplicit => {
                f.write_str("Sexually explicit content")
            },
            Self::DangerousContent => {
                f.write_str("Dangerous content")
            },
        }
    }
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

impl core::fmt::Display for HarmProbability {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Unspecified => {
                f.write_str("Probability is unspecified")
            },
            Self::Negligible => {
                f.write_str("Content has a negligible chance of being unsafe")
            },
            Self::Low => {
                f.write_str("Content has a low chance of being unsafe")
            },
            Self::Medium => {
                f.write_str("Content has a medium chance of being unsafe")
            },
            Self::High => {
                f.write_str("Content has a high chance of being unsafe")
            },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate<'a> {
    content: Option<Content<'a>>,
    finish_reason: FinishReason,
    safety_ratings: Vec<SafetyRating>,
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
