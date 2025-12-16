//! Type definitions for CourtListener API

// Common types
pub mod common;

// Resource-specific types
pub mod alerts;
pub mod audio;
pub mod citations;
pub mod courts;
pub mod dockets;
pub mod financial;
pub mod opinions;
pub mod parenthetical;
pub mod people;
pub mod school;
pub mod search;
pub mod webhooks;

// Re-export common types
pub use common::*;

// Re-export all resource types
pub use alerts::*;
pub use audio::*;
pub use citations::*;
pub use courts::*;
pub use dockets::*;
pub use financial::*;
pub use opinions::*;
pub use parenthetical::*;
pub use people::*;
pub use school::*;
pub use search::*;
pub use webhooks::*;
