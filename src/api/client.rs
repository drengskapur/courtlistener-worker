//! High-level API client for CourtListener

use worker::*;
use crate::cache::{generate_cache_key, get_cached_with_expiry, set_cached_with_expiry, get_cache_ttl};
use crate::api::request::create_api_request;

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
        
        // Parse and validate with our types
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
}

