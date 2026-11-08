//! Rotational kinematics calculations for Horizon Mechanical Works.
//!
//! This module provides calculations for rotational motion parameters including:
//! - Angular velocity conversions
//! - Angular acceleration
//! - Angular displacement
//! - Time interval validation

use std::fmt;
use crate::mechanical::core::units::{AngularVelocity, RotationalSpeed};

/// Angular acceleration measured in radians per second squared (rad/s²).
///
/// Angular acceleration represents the rate of change of angular velocity.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AngularAcceleration(pub f64);

impl fmt::Display for AngularAcceleration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} rad/s²", self.0)
    }
}

impl Default for AngularAcceleration {
    fn default() -> Self {
        AngularAcceleration(0.0)
    }
}

impl AngularAcceleration {
    /// Creates a new angular acceleration value in radians per second squared.
    pub fn new(acceleration: f64) -> Self {
        if acceleration.is_nan() || acceleration.is_infinite() || !acceleration.is_finite() {
            panic!("Angular acceleration cannot be NaN, infinite, or non-finite");
        }
        AngularAcceleration(acceleration)
    }

    /// Returns the angular acceleration value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the angular acceleration value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Calculates angular acceleration from initial and final angular velocities over time.
///
/// Formula: α = (ωf - ωi) / Δt
///
/// # Arguments
/// * `initial_velocity` - Initial angular velocity in radians per second
/// * `final_velocity` - Final angular velocity in radians per second  
/// * `time_interval` - Time interval in seconds
///
/// # Returns
/// The angular acceleration in radians per second squared
///
/// # Errors
/// * `InvalidTime` - When time interval is zero or negative
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_angular_acceleration(
    initial_velocity: AngularVelocity,
    final_velocity: AngularVelocity,
    time_interval: f64,
) -> Result<AngularAcceleration, KinematicsError> {
    if time_interval <= 0.0 {
        return Err(KinematicsError::InvalidTime);
    }

    let acceleration = (final_velocity.get() - initial_velocity.get()) / time_interval;
    
    if acceleration.is_nan() || acceleration.is_infinite() {
        return Err(KinematicsError::NonFiniteResult);
    }
    
    Ok(AngularAcceleration::new(acceleration))
}

/// Calculates angular displacement under constant angular velocity.
///
/// Formula: θ = ω × t
///
/// # Arguments
/// * `angular_velocity` - Constant angular velocity in radians per second
/// * `time_interval` - Time interval in seconds
///
/// # Returns
/// The angular displacement in radians
///
/// # Errors
/// * `InvalidTime` - When time interval is zero or negative
pub fn calculate_angular_displacement(
    angular_velocity: AngularVelocity,
    time_interval: f64,
) -> Result<f64, KinematicsError> {
    if time_interval <= 0.0 {
        return Err(KinematicsError::InvalidTime);
    }
    
    let displacement = angular_velocity.get() * time_interval;
    Ok(displacement)
}

/// Calculates final angular velocity under constant angular acceleration.
///
/// Formula: ωf = ωi + α × t
///
/// # Arguments
/// * `initial_velocity` - Initial angular velocity in radians per second
/// * `angular_acceleration` - Angular acceleration in radians per second squared
/// * `time_interval` - Time interval in seconds
///
/// # Returns
/// The final angular velocity in radians per second
///
/// # Errors
/// * `InvalidTime` - When time interval is zero or negative
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_final_angular_velocity(
    initial_velocity: AngularVelocity,
    angular_acceleration: AngularAcceleration,
    time_interval: f64,
) -> Result<AngularVelocity, KinematicsError> {
    if time_interval <= 0.0 {
        return Err(KinematicsError::InvalidTime);
    }

    let final_velocity = initial_velocity.get() + angular_acceleration.get() * time_interval;
    
    if final_velocity.is_nan() || final_velocity.is_infinite() {
        return Err(KinematicsError::NonFiniteResult);
    }
    
    Ok(AngularVelocity::new(final_velocity))
}

/// Converts RPM to radians per second.
///
/// Formula: ω = rpm × 2π / 60
///
/// # Arguments
/// * `rpm` - Rotational speed in revolutions per minute
///
/// # Returns
/// Angular velocity in radians per second
pub fn rpm_to_rad_per_sec(rpm: RotationalSpeed) -> AngularVelocity {
    let rad_per_sec = rpm.get() * 2.0 * std::f64::consts::PI / 60.0;
    AngularVelocity::new(rad_per_sec)
}

/// Converts radians per second to RPM.
///
/// Formula: rpm = ω × 60 / 2π
///
/// # Arguments
/// * `rad_per_sec` - Angular velocity in radians per second
///
/// # Returns
/// Rotational speed in revolutions per minute
pub fn rad_per_sec_to_rpm(rad_per_sec: AngularVelocity) -> RotationalSpeed {
    let rpm = rad_per_sec.get() * 60.0 / (2.0 * std::f64::consts::PI);
    RotationalSpeed::new(rpm)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpm_to_rad_per_sec_conversion() {
        let rpm = RotationalSpeed::new(60.0);
        let rad_per_sec = rpm_to_rad_per_sec(rpm);
        assert!((rad_per_sec.get() - 2.0 * std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_rad_per_sec_to_rpm_conversion() {
        let rad_per_sec = AngularVelocity::new(2.0 * std::f64::consts::PI);
        let rpm = rad_per_sec_to_rpm(rad_per_sec);
        assert!((rpm.get() - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_round_trip_conversion() {
        let rpm = RotationalSpeed::new(120.0);
        let rad_per_sec = rpm_to_rad_per_sec(rpm);
        let back_to_rpm = rad_per_sec_to_rpm(rad_per_sec);
        assert!((rpm.get() - back_to_rpm.get()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_angular_acceleration_calculation() {
        let initial_velocity = AngularVelocity::new(0.0);
        let final_velocity = AngularVelocity::new(10.0);
        let time_interval = 2.0;
        
        let acceleration = calculate_angular_acceleration(initial_velocity, final_velocity, time_interval).unwrap();
        assert_eq!(acceleration.get(), 5.0);
    }

    #[test]
    fn test_angular_acceleration_invalid_time() {
        let initial_velocity = AngularVelocity::new(0.0);
        let final_velocity = AngularVelocity::new(10.0);
        
        let result = calculate_angular_acceleration(initial_velocity, final_velocity, 0.0);
        assert!(matches!(result, Err(KinematicsError::InvalidTime)));
    }

    #[test]
    fn test_angular_displacement_calculation() {
        let angular_velocity = AngularVelocity::new(5.0);
        let time_interval = 2.0;
        
        let displacement = calculate_angular_displacement(angular_velocity, time_interval).unwrap();
        assert_eq!(displacement, 10.0);
    }

    #[test]
    fn test_angular_displacement_invalid_time() {
        let angular_velocity = AngularVelocity::new(5.0);
        
        let result = calculate_angular_displacement(angular_velocity, 0.0);
        assert!(matches!(result, Err(KinematicsError::InvalidTime)));
    }

    #[test]
    fn test_final_angular_velocity_calculation() {
        let initial_velocity = AngularVelocity::new(2.0);
        let angular_acceleration = AngularAcceleration::new(3.0);
        let time_interval = 2.0;
        
        let final_velocity = calculate_final_angular_velocity(initial_velocity, angular_acceleration, time_interval).unwrap();
        assert_eq!(final_velocity.get(), 8.0);
    }

    #[test]
    fn test_final_angular_velocity_invalid_time() {
        let initial_velocity = AngularVelocity::new(2.0);
        let angular_acceleration = AngularAcceleration::new(3.0);
        
        let result = calculate_final_angular_velocity(initial_velocity, angular_acceleration, 0.0);
        assert!(matches!(result, Err(KinematicsError::InvalidTime)));
    }
}