use reqwest::{Client, Url};
use serde::{de, Deserialize, Serialize};
use serde_json::json;

// ------------------------
// Structs for the Govee API
// ------------------------

#[derive(Serialize)]
pub struct PayloadBody {
    pub device: String,
    pub model: String,
    pub cmd: GoveeCommand,
}

#[derive(Serialize, Deserialize)]
pub struct GoveeCommand {
    pub name: String,
    pub value: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseGoveeDeviceStatus {
    code: i16,
    message: String,
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
pub struct ApiResponseGoveeAllDevices {
    code: i16,
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

// ------------------------
// Handling Govee Issues
// ------------------------
//

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    // If the incoming value is a string 'true' or 'false', return true or false
    // If the incoming value is a boolean, return the boolean
    match serde::Deserialize::deserialize(deserializer)? {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(s) if s == "true" => Ok(true),
        serde_json::Value::String(s) if s == "false" => Ok(false),
        _ => Err(serde::de::Error::custom(
            "Expected a boolean or 'true'/'false' string",
        )),
    }
}

// ------------------------
// Methods for the Govee API
// ------------------------

pub async fn sent_put_request(
    govee_root_url: &str,
    govee_api_key: &str,
    payload: PayloadBody,
) -> () {
    let client = Client::new();
    let payload_json = json!(payload);
    let endpoint = format!("{}/v1/devices/control", govee_root_url);
    let _response = client
        .put(endpoint)
        .header("Govee-API-Key", govee_api_key)
        .json(&payload_json)
        .send()
        .await
        .unwrap();
}

pub async fn get_all_devices(
    govee_root_url: &str,
    govee_api_key: &str,
) -> ApiResponseGoveeAllDevices {
    let client = Client::new();
    let endpoint = format!("{}/v1/devices", govee_root_url);
    let response = client
        .get(endpoint)
        .header("Govee-API-Key", govee_api_key)
        .send()
        .await
        .unwrap()
        .json::<ApiResponseGoveeAllDevices>();
    let response_json: ApiResponseGoveeAllDevices = response.await.unwrap();
    response_json
}

pub async fn get_device_status(
    govee_root_url: &str,
    govee_api_key: &str,
    device: &str,
    model: &str,
) -> ApiResponseGoveeDeviceStatus {
    let client = Client::new();
    let params = [("device", device), ("model", model)];
    let endpoint = format!("{}/v1/devices/state", govee_root_url);
    let url = Url::parse_with_params(&endpoint, &params).unwrap();
    let response = client
        .get(url)
        .header("Govee-API-Key", govee_api_key)
        .send()
        .await
        .unwrap()
        .json::<ApiResponseGoveeDeviceStatus>();
    let response_json: ApiResponseGoveeDeviceStatus = response.await.unwrap();
    response_json
}
