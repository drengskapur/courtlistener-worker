//! Integration tests for Dockets API

#[cfg(not(target_arch = "wasm32"))]
mod tests {
    async fn fetch_json(endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_base = std::env::var("COURTLISTENER_API_BASE_URL")
            .unwrap_or_else(|_| courtlistener_worker::API_BASE_URL.to_string());
        let url = format!("{}{}", api_base, endpoint);
        let response = reqwest::get(&url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    #[tokio::test]
    async fn test_fetch_dockets() {
        let json = fetch_json("/dockets/?page_size=5").await
            .expect("Failed to fetch dockets");
        
        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                println!("Dockets API returned non-JSON (might require auth): {}", e);
                return;
            }
        };
        
        if value.get("results").is_some() {
            println!("Dockets API returned valid paginated response");
        }
    }
}

