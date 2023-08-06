# govee-api

A blazingly fast thin wrapper around the public Govee API written in Rust 🚀.

THIS IS PRE ALPHA VERSION!

| is supported | endpoint                          | method              |
| ------------ | --------------------------------- | ------------------- |
| yes          | GET /v1/appliance/devices         | `get_appliances`    |
| yes          | PUT /v1/appliance/devices/control | `control_appliance` |
| yes          | GET /v1/devices                   | `get_devices`       |
| yes          | PUT /v1/devices/control           | `control_device`    |
| yes          | GET /v1/devices/state             | `get_device_state`  |

# Usage

It is dead simple to use the govee-api library.

```rust
// make sure to run this inside an async function
const GOVEE_API_KEY: &str = "GOVEE_API_KEY";
let govee_client = GoveeClient::new(&GOVEE_API_KEY);
// use any of the supported method from the table above
// example for get_devices()
let response: ApiResponseGoveeDevices = govee_client.get_devices().await;
let devices = response.data.unwrap();
```
