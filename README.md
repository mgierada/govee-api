# govee

A blazingly fast thin wrapper around the public Govee API written in Rust 🚀.

THIS IS PRE ALPHA VERSION!

| is supported | endpoint                          | method              |
| ------------ | --------------------------------- | ------------------- |
| yes          | GET /v1/appliance/devices         | `get_appliances`    |
| yes          | PUT /v1/appliance/devices/control | `control_appliance` |
| yes          | GET /v1/devices                   | `get_devices`       |
| yes          | PUT /v1/devices/control           | `control_device`    |
| yes          | GET /v1/devices/state             | `get_device_state`  |
