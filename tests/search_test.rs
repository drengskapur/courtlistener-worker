//! Integration tests for Search API

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
    async fn test_search_result_deserialization() {
        let json = fetch_json("/search/?q=constitution&page_size=3")
            .await
            .expect("Failed to fetch search results");

        let _value: serde_json::Value =
            serde_json::from_str(&json).expect("Failed to parse search response");
    }
}
