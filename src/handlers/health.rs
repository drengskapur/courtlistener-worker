//! Health check and status endpoints

use crate::config::{API_VERSION, API_BASE_URL};
use worker::*;

/// Enhanced health check endpoint
/// Returns JSON with worker status, API version, and cache availability
pub async fn health_check(env: &Env) -> Result<Response> {
    let mut status = serde_json::json!({
        "status": "healthy",
        "timestamp": js_sys::Date::now() as u64,
        "api_version": API_VERSION,
        "api_base_url": API_BASE_URL,
    });

    // Check KV cache availability
    let kv_status = match env.kv("CACHE") {
        Ok(_) => "available",
        Err(_) => "unavailable",
    };
    status["cache"] = serde_json::json!({
        "kv": kv_status
    });

    // Check if API token is configured (without exposing it)
    let has_token = env.secret("COURTLISTENER_API_TOKEN").is_ok();
    status["auth"] = serde_json::json!({
        "api_token_configured": has_token
    });

    let mut response = Response::from_json(&status)?;
    let headers = response.headers_mut();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "no-cache, no-store, must-revalidate")?;
    Ok(response)
}

