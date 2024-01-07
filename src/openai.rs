// ref: https://platform.openai.com/docs/api-reference/chat
pub struct OpenAi<Model> {
    http: reqwest::Client,
    token: String,
    model: core::marker::PhantomData<Model>,
}

trait Model {
    fn name() -> &'static str;
    fn price_yen(req_tokens: usize, res_tokens: usize) -> f64;
}

macro_rules! define_model {
    ($vis:vis $name:ident : $model:expr , rate = req $rreq:expr, res $rres:expr) => {
        $vis struct $name;

        impl Model for $name {
            fn name() -> &'static str {
                $model
            }

            fn price_yen(req_tokens: usize, res_tokens: usize) -> f64 {
                // あくまでも概算なので, 浮動小数点数程度の精度で十分
                ($rreq * req_tokens as f64 + $rres * res_tokens as f64) / 1000.0 * 150.0
            }
        }
    };
}

define_model!(pub GPT4Turbo:  "gpt-4-1106-preview", rate = req 0.01  , res 0.03  );
define_model!(pub GPT35Turbo: "gpt-3.5-turbo-1106", rate = req 0.0010, res 0.0020);

impl<Model> OpenAi<Model> {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token.as_ref().to_owned(),
            model: core::marker::PhantomData,
        }
    }
}

impl<Model: self::Model + Send + Sync> super::Completion for OpenAi<Model> {
    #[tracing::instrument(skip_all)]
    async fn next(
        &self,
        messages: &[super::Message],
    ) -> anyhow::Result<(super::Message, super::Metadata)> {
        let req = Request {
            model: Model::name(),
            messages: messages.iter().map(Into::into).collect(),
        };

        let raw = serde_json::to_vec(&req).map_err(|cause| {
            tracing::error!(?cause, "Failed to serialize request");
            anyhow::anyhow!("Failed to serialize request")
        })?;

        let body = reqwest::Body::from(raw);

        let res = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.token)
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

        if res.object != "text_completion" {
            tracing::error!(?res.object, "Unexpected object");
            anyhow::bail!("Failed to deserialize response");
        }

        if res.model != Model::name() {
            tracing::error!(?res.model, "Unexpected model");
            anyhow::bail!("Failed to deserialize response");
        }

        let [choice] = &res.choices[..] else {
            tracing::error!(?res.choices, "Unexpected number of choices");
            anyhow::bail!("Failed to deserialize response");
        };

        if choice.finish_reason != FinishReason::Stop {
            tracing::error!(?choice.finish_reason, "Unexpected finish reason",);
            anyhow::bail!("Failed to deserialize response");
        }

        let content = choice.message.content.trim().to_owned();
        let metadata = super::Metadata {
            tokens: res.usage.total_tokens,
            price_yen: Model::price_yen(res.usage.prompt_tokens, res.usage.completion_tokens),
            by: Model::name(),
        };

        Ok((super::Message::Model { content }, metadata))
    }
}

#[derive(Debug, serde::Serialize)]
struct Request<'a> {
    model: &'static str,
    messages: Vec<Message<'a>>,
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "role")]
#[serde(rename_all = "lowercase")]
enum Message<'a> {
    User { content: &'a str },
    Assistant { content: &'a str },
}

impl<'a> From<&'a super::Message> for Message<'a> {
    fn from(message: &'a super::Message) -> Self {
        match message {
            super::Message::User { content, .. } => Self::User {
                content: content.as_str(),
            },
            super::Message::Model { content, .. } => Self::Assistant {
                content: content.as_str(),
            },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct Response<'a> {
    // id: &'a str,
    #[serde(borrow)]
    choices: Vec<Choice<'a>>,
    // created: Timestamp,
    model: &'a str,
    // system_fingertprint: &'a str,
    object: &'a str,
    usage: Usage,
}

#[derive(Debug, serde::Deserialize)]
struct Choice<'a> {
    finish_reason: FinishReason,
    // index: usize,
    #[serde(borrow)]
    message: ChoiceMessage<'a>,
    // logprobs: Option<Logprobs>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    FunctionCall,
}

#[rustfmt::skip]
impl core::fmt::Display for FinishReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stop => {
                f.write_str("the model hit a natural stop point or a provided stop sequence")
            },
            Self::Length => {
                f.write_str("the maximum number of tokens specified in the request was reached")
            },
            Self::ContentFilter => {
                f.write_str("content was omitted due to a flag from our content filters")
            },
            Self::ToolCalls => {
                f.write_str("the model called a tool")
            },
            Self::FunctionCall => {
                f.write_str("the model called a function")
            },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct ChoiceMessage<'a> {
    // cannot use `&'a str`, futher infomation:  https://github.com/serde-rs/serde/issues/1413
    content: alloc::borrow::Cow<'a, str>,
    // tool_calls: Vec<ToolCall>,
    // role: Role,
}

#[derive(Debug, serde::Deserialize)]
struct Usage {
    completion_tokens: usize,
    prompt_tokens: usize,
    total_tokens: usize,
}
