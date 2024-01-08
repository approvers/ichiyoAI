// ref: https://platform.openai.com/docs/api-reference/images
pub struct OpenAi<Model> {
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

define_model!(pub DallE2: "dall-e-2");
define_model!(pub DallE3: "dall-e-3");

impl<Model> OpenAi<Model> {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token.as_ref().to_owned(),
            model: core::marker::PhantomData,
        }
    }
}

impl<Model: self::Model + Send + Sync> super::Generation for OpenAi<Model> {
    #[tracing::instrument(skip_all)]
    async fn create(
        &self,
        prompt: impl AsRef<str> + Send + Sync,
    ) -> Result<(super::Image, super::IMetadata), alloc::borrow::Cow<'static, str>> {
        let prompt = prompt.as_ref();

        let req = Request {
            prompt,
            model: Model::NAME,
            response_format: "b64_json",
        };

        let body = serde_json::to_vec(&req).map_err(|cause| {
            tracing::error!(?cause, "Failed to serialize request");
            "Failed to serialize request"
        })?;

        let res = self
            .http
            .post("https://api.openai.com/v1/images/generations")
            .bearer_auth(&self.token)
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

        let [image] = res.data;
        if !image.b64_json.is_png() {
            tracing::warn!("Received image is not PNG");
        }

        let image = super::Image {
            raw: image.b64_json.raw,
            prompt: image.revised_prompt.unwrap_or(prompt).to_owned(),
            ext: super::ImageExt::Png, // FIXME: maybe it's not PNG!
        };

        let metadata = super::IMetadata { model: Model::NAME };

        Ok((image, metadata))
    }
}

#[derive(serde::Serialize)]
struct Request<'a> {
    prompt: &'a str,
    model: &'a str,
    response_format: &'a str,
}

#[derive(serde::Deserialize)]
struct Response<'a> {
    #[serde(borrow)]
    data: [Image<'a>; 1],
}

#[derive(serde::Deserialize)]
struct Image<'a> {
    b64_json: Base64Image,
    revised_prompt: Option<&'a str>,
}

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct Base64Image {
    #[serde(with = "base64")]
    raw: Vec<u8>,
}

impl Base64Image {
    fn is_png(&self) -> bool {
        self.raw
            .starts_with(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])
    }
}

mod base64 {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use base64::Engine as _;
        use serde::Deserialize as _;

        base64::engine::general_purpose::STANDARD
            .decode(<&str>::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}
