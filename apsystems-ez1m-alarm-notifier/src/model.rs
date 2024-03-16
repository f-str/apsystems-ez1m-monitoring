use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Alarm {
    pub data: AlarmData,
    pub message: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlarmData {
    pub og: u8,
    pub oe: u8,
    pub isce1: u8,
    pub isce2: u8,
}

