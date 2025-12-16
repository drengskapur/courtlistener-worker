//! Search result types

use serde::{Deserialize, Serialize};

/// Search result from CourtListener search API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Option<u32>,
    #[serde(alias = "caseName")]
    pub case_name: Option<String>,
    #[serde(alias = "caseNameShort")]
    pub case_name_short: Option<String>,
    pub court: Option<String>,
    pub court_id: Option<String>,
    #[serde(alias = "dateFiled")]
    pub date_filed: Option<String>,
    pub citation: Option<Vec<String>>,
    pub citation_count: Option<u32>,
    pub cluster_id: Option<u32>,
    pub court_citation_string: Option<String>,
    #[serde(alias = "docketNumber")]
    pub docket_number: Option<String>,
    #[serde(alias = "suitNature")]
    pub suit_nature: Option<String>,
    pub cause: Option<String>,
    pub nature_of_suit: Option<String>,
    pub status: Option<String>,
    pub jurisdiction: Option<String>,
    pub region: Option<String>,
    pub division: Option<String>,
    pub subtype: Option<String>,
    pub terminating_date_filed: Option<String>,
    pub date_terminated: Option<String>,
    pub date_last_filing: Option<String>,
    pub assigned_to_str: Option<String>,
    pub referred_to_str: Option<String>,
    pub slug: Option<String>,
    pub absolute_url: Option<String>,
}
