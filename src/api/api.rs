use reqwest::{Client, Url};
use serde_json::json;

use crate::structs::govee::{ApiResponseGoveeDeviceState, ApiResponseGoveeDevices, PayloadBody};

// ------------------------
// Methods for the Govee API
// ------------------------

pub async fn control_device(govee_root_url: &str, govee_api_key: &str, payload: PayloadBody) -> () {
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

pub async fn get_devices(govee_root_url: &str, govee_api_key: &str) -> ApiResponseGoveeDevices {
    let client = Client::new();
    let endpoint = format!("{}/v1/devices", govee_root_url);
    let response = client
        .get(endpoint)
        .header("Govee-API-Key", govee_api_key)
        .send()
        .await
        .unwrap()
        .json::<ApiResponseGoveeDevices>();
    let response_json: ApiResponseGoveeDevices = response.await.unwrap();
    response_json
}

pub async fn get_device_state(
    govee_root_url: &str,
    govee_api_key: &str,
    device: &str,
    model: &str,
) -> ApiResponseGoveeDeviceState {
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
        .json::<ApiResponseGoveeDeviceState>();
    let response_json: ApiResponseGoveeDeviceState = response.await.unwrap();
    response_json
}
