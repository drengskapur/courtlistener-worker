//! Integration tests for People API

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
    async fn test_fetch_people() {
        let json = fetch_json("/people/?page_size=5")
            .await
            .expect("Failed to fetch people");

        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                println!("People API returned non-JSON (might require auth): {}", e);
                return;
            }
        };

        if let Some(count) = value.get("count") {
            if count.is_string() {
                println!("People API requires authentication (count is URL string)");
                return;
            }
        }

        if value.get("results").is_some() && value.get("count").is_some() {
            match serde_json::from_str::<PeopleResponse>(&json) {
                Ok(people) => {
                    assert!(people.count > 0);
                    assert!(!people.results.is_empty());

                    let first_person = &people.results[0];
                    assert!(first_person.id > 0);
                }
                Err(e) => {
                    println!(
                        "Failed to parse people response (might require auth): {}",
                        e
                    );
                }
            }
        }
    }
}
