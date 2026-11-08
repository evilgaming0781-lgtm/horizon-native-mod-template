//! Rotational efficiency calculations for Horizon Mechanical Works.
//!
//! This module provides modeling of mechanical efficiency and losses including:
//! - Efficiency ratios
//! - Power loss calculations
//! - Energy loss calculations

use std::fmt;
use crate::mechanical::core::units::{Power, Efficiency};

/// Calculates output power from input power and efficiency.
///
/// Formula: P_out = P_in × η
///
/// # Arguments
/// * `input_power` - Input mechanical power in watts
/// * `efficiency` - Efficiency as a fraction (0.0 to 1.0)
///
/// # Returns
/// The output mechanical power in watts
///
/// # Errors
/// * `InvalidEfficiency` - When efficiency is outside valid range [0.0, 1.0]
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_output_power(
    input_power: Power,
    efficiency: Efficiency,
) -> Result<Power, EfficiencyError> {
    if efficiency.get() < 0.0 || efficiency.get() > 1.0 {
        return Err(EfficiencyError::InvalidEfficiency);
    }

    let output_power = input_power.get() * efficiency.get();
    
    if output_power.is_nan() || output_power.is_infinite() {
        return Err(EfficiencyError::NonFiniteResult);
    }
    
    Ok(Power::new(output_power))
}

/// Calculates power loss from input power and efficiency.
///
/// Formula: P_loss = P_in - P_out = P_in × (1 - η)
///
/// # Arguments
/// * `input_power` - Input mechanical power in watts
/// * `efficiency` - Efficiency as a fraction (0.0 to 1.0)
///
/// # Returns
/// The power loss in watts
///
/// # Errors
/// * `InvalidEfficiency` - When efficiency is outside valid range [0.0, 1.0]
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_power_loss(
    input_power: Power,
    efficiency: Efficiency,
) -> Result<Power, EfficiencyError> {
    if efficiency.get() < 0.0 || efficiency.get() > 1.0 {
        return Err(EfficiencyError::InvalidEfficiency);
    }

    let power_loss = input_power.get() * (1.0 - efficiency.get());
    
    if power_loss.is_nan() || power_loss.is_infinite() {
        return Err(EfficiencyError::NonFiniteResult);
    }
    
    Ok(Power::new(power_loss))
}

/// Calculates energy loss from input energy and efficiency.
///
/// Formula: E_loss = E_in × (1 - η)
///
/// # Arguments
/// * `input_energy` - Input mechanical energy in joules
/// * `efficiency` - Efficiency as a fraction (0.0 to 1.0)
///
/// # Returns
/// The energy loss in joules
///
/// # Errors
/// * `InvalidEfficiency` - When efficiency is outside valid range [0.0, 1.0]
/// * `NonFiniteResult` - When calculation produces NaN or infinity
pub fn calculate_energy_loss(
    input_energy: Energy,
    efficiency: Efficiency,
) -> Result<Energy, EfficiencyError> {
    if efficiency.get() < 0.0 || efficiency.get() > 1.0 {
        return Err(EfficiencyError::InvalidEfficiency);
    }

    let energy_loss = input_energy.get() * (1.0 - efficiency.get());
    
    if energy_loss.is_nan() || energy_loss.is_infinite() {
        return Err(EfficiencyError::NonFiniteResult);
    }
    
    Ok(Energy::new(energy_loss))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_power_calculation() {
        let input_power = Power::new(100.0);
        let efficiency = Efficiency::new(0.8);
        
        let output_power = calculate_output_power(input_power, efficiency).unwrap();
        assert_eq!(output_power.get(), 80.0);
    }

    #[test]
    fn test_output_power_zero_efficiency() {
        let input_power = Power::new(100.0);
        let efficiency = Efficiency::new(0.0);
        
        let output_power = calculate_output_power(input_power, efficiency).unwrap();
        assert_eq!(output_power.get(), 0.0);
    }

    #[test]
    fn test_output_power_invalid_efficiency() {
        let input_power = Power::new(100.0);
        let efficiency = Efficiency::new(1.5);
        
        let result = calculate_output_power(input_power, efficiency);
        assert!(matches!(result, Err(EfficiencyError::InvalidEfficiency)));
    }

    #[test]
    fn test_power_loss_calculation() {
        let input_power = Power::new(100.0);
        let efficiency = Efficiency::new(0.8);
        
        let power_loss = calculate_power_loss(input_power, efficiency).unwrap();
        assert_eq!(power_loss.get(), 20.0);
    }

    #[test]
    fn test_energy_loss_calculation() {
        let input_energy = Energy::new(100.0);
        let efficiency = Efficiency::new(0.8);
        
        let energy_loss = calculate_energy_loss(input_energy, efficiency).unwrap();
        assert_eq!(energy_loss.get(), 20.0);
    }
}