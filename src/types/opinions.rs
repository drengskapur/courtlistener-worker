//! Opinion and OpinionCluster types

use crate::types::common::{PaginatedResponse, PrecedentialStatus};
use serde::{Deserialize, Serialize};

/// Opinion cluster (group of related opinions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpinionCluster {
    pub id: u32,
    pub case_name: Option<String>,
    pub case_name_short: Option<String>,
    pub date_filed: Option<String>,
    pub date_filed_is_approximate: Option<bool>,
    pub slug: Option<String>,
    pub case_name_full: Option<String>,
    pub scdb_id: Option<String>,
    pub scdb_decision_direction: Option<i32>,
    pub scdb_votes_majority: Option<i32>,
    pub scdb_votes_minority: Option<i32>,
    pub source: Option<String>,
    pub procedural_history: Option<String>,
    pub attorneys: Option<String>,
    pub nature_of_suit: Option<String>,
    pub posture: Option<String>,
    pub syllabus: Option<String>,
    pub citation_count: Option<u32>,
    pub precedential_status: Option<PrecedentialStatus>,
    pub date_blocked: Option<String>,
    pub blocked: Option<bool>,
    pub court_id: Option<String>,
    pub court: Option<String>,
    pub docket_id: Option<u32>,
    pub docket: Option<String>,
}

/// Opinion (individual court opinion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opinion {
    pub id: u32,
    pub author_id: Option<u32>,
    pub author: Option<String>,
    pub author_str: Option<String>,
    pub per_curiam: Option<bool>,
    pub joined_by: Option<Vec<serde_json::Value>>,
    pub joined_by_str: Option<String>,
    pub r#type: Option<String>,
    pub sha1: Option<String>,
    pub page_count: Option<u32>,
    pub download_url: Option<String>,
    pub local_path: Option<String>,
    pub plain_text: Option<String>,
    pub html: Option<String>,
    pub html_lawbox: Option<String>,
    pub html_columbia: Option<String>,
    pub xml_harvard: Option<String>,
    pub html_with_citations: Option<String>,
    pub extracted_by_ocr: Option<bool>,
    pub opinions_cited: Option<Vec<serde_json::Value>>,
    pub cluster_id: Option<u32>,
    pub cluster: Option<String>,
    pub absolute_url: Option<String>,
}

/// API Opinion Cluster response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOpinionCluster {
    pub id: u32,
    pub case_name: Option<String>,
    pub case_name_short: Option<String>,
    pub date_filed: Option<String>,
    pub court: Option<String>,
    pub court_id: Option<String>,
    pub docket: Option<String>,
    pub docket_id: Option<u32>,
    pub citation_count: Option<u32>,
    pub precedential_status: Option<PrecedentialStatus>,
}

/// API Opinion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOpinion {
    pub id: u32,
    pub cluster_id: Option<u32>,
    pub case_name: Option<String>,
    pub date_filed: Option<String>,
    pub date_created: Option<String>,
    pub plain_text: Option<String>,
    pub html: Option<String>,
    pub html_lawbox: Option<String>,
    pub html_columbia: Option<String>,
    pub xml_harvard: Option<String>,
    pub html_with_citations: Option<String>,
    pub extracted_by_ocr: Option<bool>,
    pub author_id: Option<u32>,
}

/// Paginated opinion clusters response
pub type OpinionClustersResponse = PaginatedResponse<ApiOpinionCluster>;

/// Paginated opinions response
pub type OpinionsResponse = PaginatedResponse<ApiOpinion>;
