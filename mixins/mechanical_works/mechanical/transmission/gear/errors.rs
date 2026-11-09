//! Gear-specific error handling.
//!
//! This module defines all error variants that can occur during gear operations,
//! providing meaningful feedback for failed gear management tasks.

use std::fmt;

/// A result type alias for gear operations.
pub type GearResult<T> = Result<T, GearError>;

/// Errors that can occur when managing gear components.
#[derive(Debug, Clone, PartialEq)]
pub enum GearError {
    /// Indicates that a gear configuration is invalid.
    InvalidConfiguration,
    
    /// Indicates that a gear has an invalid tooth count.
    InvalidToothCount,
    
    /// Indicates that a gear ratio is invalid.
    InvalidGearRatio,
    
    /// Indicates that a gear pair configuration is invalid.
    InvalidGearPair,
    
    /// Indicates that a gear is attempting to be paired with itself.
    SelfPairing,
    
    /// Indicates that a gear arrangement is unsupported.
    UnsupportedGearArrangement,
    
    /// Indicates that a gear's operating limits are exceeded.
    ExceededRPM,
    
    /// Indicates that a gear's torque limit is exceeded.
    ExceededTorque,
    
    /// Indicates that a gear's power limit is exceeded.
    ExceededPower,
    
    /// Indicates that a gear is disabled and cannot be used.
    DisabledGear,
    
    /// Indicates that an invalid efficiency value was provided.
    InvalidEfficiency,
    
    /// Indicates that the input for a calculation is invalid.
    InvalidCalculationInput,
}

impl fmt::Display for GearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearError::InvalidConfiguration => write!(f, "Gear configuration is invalid"),
            GearError::InvalidToothCount => write!(f, "Gear tooth count must be positive"),
            GearError::InvalidGearRatio => write!(f, "Gear ratio is invalid"),
            GearError::InvalidGearPair => write!(f, "Gear pair configuration is invalid"),
            GearError::SelfPairing => write!(f, "Cannot pair a gear with itself"),
            GearError::UnsupportedGearArrangement => write!(f, "Gear arrangement is not supported"),
            GearError::ExceededRPM => write!(f, "Rotational speed exceeds maximum limit"),
            GearError::ExceededTorque => write!(f, "Torque exceeds maximum limit"),
            GearError::ExceededPower => write!(f, "Power exceeds maximum limit"),
            GearError::DisabledGear => write!(f, "Gear is disabled"),
            GearError::InvalidEfficiency => write!(f, "Efficiency must be between 0.0 and 1.0"),
            GearError::InvalidCalculationInput => write!(f, "Calculation input is invalid"),
        }
    }
}

impl std::error::Error for GearError {}