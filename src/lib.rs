pub mod api;
pub mod structs;
pub mod tests;
pub mod utils;

pub const GOVEE_ROOT_URL: &str = "https://developer-api.govee.com";

#[derive(Debug)]
pub struct GoveeClient {
    pub govee_root_url: String,
    pub govee_api_key: String,
}

impl GoveeClient {
    pub fn new(api_key: &str) -> GoveeClient {
        GoveeClient {
            govee_root_url: GOVEE_ROOT_URL.to_string(),
            govee_api_key: api_key.to_string(),
        }
    }
}
