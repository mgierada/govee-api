use serde::de;
// ------------------------
// Handling Govee Issues
// ------------------------
//

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    // If the incoming value is a string 'true' or 'false', return true or false
    // If the incoming value is a boolean, return the boolean
    match serde::Deserialize::deserialize(deserializer)? {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(s) if s == "true" => Ok(true),
        serde_json::Value::String(s) if s == "false" => Ok(false),
        _ => Err(serde::de::Error::custom(
            "Expected a boolean or 'true'/'false' string",
        )),
    }
}
