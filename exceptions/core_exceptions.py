"""
Core exceptions for Ask Gloom.
Defines custom exceptions used throughout the framework.
"""

class GloomException(Exception):
    """Base exception class for all Ask Gloom exceptions."""
    def __init__(self, message: str = None, *args, **kwargs):
        self.message = message or "An unknown error occurred"
        super().__init__(self.message, *args)

class BrowserException(GloomException):
    """Raised when browser operations fail."""
    def __init__(self, message: str = None, *args, **kwargs):
        super().__init__(message or "Browser operation failed", *args)

class ProfileException(GloomException):
    """Raised when profile operations fail."""
    def __init__(self, message: str = None, *args, **kwargs):
        super().__init__(message or "Profile operation failed", *args)

class ConfigurationException(GloomException):
    """Raised when configuration operations fail."""
    def __init__(self, message: str = None, *args, **kwargs):
        super().__init__(message or "Configuration operation failed", *args)

class ElementException(BrowserException):
    """Raised when element operations fail."""
    def __init__(self, message: str = None, selector: str = None, *args, **kwargs):
        message = message or f"Element operation failed"
        if selector:
            message += f" (selector: {selector})"
        super().__init__(message, *args)

class NavigationException(BrowserException):
    """Raised when navigation operations fail."""
    def __init__(self, message: str = None, url: str = None, *args, **kwargs):
        message = message or f"Navigation failed"
        if url:
            message += f" (url: {url})"
        super().__init__(message, *args)

class TimeoutException(BrowserException):
    """Raised when operations timeout."""
    def __init__(self, message: str = None, timeout: float = None, *args, **kwargs):
        message = message or f"Operation timed out"
        if timeout:
            message += f" (timeout: {timeout}s)"
        super().__init__(message, *args)

class ValidationException(GloomException):
    """Raised when validation fails."""
    def __init__(self, message: str = None, field: str = None, *args, **kwargs):
        message = message or f"Validation failed"
        if field:
            message += f" (field: {field})"
        super().__init__(message, *args)

class AuthenticationException(GloomException):
    """Raised when authentication fails."""
    def __init__(self, message: str = None, *args, **kwargs):
        super().__init__(message or "Authentication failed", *args)

class NetworkException(GloomException):
    """Raised when network operations fail."""
    def __init__(self, message: str = None, status_code: int = None, *args, **kwargs):
        message = message or f"Network operation failed"
        if status_code:
            message += f" (status code: {status_code})"
        super().__init__(message, *args)

class ResourceException(GloomException):
    """Raised when resource operations fail."""
    def __init__(self, message: str = None, resource: str = None, *args, **kwargs):
        message = message or f"Resource operation failed"
        if resource:
            message += f" (resource: {resource})"
        super().__init__(message, *args)