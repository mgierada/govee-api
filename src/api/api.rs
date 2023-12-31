use reqwest::Error as ReqwestError;
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

/// Control a Govee device using the provided payload.
///
/// This method sends a PUT request to the Govee API's control endpoint in order to control a Govee device.
///
/// # Arguments
///
/// * `payload` - The payload containing control instructions for the device.
///
/// # Returns
///
/// Returns `Ok(())` if the request is successful. Otherwise, returns a `ReqwestError` indicating the failure
impl GoveeClient {
    pub async fn control_device(&self, payload: PayloadBody) -> Result<(), ReqwestError> {
        let client = Client::new();
        let payload_json = json!(payload);
        let endpoint = format!("{}/v1/devices/control", &self.govee_root_url);
        let result = client
            .put(endpoint)
            .header("Govee-API-Key", &self.govee_api_key.to_string())
            .json(&payload_json)
            .send()
            .await;
        match result {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        Ok(())
    }
}

/// Asynchronously controls multiple devices by sending payloads to the Govee API.
///
/// This method takes a vector of `PayloadBody` objects and sends control requests to the Govee API
/// for each payload asynchronously. It uses the provided `self.govee_api_key` and `self.govee_root_url`
/// to construct the API endpoint for each request.
///
/// # Arguments
///
/// - `payloads`: A vector of `PayloadBody` objects representing the control payloads for the devices.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error that occurred during the requests.
///
/// # Examples
///
/// ```rust
/// let govee_client = GoveeClient::new("your_api_key", "https://api.govee.com");
/// let payloads = vec![payload1, payload2];
/// let result = govee_client.bulk_control_devices(payloads).await;
/// match result {
///     Ok(_) => println!("Devices controlled successfully"),
///     Err(err) => eprintln!("Error controlling devices: {:?}", err),
/// }
/// `
impl GoveeClient {
    pub async fn bulk_control_devices(
        &self,
        payloads: Vec<PayloadBody>,
    ) -> Result<(), ReqwestError> {
        let client = Client::new();
        let endpoint = format!("{}/v1/devices/control", &self.govee_root_url);
        let requests = payloads
            .iter()
            .map(|payload| {
                let payload_json = json!(payload);
                let endpoint = endpoint.clone();
                let govee_api_key = self.govee_api_key.to_string();
                let client = client.clone();
                async move {
                    client
                        .put(&endpoint)
                        .header("Govee-API-Key", &govee_api_key)
                        .json(&payload_json)
                        .send()
                        .await
                }
            })
            .collect::<Vec<_>>();
        let results = futures::future::join_all(requests).await;
        for result in results {
            match result {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        }
        Ok(())
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
/// # Returns
///
/// Returns `Ok(())` if the request is successful. Otherwise, returns a `ReqwestError` indicating the failure
impl GoveeClient {
    pub async fn control_appliance(&self, payload: PayloadBody) -> Result<(), ReqwestError> {
        let client = Client::new();
        let payload_json = json!(payload);
        let endpoint = format!("{}/v1/appliance/devices/control", GOVEE_ROOT_URL);
        let result = client
            .put(endpoint)
            .header("Govee-API-Key", &self.govee_api_key)
            .json(&payload_json)
            .send()
            .await;
        match result {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        Ok(())
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
/// # Returns
///
/// Returns `Ok(())` if the request is successful. Otherwise, returns a `ReqwestError` indicating the failure
impl GoveeClient {
    pub async fn get_devices(&self) -> Result<ApiResponseGoveeDevices, ReqwestError> {
        let client = Client::new();
        let endpoint = format!("{}/v1/devices", GOVEE_ROOT_URL);
        let response = client
            .get(endpoint)
            .header("Govee-API-Key", &self.govee_api_key)
            .send()
            .await?;
        let response_json = response.json::<ApiResponseGoveeDevices>().await?;
        Ok(response_json)
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
/// # Returns
///
/// Returns `Ok(())` if the request is successful. Otherwise, returns a `ReqwestError` indicating the failure
impl GoveeClient {
    pub async fn get_appliances(&self) -> Result<ApiResponseGoveeAppliances, ReqwestError> {
        let client = Client::new();
        let endpoint = format!("{}/v1/appliance/devices", GOVEE_ROOT_URL);
        let response = client
            .get(endpoint)
            .header("Govee-API-Key", &self.govee_api_key)
            .send()
            .await?;
        let response_json = response.json::<ApiResponseGoveeAppliances>().await?;
        Ok(response_json)
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
/// # Returns
///
/// Returns `Ok(())` if the request is successful. Otherwise, returns a `ReqwestError` indicating the failure
impl GoveeClient {
    pub async fn get_device_state(
        &self,
        device: &str,
        model: &str,
    ) -> Result<ApiResponseGoveeDeviceState, ReqwestError> {
        let client = Client::new();
        let params = [("device", device), ("model", model)];
        let endpoint = format!("{}/v1/devices/state", GOVEE_ROOT_URL);
        let url = Url::parse_with_params(&endpoint, &params).unwrap();
        let response = client
            .get(url)
            .header("Govee-API-Key", &self.govee_api_key)
            .send()
            .await?;
        let response_json = response
            .json::<ApiResponseGoveeDeviceState>()
            .await
            .unwrap();
        Ok(response_json)
    }
}
