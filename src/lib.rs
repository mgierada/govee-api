pub mod api;
pub mod structs;
pub mod tests;
pub mod utils;

pub const GOVEE_ROOT_URL: &str = "https://developer-api.govee.com";

/// A client for interacting with Govee devices through their API.
///
/// This struct provides methods to control Govee devices using the Govee API.
/// It requires a valid Govee API key and root URL to initialize.
pub struct GoveeClient {
    govee_api_key: String,  // The API key for authenticating requests.
    govee_root_url: String, // The root URL of the Govee API.
}

impl GoveeClient {
    /// Creates a new instance of the GoveeClient with the specified API key and root URL.
    ///
    /// # Arguments
    ///
    /// * `govee_api_key` - A valid Govee API key for authentication.
    /// * `govee_root_url` - The root URL of the Govee API.
    ///
    /// # Returns
    ///
    /// A new GoveeClient instance.
    pub fn new(api_key: &str) -> GoveeClient {
        GoveeClient {
            govee_api_key: api_key.to_string(),
            govee_root_url: GOVEE_ROOT_URL.to_string(),
        }
    }
}
