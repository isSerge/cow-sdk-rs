use eyre::{Report, WrapErr};
use log::{debug, error};
use serde::de::DeserializeOwned;

/// Parses a JSON response body into the specified type.
pub fn parse_response_body<T: DeserializeOwned>(body: &str) -> Result<T, Report> {
    debug!("Parsing response body: {}", body);
    serde_json::from_str::<T>(body)
        .map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            e
        })
        .wrap_err("Failed to parse JSON response")
}
