//! High-level API client for CourtListener

use crate::api::request::create_api_request;
use crate::cache::{
    generate_cache_key, get_cache_ttl, get_cached_with_expiry, set_cached_with_expiry,
};
use worker::*;

/// API client for fetching data from CourtListener API
pub struct ApiClient;

impl ApiClient {
    /// Fetch and parse JSON from CourtListener API with caching
    /// Preserves query parameters from the original request
    pub async fn fetch_json<T: serde::de::DeserializeOwned>(
        env: &Env,
        endpoint: &str,
        req: &Request,
    ) -> worker::Result<T> {
        Self::fetch_json_internal(env, endpoint, req).await
    }

    /// Fetch and parse JSON from CourtListener API with caching and validation
    /// Requires the type to implement `validator::Validate`
    /// Preserves query parameters from the original request
    pub async fn fetch_json_validated<T>(env: &Env, endpoint: &str, req: &Request) -> worker::Result<T>
    where
        T: serde::de::DeserializeOwned + validator::Validate,
    {
        let parsed = Self::fetch_json_internal(env, endpoint, req).await?;
        Self::validate_parsed(&parsed)?;
        Ok(parsed)
    }

    /// Internal method for fetching JSON
    async fn fetch_json_internal<T: serde::de::DeserializeOwned>(
        env: &Env,
        endpoint: &str,
        req: &Request,
    ) -> worker::Result<T> {
        // Generate cache key
        let url = req.url().ok();
        let query = url.as_ref().and_then(|u| u.query());
        let cache_key = generate_cache_key(endpoint, query);
        let cache_ttl = get_cache_ttl(endpoint);

        // Try to get from cache first
        if let Some(cached_text) = get_cached_with_expiry(env, &cache_key).await {
            if let Ok(parsed) = serde_json::from_str::<T>(&cached_text) {
                return Ok(parsed);
            }
        }

        // Cache miss or invalid cache, fetch from API
        let api_req = create_api_request(env, endpoint, req)?;
        let mut resp = Fetch::Request(api_req).send().await?;

        // Check HTTP status
        let status = resp.status_code();
        if !(200..300).contains(&status) {
            let text = resp.text().await.unwrap_or_default();
            return Err(worker::Error::RustError(format!(
                "API returned {}: {}",
                status, text
            )));
        }

        let text = resp.text().await?;

        // Parse with our types
        let parsed: T = serde_json::from_str(&text).map_err(|e| {
            // Truncate response text in error to avoid leaking sensitive data
            let _truncated = if text.len() > 200 {
                format!("{}...", &text[..200])
            } else {
                text.clone()
            };
            worker::Error::RustError(format!("Failed to parse JSON: {} (response truncated)", e))
        })?;

        // Cache the response
        set_cached_with_expiry(env, &cache_key, &text, cache_ttl).await;

        Ok(parsed)
    }

    /// Validate parsed data
    fn validate_parsed<T: validator::Validate>(value: &T) -> worker::Result<()> {
        value.validate().map_err(|errors| {
            let error_msg = errors
                .field_errors()
                .iter()
                .map(|(field, errors)| {
                    let error_details: Vec<String> = errors
                        .iter()
                        .map(|e| format!("{:?}", e.code))
                        .collect();
                    format!("{}: {}", field, error_details.join(", "))
                })
                .collect::<Vec<String>>()
                .join("; ");
            
            worker::Error::RustError(format!("Validation failed: {}", error_msg))
        })
    }
}
