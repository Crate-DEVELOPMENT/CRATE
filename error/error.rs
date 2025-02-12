//! Detailed error implementations for the Crate library.

use std::fmt;
use thiserror::Error;

/// Detailed error information for debugging and logging.
#[derive(Debug)]
pub struct ErrorInfo {
    /// File where the error occurred
    pub file: &'static str,
    /// Line number where the error occurred
    pub line: u32,
    /// Additional context about the error
    pub context: Option<String>,
    /// Stack trace if available
    pub stack_trace: Option<String>,
}

/// Core error types for the Crate library.
#[derive(Error, Debug)]
pub enum CrateError {
    #[error("Browser error: {message}")]
    Browser {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Profile error: {message}")]
    Profile {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Configuration error: {message}")]
    Config {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Element error: {message} (selector: {selector:?})")]
    Element {
        message: String,
        selector: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Navigation error: {message} (url: {url:?})")]
    Navigation {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Timeout error: {message} ({duration:?}s)")]
    Timeout {
        message: String,
        duration: Option<f64>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Authentication error: {message}")]
    Auth {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Network error: {message} (status: {status_code:?})")]
    Network {
        message: String,
        status_code: Option<u16>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("Resource error: {message} (resource: {resource:?})")]
    Resource {
        message: String,
        resource: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        info: Option<ErrorInfo>,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl CrateError {
    /// Create a new browser error
    pub fn browser<S: Into<String>>(message: S) -> Self {
        CrateError::Browser {
            message: message.into(),
            source: None,
            info: None,
        }
    }

    /// Create a new profile error
    pub fn profile<S: Into<String>>(message: S) -> Self {
        CrateError::Profile {
            message: message.into(),
            source: None,
            info: None,
        }
    }

    /// Create a new element error
    pub fn element<S: Into<String>>(message: S, selector: Option<String>) -> Self {
        CrateError::Element {
            message: message.into(),
            selector,
            source: None,
            info: None,
        }
    }

    /// Add source error to any variant
    pub fn with_source<E>(mut self, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        match &mut self {
            CrateError::Browser { source, .. } |
            CrateError::Profile { source, .. } |
            CrateError::Config { source, .. } |
            CrateError::Element { source, .. } |
            CrateError::Navigation { source, .. } |
            CrateError::Timeout { source, .. } |
            CrateError::Auth { source, .. } |
            CrateError::Network { source, .. } |
            CrateError::Resource { source, .. } => {
                *source = Some(Box::new(error));
            }
            _ => {}
        }
        self
    }

    /// Add error info to any variant
    pub fn with_info(mut self, file: &'static str, line: u32, context: Option<String>) -> Self {
        let info = ErrorInfo {
            file,
            line,
            context,
            stack_trace: std::backtrace::Backtrace::capture().to_string().into(),
        };

        match &mut self {
            CrateError::Browser { info: i, .. } |
            CrateError::Profile { info: i, .. } |
            CrateError::Config { info: i, .. } |
            CrateError::Element { info: i, .. } |
            CrateError::Navigation { info: i, .. } |
            CrateError::Timeout { info: i, .. } |
            CrateError::Auth { info: i, .. } |
            CrateError::Network { info: i, .. } |
            CrateError::Resource { info: i, .. } => {
                *i = Some(info);
            }
            _ => {}
        }
        self
    }
}

/// Macro to create errors with file and line information
#[macro_export]
macro_rules! crate_error {
    ($variant:ident, $message:expr) => {
        CrateError::$variant {
            message: $message.into(),
            source: None,
            info: Some(ErrorInfo {
                file: file!(),
                line: line!(),
                context: None,
                stack_trace: Some(std::backtrace::Backtrace::capture().to_string()),
            })
        }
    };
    ($variant:ident, $message:expr, $($field:ident: $value:expr),*) => {
        CrateError::$variant {
            message: $message.into(),
            $($field: $value,)*
            source: None,
            info: Some(ErrorInfo {
                file: file!(),
                line: line!(),
                context: None,
                stack_trace: Some(std::backtrace::Backtrace::capture().to_string()),
            })
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = CrateError::browser("test error");
        assert!(matches!(err, CrateError::Browser { .. }));

        let err = CrateError::element("not found", Some("#test".into()));
        assert!(matches!(err, CrateError::Element { .. }));
    }

    #[test]
    fn test_error_with_source() {
        let source = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let err = CrateError::browser("test error").with_source(source);
        assert!(matches!(err, CrateError::Browser { source: Some(_), .. }));
    }

    #[test]
    fn test_error_macro() {
        let err = crate_error!(Browser, "test error");
        assert!(matches!(err, CrateError::Browser { info: Some(_), .. }));
    }
}