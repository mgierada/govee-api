#[cfg(test)]
mod tests {
    use mockito;

    use crate::{
        api::api::{control_device, get_device_state, get_devices, control_appliance, get_appliances},
        structs::govee::{GoveeCommand, PayloadBody},
    };

    #[tokio::test]
    async fn test_control_device() {
        let mut server = mockito::Server::new();
        let govee_root_url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
            .mock("put", "/v1/devices/control")
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
        control_device(&govee_root_url, &govee_api_key, payload).await;
        mock_endpoint.assert();
    }
    
    #[tokio::test]
    async fn test_control_appliance() {
        let mut server = mockito::Server::new();
        let govee_root_url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
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
        control_appliance(&govee_root_url, &govee_api_key, payload).await;
        mock_endpoint.assert();
    }

    #[tokio::test]
    async fn test_get_devices() {
        let mut server = mockito::Server::new();
        let govee_root_url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
            .mock("get", "/v1/devices")
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
        get_devices(&govee_root_url, &govee_api_key).await;
        mock_endpoint.assert();
    }
    
    #[tokio::test]
    async fn test_get_appliances() {
        let mut server = mockito::Server::new();
        let govee_root_url = server.url();
        let govee_api_key = "1234567890";
        let mock_endpoint = server
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
        get_appliances(&govee_root_url, &govee_api_key).await;
        mock_endpoint.assert();
    }

    #[tokio::test]
    async fn test_get_device_state() {
        let mut server = mockito::Server::new();
        let gooee_root_url = server.url();
        let govee_api_key = "1234567890";
        let device = "device_name";
        let model = "model_name";
        let mock_endpoint = server
            .mock("get", "/v1/devices/state")
            .match_header("govee-api-key", govee_api_key)
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("device".into(), device.into()),
                mockito::Matcher::UrlEncoded("model".into(), model.into()),
            ]))
            .with_status(200)
            .with_body(
                r#"{
                    "code": 200,
                    "message": "Success",
                    "device": "device_id",
                    "model": "model_id",
                    "properties": [
                        {
                            "online": true
                        },
                        {
                            "powerState": "on"
                        },
                        {
                            "brightness": 100
                        },
                        {
                            "colorTemInKelvin": 2000
                        },
                        {
                            "colorTem": 2000
                        }
                    ]
                }"#,
            )
            .create();
        get_device_state(&gooee_root_url, &govee_api_key, device, model).await;
        mock_endpoint.assert();
    }
}
