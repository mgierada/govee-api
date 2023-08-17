use serde::de;
// ------------------------
// Handling Govee Issues
// ------------------------
//

/// Deserialize a boolean value from a given deserializer.
///
/// This function deserializes a boolean value from the provided deserializer. It handles
/// both boolean values and string representations of 'true' and 'false'. If the incoming
/// value is a boolean, it is returned directly. If the incoming value is a string 'true',
/// it returns true; if it's a string 'false', it returns false. For any other input, an
/// error is returned.
///
/// # Arguments
///
/// * `deserializer` - The deserializer implementing the `de::Deserializer` trait.
///
/// # Returns
///
/// Returns a `Result` containing the deserialized boolean value if successful, or an
/// error of type `D::Error` if deserialization fails or the input is not a valid boolean
/// representation.
///
/// # Examples
///
/// ```rust
/// use serde::de::Deserialize;
///
/// let json = r#""true""#;
/// let deserializer = serde_json::Deserializer::from_str(json);
/// let result: Result<bool, _> = deserialize_bool(deserializer);
/// assert_eq!(result, Ok(true));
/// ```
///
/// ```rust
/// use serde::de::Deserialize;
///
/// let json = r#"false"#;
/// let deserializer = serde_json::Deserializer::from_str(json);
/// let result: Result<bool, _> = deserialize_bool(deserializer);
/// assert_eq!(result, Ok(false));
/// ```
///
/// ```rust
/// use serde::de::Deserialize;
///
/// let json = r#"42"#; // This input is not a valid boolean representation
/// let deserializer = serde_json::Deserializer::from_str(json);
/// let result: Result<bool, _> = deserialize_bool(deserializer);
/// assert!(result.is_err());
/// ```
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
