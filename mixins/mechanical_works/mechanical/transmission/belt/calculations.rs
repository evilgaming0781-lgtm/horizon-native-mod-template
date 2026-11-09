//! Belt transmission calculations.
//!
//! This module implements the physical calculations for belt transmission systems.

use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity, Efficiency};
use crate::mechanical::transmission::belt::{pair::BeltPair, ratio::BeltRatio};

/// Calculates the linear velocity of a belt.
///
/// Linear velocity = ω_in × r_in = ω_out × r_out
///
/// # Arguments
/// * `input_rpm` - Input rotational speed in RPM
/// * `input_radius` - Input pulley radius in meters
///
/// # Returns
/// The linear velocity of the belt in meters per second
pub fn calculate_belt_linear_velocity(input_rpm: RotationalSpeed, input_radius: f64) -> f64 {
    let angular_velocity = input_rpm.to_rad_per_sec();
    angular_velocity.get() * input_radius
}

/// Calculates the output angular velocity based on input conditions.
///
/// For ideal belt transmission:
/// ω_out = ω_in × (r_in / r_out)
///
/// # Arguments
/// * `input_angular_velocity` - Input angular velocity in rad/s
/// * `input_radius` - Input pulley radius in meters
/// * `output_radius` - Output pulley radius in meters
/// * `category` - Belt arrangement type (determines rotation direction)
///
/// # Returns
/// The output angular velocity in rad/s
pub fn calculate_output_angular_velocity(
    input_angular_velocity: AngularVelocity,
    input_radius: f64,
    output_radius: f64,
    category: crate::mechanical::transmission::belt::types::BeltCategory,
) -> AngularVelocity {
    let ratio = if input_radius > 0.0 {
        output_radius / input_radius
    } else {
        1.0
    };
    
    // For crossed belts, rotation direction reverses
    let adjusted_ratio = match category {
        crate::mechanical::transmission::belt::types::BeltCategory::CrossedBelt => -ratio,
        _ => ratio,
    };
    
    let output_angular_velocity = input_angular_velocity.get() * adjusted_ratio;
    AngularVelocity::new(output_angular_velocity)
}

/// Calculates the output RPM based on input conditions.
///
/// For ideal belt transmission:
/// RPM_out = RPM_in × (r_in / r_out)
///
/// # Arguments
/// * `input_rpm` - Input rotational speed in RPM
/// * `input_radius` - Input pulley radius in meters
/// * `output_radius` - Output pulley radius in meters
/// * `category` - Belt arrangement type (determines rotation direction)
///
/// # Returns
/// The output rotational speed in RPM
pub fn calculate_output_rpm(
    input_rpm: RotationalSpeed,
    input_radius: f64,
    output_radius: f64,
    category: crate::mechanical::transmission::belt::types::BeltCategory,
) -> RotationalSpeed {
    let ratio = if input_radius > 0.0 {
        output_radius / input_radius
    } else {
        1.0
    };
    
    // For crossed belts, rotation direction reverses (but RPM is always positive)
    let adjusted_ratio = match category {
        crate::mechanical::transmission::belt::types::BeltCategory::CrossedBelt => -ratio,
        _ => ratio,
    };
    
    let output_rpm = input_rpm.get() * adjusted_ratio.abs();
    RotationalSpeed::new(output_rpm)
}

/// Calculates the ideal output torque based on input conditions.
///
/// For ideal belt transmission:
/// τ_out = τ_in × (r_out / r_in)
///
/// # Arguments
/// * `input_torque` - Input torque in N·m
/// * `input_radius` - Input pulley radius in meters
/// * `output_radius` - Output pulley radius in meters
///
/// # Returns
/// The ideal output torque in N·m
pub fn calculate_ideal_output_torque(
    input_torque: Torque,
    input_radius: f64,
    output_radius: f64,
) -> Torque {
    let ratio = if input_radius > 0.0 {
        output_radius / input_radius
    } else {
        1.0
    };
    
    let ideal_output_torque = input_torque.get() * ratio;
    Torque::new(ideal_output_torque)
}

/// Calculates the efficiency-adjusted output torque.
///
/// τ_out = τ_in × (r_out / r_in) × η
///
/// # Arguments
/// * `input_torque` - Input torque in N·m
/// * `input_radius` - Input pulley radius in meters
/// * `output_radius` - Output pulley radius in meters
/// * `efficiency` - Transmission efficiency (0.0 to 1.0)
///
/// # Returns
/// The efficiency-adjusted output torque in N·m
pub fn calculate_output_torque(
    input_torque: Torque,
    input_radius: f64,
    output_radius: f64,
    efficiency: Efficiency,
) -> Torque {
    let ratio = if input_radius > 0.0 {
        output_radius / input_radius
    } else {
        1.0
    };
    
    let ideal_output_torque = input_torque.get() * ratio;
    let adjusted_output_torque = ideal_output_torque * efficiency.get();
    Torque::new(adjusted_output_torque)
}

/// Calculates the output power based on input conditions and efficiency.
///
/// P_out = P_in × η
///
/// # Arguments
/// * `input_power` - Input power in watts
/// * `efficiency` - Transmission efficiency (0.0 to 1.0)
///
/// # Returns
/// The output power in watts
pub fn calculate_output_power(input_power: Power, efficiency: Efficiency) -> Power {
    let output_power = input_power.get() * efficiency.get();
    Power::new(output_power)
}

/// Calculates the power loss due to transmission inefficiency.
///
/// P_loss = P_in - P_out = P_in × (1 - η)
///
/// # Arguments
/// * `input_power` - Input power in watts
/// * `efficiency` - Transmission efficiency (0.0 to 1.0)
///
/// # Returns
/// The power loss in watts
pub fn calculate_power_loss(input_power: Power, efficiency: Efficiency) -> Power {
    let power_loss = input_power.get() * (1.0 - efficiency.get());
    Power::new(power_loss)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity, Efficiency};
    use crate::mechanical::transmission::belt::types::{BeltCategory};

    #[test]
    fn test_belt_linear_velocity_calculation() {
        let rpm = RotationalSpeed::new(120.0);
        let radius = 0.10;
        let velocity = calculate_belt_linear_velocity(rpm, radius);
        // v = ω × r = (120 * 2π / 60) * 0.10 = 4π * 0.10 ≈ 1.257 m/s
        assert!((velocity - 1.257).abs() < 0.001);
    }

    #[test]
    fn test_output_angular_velocity_calculation() {
        let input_omega = AngularVelocity::new(12.566); // 2π rad/s
        let input_radius = 0.10;
        let output_radius = 0.20;
        let category = BeltCategory::OpenBelt;
        
        let result = calculate_output_angular_velocity(input_omega, input_radius, output_radius, category);
        // ω_out = ω_in × (r_in / r_out) = 12.566 * (0.10 / 0.20) = 6.283 rad/s
        assert!((result.get() - 6.283).abs() < 0.001);
    }

    #[test]
    fn test_output_rpm_calculation() {
        let input_rpm = RotationalSpeed::new(120.0);
        let input_radius = 0.10;
        let output_radius = 0.20;
        let category = BeltCategory::OpenBelt;
        
        let result = calculate_output_rpm(input_rpm, input_radius, output_radius, category);
        // RPM_out = RPM_in × (r_in / r_out) = 120 * (0.10 / 0.20) = 60 RPM
        assert!((result.get() - 60.0).abs() < 0.001);
    }

    #[test]
    fn test_ideal_output_torque_calculation() {
        let input_torque = Torque::new(10.0);
        let input_radius = 0.10;
        let output_radius = 0.20;
        
        let result = calculate_ideal_output_torque(input_torque, input_radius, output_radius);
        // τ_out = τ_in × (r_out / r_in) = 10 * (0.20 / 0.10) = 20 N·m
        assert!((result.get() - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_output_torque_with_efficiency() {
        let input_torque = Torque::new(10.0);
        let input_radius = 0.10;
        let output_radius = 0.20;
        let efficiency = Efficiency::new(0.90);
        
        let result = calculate_output_torque(input_torque, input_radius, output_radius, efficiency);
        // τ_out = τ_in × (r_out / r_in) × η = 10 * (0.20 / 0.10) * 0.90 = 18 N·m
        assert!((result.get() - 18.0).abs() < 0.001);
    }

    #[test]
    fn test_output_power_calculation() {
        let input_power = Power::new(100.0);
        let efficiency = Efficiency::new(0.90);
        
        let result = calculate_output_power(input_power, efficiency);
        // P_out = P_in × η = 100 * 0.90 = 90 W
        assert!((result.get() - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_power_loss_calculation() {
        let input_power = Power::new(100.0);
        let efficiency = Efficiency::new(0.90);
        
        let result = calculate_power_loss(input_power, efficiency);
        // P_loss = P_in × (1 - η) = 100 * (1 - 0.90) = 10 W
        assert!((result.get() - 10.0).abs() < 0.001);
    }
}