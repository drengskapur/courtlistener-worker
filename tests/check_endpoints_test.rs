//! Integration tests for the /check-endpoints endpoint

#[cfg(not(target_arch = "wasm32"))]
mod tests {
    // Note: These tests require the worker to be running locally
    // Run with: npx wrangler dev
    // Then set WORKER_URL environment variable or use default localhost:8787
    fn get_worker_url() -> String {
        std::env::var("WORKER_URL")
            .unwrap_or_else(|_| "http://localhost:8787".to_string())
    }

    /// Check if the worker is available, skip test if not
    async fn check_worker_available() -> bool {
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        reqwest::get(&url).await.is_ok()
    }

    #[tokio::test]
    async fn test_check_endpoints_structure() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        // Verify top-level structure
        assert!(json.get("api_endpoints").is_some(), "Missing 'api_endpoints' field");
        assert!(json.get("our_endpoints").is_some(), "Missing 'our_endpoints' field");
        assert!(json.get("missing").is_some(), "Missing 'missing' field");
        assert!(json.get("extra").is_some(), "Missing 'extra' field");
        assert!(json.get("coverage").is_some(), "Missing 'coverage' field");
    }

    #[tokio::test]
    async fn test_check_endpoints_api_endpoints() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        let api_endpoints = json.get("api_endpoints").expect("Missing api_endpoints");
        assert!(api_endpoints.get("total").is_some(), "Missing 'total' in api_endpoints");
        assert!(api_endpoints.get("list").is_some(), "Missing 'list' in api_endpoints");
        
        let total = api_endpoints.get("total").and_then(|v| v.as_u64()).expect("total should be a number");
        assert!(total > 0, "API should have at least one endpoint");
        
        let list = api_endpoints.get("list").and_then(|v| v.as_array()).expect("list should be an array");
        assert_eq!(list.len() as u64, total, "List length should match total");
        
        // Verify common endpoints exist
        let endpoints: Vec<String> = list.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        
        assert!(endpoints.contains(&"courts".to_string()), "Should include 'courts' endpoint");
        assert!(endpoints.contains(&"opinions".to_string()), "Should include 'opinions' endpoint");
        assert!(endpoints.contains(&"people".to_string()), "Should include 'people' endpoint");
    }

    #[tokio::test]
    async fn test_check_endpoints_our_endpoints() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        let our_endpoints = json.get("our_endpoints").expect("Missing our_endpoints");
        assert!(our_endpoints.get("total").is_some(), "Missing 'total' in our_endpoints");
        assert!(our_endpoints.get("list").is_some(), "Missing 'list' in our_endpoints");
        
        let total = our_endpoints.get("total").and_then(|v| v.as_u64()).expect("total should be a number");
        assert!(total > 0, "We should support at least one endpoint");
        
        let list = our_endpoints.get("list").and_then(|v| v.as_array()).expect("list should be an array");
        assert_eq!(list.len() as u64, total, "List length should match total");
        
        // Verify we support expected endpoints
        let endpoints: Vec<String> = list.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        
        assert!(endpoints.contains(&"courts".to_string()), "Should support 'courts'");
        assert!(endpoints.contains(&"opinions".to_string()), "Should support 'opinions'");
        assert!(endpoints.contains(&"people".to_string()), "Should support 'people'");
        assert!(endpoints.contains(&"dockets".to_string()), "Should support 'dockets'");
        assert!(endpoints.contains(&"search".to_string()), "Should support 'search'");
    }

    #[tokio::test]
    async fn test_check_endpoints_missing() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        let missing = json.get("missing").expect("Missing 'missing' field");
        assert!(missing.get("count").is_some(), "Missing 'count' in missing");
        assert!(missing.get("list").is_some(), "Missing 'list' in missing");
        assert!(missing.get("note").is_some(), "Missing 'note' in missing");
        
        let count = missing.get("count").and_then(|v| v.as_u64()).expect("count should be a number");
        let list = missing.get("list").and_then(|v| v.as_array()).expect("list should be an array");
        assert_eq!(list.len() as u64, count, "List length should match count");
    }

    #[tokio::test]
    async fn test_check_endpoints_extra() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        let extra = json.get("extra").expect("Missing 'extra' field");
        assert!(extra.get("count").is_some(), "Missing 'count' in extra");
        assert!(extra.get("list").is_some(), "Missing 'list' in extra");
        
        let count = extra.get("count").and_then(|v| v.as_u64()).expect("count should be a number");
        let list = extra.get("list").and_then(|v| v.as_array()).expect("list should be an array");
        assert_eq!(list.len() as u64, count, "List length should match count");
    }

    #[tokio::test]
    async fn test_check_endpoints_coverage() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        let coverage = json.get("coverage").expect("Missing 'coverage' field");
        assert!(coverage.get("percentage").is_some(), "Missing 'percentage' in coverage");
        assert!(coverage.get("all_covered").is_some(), "Missing 'all_covered' in coverage");
        
        let percentage = coverage.get("percentage").and_then(|v| v.as_f64()).expect("percentage should be a number");
        assert!(percentage >= 0.0 && percentage <= 100.0, "Percentage should be between 0 and 100");
        
        let all_covered = coverage.get("all_covered").and_then(|v| v.as_bool()).expect("all_covered should be a boolean");
        
        // If all_covered is true, percentage should be 100.0
        if all_covered {
            assert_eq!(percentage, 100.0, "If all_covered is true, percentage should be 100.0");
        }
    }

    #[tokio::test]
    async fn test_check_endpoints_coverage_consistency() {
        if !check_worker_available().await {
            eprintln!("Skipping test: Worker not available at {}", get_worker_url());
            return;
        }
        let worker_url = get_worker_url();
        let url = format!("{}/check-endpoints", worker_url);
        let response = reqwest::get(&url).await.expect("Failed to fetch check-endpoints");
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        
        let api_total = json.get("api_endpoints")
            .and_then(|v| v.get("total"))
            .and_then(|v| v.as_u64())
            .expect("Should have api_endpoints.total");
        
        let our_total = json.get("our_endpoints")
            .and_then(|v| v.get("total"))
            .and_then(|v| v.as_u64())
            .expect("Should have our_endpoints.total");
        
        let missing_count = json.get("missing")
            .and_then(|v| v.get("count"))
            .and_then(|v| v.as_u64())
            .expect("Should have missing.count");
        
        let coverage_percentage = json.get("coverage")
            .and_then(|v| v.get("percentage"))
            .and_then(|v| v.as_f64())
            .expect("Should have coverage.percentage");
        
        // Verify coverage calculation: (our_total / api_total) * 100
        if api_total > 0 {
            let expected_percentage = (our_total as f64 / api_total as f64) * 100.0;
            assert!((coverage_percentage - expected_percentage).abs() < 0.01, 
                "Coverage percentage should match calculation: expected {}, got {}", 
                expected_percentage, coverage_percentage);
        }
        
        // Verify: api_total = our_total + missing_count (approximately, since we might have extras)
        // Actually, it's: api_endpoints = our_endpoints ∪ missing_endpoints
        // So: api_total >= our_total - extra_count + missing_count
        let extra_count = json.get("extra")
            .and_then(|v| v.get("count"))
            .and_then(|v| v.as_u64())
            .expect("Should have extra.count");
        
        // The relationship: api_total should equal our_unique + missing
        // where our_unique = our_total - extra_count (endpoints we have that aren't in API)
        let our_unique = our_total.saturating_sub(extra_count);
        assert_eq!(api_total, our_unique + missing_count,
            "API total should equal our unique endpoints plus missing: {} = {} + {}",
            api_total, our_unique, missing_count);
    }
}
