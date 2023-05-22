use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum CoreRequest {
    Config {
        open_ai_token: String,
        open_ai_url: String,
    },
    Version {
        version: String,
    }
}
