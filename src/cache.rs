//! Caching utilities for the CourtListener Worker
//!
//! Provides:
//! - KV-based distributed caching
//! - HTTP cache header management
//! - Cache key generation
//! - TTL management

use serde::{Deserialize, Serialize};
use worker::*;

/// Cache configuration
#[allow(dead_code)]
pub struct CacheConfig {
    /// Default TTL in seconds (10 minutes)
    pub default_ttl: u64,
    /// Maximum TTL in seconds (1 hour)
    pub max_ttl: u64,
    /// Whether to use KV cache
    pub use_kv: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: 600, // 10 minutes
            max_ttl: 3600,    // 1 hour
            use_kv: true,
        }
    }
}

/// Generate a cache key from endpoint and query parameters
pub fn generate_cache_key(endpoint: &str, query: Option<&str>) -> String {
    let base_key = endpoint.trim_start_matches('/');
    if let Some(q) = query {
        if !q.is_empty() {
            // Hash long query strings to avoid KV key length limits
            if q.len() > 200 {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                q.hash(&mut hasher);
                format!("{}:q:{}", base_key, hasher.finish())
            } else {
                format!("{}:{}", base_key, q)
            }
        } else {
            base_key.to_string()
        }
    } else {
        base_key.to_string()
    }
}

/// Get cached response from KV
pub async fn get_cached_response(env: &Env, key: &str) -> Option<String> {
    if let Ok(kv) = env.kv("CACHE") {
        match kv.get(key).text().await {
            Ok(Some(text)) => {
                worker::console_log!("Cache HIT: {}", key);
                Some(text)
            }
            Ok(None) => {
                worker::console_log!("Cache MISS: {}", key);
                None
            }
            Err(e) => {
                worker::console_log!("Cache ERROR: {} - {}", key, e);
                None
            }
        }
    } else {
        None
    }
}

/// Store response in KV cache
pub async fn set_cached_response(env: &Env, key: &str, value: &str, ttl: u64) {
    if let Ok(kv) = env.kv("CACHE") {
        // Use the builder pattern for KV put with expiration
        match kv.put(key, value) {
            Ok(mut put_builder) => {
                put_builder = put_builder.expiration_ttl(ttl);
                if let Err(e) = put_builder.execute().await {
                    worker::console_log!("Failed to cache response: {} - {}", key, e);
                } else {
                    worker::console_log!("Cached response: {} (TTL: {}s)", key, ttl);
                }
            }
            Err(e) => {
                worker::console_log!("Failed to create KV put builder: {} - {}", key, e);
            }
        }
    }
}

/// Determine cache TTL based on endpoint type
pub fn get_cache_ttl(endpoint: &str) -> u64 {
    // Different endpoints have different cache strategies
    if endpoint.contains("/search/") {
        // Search results: shorter cache (5 minutes) as they can change frequently
        300
    } else if endpoint.contains("/dockets/") || endpoint.contains("/docket-alerts") {
        // Dockets: medium cache (15 minutes)
        900
    } else if endpoint.contains("/opinions/") || endpoint.contains("/clusters/") {
        // Opinions: longer cache (30 minutes) as they rarely change
        1800
    } else if endpoint.contains("/courts/") || endpoint.contains("/people/") {
        // Courts and people: longest cache (1 hour) as they rarely change
        3600
    } else {
        // Default: 10 minutes
        600
    }
}

/// Add cache headers to response based on cache status
pub fn add_cache_headers(headers: &mut Headers, cache_ttl: u64, from_cache: bool) -> Result<()> {
    if from_cache {
        // If served from cache, indicate it's cached
        headers.set("X-Cache", "HIT")?;
        headers.set(
            "Cache-Control",
            &format!("public, max-age={}, s-maxage={}", cache_ttl, cache_ttl),
        )?;
    } else {
        // If not from cache, set cache headers for future requests
        headers.set("X-Cache", "MISS")?;
        headers.set(
            "Cache-Control",
            &format!("public, max-age={}, s-maxage={}", cache_ttl, cache_ttl),
        )?;
        // Add ETag support for conditional requests
        headers.set("Vary", "Accept, Authorization")?;
    }

    // Add stale-while-revalidate for better performance
    headers.set(
        "Cache-Control",
        &format!(
            "public, max-age={}, s-maxage={}, stale-while-revalidate={}",
            cache_ttl,
            cache_ttl,
            cache_ttl / 2
        ),
    )?;

    Ok(())
}

/// Cache entry with metadata
#[derive(Serialize, Deserialize)]
struct CacheEntry {
    data: String,
    timestamp: u64,
    ttl: u64,
}

impl CacheEntry {
    fn is_expired(&self) -> bool {
        // Use Date::now() from web-sys via worker crate
        // Note: In Workers, we rely on KV TTL for expiration, but check timestamp as backup
        // Timestamp is in seconds since epoch
        let now = js_sys::Date::now() as u64 / 1000;
        now > (self.timestamp + self.ttl)
    }

    fn new(data: String, ttl: u64) -> Self {
        let timestamp = js_sys::Date::now() as u64 / 1000;
        Self {
            data,
            timestamp,
            ttl,
        }
    }
}

/// Get cached response with expiration check
pub async fn get_cached_with_expiry(env: &Env, key: &str) -> Option<String> {
    if let Some(cached) = get_cached_response(env, key).await {
        // Parse cache entry
        if let Ok(entry) = serde_json::from_str::<CacheEntry>(&cached) {
            if !entry.is_expired() {
                return Some(entry.data);
            } else {
                // Expired, remove from cache
                if let Ok(kv) = env.kv("CACHE") {
                    let _ = kv.delete(key).await;
                }
            }
        } else {
            // Legacy format (plain string), return as-is
            return Some(cached);
        }
    }
    None
}

/// Store response with expiration metadata
pub async fn set_cached_with_expiry(env: &Env, key: &str, value: &str, ttl: u64) {
    let entry = CacheEntry::new(value.to_string(), ttl);
    if let Ok(json) = serde_json::to_string(&entry) {
        set_cached_response(env, key, &json, ttl + 60).await; // Add 1 minute buffer for expiration
    }
}
