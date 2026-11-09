//! Error types for belt transmission systems.
//!
//! This module defines structured error types for belt-related operations.

use std::fmt;

/// Errors that can occur during belt transmission operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BeltError {
    /// Invalid pulley radius (zero or negative).
    InvalidRadius,
    
    /// Invalid efficiency value (outside 0.0 to 1.0 range).
    InvalidEfficiency,
    
    /// Invalid configuration values.
    InvalidConfiguration,
    
    /// Invalid torque value.
    InvalidTorque,
    
    /// Invalid RPM value.
    InvalidRpm,
    
    /// Invalid power value.
    InvalidPower,
    
    /// Invalid ratio calculation.
    InvalidRatio,
    
    /// Invalid pulley pair (same pulley used for input and output).
    InvalidPair,
    
    /// Belt is disabled.
    DisabledBelt,
}

impl fmt::Display for BeltError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeltError::InvalidRadius => write!(f, "Invalid pulley radius"),
            BeltError::InvalidEfficiency => write!(f, "Invalid efficiency value"),
            BeltError::InvalidConfiguration => write!(f, "Invalid configuration values"),
            BeltError::InvalidTorque => write!(f, "Invalid torque value"),
            BeltError::InvalidRpm => write!(f, "Invalid RPM value"),
            BeltError::InvalidPower => write!(f, "Invalid power value"),
            BeltError::InvalidRatio => write!(f, "Invalid ratio calculation"),
            BeltError::InvalidPair => write!(f, "Invalid pulley pair (same pulley used for input and output)"),
            BeltError::DisabledBelt => write!(f, "Belt is disabled"),
        }
    }
}

impl std::error::Error for BeltError {}

/// Result type for belt operations.
pub type BeltResult<T> = Result<T, BeltError>;