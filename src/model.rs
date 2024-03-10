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
    pub p1: f32,
    pub e1: f32,
    pub t1: f32,
    pub p2: f32,
    pub e2: f32,
    pub t2: f32,
}
