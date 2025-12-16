//! Core types and utilities for CourtListener API

use serde::{Deserialize, Serialize};

/// Paginated API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

/// Type aliases for IDs
pub type CourtId = String;
pub type PersonId = u32;
pub type OpinionId = u32;
pub type ClusterId = u32;
pub type CitationId = u32;
pub type DocketId = u32;

/// Jurisdiction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Jurisdiction {
    /// Federal
    F,
    /// State
    S,
    /// Tribal
    T,
    /// Municipal
    M,
    /// Private
    P,
    /// Local
    L,
}

/// Court types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CourtType {
    F,
    S,
    T,
    M,
    P,
    L,
}

/// Opinion types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpinionType {
    #[serde(rename = "010combined")]
    Combined,
    #[serde(rename = "015unamimous")]
    Unanimous,
    #[serde(rename = "020lead")]
    Lead,
    #[serde(rename = "025plurality")]
    Plurality,
    #[serde(rename = "030concurrence")]
    Concurrence,
    #[serde(rename = "035concurrenceinpart")]
    ConcurrenceInPart,
    #[serde(rename = "040dissent")]
    Dissent,
    #[serde(rename = "050addendum")]
    Addendum,
    #[serde(rename = "060errata")]
    Errata,
    #[serde(rename = "070supplement")]
    Supplement,
    #[serde(rename = "080rehearing")]
    Rehearing,
    #[serde(rename = "090rehearingrehearing")]
    RehearingRehearing,
    #[serde(rename = "100specialmaster")]
    SpecialMaster,
    #[serde(rename = "110statement")]
    Statement,
    #[serde(rename = "120recusation")]
    Recusation,
    #[serde(rename = "130register")]
    Register,
    #[serde(rename = "140percuriam")]
    PerCuriam,
    #[serde(rename = "150inaformal")]
    InaFormal,
    #[serde(rename = "160unknown")]
    Unknown,
    #[serde(rename = "800memo")]
    Memo,
    #[serde(rename = "810designation")]
    Designation,
    #[serde(rename = "820judgment")]
    Judgment,
    #[serde(rename = "830order")]
    Order,
    #[serde(rename = "840opinion")]
    Opinion,
    #[serde(rename = "850decree")]
    Decree,
}

/// Case status types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaseStatus {
    Argued,
    Decided,
    Granted,
    Opinion,
    Petition,
    Rehearing,
    Remanded,
    Terminated,
    Unknown,
}

/// Precedential status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrecedentialStatus {
    Published,
    Unpublished,
    Errata,
    Separate,
    #[serde(rename = "In-chambers")]
    InChambers,
    #[serde(rename = "Relating-to")]
    RelatingTo,
    Unknown,
}

/// Source types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "C")]
    Court, // Court website
    #[serde(rename = "R")]
    Recap, // RECAP
    #[serde(rename = "D")]
    Direct, // Direct court input
    #[serde(rename = "M")]
    Manual,
    #[serde(rename = "A")]
    Administrative, // Administrative Action
    #[serde(rename = "L")]
    LawBox,
    #[serde(rename = "S")]
    SlipOpinions,
    #[serde(rename = "P")]
    PressRelease,
    #[serde(rename = "I")]
    Internet,
    #[serde(rename = "U")]
    Unknown,
}

/// Blocked status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockedStatus {
    Blocked,
    Unblocked,
    Pending,
    Unknown,
}
