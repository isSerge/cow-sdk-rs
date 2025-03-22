use eyre::{Report, WrapErr};
use serde::de::DeserializeOwned;

/// Parses a JSON response body into the specified type.
pub fn parse_response<T: DeserializeOwned>(body: &str) -> Result<T, Report> {
    serde_json::from_str::<T>(body).wrap_err("Failed to parse JSON response")
}
