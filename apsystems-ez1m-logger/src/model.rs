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
    pub te1: f32,
    pub p2: f32,
    pub e2: f32,
    pub te2: f32,
}


impl CurrentOutput {
    pub fn new(device_id: &String, message: &String) -> Self {
        CurrentOutput {
            data: CurrentOutputData {
                p1: 0.0,
                e1: 0.0,
                te1: 0.0,
                p2: 0.0,
                e2: 0.0,
                te2: 0.0,
            },
            message: message.clone(),
            device_id: device_id.clone(),
        }
    }
}
