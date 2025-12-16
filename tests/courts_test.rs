//! Integration tests for Courts API

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
    async fn test_fetch_courts() {
        let json = fetch_json("/courts/")
            .await
            .expect("Failed to fetch courts");

        let courts: CourtsResponse =
            serde_json::from_str(&json).expect("Failed to parse courts response");

        assert!(courts.count > 0, "Should have at least one court");
        assert!(!courts.results.is_empty(), "Results should not be empty");

        let first_court = &courts.results[0];
        assert!(!first_court.id.is_empty(), "Court ID should not be empty");
    }

    #[tokio::test]
    async fn test_fetch_specific_court() {
        let json = fetch_json("/courts/scotus/")
            .await
            .expect("Failed to fetch SCOTUS");

        let court: ApiCourt = serde_json::from_str(&json).expect("Failed to parse court response");

        assert_eq!(court.id, "scotus");
        assert!(court.name.is_some() || court.full_name.is_some());
    }
}
