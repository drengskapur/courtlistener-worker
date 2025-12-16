//! School types

use serde::{Deserialize, Serialize};

/// School (educational institution)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub id: u32,
    pub name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
}


