use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdValue {
    pub warning: i32,
    pub safe: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusThresholds {
    pub time: ThresholdValue,
    pub balance: ThresholdValue,
    #[serde(rename = "timeMax")]
    pub time_max: i32,
    #[serde(rename = "balanceMax")]
    pub balance_max: i32,
}

impl Default for StatusThresholds {
    fn default() -> Self {
        Self {
            time: ThresholdValue {
                warning: 10,
                safe: 20,
            },
            balance: ThresholdValue {
                warning: 10000,
                safe: 20000,
            },
            time_max: 365,
            balance_max: 1000000,
        }
    }
}

