use crate::utils::request::deserialize_bool;
use serde::{Deserialize, Serialize};

// ------------------------
// Structs for the Govee API
// ------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseGoveeDeviceState {
    pub code: i16,
    pub message: String,
    pub data: Option<GoveeDataDeviceStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoveeDataDeviceStatus {
    pub device: String,
    pub model: String,
    pub properties: Vec<GoveeDeviceProperty>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum GoveeDeviceProperty {
    #[serde(rename = "online")]
    #[serde(deserialize_with = "deserialize_bool")]
    // Online can be a boolean or a string
    Online(bool),
    #[serde(rename = "powerState")]
    PowerState(String),
    #[serde(rename = "brightness")]
    Brightness(i16),
    #[serde(rename = "color")]
    Color(Color),
    #[serde(rename = "colorTem")]
    ColorTem(i16),
    #[serde(rename = "colorTemInKelvin")]
    ColorTemInKelvin(i16),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseGoveeDevices {
    code: Option<i16>,
    message: String,
    pub data: Option<GoveeData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseGoveeAppliances {
    code: Option<i16>,
    message: String,
    pub data: Option<GoveeData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoveeData {
    pub devices: Vec<GoveeDevice>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GoveeDevice {
    pub device: String,
    pub model: String,
    pub deviceName: String,
    pub controllable: bool,
    pub retrievable: bool,
    pub supportCmds: Vec<String>,
    pub properties: Properties,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Properties {
    pub colorTem: ColorTem,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorTem {
    pub range: ColorTemRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorTemRange {
    pub min: i16,
    pub max: i16,
}

#[derive(Serialize, Clone)]
pub struct PayloadBody {
    pub device: String,
    pub model: String,
    pub cmd: GoveeCommand,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GoveeCommand {
    pub name: String,
    pub value: String,
}
