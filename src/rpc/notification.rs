use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum CoreNotification {
    TracingConfig {
        enabled: bool,
    },
    SendInitialize {},
    ClientStarted {
        #[serde(default)]
        config_dir: Option<PathBuf>,
        #[serde(default)]
        client_extras_dir: Option<PathBuf>,
    },
}
