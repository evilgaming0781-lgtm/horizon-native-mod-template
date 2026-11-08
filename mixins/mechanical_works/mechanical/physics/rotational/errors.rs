//! Error types for rotational physics calculations in Horizon Mechanical Works.
//!
//! This module provides comprehensive error handling for all rotational physics operations.

use std::fmt;

/// Comprehensive error type for rotational physics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum RotationalPhysicsError {
    /// Kinematics-related errors
    Kinematics(KinematicsError),
    /// Dynamics-related errors  
    Dynamics(DynamicsError),
    /// Energy-related errors
    Energy(EnergyError),
    /// Efficiency-related errors
    Efficiency(EfficiencyError),
    /// Limits-related errors
    Limits(LimitError),
}

impl fmt::Display for RotationalPhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RotationalPhysicsError::Kinematics(err) => write!(f, "Kinematics error: {}", err),
            RotationalPhysicsError::Dynamics(err) => write!(f, "Dynamics error: {}", err),
            RotationalPhysicsError::Energy(err) => write!(f, "Energy error: {}", err),
            RotationalPhysicsError::Efficiency(err) => write!(f, "Efficiency error: {}", err),
            RotationalPhysicsError::Limits(err) => write!(f, "Limit validation error: {}", err),
        }
    }
}

impl std::error::Error for RotationalPhysicsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RotationalPhysicsError::Kinematics(err) => Some(err),
            RotationalPhysicsError::Dynamics(err) => Some(err),
            RotationalPhysicsError::Energy(err) => Some(err),
            RotationalPhysicsError::Efficiency(err) => Some(err),
            RotationalPhysicsError::Limits(err) => Some(err),
        }
    }
}

/// Error types for kinematics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum KinematicsError {
    /// Time interval is zero or negative
    InvalidTime,
    /// Calculation produced non-finite result (NaN or infinity)
    NonFiniteResult,
}

impl fmt::Display for KinematicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KinematicsError::InvalidTime => write!(f, "Time interval must be positive"),
            KinematicsError::NonFiniteResult => write!(f, "Calculation resulted in non-finite value"),
        }
    }
}

impl std::error::Error for KinematicsError {}

/// Error types for dynamics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicsError {
    /// Moment of inertia is zero or effectively zero
    ZeroInertia,
    /// Angular velocity is zero (cannot calculate load)
    ZeroVelocity,
    /// Calculation produced non-finite result (NaN or infinity)
    NonFiniteResult,
}

impl fmt::Display for DynamicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynamicsError::ZeroInertia => write!(f, "Moment of inertia cannot be zero"),
            DynamicsError::ZeroVelocity => write!(f, "Angular velocity cannot be zero for load calculation"),
            DynamicsError::NonFiniteResult => write!(f, "Calculation resulted in non-finite value"),
        }
    }
}

impl std::error::Error for DynamicsError {}

/// Error types for energy calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EnergyError {
    /// Time interval is zero or negative
    InvalidTime,
    /// Calculation produced non-finite result (NaN or infinity)
    NonFiniteResult,
}

impl fmt::Display for EnergyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnergyError::InvalidTime => write!(f, "Time interval must be positive"),
            EnergyError::NonFiniteResult => write!(f, "Calculation resulted in non-finite value"),
        }
    }
}

impl std::error::Error for EnergyError {}

/// Error types for efficiency calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EfficiencyError {
    /// Efficiency value is outside valid range [0.0, 1.0]
    InvalidEfficiency,
    /// Calculation produced non-finite result (NaN or infinity)
    NonFiniteResult,
}

impl fmt::Display for EfficiencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EfficiencyError::InvalidEfficiency => write!(f, "Efficiency must be between 0.0 and 1.0"),
            EfficiencyError::NonFiniteResult => write!(f, "Calculation resulted in non-finite value"),
        }
    }
}

impl std::error::Error for EfficiencyError {}

/// Error types for limit validation.
#[derive(Debug, Clone, PartialEq)]
pub enum LimitError {
    /// Angular velocity exceeds maximum allowed
    ExceededAngularVelocity,
    /// RPM exceeds maximum allowed  
    ExceededRPM,
    /// Torque exceeds maximum allowed
    ExceededTorque,
    /// Power exceeds maximum allowed
    ExceededPower,
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LimitError::ExceededAngularVelocity => write!(f, "Angular velocity exceeds maximum limit"),
            LimitError::ExceededRPM => write!(f, "Rotational speed exceeds maximum limit"),
            LimitError::ExceededTorque => write!(f, "Torque exceeds maximum limit"),
            LimitError::ExceededPower => write!(f, "Power exceeds maximum limit"),
        }
    }
}

impl std::error::Error for LimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let kinematics_err = RotationalPhysicsError::Kinematics(KinematicsError::InvalidTime);
        assert_eq!(format!("{}", kinematics_err), "Kinematics error: Time interval must be positive");

        let dynamics_err = RotationalPhysicsError::Dynamics(DynamicsError::ZeroInertia);
        assert_eq!(format!("{}", dynamics_err), "Dynamics error: Moment of inertia cannot be zero");
    }
}