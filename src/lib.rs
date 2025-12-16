//! CourtListener API Rust Library
//!
//! A Rust library for interacting with the CourtListener API,
//! with complete type definitions for all API responses.
//!
//! This library provides:
//! - Complete type definitions for all CourtListener API resources
//! - High-level API client for Cloudflare Workers
//! - Validation support using the `validator` crate
//! - Configuration and utilities
//!
//! ## Library Usage (for lawforge or other projects)
//!
//! ```no_run
//! use courtlistener_worker::*;
//!
//! // All types are available
//! let court = Court {
//!     id: "us".to_string(),
//!     name: Some("Supreme Court of the United States".to_string()),
//!     full_name: Some("Supreme Court of the United States".to_string()),
//!     abbreviation: Some("SCOTUS".to_string()),
//! };
//! ```
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
//!
//! ## Cloudflare Worker Usage
//!
//! For Cloudflare Workers, use the `ApiClient`:
//!
//! ```no_run
//! use courtlistener_worker::{ApiClient, CourtsResponse};
//!
//! // In your worker handler
//! #[event(fetch)]
//! async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
//!     // Use the high-level client to fetch data
//!     let courts: CourtsResponse = ApiClient::fetch_json(&env, "/courts/", &req).await?;
//!     Response::from_json(&courts)
//! }
//! ```
//!
//! ## Validation
//!
//! For types that implement `validator::Validate`, use `fetch_json_validated`:
//!
//! ```no_run
//! use courtlistener_worker::{ApiClient, PrayAndPayWebhookPayload};
//! use validator::Validate;
//!
//! // This will validate the response before returning
//! let payload: PrayAndPayWebhookPayload = ApiClient::fetch_json_validated(&env, "/webhook", &req).await?;
//! ```
//!
//! ## Worker Implementation
//!
//! The full Cloudflare Worker implementation is available in the `worker` module.
//! To use it in your own worker, you can either:
//!
//! 1. Use the provided worker implementation:
//! ```no_run
//! // In your worker's lib.rs or main.rs
//! use courtlistener_worker::worker::main;
//! ```
//!
//! 2. Or create your own worker that uses the library:
//! ```no_run
//! use courtlistener_worker::{ApiClient, CourtsResponse, types::*};
//! use worker::*;
//!
//! #[event(fetch)]
//! async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
//!     // Use the library types and client
//!     let courts: CourtsResponse = ApiClient::fetch_json(&env, "/courts/", &req).await?;
//!     Response::from_json(&courts)
//! }
//! ```

// Core library modules (public API - always available)
pub mod config;
pub mod errors;
pub mod types;

// Worker-dependent modules (optional, requires worker feature)
#[cfg(feature = "worker")]
pub mod api;

// Internal modules (not part of public API, require worker)
#[cfg(feature = "worker")]
mod cache;
#[cfg(feature = "worker")]
mod handlers;
#[cfg(feature = "worker")]
mod utils;

// Worker implementation (optional, for Cloudflare Workers)
#[cfg(feature = "worker")]
pub mod worker;

// Re-exports
// All types are re-exported from the types module
pub use types::*;

// Re-export config constants for convenience
pub use config::{get_api_base_url, API_BASE_URL, API_VERSION, API_VERSION_PATH};

// Re-export high-level client interfaces (only when worker feature is enabled)
#[cfg(feature = "worker")]
pub use api::ApiClient;

// Re-export worker functions when worker feature is enabled
#[cfg(feature = "worker")]
pub use worker::get_current_api_version;

// Main worker entry point (must be at crate root for #[event] attribute)
// The #[event] macro automatically imports Request, Env, Context, Result, and Response
#[cfg(feature = "worker")]
#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Delegate to the worker module's main function which contains all routing logic
    crate::worker::main(req, env, _ctx).await
}

