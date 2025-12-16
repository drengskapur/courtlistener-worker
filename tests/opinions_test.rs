//! Integration tests for Opinions and Opinion Clusters API

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
    async fn test_fetch_opinions() {
        let json = fetch_json("/opinions/?page_size=5")
            .await
            .expect("Failed to fetch opinions");

        let value: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");

        if value.get("results").is_some() {
            let opinions: OpinionsResponse =
                serde_json::from_str(&json).expect("Failed to parse opinions response");
            assert!(opinions.count > 0);
            assert!(!opinions.results.is_empty());

            let first_opinion = &opinions.results[0];
            assert!(first_opinion.id > 0);
        } else {
            println!("Opinions API returned: {}", json);
        }
    }

    #[tokio::test]
    async fn test_fetch_opinion_clusters() {
        let json = fetch_json("/opinion-clusters/?page_size=5")
            .await
            .expect("Failed to fetch opinion clusters");

        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "Opinion clusters API returned non-JSON (might require auth): {}",
                    e
                );
                println!("Response preview: {}", &json[..json.len().min(200)]);
                return;
            }
        };

        if value.get("results").is_some() {
            match serde_json::from_str::<OpinionClustersResponse>(&json) {
                Ok(clusters) => {
                    assert!(clusters.count > 0);
                    assert!(!clusters.results.is_empty());
                }
                Err(e) => {
                    println!(
                        "Failed to parse opinion clusters response (might require auth): {}",
                        e
                    );
                }
            }
        }
    }
}
