//! Low-level request building for CourtListener API

use worker::*;
use crate::config::get_api_base_url;

/// Create an authenticated request to CourtListener API
/// Preserves query parameters from the original request
pub fn create_api_request(env: &Env, endpoint: &str, req: &Request) -> worker::Result<Request> {
    let api_base = get_api_base_url();
    let url = req.url()?;
    
    // Build endpoint with query parameters
    let mut full_endpoint = endpoint.to_string();
    if let Some(query) = url.query() {
        if !query.is_empty() {
            full_endpoint = format!("{}?{}", full_endpoint, query);
        }
    }
    
    let api_url = format!("{}{}", api_base, full_endpoint);
    let mut api_req = Request::new(&api_url, Method::Get)?;
    
    // Set headers - respect Accept header from original request, default to JSON
    let accept = match req.headers().get("Accept") {
        Ok(Some(val)) => val,
        _ => "application/json".to_string(),
    };
    api_req.headers_mut()?.set("Accept", &accept)?;
    api_req.headers_mut()?.set("User-Agent", &format!("courtlistener-worker/{}", env!("CARGO_PKG_VERSION")))?;
    
    // Add API token if available
    if let Ok(token) = env.secret("COURTLISTENER_API_TOKEN") {
        api_req.headers_mut()?.set("Authorization", &format!("Token {}", token.to_string()))?;
    }
    
    Ok(api_req)
}

