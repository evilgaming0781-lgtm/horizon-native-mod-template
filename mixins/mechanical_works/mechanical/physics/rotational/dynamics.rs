//! Rotational dynamics calculations for Horizon Mechanical Works.
//!
//! This module provides calculations for rotational forces and torques including:
//! - Net torque calculation
//! - Angular acceleration from torque and inertia
//! - Rotational load calculations

use std::fmt;
use crate::mechanical::core::units::{Torque, AngularVelocity, AngularAcceleration};

/// Moment of inertia measured in kilogram-meter squared (kg·m²).
///
/// Moment of inertia represents an object's resistance to changes in its rotational motion.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MomentOfInertia(pub f64);

impl fmt::Display for MomentOfInertia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} kg·m²", self.0)
    }
}

impl Default for MomentOfInertia {
    fn default() -> Self {
        MomentOfInertia(0.0)
    }
}

impl MomentOfInertia {
    /// Creates a new moment of inertia value in kilogram-meter squared.
    pub fn new(inertia: f64) -> Self {
        if inertia.is_nan() || inertia.is_infinite() || inertia <= 0.0 {
            panic!("Moment of inertia must be positive, finite value");
        }
        MomentOfInertia(inertia)
    }

    /// Returns the moment of inertia value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the moment of inertia value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Calculates net torque from angular acceleration and moment of inertia.
///
/// Formula: τ_net = I × α
///
/// # Arguments
/// * `moment_of_inertia` - Moment of inertia in kg·m²
/// * `angular_acceleration` - Angular acceleration in rad/s²
///
/// # Returns
/// The net torque in newton-metres
///
/// # Errors
/// * `ZeroInertia` - When moment of inertia is zero or effectively zero
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_net_torque(
    moment_of_inertia: MomentOfInertia,
    angular_acceleration: AngularAcceleration,
) -> Result<Torque, DynamicsError> {
    if moment_of_inertia.is_zero() {
        return Err(DynamicsError::ZeroInertia);
    }

    let torque = moment_of_inertia.get() * angular_acceleration.get();
    
    if torque.is_nan() || torque.is_infinite() {
        return Err(DynamicsError::NonFiniteResult);
    }
    
    Ok(Torque::new(torque))
}

/// Calculates angular acceleration from net torque and moment of inertia.
///
/// Formula: α = τ_net / I
///
/// # Arguments
/// * `net_torque` - Net torque in newton-metres
/// * `moment_of_inertia` - Moment of inertia in kg·m²
///
/// # Returns
/// The angular acceleration in radians per second squared
///
/// # Errors
/// * `ZeroInertia` - When moment of inertia is zero or effectively zero
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_angular_acceleration_from_torque(
    net_torque: Torque,
    moment_of_inertia: MomentOfInertia,
) -> Result<AngularAcceleration, DynamicsError> {
    if moment_of_inertia.is_zero() {
        return Err(DynamicsError::ZeroInertia);
    }

    let acceleration = net_torque.get() / moment_of_inertia.get();
    
    if acceleration.is_nan() || acceleration.is_infinite() {
        return Err(DynamicsError::NonFiniteResult);
    }
    
    Ok(AngularAcceleration::new(acceleration))
}

/// Calculates rotational load based on torque and angular velocity.
///
/// This represents the effective load that a rotating system must overcome.
///
/// # Arguments
/// * `torque` - Applied torque in newton-metres  
/// * `angular_velocity` - Angular velocity in radians per second
///
/// # Returns
/// The rotational load (dimensionless, or normalized)
///
/// # Errors
/// * `ZeroVelocity` - When angular velocity is zero (cannot calculate load)
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_rotational_load(
    torque: Torque,
    angular_velocity: AngularVelocity,
) -> Result<f64, DynamicsError> {
    if angular_velocity.is_zero() {
        return Err(DynamicsError::ZeroVelocity);
    }

    let load = torque.get() / angular_velocity.get();
    
    if load.is_nan() || load.is_infinite() {
        return Err(DynamicsError::NonFiniteResult);
    }
    
    Ok(load)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_torque_calculation() {
        let moment_of_inertia = MomentOfInertia::new(2.0);
        let angular_acceleration = AngularAcceleration::new(3.0);
        
        let torque = calculate_net_torque(moment_of_inertia, angular_acceleration).unwrap();
        assert_eq!(torque.get(), 6.0);
    }

    #[test]
    fn test_net_torque_zero_inertia() {
        let moment_of_inertia = MomentOfInertia::new(0.0);
        let angular_acceleration = AngularAcceleration::new(3.0);
        
        let result = calculate_net_torque(moment_of_inertia, angular_acceleration);
        assert!(matches!(result, Err(DynamicsError::ZeroInertia)));
    }

    #[test]
    fn test_angular_acceleration_from_torque() {
        let net_torque = Torque::new(10.0);
        let moment_of_inertia = MomentOfInertia::new(2.0);
        
        let acceleration = calculate_angular_acceleration_from_torque(net_torque, moment_of_inertia).unwrap();
        assert_eq!(acceleration.get(), 5.0);
    }

    #[test]
    fn test_angular_acceleration_from_torque_zero_inertia() {
        let net_torque = Torque::new(10.0);
        let moment_of_inertia = MomentOfInertia::new(0.0);
        
        let result = calculate_angular_acceleration_from_torque(net_torque, moment_of_inertia);
        assert!(matches!(result, Err(DynamicsError::ZeroInertia)));
    }

    #[test]
    fn test_rotational_load_calculation() {
        let torque = Torque::new(10.0);
        let angular_velocity = AngularVelocity::new(2.0);
        
        let load = calculate_rotational_load(torque, angular_velocity).unwrap();
        assert_eq!(load, 5.0);
    }

    #[test]
    fn test_rotational_load_zero_velocity() {
        let torque = Torque::new(10.0);
        let angular_velocity = AngularVelocity::new(0.0);
        
        let result = calculate_rotational_load(torque, angular_velocity);
        assert!(matches!(result, Err(DynamicsError::ZeroVelocity)));
    }
}