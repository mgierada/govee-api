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
//

/// Controls a Govee device using the provided payload.
///
/// This method sends a PUT request to the Govee API to control a device
/// with the specified payload. The payload should be in the form of a PayloadBody struct.
///
/// # Arguments
///
/// * `payload` - The payload containing the control instructions for the device.
///
/// # Panics
///
/// This method will panic if the PUT request encounters an error.
impl GoveeClient {
    pub async fn control_device(&self, payload: PayloadBody) -> () {
        let client = Client::new();
        let payload_json = json!(payload);
        let endpoint = format!("{}/v1/devices/control", &self.govee_root_url);
        let _response = client
            .put(endpoint)
            .header("Govee-API-Key", &self.govee_api_key.to_string())
            .json(&payload_json)
            .send()
            .await
            .unwrap();
    }
}

/// Controls a Govee appliance using the provided payload.
///
/// This method sends a PUT request to the Govee API to control an appliance
/// with the specified payload. The payload should be in the form of a PayloadBody struct.
///
/// # Arguments
///
/// * `payload` - The payload containing the control instructions for the appliance.
///
/// # Panics
///
/// This method will panic if the PUT request encounters an error.
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

/// Retrieves a list of Govee devices.
///
/// This method sends a GET request to the Govee API to retrieve a list of devices
/// associated with the Govee account.
///
/// # Returns
///
/// An `ApiResponseGoveeDevices` containing information about the devices.
///
/// # Panics
///
/// This method will panic if the GET request encounters an error.
impl GoveeClient {
    pub async fn get_devices(&self) -> ApiResponseGoveeDevices {
        let client = Client::new();
        let endpoint = format!("{}/v1/devices", GOVEE_ROOT_URL);
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

/// Retrieves a list of Govee appliances.
///
/// This method sends a GET request to the Govee API to retrieve a list of appliances
/// associated with the Govee account.
///
/// # Returns
///
/// An `ApiResponseGoveeAppliances` containing information about the appliances.
///
/// # Panics
///
/// This method will panic if the GET request encounters an error.
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

/// Retrieves the state of a Govee device.
///
/// This method sends a GET request to the Govee API to retrieve the state of a specific device.
///
/// # Arguments
///
/// * `device` - The device ID or name.
/// * `model` - The model of the device.
///
/// # Returns
///
/// An `ApiResponseGoveeDeviceState` containing the current state of the device.
///
/// # Panics
///
/// This method will panic if the GET request encounters an error.
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
