//! Rotational energy calculations for Horizon Mechanical Works.
//!
//! This module provides calculations for mechanical power and energy transfer including:
//! - Power from torque and angular velocity
//! - Energy transferred over time
//! - Work done by rotational systems

use std::fmt;
use crate::mechanical::core::units::{Torque, AngularVelocity, Power, Energy};

/// Calculates mechanical power from torque and angular velocity.
///
/// Formula: P = τ × ω
///
/// # Arguments
/// * `torque` - Torque in newton-metres
/// * `angular_velocity` - Angular velocity in radians per second
///
/// # Returns
/// The mechanical power in watts
///
/// # Errors
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_power(
    torque: Torque,
    angular_velocity: AngularVelocity,
) -> Result<Power, EnergyError> {
    let power = torque.get() * angular_velocity.get();
    
    if power.is_nan() || power.is_infinite() {
        return Err(EnergyError::NonFiniteResult);
    }
    
    Ok(Power::new(power))
}

/// Calculates energy transferred over a time interval with constant power.
///
/// Formula: E = P × t
///
/// # Arguments
/// * `power` - Constant mechanical power in watts
/// * `time_interval` - Time interval in seconds
///
/// # Returns
/// The energy transferred in joules
///
/// # Errors
/// * `InvalidTime` - When time interval is zero or negative
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_energy_transferred(
    power: Power,
    time_interval: f64,
) -> Result<Energy, EnergyError> {
    if time_interval <= 0.0 {
        return Err(EnergyError::InvalidTime);
    }
    
    let energy = power.get() * time_interval;
    
    if energy.is_nan() || energy.is_infinite() {
        return Err(EnergyError::NonFiniteResult);
    }
    
    Ok(Energy::new(energy))
}

/// Calculates energy from torque and angular displacement (work done).
///
/// Formula: E = τ × θ
///
/// # Arguments
/// * `torque` - Torque in newton-metres
/// * `angular_displacement` - Angular displacement in radians
///
/// # Returns
/// The energy transferred in joules
///
/// # Errors
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_energy_from_torque_and_displacement(
    torque: Torque,
    angular_displacement: f64,
) -> Result<Energy, EnergyError> {
    let energy = torque.get() * angular_displacement;
    
    if energy.is_nan() || energy.is_infinite() {
        return Err(EnergyError::NonFiniteResult);
    }
    
    Ok(Energy::new(energy))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_calculation() {
        let torque = Torque::new(5.0);
        let angular_velocity = AngularVelocity::new(2.0);
        
        let power = calculate_power(torque, angular_velocity).unwrap();
        assert_eq!(power.get(), 10.0);
    }

    #[test]
    fn test_power_calculation_zero_velocity() {
        let torque = Torque::new(5.0);
        let angular_velocity = AngularVelocity::new(0.0);
        
        let power = calculate_power(torque, angular_velocity).unwrap();
        assert_eq!(power.get(), 0.0);
    }

    #[test]
    fn test_energy_transferred_calculation() {
        let power = Power::new(10.0);
        let time_interval = 5.0;
        
        let energy = calculate_energy_transferred(power, time_interval).unwrap();
        assert_eq!(energy.get(), 50.0);
    }

    #[test]
    fn test_energy_transferred_invalid_time() {
        let power = Power::new(10.0);
        
        let result = calculate_energy_transferred(power, 0.0);
        assert!(matches!(result, Err(EnergyError::InvalidTime)));
    }

    #[test]
    fn test_energy_from_torque_and_displacement() {
        let torque = Torque::new(5.0);
        let angular_displacement = 2.0;
        
        let energy = calculate_energy_from_torque_and_displacement(torque, angular_displacement).unwrap();
        assert_eq!(energy.get(), 10.0);
    }
}