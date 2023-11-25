use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
pub struct DaLLEResponseModel {
    pub model: String,
    pub prompt: String,
    pub res_image_url: String,
}
