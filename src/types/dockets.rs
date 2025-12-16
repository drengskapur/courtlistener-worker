//! Docket types

use serde::{Deserialize, Serialize};
use crate::types::common::PaginatedResponse;

/// Docket (case information)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Docket {
    pub id: u32,
    pub resource_uri: Option<String>,
    pub court: Option<String>, // URL or court_id
    pub court_id: Option<String>,
    pub original_court_info: Option<String>,
    pub idb_data: Option<serde_json::Value>,
    pub bankruptcy_information: Option<serde_json::Value>,
    pub clusters: Option<Vec<serde_json::Value>>,
    pub audio_files: Option<Vec<serde_json::Value>>,
    pub assigned_to: Option<String>, // URL or person_id
    pub referred_to: Option<String>,
    pub absolute_url: Option<String>,
    pub date_created: Option<String>,
    pub date_modified: Option<String>,
    pub source: Option<u32>,
    pub appeal_from_str: Option<String>,
    pub assigned_to_str: Option<String>,
    pub referred_to_str: Option<String>,
    pub panel_str: Option<String>,
    pub date_last_index: Option<String>,
    pub date_cert_granted: Option<String>,
    pub date_cert_denied: Option<String>,
    pub date_argued: Option<String>,
    pub date_reargued: Option<String>,
    pub date_reargument_denied: Option<String>,
    pub date_filed: Option<String>,
    pub date_terminated: Option<String>,
    pub date_last_filing: Option<String>,
    pub case_name_short: Option<String>,
    pub case_name: Option<String>,
    pub case_name_full: Option<String>,
    pub slug: Option<String>,
    pub docket_number: Option<String>,
    pub docket_number_core: Option<String>,
    pub pacer_case_id: Option<String>,
    pub cause: Option<String>,
    pub nature_of_suit: Option<String>,
    pub jury_demand: Option<String>,
    pub jurisdiction_type: Option<String>,
    pub appellate_fee_status: Option<String>,
    pub appellate_case_type_information: Option<String>,
    pub mdl_status: Option<String>,
    pub filepath_ia: Option<String>,
    pub filepath_ia_json: Option<String>,
    pub ia_upload_failure_count: Option<u32>,
    pub ia_needs_upload: Option<bool>,
    pub ia_date_first_change: Option<String>,
    pub date_blocked: Option<String>,
    pub blocked: Option<bool>,
    pub appeal_from: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub panel: Option<Vec<serde_json::Value>>,
}

/// API response type
pub type DocketsResponse = PaginatedResponse<Docket>;
