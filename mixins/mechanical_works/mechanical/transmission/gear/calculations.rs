//! Gear calculations for rotational speed, torque, and power transformations.
//!
//! This module implements all the mathematical formulas for gear-based mechanical 
//! power transmission calculations including:
//! - Output angular velocity calculation
//! - Output RPM calculation  
//! - Output torque calculation (ideal and with efficiency)
//! - Power conservation calculations
//! - Power loss calculations

use crate::mechanical::core::units::{Torque, RotationalSpeed, AngularVelocity, Power, Efficiency};
use crate::mechanical::transmission::gear::{GearRatio, Gear};
use crate::mechanical::transmission::gear::errors::GearError;

/// Calculates the output angular velocity from input angular velocity and gear ratio.
///
/// Formula: ω_out = ω_in / i
///
/// # Arguments
/// * `input_angular_velocity` - Input angular velocity in rad/s  
/// * `gear_ratio` - The gear ratio (output/input)
///
/// # Returns
/// Output angular velocity in rad/s
pub fn calculate_output_angular_velocity(
    input_angular_velocity: AngularVelocity, 
    gear_ratio: &GearRatio
) -> AngularVelocity {
    let output_angular_velocity = input_angular_velocity.get() / gear_ratio.ratio();
    AngularVelocity::new(output_angular_velocity)
}

/// Calculates the output RPM from input RPM and gear ratio.
///
/// Formula: RPM_out = RPM_in / i
///
/// # Arguments
/// * `input_rpm` - Input rotational speed in RPM
/// * `gear_ratio` - The gear ratio (output/input)
///
/// # Returns
/// Output rotational speed in RPM
pub fn calculate_output_rpm(
    input_rpm: RotationalSpeed, 
    gear_ratio: &GearRatio
) -> RotationalSpeed {
    let output_rpm = input_rpm.get() / gear_ratio.ratio();
    RotationalSpeed::new(output_rpm)
}

/// Calculates the ideal output torque from input torque and gear ratio.
///
/// Formula: τ_out = τ_in * (N_out / N_in) = τ_in * i
///
/// # Arguments
/// * `input_torque` - Input torque in N·m
/// * `gear_ratio` - The gear ratio (output/input)
///
/// # Returns
/// Output torque in N·m
pub fn calculate_output_torque_ideal(
    input_torque: Torque, 
    gear_ratio: &GearRatio
) -> Torque {
    let output_torque = input_torque.get() * gear_ratio.ratio();
    Torque::new(output_torque)
}

/// Calculates the output torque with efficiency consideration.
///
/// Formula: τ_out = τ_in * (N_out / N_in) * η = τ_in * i * η
///
/// # Arguments
/// * `input_torque` - Input torque in N·m
/// * `gear_ratio` - The gear ratio (output/input)
/// * `efficiency` - Efficiency as a fraction (0.0 to 1.0)
///
/// # Returns
/// Output torque in N·m
pub fn calculate_output_torque_with_efficiency(
    input_torque: Torque, 
    gear_ratio: &GearRatio,
    efficiency: Efficiency
) -> Torque {
    let output_torque = input_torque.get() * gear_ratio.ratio() * efficiency.get();
    Torque::new(output_torque)
}

/// Calculates the output power based on input power and efficiency.
///
/// Formula: P_out = P_in * η
///
/// # Arguments
/// * `input_power` - Input power in watts  
/// * `efficiency` - Efficiency as a fraction (0.0 to 1.0)
///
/// # Returns
/// Output power in watts
pub fn calculate_output_power(
    input_power: Power, 
    efficiency: Efficiency
) -> Power {
    let output_power = input_power.get() * efficiency.get();
    Power::new(output_power)
}

/// Calculates the power loss based on input power and efficiency.
///
/// Formula: P_loss = P_in - P_out = P_in * (1 - η)
///
/// # Arguments
/// * `input_power` - Input power in watts
/// * `efficiency` - Efficiency as a fraction (0.0 to 1.0)
///
/// # Returns
/// Power loss in watts
pub fn calculate_power_loss(
    input_power: Power, 
    efficiency: Efficiency
) -> Power {
    let power_loss = input_power.get() * (1.0 - efficiency.get());
    Power::new(power_loss)
}

/// Calculates the rotation direction based on gear ratio.
///
/// For external meshing gears:
/// - If ratio > 1 (speed reduction), output rotates opposite to input
/// - If ratio < 1 (speed increase), output rotates opposite to input  
/// - If ratio = 1 (neutral), output rotates same as input
///
/// # Arguments
/// * `gear_ratio` - The gear ratio (output/input)
/// * `input_direction` - Input rotation direction (+1 for CW, -1 for CCW)
///
/// # Returns
/// Output rotation direction (+1 for CW, -1 for CCW)
pub fn calculate_rotation_direction(
    gear_ratio: &GearRatio,
    input_direction: i8
) -> i8 {
    if gear_ratio.is_reversed() {
        -input_direction
    } else {
        input_direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::core::units::{Torque, RotationalSpeed, AngularVelocity, Power, Efficiency};
    
    #[test]
    fn test_output_angular_velocity_calculation() {
        let input_angular_velocity = AngularVelocity::new(10.0); // 10 rad/s
        let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
        
        let output_angular_velocity = calculate_output_angular_velocity(input_angular_velocity, &gear_ratio);
        assert_eq!(output_angular_velocity.get(), 5.0); // Should be 10 / 2 = 5 rad/s
    }
    
    #[test]
    fn test_output_rpm_calculation() {
        let input_rpm = RotationalSpeed::new(120.0); // 120 RPM
        let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
        
        let output_rpm = calculate_output_rpm(input_rpm, &gear_ratio);
        assert_eq!(output_rpm.get(), 60.0); // Should be 120 / 2 = 60 RPM
    }
    
    #[test]
    fn test_ideal_torque_calculation() {
        let input_torque = Torque::new(10.0); // 10 N·m
        let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
        
        let output_torque = calculate_output_torque_ideal(input_torque, &gear_ratio);
        assert_eq!(output_torque.get(), 20.0); // Should be 10 * 2 = 20 N·m
    }
    
    #[test]
    fn test_torque_with_efficiency() {
        let input_torque = Torque::new(10.0); // 10 N·m
        let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
        let efficiency = Efficiency::new(0.9); // 90% efficiency
        
        let output_torque = calculate_output_torque_with_efficiency(input_torque, &gear_ratio, efficiency);
        assert_eq!(output_torque.get(), 18.0); // Should be 10 * 2 * 0.9 = 18 N·m
    }
    
    #[test]
    fn test_output_power_calculation() {
        let input_power = Power::new(100.0); // 100 W
        let efficiency = Efficiency::new(0.9); // 90% efficiency
        
        let output_power = calculate_output_power(input_power, efficiency);
        assert_eq!(output_power.get(), 90.0); // Should be 100 * 0.9 = 90 W
    }
    
    #[test]
    fn test_power_loss_calculation() {
        let input_power = Power::new(100.0); // 100 W
        let efficiency = Efficiency::new(0.9); // 90% efficiency
        
        let power_loss = calculate_power_loss(input_power, efficiency);
        assert_eq!(power_loss.get(), 10.0); // Should be 100 * (1 - 0.9) = 10 W
    }
    
    #[test]
    fn test_rotation_direction() {
        let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
        
        // Test with CW input (+1)
        let output_direction = calculate_rotation_direction(&gear_ratio, 1);
        assert_eq!(output_direction, -1); // Should be CCW (-1) for external meshing
        
        // Test with CCW input (-1)
        let output_direction = calculate_rotation_direction(&gear_ratio, -1);
        assert_eq!(output_direction, 1); // Should be CW (+1) for external meshing
    }
}