"""
Exceptions module for Ask Gloom.
Provides custom exceptions for error handling throughout the framework.
"""

from .core_exceptions import (
    GloomException,
    BrowserException,
    ProfileException,
    ConfigurationException,
    ElementException,
    NavigationException,
    TimeoutException,
    ValidationException,
    AuthenticationException,
    NetworkException,
    ResourceException,
)

__all__ = [
    'GloomException',
    'BrowserException',
    'ProfileException',
    'ConfigurationException',
    'ElementException',
    'NavigationException',
    'TimeoutException',
    'ValidationException',
    'AuthenticationException',
    'NetworkException',
    'ResourceException',
]