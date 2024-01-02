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

impl<Model: self::Model + Send + Sync> super::Image for OpenAi<Model> {
    async fn create(
        &self,
        prompt: impl AsRef<str> + Send + Sync,
    ) -> anyhow::Result<super::GeneratedImage> {
        let req = Request {
            prompt: prompt.as_ref(),
            model: Model::NAME,
            response_format: "b64_json",
        };

        let res = self
            .http
            .post("https://api.openai.com/v1/images/generations")
            .bearer_auth(&self.token)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&req)?)
            .send()
            .await?
            .error_for_status()?;

        if res.status() != reqwest::StatusCode::OK {
            anyhow::bail!("unexpected status code: {}", res.status());
        }

        let res = res.bytes().await?;
        let res = serde_json::from_slice::<Response>(&res)?;

        let [image] = res.data;
        assert!(image.b64_json.is_png());

        Ok(super::GeneratedImage {
            image: image.b64_json.0,
            prompt: image.revised_prompt.unwrap_or(prompt.as_ref()).to_owned(),
            ext: super::ImageExt::Png,
        })
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
struct Base64Image(#[serde(with = "base64")] Vec<u8>);

impl Base64Image {
    fn is_png(&self) -> bool {
        self.0
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