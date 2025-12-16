//! Citation types

use crate::types::common::PaginatedResponse;
use serde::{Deserialize, Serialize};

/// Citation lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationLookup {
    pub id: u32,
    pub volume: Option<u32>,
    pub reporter: Option<String>,
    pub page: Option<String>,
    pub r#type: Option<u32>,
    pub cluster_id: Option<u32>,
}

/// Opinion citation relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpinionCited {
    pub id: u32,
    pub citing_opinion_id: Option<u32>,
    pub cited_opinion_id: Option<u32>,
    pub citing_opinion: Option<String>,
    pub cited_opinion: Option<String>,
    pub depth: Option<String>,
}

/// API Citation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCitation {
    pub id: u32,
    pub citing_opinion_id: Option<u32>,
    pub cited_opinion_id: Option<u32>,
    pub citing_opinion: Option<String>,
    pub cited_opinion: Option<String>,
    pub depth: Option<String>,
}

/// Paginated citations response
pub type CitationsResponse = PaginatedResponse<ApiCitation>;
