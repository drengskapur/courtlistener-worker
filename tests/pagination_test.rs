//! Integration tests for pagination structure

#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use courtlistener_worker::*;
    
    async fn fetch_json(endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_base = std::env::var("COURTLISTENER_API_BASE_URL")
            .unwrap_or_else(|_| courtlistener_worker::API_BASE_URL.to_string());
        let url = format!("{}{}", api_base, endpoint);
        let response = reqwest::get(&url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    #[tokio::test]
    async fn test_paginated_response_structure() {
        let json = fetch_json("/courts/").await.expect("Failed to fetch");
        
        let response: PaginatedResponse<serde_json::Value> = serde_json::from_str(&json)
            .expect("Failed to parse paginated response");
        
        assert!(response.count > 0);
        // next and previous can be null, which is fine
    }
}

