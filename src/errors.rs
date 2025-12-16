//! Error types for the CourtListener Worker

#[cfg(feature = "worker")]
use worker::Error as WorkerError;

/// Main error type for the CourtListener Worker
#[derive(Debug)]
#[allow(dead_code)]
pub enum CourtListenerError {
    /// Worker framework error (only available with worker feature)
    #[cfg(feature = "worker")]
    Worker(WorkerError),
    /// JSON deserialization error
    Json(String),
    /// HTTP request error
    Http(String),
    /// Cache error
    Cache(String),
    /// Validation error
    Validation(String),
    /// Invalid request error
    InvalidRequest(String),
}

#[cfg(feature = "worker")]
impl From<WorkerError> for CourtListenerError {
    fn from(err: WorkerError) -> Self {
        Self::Worker(err)
    }
}

impl From<serde_json::Error> for CourtListenerError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
}

impl std::fmt::Display for CourtListenerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "worker")]
            Self::Worker(e) => write!(f, "Worker error: {}", e),
            Self::Json(e) => write!(f, "JSON error: {}", e),
            Self::Http(e) => write!(f, "HTTP error: {}", e),
            Self::Cache(e) => write!(f, "Cache error: {}", e),
            Self::Validation(e) => write!(f, "Validation error: {}", e),
            Self::InvalidRequest(e) => write!(f, "Invalid request: {}", e),
        }
    }
}

impl std::error::Error for CourtListenerError {}

/// Result type alias for convenience
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, CourtListenerError>;

/// Convert to worker::Result for compatibility
#[cfg(feature = "worker")]
impl CourtListenerError {
    #[allow(dead_code)]
    pub fn to_worker_result<T>(result: Result<T>) -> worker::Result<T> {
        result.map_err(|e| match e {
            CourtListenerError::Worker(err) => err,
            other => WorkerError::RustError(other.to_string()),
        })
    }
}
