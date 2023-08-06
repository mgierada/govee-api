use reqwest::{Client, Url};
use serde_json::json;

use crate::{
    structs::govee::{
        ApiResponseGoveeAppliances, ApiResponseGoveeDeviceState, ApiResponseGoveeDevices,
        PayloadBody,
    },
    GoveeClient, GOVEE_ROOT_URL,
};

// ------------------------
// Methods for the Govee API
// ------------------------
impl GoveeClient {
    pub async fn control_device(&self, payload: PayloadBody) -> () {
        let client = Client::new();
        let payload_json = json!(payload);
        let endpoint = format!("{}/v1/devices/control", &self.govee_root_url);
        println!("endpoint: {}", endpoint);
        println!("api_key: {}", &self.govee_api_key.to_string());
        let _response = client
            .put(endpoint)
            .header("Govee-API-Key", &self.govee_api_key.to_string())
            .json(&payload_json)
            .send()
            .await
            .unwrap();
    }
}

impl GoveeClient {
    pub async fn control_appliance(&self, payload: PayloadBody) -> () {
        let client = Client::new();
        let payload_json = json!(payload);
        let endpoint = format!("{}/v1/appliance/devices/control", GOVEE_ROOT_URL);
        let _response = client
            .put(endpoint)
            .header("Govee-API-Key", &self.govee_api_key)
            .json(&payload_json)
            .send()
            .await
            .unwrap();
    }
}

impl GoveeClient {
    pub async fn get_devices(&self) -> ApiResponseGoveeDevices {
        let client = Client::new();
        let endpoint = format!("{}/v1/devices", GOVEE_ROOT_URL);
        println!("endpoint: {}", endpoint);
        let response = client
            .get(endpoint)
            .header("Govee-API-Key", &self.govee_api_key)
            .send()
            .await
            .unwrap()
            .json::<ApiResponseGoveeDevices>();
        let response_json: ApiResponseGoveeDevices = response.await.unwrap();
        response_json
    }
}

impl GoveeClient {
    pub async fn get_appliances(&self) -> ApiResponseGoveeAppliances {
        let client = Client::new();
        let endpoint = format!("{}/v1/appliance/devices", GOVEE_ROOT_URL);
        let response = client
            .get(endpoint)
            .header("Govee-API-Key", &self.govee_api_key)
            .send()
            .await
            .unwrap()
            .json::<ApiResponseGoveeAppliances>();
        let response_json: ApiResponseGoveeAppliances = response.await.unwrap();
        response_json
    }
}

impl GoveeClient {
    pub async fn get_device_state(&self, device: &str, model: &str) -> ApiResponseGoveeDeviceState {
        let client = Client::new();
        let params = [("device", device), ("model", model)];
        let endpoint = format!("{}/v1/devices/state", GOVEE_ROOT_URL);
        let url = Url::parse_with_params(&endpoint, &params).unwrap();
        let response = client
            .get(url)
            .header("Govee-API-Key", &self.govee_api_key)
            .send()
            .await
            .unwrap()
            .json::<ApiResponseGoveeDeviceState>();
        let response_json: ApiResponseGoveeDeviceState = response.await.unwrap();
        response_json
    }
}
