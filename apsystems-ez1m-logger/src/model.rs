use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentOutput {
    pub data: CurrentOutputData,
    pub message: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentOutputData {
    pub p1: f64,
    pub e1: f64,
    pub te1: f64,
    pub p2: f64,
    pub e2: f64,
    pub te2: f64,
}

