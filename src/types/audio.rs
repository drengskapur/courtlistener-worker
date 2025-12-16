//! Audio types

use serde::{Deserialize, Serialize};
use crate::types::common::PaginatedResponse;

/// Audio recording (oral argument)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub id: u32,
    pub resource_uri: Option<String>,
    pub docket: Option<String>, // URL or docket_id
    pub docket_id: Option<u32>,
    pub source: Option<String>,
    pub case_name: Option<String>,
    pub case_name_short: Option<String>,
    pub case_name_full: Option<String>,
    pub date_created: Option<String>,
    pub date_modified: Option<String>,
    pub date_argued: Option<String>,
    pub court: Option<String>, // URL or court_id
    pub court_id: Option<String>,
    pub download_url: Option<String>, // Original file from court
    pub local_path_mp3: Option<String>, // Enhanced MP3 file path
    pub duration: Option<f64>, // Duration in seconds (estimated)
    pub sha1: Option<String>,
    pub filepath_ia: Option<String>, // Internet Archive path
    pub filepath_ia_json: Option<String>,
    pub ia_upload_failure_count: Option<u32>,
    pub ia_needs_upload: Option<bool>,
    pub ia_date_first_change: Option<String>,
    pub date_blocked: Option<String>,
    pub blocked: Option<bool>,
    pub judges: Option<Vec<String>>, // URLs or person_ids
    pub absolute_url: Option<String>,
}

/// API response type
pub type AudioResponse = PaginatedResponse<Audio>;
