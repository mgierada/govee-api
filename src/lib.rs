use reqwest::{Client, Url};
use serde::{de, Deserialize, Serialize};
use serde_json::json;
use structs::govee::{ApiResponseGoveeDeviceStatus, ApiResponseGoveeAllDevices};

pub mod tests;
pub mod structs;


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
