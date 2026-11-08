//! Rotational operating limits for Horizon Mechanical Works.
//!
//! This module provides validation of rotational operating conditions including:
//! - Maximum angular velocity limits
//! - Maximum RPM limits
//! - Maximum torque limits
//! - Maximum power limits
//! - Load capacity checks

use std::fmt;
use crate::mechanical::core::units::{AngularVelocity, RotationalSpeed, Torque, Power};

/// Configuration for rotational operating limits.
#[derive(Debug, Clone)]
pub struct RotationalLimits {
    /// Maximum angular velocity in radians per second
    pub max_angular_velocity: AngularVelocity,
    /// Maximum rotational speed in revolutions per minute  
    pub max_rpm: RotationalSpeed,
    /// Maximum torque in newton-metres
    pub max_torque: Torque,
    /// Maximum power in watts
    pub max_power: Power,
}

impl RotationalLimits {
    /// Creates a new set of rotational limits.
    pub fn new(
        max_angular_velocity: AngularVelocity,
        max_rpm: RotationalSpeed,
        max_torque: Torque,
        max_power: Power,
    ) -> Self {
        RotationalLimits {
            max_angular_velocity,
            max_rpm,
            max_torque,
            max_power,
        }
    }
}

/// Validates if a rotational condition is within configured limits.
///
/// # Arguments
/// * `angular_velocity` - Angular velocity to check
/// * `rpm` - Rotational speed to check  
/// * `torque` - Torque to check
/// * `power` - Power to check
/// * `limits` - Configured operating limits
///
/// # Returns
/// `Ok(())` if all conditions are within limits, or an error describing which limit was exceeded
pub fn validate_rotational_conditions(
    angular_velocity: AngularVelocity,
    rpm: RotationalSpeed,
    torque: Torque,
    power: Power,
    limits: &RotationalLimits,
) -> Result<(), LimitError> {
    if angular_velocity > limits.max_angular_velocity {
        return Err(LimitError::ExceededAngularVelocity);
    }
    
    if rpm > limits.max_rpm {
        return Err(LimitError::ExceededRPM);
    }
    
    if torque > limits.max_torque {
        return Err(LimitError::ExceededTorque);
    }
    
    if power > limits.max_power {
        return Err(LimitError::ExceededPower);
    }
    
    Ok(())
}

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
    fn test_limits_within_bounds() {
        let limits = RotationalLimits::new(
            AngularVelocity::new(100.0),
            RotationalSpeed::new(1000.0),
            Torque::new(50.0),
            Power::new(1000.0),
        );
        
        let result = validate_rotational_conditions(
            AngularVelocity::new(50.0),
            RotationalSpeed::new(500.0),
            Torque::new(25.0),
            Power::new(500.0),
            &limits,
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_limits_exceeded_angular_velocity() {
        let limits = RotationalLimits::new(
            AngularVelocity::new(10.0),
            RotationalSpeed::new(1000.0),
            Torque::new(50.0),
            Power::new(1000.0),
        );
        
        let result = validate_rotational_conditions(
            AngularVelocity::new(15.0),
            RotationalSpeed::new(500.0),
            Torque::new(25.0),
            Power::new(500.0),
            &limits,
        );
        
        assert!(matches!(result, Err(LimitError::ExceededAngularVelocity)));
    }

    #[test]
    fn test_limits_exceeded_rpm() {
        let limits = RotationalLimits::new(
            AngularVelocity::new(100.0),
            RotationalSpeed::new(100.0),
            Torque::new(50.0),
            Power::new(1000.0),
        );
        
        let result = validate_rotational_conditions(
            AngularVelocity::new(50.0),
            RotationalSpeed::new(150.0),
            Torque::new(25.0),
            Power::new(500.0),
            &limits,
        );
        
        assert!(matches!(result, Err(LimitError::ExceededRPM)));
    }

    #[test]
    fn test_limits_exceeded_torque() {
        let limits = RotationalLimits::new(
            AngularVelocity::new(100.0),
            RotationalSpeed::new(1000.0),
            Torque::new(10.0),
            Power::new(1000.0),
        );
        
        let result = validate_rotational_conditions(
            AngularVelocity::new(50.0),
            RotationalSpeed::new(500.0),
            Torque::new(15.0),
            Power::new(500.0),
            &limits,
        );
        
        assert!(matches!(result, Err(LimitError::ExceededTorque)));
    }

    #[test]
    fn test_limits_exceeded_power() {
        let limits = RotationalLimits::new(
            AngularVelocity::new(100.0),
            RotationalSpeed::new(1000.0),
            Torque::new(50.0),
            Power::new(100.0),
        );
        
        let result = validate_rotational_conditions(
            AngularVelocity::new(50.0),
            RotationalSpeed::new(500.0),
            Torque::new(25.0),
            Power::new(150.0),
            &limits,
        );
        
        assert!(matches!(result, Err(LimitError::ExceededPower)));
    }
}