//! Court types and API responses

use crate::types::common::PaginatedResponse;
use serde::{Deserialize, Serialize};

/// Court information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Court {
    pub id: String,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub abbreviation: Option<String>,
}

/// API Court response (matches actual API structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCourt {
    pub id: String,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub abbreviation: Option<String>,
}

/// Paginated courts response
pub type CourtsResponse = PaginatedResponse<ApiCourt>;
