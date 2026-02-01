use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThunderConfig {
    pub telemetry: bool,
    pub adblock: bool,
    pub private_mode: bool,
}

impl Default for ThunderConfig {
    fn default() -> Self {
        Self {
            telemetry: false,
            adblock: true,
            private_mode: true,
        }
    }
}
