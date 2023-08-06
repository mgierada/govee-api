#[cfg(test)]
mod tests {
    use mockito;

    use crate::{
        structs::govee::{GoveeCommand, PayloadBody},
        GoveeClient,
    };

    // Mock API key for testing
    const MOCK_API_KEY: &str = "mock-api-key";

    #[tokio::test]
    async fn test_control_device() {
        // Arrange
        let mut server = mockito::Server::new();
        let _mock = server.mock("PUT", "/v1/devices/control")
            .match_header("Govee-API-Key", MOCK_API_KEY)
            .with_status(200)
            .create();

        let test_client = GoveeClient::new(MOCK_API_KEY);

        let command = GoveeCommand {
            name: "turn".to_string(),
            value: "on".to_string(),
        };
        let payload = PayloadBody {
            device: "device_id".to_string(),
            model: "model_id".to_string(),
            cmd: command,
        };

        // Act
        test_client.control_device(payload).await;

        // Assert that the mock expectations were satisfied
        // mock.assert();
    }

    #[tokio::test]
    async fn test_control_appliance() {
        let mut server = mockito::Server::new();
        let govee_api_key = "1234567890";
        let _mock_endpoint = server
            .mock("PUT", "/v1/appliance/devices/control")
            .match_header("Govee-API-Key", MOCK_API_KEY)
            .with_status(200)
            .create();
        let command = GoveeCommand {
            name: "mode".to_string(),
            value: "16".to_string(),
        };
        let payload = PayloadBody {
            device: "device_id".to_string(),
            model: "model_id".to_string(),
            cmd: command,
        };
        // Create the GoveeClient instance
        let govee_client = GoveeClient::new(govee_api_key);
        govee_client.control_appliance(payload).await;
        // mock_endpoint.assert();
    }

    #[tokio::test]
    async fn test_get_devices() {
        let mut server = mockito::Server::new();
        let govee_api_key = "1234567890";
        let _mock_endpoint = server
            .mock("GET", "https://developer-api.govee.com/v1/devices")
            .match_header("Govee-API-Key", MOCK_API_KEY)
            .with_status(200)
            .with_body(
                r#"{
                    "code": 200,
                    "message": "Success",
                    "devices": [
                        {
                            "device": "device_id",
                            "model": "model_id",
                            "deviceName": "device_name",
                            "controllable": true,
                            "retrievable": true,
                            "supportCmds": [
                                "turn",
                                "brightness",
                                "color",
                                "colorTem"
                            ],
                            "properties": {
                                "colorTem": {
                                    "range": {
                                        "min": 2000,
                                        "max": 9001
                                    }
                                }
                            }
                        }
                    ]
                }"#,
            )
            .create();
        let govee_client = GoveeClient::new(govee_api_key);
        govee_client.get_devices().await;
        // mock_endpoint.assert();
    }

    #[tokio::test]
    async fn test_get_appliances() {
        let mut server = mockito::Server::new();
        let govee_api_key = "1234567890";
        let _mock_endpoint = server
            .mock("get", "/v1/appliance/devices")
            .match_header("govee-api-key", govee_api_key)
            .with_status(200)
            .with_body(
                r#"{
                    "code": 200,
                    "message": "Success",
                    "devices": [
                        {
                            "device": "appliance_id",
                            "model": "model_id",
                            "deviceName": "device_name",
                            "controllable": true,
                            "retrievable": true,
                            "supportCmds": [
                                "turn",
                                "mode"
                            ],
                            "properties": {
                                "mode": {
                                    "options": [
                                        {
                                            "name": "Low",
                                            "value": 1
                                        },
                                        {
                                            "name": "Medium",
                                            "value": 2
                                        },
                                        {
                                            "name": "High",
                                            "value": 3
                                        },
                                                                                {
                                            "name": "Sleep",
                                            "value": 4
                                        }
                                    ]
                                }
                            }
                        }
                    ]
                }"#,
            )
            .create();
        let govee_client = GoveeClient::new(govee_api_key);
        govee_client.get_appliances().await;
        // mock_endpoint.assert();
    }
}
