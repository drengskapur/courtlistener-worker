//! Alert types for CourtListener API

use serde::{Deserialize, Serialize};
use crate::types::common::PaginatedResponse;

/// Docket Alert - subscription to receive notifications when a docket is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocketAlert {
    pub id: u32,
    pub date_created: Option<String>,
    pub date_last_hit: Option<String>,
    pub secret_key: Option<String>,
    pub alert_type: Option<u32>, // 0 = disabled, 1 = enabled
    pub docket: Option<u32>, // Docket ID
    pub docket_id: Option<u32>, // Alternative field name
}

/// Search Alert - subscription to receive notifications when search results change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAlert {
    pub id: u32,
    pub name: Option<String>,
    pub query: Option<String>,
    pub rate: Option<String>, // "rt" = realtime, "dly" = daily, "wly" = weekly, "mly" = monthly
    pub date_created: Option<String>,
    pub date_last_hit: Option<String>,
    pub secret_key: Option<String>,
}

/// Alert rate types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertRate {
    /// Real-time alerts
    #[serde(rename = "rt")]
    Realtime,
    /// Daily alerts
    #[serde(rename = "dly")]
    Daily,
    /// Weekly alerts
    #[serde(rename = "wly")]
    Weekly,
    /// Monthly alerts
    #[serde(rename = "mly")]
    Monthly,
}

/// Alert type (for docket alerts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertType {
    /// Alert disabled
    #[serde(rename = "0")]
    Disabled,
    /// Alert enabled
    #[serde(rename = "1")]
    Enabled,
}

/// API response types
pub type DocketAlertsResponse = PaginatedResponse<DocketAlert>;
pub type SearchAlertsResponse = PaginatedResponse<SearchAlert>;


