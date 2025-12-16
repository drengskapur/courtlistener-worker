//! Common utilities for integration tests

// Use the same API base URL as the worker (v4 path, but API version is 4.3.0)
pub const API_BASE_URL: &str = "https://www.courtlistener.com/api/rest/v4";

/// Helper to fetch JSON from CourtListener API
pub async fn fetch_json(endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}{}", API_BASE_URL, endpoint);
    let response = reqwest::get(&url).await?;
    let text = response.text().await?;
    Ok(text)
}
