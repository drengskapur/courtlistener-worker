//! Webhook types for CourtListener API

use serde::{Deserialize, Serialize};

/// Webhook event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMetadata {
    pub version: Option<String>,
    pub event_type: Option<String>,
    pub date_created: Option<String>,
}

/// Webhook event payload wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub payload: serde_json::Value, // Event-specific payload
    pub webhook: WebhookMetadata,
}

/// Docket Alert webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocketAlertWebhookPayload {
    pub results: Vec<serde_json::Value>, // Docket entries (based on Docket Entry API)
}

/// Search Alert webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAlertWebhookPayload {
    pub results: Vec<serde_json::Value>, // Search results (based on Search API)
    pub alert: serde_json::Value, // Search Alert details
}

/// Old Docket Alert webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OldDocketAlertWebhookPayload {
    pub old_alerts: Vec<serde_json::Value>, // Alerts about to be disabled
    pub disabled_alerts: Vec<serde_json::Value>, // Alerts that were disabled
}

/// RECAP Fetch webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecapFetchWebhookPayload {
    pub id: Option<u32>,
    pub status: Option<String>,
    pub date_created: Option<String>,
    pub date_completed: Option<String>,
    // Add other RECAP Fetch fields as needed
}

/// Pray and Pay webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayAndPayWebhookPayload {
    pub id: u32,
    pub date_created: String,
    pub status: u32, // 1 = Waiting, 2 = Granted
    pub recap_document: u32, // RECAP document ID
}

