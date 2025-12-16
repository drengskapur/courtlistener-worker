//! Parenthetical types

use serde::{Deserialize, Serialize};

/// Parenthetical (summary/description of an opinion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parenthetical {
    pub id: u32,
    pub text: Option<String>,
    pub opinion_id: Option<u32>,
}

