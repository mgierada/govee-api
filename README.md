# govee-api

A blazingly fast thin wrapper around the public Govee API written in Rust 🚀.

All REST methods of the official public Govee API are supported. See the table below for mapping of methods to endpoints.

| is supported | endpoint                          | method              |
| ------------ | --------------------------------- | ------------------- |
| yes          | GET /v1/appliance/devices         | `get_appliances`    |
| yes          | PUT /v1/appliance/devices/control | `control_appliance` |
| yes          | GET /v1/devices                   | `get_devices`       |
| yes          | PUT /v1/devices/control           | `control_device`    |
| yes          | GET /v1/devices/state             | `get_device_state`  |

# Prerequisite

To use the library you need to obtain a Govee Developer API key and set it to `GOVEE_API_KEY` env variable. It is highly suggested to use .env file.

See below a short manual copied directly from the [Govee API documentation](https://app-h5.govee.com/share/community?client=0&postId=124855). Please refer to that documentation in case the info below is not sufficient or it changed over time.

📋 Steps to obtain a Govee Developer API Key

1. Download the Govee Home App
   - iOS: https://apps.apple.com/us/app/govee-home/id1395696823
   - Android: https://play.google.com/store/apps/details?id=com.govee.home
2. Navigate to the My Profile page by clicking on the 👤 icon
3. Click on the ⚙️ icon on the top right corner to get to Settings
4. Click on “Apply for API Key”
5. Fill in the required fields for “Name” and “Reason for application”
   - Possible reasons can include: home automation, 3rd party integration, API Days Tutorial (education & research)
6. Read the Govee Developer API Terms of Service, then click the checkbox to accept
7. Click Submit

# Usage

It is dead simple to use the `govee-api` library.

```rust
// make sure to run this inside an async function
const GOVEE_API_KEY: &str = "GOVEE_API_KEY"; // for the sake of security, please make sure this is read from env variable.
let govee_client = GoveeClient::new(&GOVEE_API_KEY);
// use any of the supported method from the table above
// example for get_devices()
let response: ApiResponseGoveeDevices = govee_client.get_devices().await;
let devices = response.data.unwrap();
```

See this [repo](https://github.com/mgierada/rust_that_light) for an inspiration how to use `govee-api` in various scenarios.
