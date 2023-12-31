// ref: https://platform.openai.com/docs/api-reference/chat
const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAi<Model> {
    http: reqwest::Client,
    token: String,
    model: core::marker::PhantomData<Model>,
}

trait Model: for<'de> serde::de::Deserialize<'de> {
    fn name() -> &'static str;
}

macro_rules! define_model {
    ($vis:vis $name:ident : $model:expr) => {
        $vis struct $name;

        impl Model for $name {
            fn name() -> &'static str {
                $model
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::de::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
                unreachable!()
            }
        }
    };
}

define_model!(pub GPT4Turbo: "gpt-4-1106-preview");
define_model!(pub GPT35Turbo: "gpt3.5-turbo-1106");

#[allow(private_bounds)]
impl<Model: self::Model> OpenAi<Model> {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token.as_ref().to_owned(),
            model: core::marker::PhantomData,
        }
    }
}

impl<Model: self::Model + Send + Sync> super::Completion for OpenAi<Model> {
    type Metadata = Metadata;

    async fn next<I: Send + Sync>(
        &self,
        messages: &[super::Message<I>],
    ) -> anyhow::Result<(super::Message<()>, Self::Metadata)> {
        let req = Request {
            model: Model::name(),
            messages: messages.iter().map(Into::into).collect(),
        };

        let res = self
            .http
            .post(ENDPOINT)
            .bearer_auth(&self.token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(reqwest::Body::from(serde_json::to_vec(&req)?))
            .send()
            .await?
            .error_for_status()?;

        if res.status() != reqwest::StatusCode::OK {
            anyhow::bail!("unexpected status code: {}", res.status());
        }

        let res = res.bytes().await?;
        let res = serde_json::from_slice::<Response<Model>>(&res)?;

        let [choice] = &res.choices[..] else {
            anyhow::bail!("unexpected number of choices: {}", res.choices.len());
        };

        if choice.finish_reason != FinishReason::Stop {
            anyhow::bail!("unexpected finish reason: {}", choice.finish_reason);
        }

        let content = choice.message.content.trim().to_owned();
        let metadata = Metadata {
            tokens: res.usage.total_tokens,
        };

        Ok((super::Message::Model { id: (), content }, metadata))
    }
}

pub struct Metadata {
    pub tokens: usize,
}

#[derive(serde::Serialize)]
struct Request<'a> {
    model: &'static str,
    messages: Vec<Message<'a>>,
}

#[derive(serde::Serialize)]
#[serde(tag = "role")]
#[serde(rename_all = "lowercase")]
enum Message<'a> {
    User {
        // cannot use `&'a str`, futher infomation:  https://github.com/serde-rs/serde/issues/1413
        content: alloc::borrow::Cow<'a, str>,
    },
    Assistant {
        // cannot use `&'a str`, futher infomation:  https://github.com/serde-rs/serde/issues/1413
        content: alloc::borrow::Cow<'a, str>,
    },
}

impl<'a, I> From<&'a super::Message<I>> for Message<'a> {
    fn from(message: &'a super::Message<I>) -> Self {
        match message {
            super::Message::User { content, .. } => Self::User {
                content: content.into(),
            },
            super::Message::Model { content, .. } => Self::Assistant {
                content: content.into(),
            },
        }
    }
}

#[derive(serde::Deserialize)]
struct Response<'a, Model: self::Model> {
    // id: &'a str,
    #[serde(borrow)]
    choices: Vec<Choice<'a>>,
    // created: Timestamp,
    #[allow(unused)]
    #[serde(with = "ModelName")]
    model: ModelName<Model>,
    // system_fingertprint: &'a str,
    #[allow(unused)]
    object: Object,
    usage: Usage,
}

#[derive(serde::Deserialize)]
struct Choice<'a> {
    finish_reason: FinishReason,
    // index: usize,
    #[serde(borrow)]
    message: ChoiceMessage<'a>,
    // logprobs: Option<Logprobs>,
}

#[derive(serde::Deserialize, PartialEq)]
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

#[derive(serde::Deserialize)]
struct ChoiceMessage<'a> {
    content: &'a str,
    // tool_calls: Vec<ToolCall>,
    // role: Role,
}

#[derive(serde::Deserialize)]
struct Usage {
    // completion_tokens: usize,
    // prompt_tokens: usize,
    total_tokens: usize,
}

macro_rules! tag_struct {
    ($name:ident $(<$ty:ident : $bound:path>)? is $tag:expr, expects $expect:expr) => {
        struct $name$(<$ty : $bound>(core::marker::PhantomData<$ty>))?;

        impl<'de, $($ty: $bound)?> serde::de::Deserialize<'de> for $name<$($ty)?> {
            fn deserialize<D: serde::de::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                deserializer.deserialize_str(Self$((core::marker::PhantomData::<$ty>))?)
            }
        }

        impl <'de, $($ty: $bound)?> serde::de::Visitor<'de> for $name<$($ty)?> {
            type Value = Self;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str($expect)
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value == $tag {
                    Ok(Self$((core::marker::PhantomData::<$ty>))?)
                } else {
                    Err(E::custom(format!("unknown value: {}", value)))
                }
            }
        }
    };
}

tag_struct!(ModelName<Model: self::Model> is Model::name(), expects "a model name");
tag_struct!(Object is "chat.completion", expects "a object name");
