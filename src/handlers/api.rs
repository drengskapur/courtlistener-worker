//! API endpoint handlers for CourtListener resources

use crate::api::ApiClient;
use crate::config::{get_api_base_url, get_cors_origins};
use crate::utils::{json_response, sanitize_error};
use worker::*;

/// Fetch API root - lists all available APIs
pub async fn fetch_api_root(env: &Env, req: &Request) -> Result<Response> {
    let root: serde_json::Value = ApiClient::fetch_json(env, "/", req).await?;
    json_response(&root)
}

/// Handle OPTIONS requests for API discovery (returns API metadata)
pub async fn fetch_api_options(env: &Env, req: &Request) -> Result<Response> {
    // Forward OPTIONS request to CourtListener API for discovery
    let url = req.url()?;
    let path = url.path();

    // Map worker path to CourtListener API path
    let api_path = if path.starts_with("/api") {
        path.replace("/api", "")
    } else {
        path.to_string()
    };

    let api_base = get_api_base_url();
    let api_url = format!("{}{}", api_base, api_path);
    let mut api_req = Request::new(&api_url, Method::Options)?;

    // Set headers
    api_req.headers_mut()?.set("Accept", "application/json")?;
    api_req.headers_mut()?.set(
        "User-Agent",
        &format!("courtlistener-worker/{}", env!("CARGO_PKG_VERSION")),
    )?;

    // Add API token if available
    if let Ok(token) = env.secret("COURTLISTENER_API_TOKEN") {
        api_req
            .headers_mut()?
            .set("Authorization", &format!("Token {}", token))?;
    }

    let mut resp = Fetch::Request(api_req).send().await?;
    let text = resp.text().await?;

    // Return the OPTIONS response with CORS headers
    let mut response = Response::ok(text)?;
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &get_cors_origins())?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization",
    )?;
    headers.set("Content-Type", "application/json")?;
    Ok(response)
}

// --- Courts ---

/// Fetch courts from CourtListener API
/// Supports all query parameters: filtering, ordering, field selection, pagination
/// Examples: ?court__jurisdiction=F&order_by=-date_modified&fields=id,name
pub async fn fetch_courts(env: &Env, req: &Request) -> Result<Response> {
    let courts: crate::CourtsResponse = ApiClient::fetch_json(env, "/courts/", req).await?;
    json_response(&courts)
}

/// Fetch a specific court by ID
/// Supports query parameters for field selection: ?fields=id,name,full_name
pub async fn fetch_court(id: &str, env: &Env, req: &Request) -> Result<Response> {
    let court: crate::ApiCourt =
        ApiClient::fetch_json(env, &format!("/courts/{}/", id), req).await?;
    json_response(&court)
}

// --- Opinions & Clusters ---

/// Fetch opinions from CourtListener API
/// Supports filtering, ordering, field selection, pagination
/// Examples: ?cluster__docket__court=scotus&order_by=-date_filed&fields=id,cluster_id
pub async fn fetch_opinions(env: &Env, req: &Request) -> Result<Response> {
    let opinions: crate::OpinionsResponse = ApiClient::fetch_json(env, "/opinions/", req).await?;
    json_response(&opinions)
}

/// Fetch opinion clusters from CourtListener API
/// Supports filtering, ordering, field selection, pagination
pub async fn fetch_opinion_clusters(env: &Env, req: &Request) -> Result<Response> {
    let clusters: crate::OpinionClustersResponse =
        ApiClient::fetch_json(env, "/clusters/", req).await?;
    json_response(&clusters)
}

// --- People ---

/// Fetch people from CourtListener API
/// Supports filtering, ordering, field selection, pagination
pub async fn fetch_people(env: &Env, req: &Request) -> Result<Response> {
    let people: crate::PeopleResponse = ApiClient::fetch_json(env, "/people/", req).await?;
    json_response(&people)
}

// --- Dockets ---

/// Fetch dockets from CourtListener API
/// Supports filtering, ordering, field selection, pagination
/// Examples: ?court=scotus&id__range=500,1000&order_by=-date_modified
pub async fn fetch_dockets(env: &Env, req: &Request) -> Result<Response> {
    let dockets: crate::DocketsResponse = ApiClient::fetch_json(env, "/dockets/", req).await?;
    json_response(&dockets)
}

// --- Search ---

/// Fetch search results from CourtListener API (GET)
/// Supports search query parameters: ?q=constitution&page_size=10&type=o&semantic=true
///
/// Result types:
/// - type=o: Case law opinion clusters (default) - only published results by default
/// - type=r: Federal cases (dockets) with up to 3 nested documents
/// - type=rd: Federal filing documents from PACER
/// - type=d: Federal cases (dockets) from PACER
/// - type=p: Judges
/// - type=oa: Oral argument audio files
///
/// Special parameters:
/// - semantic=true: Use semantic search instead of keyword search (only available for type=o)
/// - highlight=on: Enable highlighting in snippet fields (uses HTML5 <mark> elements)
/// - order_by: Sort results (Citegeist sorts by relevancy when ordering by relevancy)
///
/// Important notes:
/// - Search API is powered by search engine, not database (different from other APIs)
/// - Results are cached for 10 minutes
/// - Result counts for type=d and type=r have ±6% error if over 2000 results
/// - When highlighting disabled, snippet shows first 500 characters
/// - Snippet field only responds to q parameter and only displays Opinion text content
pub async fn fetch_search(env: &Env, req: &Request) -> Result<Response> {
    // Use generic JSON value since search results vary
    let results: serde_json::Value = ApiClient::fetch_json(env, "/search/", req).await?;
    json_response(&results)
}

/// Fetch search results from CourtListener API (POST)
/// Supports semantic search with pre-computed embeddings for privacy
///
/// POST body should be JSON with embedding array:
/// {
///   "embedding": [0.123, 0.456, -0.789, ...]  // 768 dimensions (required)
/// }
///
/// Query parameters can still be used for filtering, type selection, etc.
///
/// Important notes:
/// - Semantic search (POST with embeddings) is only available for case law (type=o)
/// - Embeddings should be calculated using CourtListener's Inception microservice
/// - Use fine-tuned model for proper embedding calculation
pub async fn fetch_search_post(env: &Env, req: &Request, body: &str) -> Result<Response> {
    let api_base = get_api_base_url();
    let url = req.url()?;

    // Build endpoint with query parameters
    let mut endpoint = "/search/".to_string();
    if let Some(query) = url.query() {
        if !query.is_empty() {
            endpoint = format!("{}?{}", endpoint, query);
        }
    }

    let api_url = format!("{}{}", api_base, endpoint);

    // Create POST request using RequestInit
    use worker::wasm_bindgen::JsValue;
    use worker::RequestInit;

    let headers = worker::Headers::new();
    headers.set("Accept", "application/json")?;
    headers.set("Content-Type", "application/json")?;
    headers.set(
        "User-Agent",
        &format!("courtlistener-worker/{}", env!("CARGO_PKG_VERSION")),
    )?;

    // Add API token if available
    if let Ok(token) = env.secret("COURTLISTENER_API_TOKEN") {
        headers.set("Authorization", &format!("Token {}", token))?;
    }

    let init = RequestInit {
        method: Method::Post,
        headers,
        body: if body.is_empty() {
            None
        } else {
            Some(JsValue::from_str(body))
        },
        ..Default::default()
    };

    let api_req = Request::new_with_init(&api_url, &init)?;
    let mut resp = Fetch::Request(api_req).send().await?;

    // Check HTTP status
    let status = resp.status_code();
    if !(200..300).contains(&status) {
        let text = resp.text().await.unwrap_or_default();
        return Err(worker::Error::RustError(format!(
            "API returned {}: {}",
            status,
            sanitize_error(&text)
        )));
    }

    let text = resp.text().await?;
    let results: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse search response: {}", e)))?;

    json_response(&results)
}

// --- Citations ---

/// Fetch citations from CourtListener API
/// Supports filtering, ordering, field selection, pagination
pub async fn fetch_citations(env: &Env, req: &Request) -> Result<Response> {
    let citations: crate::CitationsResponse =
        ApiClient::fetch_json(env, "/opinions-cited/", req).await?;
    json_response(&citations)
}

// --- Audio ---

/// Fetch audio from CourtListener API
/// Supports filtering, ordering, field selection, pagination
pub async fn fetch_audio(env: &Env, req: &Request) -> Result<Response> {
    let audio: crate::AudioResponse = ApiClient::fetch_json(env, "/audio/", req).await?;
    json_response(&audio)
}

/// Stream audio file from CourtListener
/// Supports streaming large audio files efficiently using Cloudflare Workers streaming
///
/// Query parameters:
/// - url: Direct URL to audio file (download_url or local_path_mp3 from audio API)
/// - id: Audio ID to fetch metadata first, then stream local_path_mp3
///
/// Example:
/// curl "http://localhost:8787/api/audio/stream?url=https://www.courtlistener.com/media/audio/..."
pub async fn stream_audio_file(req: &Request, env: &Env) -> Result<Response> {
    let url = req.url()?;

    // Get audio file URL from query parameters
    let audio_url = if let Some(url_param) = url.query_pairs().find(|(k, _)| k == "url") {
        url_param.1.to_string()
    } else if let Some(id_param) = url.query_pairs().find(|(k, _)| k == "id") {
        // Fetch audio metadata first to get local_path_mp3
        let audio_id = id_param.1.to_string();
        let audio: serde_json::Value =
            ApiClient::fetch_json(env, &format!("/audio/{}/", audio_id), req).await?;

        // Try local_path_mp3 first (enhanced version), fall back to download_url
        if let Some(local_path) = audio.get("local_path_mp3").and_then(|v| v.as_str()) {
            if local_path.starts_with("http") {
                local_path.to_string()
            } else {
                // Relative path, construct full URL
                format!("https://www.courtlistener.com{}", local_path)
            }
        } else if let Some(download_url) = audio.get("download_url").and_then(|v| v.as_str()) {
            download_url.to_string()
        } else {
            return Response::error("Audio file URL not found in metadata", 404);
        }
    } else {
        return Response::error("Missing 'url' or 'id' parameter", 400);
    };

    // Security: Validate URL is from CourtListener or archive.org
    if !audio_url.contains("courtlistener.com") && !audio_url.contains("archive.org") {
        return Response::error("Invalid audio URL domain", 400);
    }

    // Create request to fetch audio file
    let mut audio_req = Request::new(&audio_url, Method::Get)?;
    audio_req.headers_mut()?.set(
        "User-Agent",
        &format!("courtlistener-worker/{}", env!("CARGO_PKG_VERSION")),
    )?;

    // Fetch the audio file - Cloudflare Workers automatically streams large responses
    let mut resp = Fetch::Request(audio_req).send().await?;

    let status = resp.status_code();
    if !(200..300).contains(&status) {
        let text = resp.text().await.unwrap_or_default();
        return Err(worker::Error::RustError(format!(
            "Failed to fetch audio file: {} - {}",
            status,
            sanitize_error(&text)
        )));
    }

    // Get response headers
    let content_type = match resp.headers().get("Content-Type") {
        Ok(Some(val)) => val,
        _ => "audio/mpeg".to_string(),
    };
    let content_length = resp.headers().get("Content-Length").ok().flatten();

    // Create streaming response by reading the body and creating a new response
    let body = resp.bytes().await?;
    let mut response = Response::from_bytes(body)?;
    let headers = response.headers_mut();
    headers.set("Content-Type", &content_type)?;
    headers.set("Access-Control-Allow-Origin", &get_cors_origins())?;
    headers.set("Cache-Control", "public, max-age=86400")?; // Cache for 24 hours

    if let Some(len) = content_length {
        headers.set("Content-Length", &len)?;
    }

    // Set Content-Disposition for file download
    if let Some(filename) = audio_url.split('/').next_back() {
        headers.set(
            "Content-Disposition",
            &format!("attachment; filename=\"{}\"", filename),
        )?;
    }

    Ok(response)
}
