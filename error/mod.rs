//! Error handling for the Crate library.
//! Provides custom error types and result aliases.

use std::fmt;
use std::error::Error as StdError;
use std::result::Result as StdResult;

/// A specialized Result type for Crate operations.
pub type Result<T> = StdResult<T, Error>;

/// Represents all possible errors that can occur in the Crate library.
#[derive(Debug)]
pub enum Error {
    /// Errors related to browser operations
    BrowserError(String),
    
    /// Errors related to profile management
    ProfileError(String),
    
    /// Errors related to configuration
    ConfigError(String),
    
    /// Errors related to element interactions
    ElementError {
        message: String,
        selector: Option<String>,
    },
    
    /// Errors related to navigation
    NavigationError {
        message: String,
        url: Option<String>,
    },
    
    /// Timeout errors
    TimeoutError {
        message: String,
        duration: Option<f64>,
    },
    
    /// Validation errors
    ValidationError {
        message: String,
        field: Option<String>,
    },
    
    /// Authentication errors
    AuthError(String),
    
    /// Network-related errors
    NetworkError {
        message: String,
        status_code: Option<u16>,
    },
    
    /// Resource-related errors
    ResourceError {
        message: String,
        resource: Option<String>,
    },
    
    /// IO errors
    IoError(std::io::Error),
    
    /// Serialization errors
    SerdeError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BrowserError(msg) => write!(f, "Browser error: {}", msg),
            Error::ProfileError(msg) => write!(f, "Profile error: {}", msg),
            Error::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Error::ElementError { message, selector } => {
                if let Some(sel) = selector {
                    write!(f, "Element error: {} (selector: {})", message, sel)
                } else {
                    write!(f, "Element error: {}", message)
                }
            },
            Error::NavigationError { message, url } => {
                if let Some(u) = url {
                    write!(f, "Navigation error: {} (url: {})", message, u)
                } else {
                    write!(f, "Navigation error: {}", message)
                }
            },
            Error::TimeoutError { message, duration } => {
                if let Some(d) = duration {
                    write!(f, "Timeout error: {} ({}s)", message, d)
                } else {
                    write!(f, "Timeout error: {}", message)
                }
            },
            Error::ValidationError { message, field } => {
                if let Some(f) = field {
                    write!(f, "Validation error: {} (field: {})", message, f)
                } else {
                    write!(f, "Validation error: {}", message)
                }
            },
            Error::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            Error::NetworkError { message, status_code } => {
                if let Some(code) = status_code {
                    write!(f, "Network error: {} (status: {})", message, code)
                } else {
                    write!(f, "Network error: {}", message)
                }
            },
            Error::ResourceError { message, resource } => {
                if let Some(r) = resource {
                    write!(f, "Resource error: {} (resource: {})", message, r)
                } else {
                    write!(f, "Resource error: {}", message)
                }
            },
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::SerdeError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            Error::SerdeError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::BrowserError("test error".into());
        assert_eq!(err.to_string(), "Browser error: test error");

        let err = Error::ElementError {
            message: "not found".into(),
            selector: Some("#test".into()),
        };
        assert_eq!(err.to_string(), "Element error: not found (selector: #test)");
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::IoError(_)));
    }
}