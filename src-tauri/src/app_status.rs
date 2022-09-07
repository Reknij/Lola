use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppStatus {
    pub lcu_loaded: bool,
}

impl AppStatus {
    pub fn default() -> Self {
        AppStatus { lcu_loaded: false }
    }
}
