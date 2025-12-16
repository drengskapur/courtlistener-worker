//! Internal utility functions

use worker::*;

/// Sanitize error messages to prevent sensitive data leaks
pub(crate) fn sanitize_error(text: &str) -> String {
    // Truncate long error messages
    let truncated = if text.len() > 200 { &text[..200] } else { text };
    truncated.to_string()
}

/// Create CORS preflight response for OPTIONS requests
pub(crate) fn cors_preflight_response() -> worker::Result<Response> {
    use crate::config::get_cors_origins;

    let mut response = Response::ok("")?;
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &get_cors_origins())?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    )?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization",
    )?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(response)
}

/// Create a JSON response with CORS headers
pub(crate) fn json_response<T: serde::Serialize>(data: &T) -> worker::Result<Response> {
    json_response_with_cache(data, None, false)
}

/// Create a JSON response with CORS headers and cache headers
/// `endpoint`: Optional endpoint path for determining cache TTL
/// `from_cache`: Whether this response was served from cache
pub(crate) fn json_response_with_cache<T: serde::Serialize>(
    data: &T,
    endpoint: Option<&str>,
    from_cache: bool,
) -> worker::Result<Response> {
    use crate::cache::{add_cache_headers, get_cache_ttl};
    use crate::config::get_cors_origins;

    let mut response = Response::from_json(data)?;
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &get_cors_origins())?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    )?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization",
    )?;
    headers.set("Content-Type", "application/json")?;

    // Add cache headers if endpoint is provided
    if let Some(endpoint) = endpoint {
        let cache_ttl = get_cache_ttl(endpoint);
        add_cache_headers(headers, cache_ttl, from_cache)?;
    }

    Ok(response)
}

/// Generate a request ID for tracing requests
/// Uses the X-Request-ID header if present, otherwise generates a new one
pub(crate) fn get_or_create_request_id(req: &Request) -> String {
    // Check for existing request ID in headers
    if let Ok(Some(id)) = req.headers().get("X-Request-ID") {
        return id;
    }
    
    // Generate a new request ID (simple UUID-like format)
    // Format: timestamp-random (e.g., 1704067200-a1b2c3d4)
    let timestamp = (js_sys::Date::now() as u64) / 1000;
    let random = format!("{:x}", (js_sys::Date::now() as u64) % 0xffffffff);
    format!("{}-{}", timestamp, random)
}

/// Log a structured message with request ID
pub(crate) fn log_with_request_id(request_id: &str, level: &str, message: &str) {
    worker::console_log!(
        "[{}] [{}] {}",
        request_id,
        level,
        message
    );
}

/// Log a structured message with request ID and additional context
pub(crate) fn log_with_context(request_id: &str, level: &str, message: &str, context: &serde_json::Value) {
    worker::console_log!(
        "[{}] [{}] {} | Context: {}",
        request_id,
        level,
        message,
        serde_json::to_string(context).unwrap_or_else(|_| "{}".to_string())
    );
}
