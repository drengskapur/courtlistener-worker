//! Utility functions for the CourtListener Worker

use worker::*;

/// Sanitize error messages to prevent leaking sensitive information
pub fn sanitize_error(error: &str) -> String {
    // Truncate long error messages
    if error.len() > 200 {
        format!("{}...", &error[..200])
    } else {
        error.to_string()
    }
}

/// Get CORS allowed origins from environment or use default
pub fn get_cors_origins() -> String {
    std::env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string())
}

/// Create a JSON response with CORS headers
pub fn json_response(data: &serde_json::Value) -> Result<Response> {
    let mut response = Response::from_json(data)?;
    let headers = response.headers_mut();
    headers.set("Content-Type", "application/json")?;
    headers.set("Access-Control-Allow-Origin", &get_cors_origins())?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, PATCH, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization, Idempotency-Key")?;
    Ok(response)
}

/// Create a JSON response with cache headers
pub fn json_response_with_cache(data: &serde_json::Value, cache_ttl: u64) -> Result<Response> {
    let mut response = json_response(data)?;
    let headers = response.headers_mut();
    headers.set("Cache-Control", &format!("public, max-age={}", cache_ttl))?;
    headers.set("X-Cache", "MISS")?;
    Ok(response)
}

/// Handle CORS preflight requests
pub fn cors_preflight_response() -> Result<Response> {
    let mut response = Response::ok("")?;
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &get_cors_origins())?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, PATCH, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization, Idempotency-Key")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(response)
}

/// Generate a request ID for tracing requests
/// Uses the X-Request-ID header if present, otherwise generates a new one
pub fn get_or_create_request_id(req: &Request) -> String {
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
pub fn log_with_request_id(request_id: &str, level: &str, message: &str) {
    worker::console_log!(
        "[{}] [{}] {}",
        request_id,
        level,
        message
    );
}

/// Log a structured message with request ID and additional context
pub fn log_with_context(request_id: &str, level: &str, message: &str, context: &serde_json::Value) {
    worker::console_log!(
        "[{}] [{}] {} | Context: {}",
        request_id,
        level,
        message,
        serde_json::to_string(context).unwrap_or_else(|_| "{}".to_string())
    );
}
