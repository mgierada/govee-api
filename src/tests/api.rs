#[cfg(test)]
mod tests {
    use mockito;

    use crate::{
        structs::govee::{GoveeCommand, PayloadBody},
        GoveeClient,
    };

    #[tokio::test]
    async fn test_control_device() {
        let mut server = mockito::Server::new();
        let govee_api_key = "1234567890";
        let _mock_endpoint = server
            .mock("put", "https://developer-api.govee.com/v1/devices/control")
            .match_header("govee-api-key", govee_api_key)
            .with_status(200)
            .create();
        let command = GoveeCommand {
            name: "turn".to_string(),
            value: "on".to_string(),
        };
        let payload = PayloadBody {
            device: "device_id".to_string(),
            model: "model_id".to_string(),
            cmd: command,
        };
        // Create the GoveeClient instance
        let govee_client = GoveeClient::new(govee_api_key);
        println!("govee_client: {:?}", govee_client);
        govee_client.control_device(payload).await;
    }

    #[tokio::test]
    async fn test_control_appliance() {
        let mut server = mockito::Server::new();
        let govee_api_key = "1234567890";
        let _mock_endpoint = server
            .mock("put", "/v1/appliance/devices/control")
            .match_header("govee-api-key", govee_api_key)
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
            .mock("get", "https://developer-api.govee.com/v1/devices")
            .match_header("govee-api-key", govee_api_key)
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

    #[cfg(test)]
    mod tests {
        use crate::structs::govee::ApiResponseGoveeDeviceState;

        use super::*;
        use mockito::{mock, Matcher};

        #[tokio::test]
        async fn test_get_device_state() {
            // Set up mock server
            let mock_response = ApiResponseGoveeDeviceState {
            // Define the expected response here
        };
            let mock_server = mock("GET", "/v1/devices/state")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("device".into(), "test_device".into()),
                    Matcher::UrlEncoded("model".into(), "test_model".into()),
                ]))
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(&mock_response).unwrap())
                .create();

            // Set up GoveeClient with mock server URL
            let govee_client = GoveeClient {
                govee_api_key: "test_api_key".into(),
                govee_root_url: mockito::server_url(),
            };

            // Call the method being tested
            let result = govee_client
                .get_device_state("test_device", "test_model")
                .await;

            // Assert that the result matches the expected response
            assert_eq!(result, mock_response);

            // Ensure that the mock server was called as expected
            mock_server.assert();
        }
    }
}

//     #[tokio::test]
//     async fn test_get_device_state() {
//         let mut server = mockito::Server::new();
//         let govee_api_key = "1234567890";
//         let device = "device_name";
//         let model = "model_name";
//         let _mock_endpoint = server
//             .mock("get", "/v1/devices/state")
//             .match_header("govee-api-key", govee_api_key)
//             .match_query(mockito::Matcher::AllOf(vec![
//                 mockito::Matcher::UrlEncoded("device".into(), device.into()),
//                 mockito::Matcher::UrlEncoded("model".into(), model.into()),
//             ]))
//             .with_status(200)
//             .with_body(
//                 r#"{
//                     "code": 200,
//                     "message": "Success",
//                     "device": "device_id",
//                     "model": "model_id",
//                     "properties": [
//                         {
//                             "online": true
//                         },
//                         {
//                             "powerState": "on"
//                         },
//                         {
//                             "brightness": 100
//                         },
//                         {
//                             "colorTemInKelvin": 2000
//                         },
//                         {
//                             "colorTem": 2000
//                         }
//                     ]
//                 }"#,
//             )
//             .create();
//         let govee_client = GoveeClient::new(govee_api_key);
//         govee_client.get_device_state(device, model).await;
//         // mock_endpoint.assert();
//     }
// }
