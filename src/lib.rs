//! CourtListener API Cloudflare Worker
//! 
//! A Cloudflare Worker for interacting with the CourtListener API,
//! with Rust types for all API responses.
//! 
//! # Example
//! 
//! ```no_run
//! use courtlistener_worker::*;
//! 
//! // Types are available for use in your worker
//! let court = Court {
//!     id: "us".to_string(),
//!     name: Some("Supreme Court of the United States".to_string()),
//!     full_name: Some("Supreme Court of the United States".to_string()),
//!     abbreviation: Some("SCOTUS".to_string()),
//! };
//! ```

use worker::*;

// Core modules
mod api;
mod cache;
mod config;
mod errors;
mod handlers;
mod types;
mod utils;

// Re-exports
// All types are re-exported from the types module
pub use types::*;

// Re-export config constants for convenience
pub use config::{API_VERSION, API_VERSION_PATH, API_BASE_URL, get_api_base_url};

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    worker::console_log!("Request: {} {}", req.method(), req.path());

    Router::new()
        .get("/", |_req, _ctx| Response::ok("CourtListener Worker API\n\nVisit /docs for API documentation"))
        .get("/health", |_req, _ctx| Response::ok("OK"))
        // Endpoint comparison tool
        .get_async("/check-endpoints", |_req, ctx| async move {
            handlers::check_endpoints(&ctx.env).await
        })
        // API Documentation
        .get("/docs", |req, _ctx| handlers::serve_docs_ui("swagger", &req))
        .get("/docs/swagger", |req, _ctx| handlers::serve_docs_ui("swagger", &req))
        .get("/docs/redoc", |req, _ctx| handlers::serve_docs_ui("redoc", &req))
        .get("/docs/scalar", |req, _ctx| handlers::serve_docs_ui("scalar", &req))
        .get_async("/docs/openapi.json", |req, ctx| async move {
            // Check if ?fresh=true to generate on-demand, otherwise use static file
            let url = req.url().ok();
            let fresh = url.and_then(|u| {
                u.query_pairs().find(|(k, _)| k == "fresh").map(|(_, v)| v == "true")
            }).unwrap_or(false);
            
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
