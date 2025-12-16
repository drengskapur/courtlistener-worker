//! Financial disclosure and FJC database types

use serde::{Deserialize, Serialize};

/// Financial disclosure document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialDisclosure {
    pub id: u32,
    pub judge: Option<u32>,
    pub year: Option<u32>,
    pub page_number: Option<u32>,
    pub redacted: Option<bool>,
    pub download_url: Option<String>,
    pub thumbnail: Option<String>,
    pub thumbnail_size: Option<u32>,
}

/// Federal Judicial Center database entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FJCDatabase {
    pub id: u32,
    pub judge_id: Option<u32>,
    pub date_created: Option<String>,
    pub date_modified: Option<String>,
    pub year: Option<u32>,
    pub nid: Option<u32>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub court: Option<String>,
    pub source_url: Option<String>,
}


