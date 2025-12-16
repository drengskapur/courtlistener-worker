//! Cloudflare Worker implementation for CourtListener API
//!
//! This module contains the worker-specific routing and handlers.
//! The core library (types, ApiClient, config) can be used independently.

use crate::handlers;
use crate::utils;
use worker::*;

/// Get the current API version from CourtListener's GitHub repository changelog
/// Returns the version in format "v4.4" (major.minor, no patch)
/// This fetches the latest version from the changelog at runtime
pub async fn get_current_api_version() -> worker::Result<String> {
    let url = "https://raw.githubusercontent.com/freelawproject/courtlistener/refs/heads/main/cl/api/templates/rest-change-log.html";

    let req = Request::new(url, Method::Get)?;
    let mut resp = Fetch::Request(req).send().await?;

    if !resp.status_code().eq(&200) {
        return Err(worker::Error::RustError(format!(
            "Failed to fetch changelog: HTTP {}",
            resp.status_code()
        )));
    }

    let html = resp.text().await?;

    // Look for the first version entry in the changelog: <strong>v4.4</strong>
    // The changelog lists versions with the latest first
    let re = regex::Regex::new(r"<strong>v(\d+)\.(\d+)")
        .map_err(|e| worker::Error::RustError(format!("Failed to create regex: {}", e)))?;

    if let Some(caps) = re.captures(&html) {
        if let (Some(major), Some(minor)) = (caps.get(1), caps.get(2)) {
            return Ok(format!("v{}.{}", major.as_str(), minor.as_str()));
        }
    }

    Err(worker::Error::RustError(
        "Could not find version pattern in GitHub changelog".to_string(),
    ))
}

/// Main worker entry point (called from crate root)
/// This function contains all the routing logic
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<worker::Response> {
    worker::console_log!("Request: {} {}", req.method(), req.path());

    Router::new()
        .get("/", |_req, _ctx| {
            Response::ok("CourtListener Worker API\n\nVisit /docs for API documentation")
        })
        .get("/health", |_req, _ctx| Response::ok("OK"))
        // Endpoint comparison tool
        .get_async("/check-endpoints", |_req, ctx| async move {
            handlers::check_endpoints(&ctx.env).await
        })
        // API Documentation (Scalar)
        .get("/docs", |req, _ctx| {
            handlers::serve_docs_ui("scalar", &req)
        })
        .get_async("/docs/openapi.json", |req, ctx| async move {
            // Check if ?fresh=true to generate on-demand, otherwise use static file
            let url = req.url().ok();
            let fresh = url
                .and_then(|u| {
                    u.query_pairs()
                        .find(|(k, _)| k == "fresh")
                        .map(|(_, v)| v == "true")
                })
                .unwrap_or(false);

            if fresh {
                handlers::generate_openapi_spec(&ctx.env).await
            } else {
                handlers::serve_openapi_spec()
            }
        })
        // API root - list all available APIs
        .get_async("/api", |req, ctx| async move {
            handlers::fetch_api_root(&ctx.env, &req).await
        })
        .options_async("/api", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // API endpoints - fetch from CourtListener API
        // Courts
        .get_async("/api/courts", |req, ctx| async move {
            handlers::fetch_courts(&ctx.env, &req).await
        })
        .options_async("/api/courts", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        .get_async("/api/courts/:id", |req, ctx| async move {
            if let Some(id) = ctx.param("id") {
                handlers::fetch_court(id, &ctx.env, &req).await
            } else {
                Response::error("Missing court ID", 400)
            }
        })
        .options_async("/api/courts/:id", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Opinions
        .get_async("/api/opinions", |req, ctx| async move {
            handlers::fetch_opinions(&ctx.env, &req).await
        })
        .options_async("/api/opinions", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        .get_async("/api/clusters", |req, ctx| async move {
            handlers::fetch_opinion_clusters(&ctx.env, &req).await
        })
        .options_async("/api/clusters", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // People
        .get_async("/api/people", |req, ctx| async move {
            handlers::fetch_people(&ctx.env, &req).await
        })
        .options_async("/api/people", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Dockets
        .get_async("/api/dockets", |req, ctx| async move {
            handlers::fetch_dockets(&ctx.env, &req).await
        })
        .options_async("/api/dockets", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Search (supports both GET and POST for semantic search with embeddings)
        .get_async("/api/search", |req, ctx| async move {
            handlers::fetch_search(&ctx.env, &req).await
        })
        .post_async("/api/search", |mut req, ctx| async move {
            let body = req.text().await.ok().unwrap_or_default();
            handlers::fetch_search_post(&ctx.env, &req, &body).await
        })
        // Note: Search API doesn't support OPTIONS requests (unlike other APIs)
        // Citations
        .get_async("/api/citations", |req, ctx| async move {
            handlers::fetch_citations(&ctx.env, &req).await
        })
        .options_async("/api/citations", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Audio
        .get_async("/api/audio", |req, ctx| async move {
            handlers::fetch_audio(&ctx.env, &req).await
        })
        .options_async("/api/audio", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Audio file streaming (for downloading MP3 files)
        // Usage: /api/audio/stream?url=https://... or /api/audio/stream?id=12345
        .get_async("/api/audio/stream", |req, ctx| async move {
            handlers::stream_audio_file(&req, &ctx.env).await
        })
        // Alerts - Docket Alerts
        .get_async("/api/docket-alerts", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .post_async("/api/docket-alerts", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .get_async("/api/docket-alerts/:id", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .patch_async("/api/docket-alerts/:id", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .delete_async("/api/docket-alerts/:id", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .options_async("/api/docket-alerts", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        .options_async("/api/docket-alerts/:id", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Alerts - Search Alerts
        .get_async("/api/alerts", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .post_async("/api/alerts", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .get_async("/api/alerts/:id", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .patch_async("/api/alerts/:id", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .delete_async("/api/alerts/:id", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .options_async("/api/alerts", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        .options_async("/api/alerts/:id", |req, ctx| async move {
            handlers::fetch_api_options(&ctx.env, &req).await
        })
        // Webhook receiver endpoint (for receiving webhooks FROM CourtListener)
        // Point your domain to this worker and configure webhook URL to: https://your-domain.com/webhook
        .post_async("/webhook", |mut req, ctx| async move {
            let body = req.text().await.ok().unwrap_or_default();
            handlers::receive_webhook(&req, &ctx.env, &body).await
        })
        .post_async("/webhook/:secret", |mut req, ctx| async move {
            let body = req.text().await.ok().unwrap_or_default();
            handlers::receive_webhook(&req, &ctx.env, &body).await
        })
        // Generic proxy for any CourtListener API endpoint
        // Usage: /api/proxy/courts/ or /api/proxy/search/?q=constitution
        .get_async("/api/proxy/*path", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .post_async("/api/proxy/*path", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .put_async("/api/proxy/*path", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .patch_async("/api/proxy/*path", |mut req, ctx| async move {
            let body = req.text().await.ok();
            handlers::proxy_api_request(&req, &ctx.env, body.as_deref()).await
        })
        .delete_async("/api/proxy/*path", |req, ctx| async move {
            handlers::proxy_api_request(&req, &ctx.env, None).await
        })
        .options_async("/api/proxy/*path", |_req, _ctx| async move {
            utils::cors_preflight_response()
        })
        .run(req, env)
        .await
}

