//! People (judges, attorneys, court personnel) types

use crate::types::common::PaginatedResponse;
use serde::{Deserialize, Serialize};

/// Person (judge, attorney, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub positions: Option<Vec<Position>>,
}

/// Position (judicial position held by a person)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: u32,
    pub person: Option<u32>,
    pub court: Option<String>,
    pub position_type: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// API Person response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPerson {
    pub id: u32,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub positions: Option<Vec<serde_json::Value>>,
}

/// Paginated people response
pub type PeopleResponse = PaginatedResponse<ApiPerson>;
