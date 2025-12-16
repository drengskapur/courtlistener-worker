//! Generic proxy handler for CourtListener API endpoints

use worker::*;
use crate::api::ApiClient;
use crate::config::get_api_base_url;
use crate::utils::{json_response, sanitize_error};

/// Generic proxy endpoint - forwards requests to any CourtListener API endpoint
/// Preserves query parameters from the original request
/// 
/// Security: Validates that the path is a valid CourtListener API endpoint
/// Supports GET, POST, PUT, PATCH, DELETE methods
/// Handles both /api/proxy/*path and direct endpoints like /api/docket-alerts
pub async fn proxy_api_request(req: &Request, env: &Env, body: Option<&str>) -> Result<Response> {
    let url = req.url()?;
    let path = url.path();
    let method = req.method();
    
    // Extract the API endpoint path
    let endpoint_path = if path.starts_with("/api/proxy/") {
        // Extract path after /api/proxy/
        let proxy_path = &path["/api/proxy".len()..];
        if proxy_path.is_empty() || proxy_path == "/" {
            return Response::error("Missing proxy path", 400);
        }
        proxy_path.to_string()
    } else if path.starts_with("/api/docket-alerts") {
        // Map /api/docket-alerts to /docket-alerts/
        path.replace("/api/docket-alerts", "/docket-alerts")
    } else if path.starts_with("/api/alerts") {
        // Map /api/alerts to /alerts/
        path.replace("/api/alerts", "/alerts")
    } else {
        return Response::error("Invalid proxy path", 400);
    };
    
    // Security: Validate path to prevent SSRF attacks
    if endpoint_path.contains("..") || endpoint_path.contains("//") || endpoint_path.contains("@") {
        return Response::error("Invalid path: contains dangerous characters", 400);
    }
    
    // Ensure path starts with /
    let endpoint = if endpoint_path.starts_with('/') {
        endpoint_path
    } else {
        format!("/{}", endpoint_path)
    };
    
    // Security: Limit endpoint length
    if endpoint.len() > 500 {
        return Response::error("Path too long", 400);
    }
    
    // Preserve query string
    let full_endpoint = if let Some(query) = url.query() {
        if !query.is_empty() {
            if query.len() > 2000 {
                return Response::error("Query string too long", 400);
            }
            format!("{}?{}", endpoint, query)
        } else {
            endpoint
        }
    } else {
        endpoint
    };
    
    let api_base = get_api_base_url();
    let api_url = format!("{}{}", api_base, full_endpoint);
    
    // Handle different HTTP methods
    match method {
        Method::Get => {
            let data: serde_json::Value = ApiClient::fetch_json(env, &full_endpoint, req).await?;
            json_response(&data)
        }
        Method::Post | Method::Put | Method::Patch => {
            // Body is passed as parameter (read in router handler)
            let body_str = body.unwrap_or("");
            
            // Create request with body
            use worker::wasm_bindgen::JsValue;
            use worker::RequestInit;
            
            let headers = worker::Headers::new();
            headers.set("Accept", "application/json")?;
            headers.set("Content-Type", "application/json")?;
            headers.set("User-Agent", &format!("courtlistener-worker/{}", env!("CARGO_PKG_VERSION")))?;
            
            // Add API token if available
            if let Ok(token) = env.secret("COURTLISTENER_API_TOKEN") {
                headers.set("Authorization", &format!("Token {}", token.to_string()))?;
            }
            
            let init = RequestInit {
                method: method.clone(),
                headers,
                body: if body_str.is_empty() { None } else { Some(JsValue::from_str(body_str)) },
                ..Default::default()
            };
            
            let api_req = Request::new_with_init(&api_url, &init)?;
            let mut resp = Fetch::Request(api_req).send().await?;
            
            let status = resp.status_code();
            let text = resp.text().await?;
            
            if !(200..300).contains(&status) {
                return Err(worker::Error::RustError(format!(
                    "API returned {}: {}",
                    status, sanitize_error(&text)
                )));
            }
            
            let data: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| worker::Error::RustError(format!("Failed to parse response: {}", e)))?;
            json_response(&data)
        }
        Method::Delete => {
            let headers = worker::Headers::new();
            headers.set("Accept", "application/json")?;
            headers.set("User-Agent", &format!("courtlistener-worker/{}", env!("CARGO_PKG_VERSION")))?;
            
            if let Ok(token) = env.secret("COURTLISTENER_API_TOKEN") {
                headers.set("Authorization", &format!("Token {}", token.to_string()))?;
            }
            
            let init = RequestInit {
                method: Method::Delete,
                headers,
                body: None,
                ..Default::default()
            };
            
            let api_req = Request::new_with_init(&api_url, &init)?;
            let mut resp = Fetch::Request(api_req).send().await?;
            
            let status = resp.status_code();
            let text = resp.text().await.unwrap_or_default();
            
            if !(200..300).contains(&status) {
                return Err(worker::Error::RustError(format!(
                    "API returned {}: {}",
                    status, sanitize_error(&text)
                )));
            }
            
            // DELETE may return empty body or JSON
            if text.is_empty() {
                Response::ok("")
            } else {
                let data: serde_json::Value = serde_json::from_str(&text)
                    .map_err(|e| worker::Error::RustError(format!("Failed to parse response: {}", e)))?;
                json_response(&data)
            }
        }
        _ => Response::error("Method not supported", 405),
    }
}

