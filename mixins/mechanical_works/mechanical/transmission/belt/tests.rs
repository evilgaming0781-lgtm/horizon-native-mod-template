//! Tests for the belt transmission system.
//!
//! This module contains unit tests for all aspects of the belt subsystem.

use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity, Efficiency};
use crate::mechanical::transmission::belt::{
    types::{BeltId, BeltCategory, BeltConfiguration},
    pulley::Pulley,
    pair::BeltPair,
    ratio::BeltRatio,
    belt::Belt,
    calculations,
    validation,
    errors::BeltError,
};

#[test]
fn test_belt_linear_velocity_calculation() {
    let rpm = RotationalSpeed::new(120.0);
    let radius = 0.10;
    let velocity = calculations::calculate_belt_linear_velocity(rpm, radius);
    // v = ω × r = (120 * 2π / 60) * 0.10 = 4π * 0.10 ≈ 1.257 m/s
    assert!((velocity - 1.257).abs() < 0.001);
}

#[test]
fn test_output_angular_velocity_calculation() {
    let input_omega = AngularVelocity::new(12.566); // 2π rad/s
    let input_radius = 0.10;
    let output_radius = 0.20;
    let category = BeltCategory::OpenBelt;
    
    let result = calculations::calculate_output_angular_velocity(input_omega, input_radius, output_radius, category);
    // ω_out = ω_in × (r_in / r_out) = 12.566 * (0.10 / 0.20) = 6.283 rad/s
    assert!((result.get() - 6.283).abs() < 0.001);
}

#[test]
fn test_output_rpm_calculation_speed_reduction() {
    let input_rpm = RotationalSpeed::new(120.0);
    let input_radius = 0.10;
    let output_radius = 0.20;
    let category = BeltCategory::OpenBelt;
    
    let result = calculations::calculate_output_rpm(input_rpm, input_radius, output_radius, category);
    // RPM_out = RPM_in × (r_in / r_out) = 120 * (0.10 / 0.20) = 60 RPM
    assert!((result.get() - 60.0).abs() < 0.001);
}

#[test]
fn test_output_rpm_calculation_speed_increase() {
    let input_rpm = RotationalSpeed::new(60.0);
    let input_radius = 0.20;
    let output_radius = 0.10;
    let category = BeltCategory::OpenBelt;
    
    let result = calculations::calculate_output_rpm(input_rpm, input_radius, output_radius, category);
    // RPM_out = RPM_in × (r_in / r_out) = 60 * (0.20 / 0.10) = 120 RPM
    assert!((result.get() - 120.0).abs() < 0.001);
}

#[test]
fn test_ideal_output_torque_calculation() {
    let input_torque = Torque::new(10.0);
    let input_radius = 0.10;
    let output_radius = 0.20;
    
    let result = calculations::calculate_ideal_output_torque(input_torque, input_radius, output_radius);
    // τ_out = τ_in × (r_out / r_in) = 10 * (0.20 / 0.10) = 20 N·m
    assert!((result.get() - 20.0).abs() < 0.001);
}

#[test]
fn test_output_torque_with_efficiency() {
    let input_torque = Torque::new(10.0);
    let input_radius = 0.10;
    let output_radius = 0.20;
    let efficiency = Efficiency::new(0.90);
    
    let result = calculations::calculate_output_torque(input_torque, input_radius, output_radius, efficiency);
    // τ_out = τ_in × (r_out / r_in) × η = 10 * (0.20 / 0.10) * 0.90 = 18 N·m
    assert!((result.get() - 18.0).abs() < 0.001);
}

#[test]
fn test_output_power_calculation() {
    let input_power = Power::new(100.0);
    let efficiency = Efficiency::new(0.90);
    
    let result = calculations::calculate_output_power(input_power, efficiency);
    // P_out = P_in × η = 100 * 0.90 = 90 W
    assert!((result.get() - 90.0).abs() < 0.001);
}

#[test]
fn test_power_loss_calculation() {
    let input_power = Power::new(100.0);
    let efficiency = Efficiency::new(0.90);
    
    let result = calculations::calculate_power_loss(input_power, efficiency);
    // P_loss = P_in × (1 - η) = 100 * (1 - 0.90) = 10 W
    assert!((result.get() - 10.0).abs() < 0.001);
}

#[test]
fn test_belt_ratio_creation() {
    let ratio = BeltRatio::new(0.10, 0.20, BeltCategory::OpenBelt);
    assert_eq!(ratio.input_radius, 0.10);
    assert_eq!(ratio.output_radius, 0.20);
    assert_eq!(ratio.ratio, 2.0);
    assert_eq!(ratio.category, BeltCategory::OpenBelt);
}

#[test]
fn test_belt_ratio_speed_reduction() {
    let ratio = BeltRatio::new(0.10, 0.20, BeltCategory::OpenBelt);
    assert!(ratio.is_speed_reduction());
    assert!(!ratio.is_speed_increase());
    assert!(!ratio.is_neutral());
}

#[test]
fn test_belt_ratio_speed_increase() {
    let ratio = BeltRatio::new(0.20, 0.10, BeltCategory::OpenBelt);
    assert!(!ratio.is_speed_reduction());
    assert!(ratio.is_speed_increase());
    assert!(!ratio.is_neutral());
}

#[test]
fn test_belt_ratio_neutral() {
    let ratio = BeltRatio::new(0.10, 0.10, BeltCategory::OpenBelt);
    assert!(!ratio.is_speed_reduction());
    assert!(!ratio.is_speed_increase());
    assert!(ratio.is_neutral());
}

#[test]
fn test_belt_ratio_rotation_direction() {
    let open_belt = BeltRatio::new(0.10, 0.20, BeltCategory::OpenBelt);
    let crossed_belt = BeltRatio::new(0.10, 0.20, BeltCategory::CrossedBelt);
    
    assert_eq!(open_belt.rotation_direction_changes(), false);
    assert_eq!(crossed_belt.rotation_direction_changes(), true);
}

#[test]
fn test_valid_pulley_configuration() {
    let config = BeltConfiguration::new(
        0.10, // radius
        1000.0, // max_rpm
        100.0, // max_torque
        1000.0, // max_power
        0.95, // efficiency
        BeltCategory::OpenBelt,
    );
    
    let pulley = Pulley::new(BeltId::new(1), config);
    assert!(pulley.validate_pulley().is_ok());
}

#[test]
fn test_invalid_pulley_radius() {
    let config = BeltConfiguration::new(
        -0.10, // invalid negative radius
        1000.0,
        100.0,
        1000.0,
        0.95,
        BeltCategory::OpenBelt,
    );
    
    let pulley = Pulley::new(BeltId::new(1), config);
    assert!(matches!(pulley.validate_pulley(), Err(BeltError::InvalidRadius)));
}

#[test]
fn test_valid_belt_pair() {
    let input_config = BeltConfiguration::new(
        0.10,
        1000.0,
        100.0,
        1000.0,
        0.95,
        BeltCategory::OpenBelt,
    );
    
    let output_config = BeltConfiguration::new(
        0.20,
        1000.0,
        100.0,
        1000.0,
        0.95,
        BeltCategory::OpenBelt,
    );
    
    let input_pulley = Pulley::new(BeltId::new(1), input_config);
    let output_pulley = Pulley::new(BeltId::new(2), output_config);
    let pair = BeltPair::new(input_pulley, output_pulley);
    
    assert!(pair.validate_pair().is_ok());
}

#[test]
fn test_invalid_belt_pair_same_pulley() {
    let config = BeltConfiguration::new(
        0.10,
        1000.0,
        100.0,
        1000.0,
        0.95,
        BeltCategory::OpenBelt,
    );
    
    let pulley = Pulley::new(BeltId::new(1), config);
    let pair = BeltPair::new(pulley.clone(), pulley);
    
    assert!(matches!(pair.validate_pair(), Err(BeltError::InvalidPair)));
}

#[test]
fn test_case_a_speed_reduction() {
    // Input:
    // - Input pulley radius: 0.10 m
    // - Output pulley radius: 0.20 m
    // - Input speed: 120 RPM
    // - Efficiency: 1.0
    
    // Expected:
    // - Output speed: 60 RPM
    // - Ideal torque multiplier: 2
    // - Output power equals input power.
    
    let input_rpm = RotationalSpeed::new(120.0);
    let input_radius = 0.10;
    let output_radius = 0.20;
    let efficiency = Efficiency::new(1.0);
    
    let output_rpm = calculations::calculate_output_rpm(input_rpm, input_radius, output_radius, BeltCategory::OpenBelt);
    assert_eq!(output_rpm.get(), 60.0);
    
    let ideal_torque_multiplier = output_radius / input_radius;
    assert_eq!(ideal_torque_multiplier, 2.0);
    
    // With efficiency = 1.0, output power should equal input power
    let input_power = Power::new(100.0); // arbitrary value for testing
    let output_power = calculations::calculate_output_power(input_power, efficiency);
    assert_eq!(input_power.get(), output_power.get());
}

#[test]
fn test_case_b_speed_increase() {
    // Input:
    // - Input pulley radius: 0.20 m
    // - Output pulley radius: 0.10 m
    // - Input speed: 60 RPM
    // - Efficiency: 1.0
    
    // Expected:
    // - Output speed: 120 RPM
    // - Ideal torque multiplier: 0.5
    // - Output power equals input power.
    
    let input_rpm = RotationalSpeed::new(60.0);
    let input_radius = 0.20;
    let output_radius = 0.10;
    let efficiency = Efficiency::new(1.0);
    
    let output_rpm = calculations::calculate_output_rpm(input_rpm, input_radius, output_radius, BeltCategory::OpenBelt);
    assert_eq!(output_rpm.get(), 120.0);
    
    let ideal_torque_multiplier = output_radius / input_radius;
    assert_eq!(ideal_torque_multiplier, 0.5);
    
    // With efficiency = 1.0, output power should equal input power
    let input_power = Power::new(100.0); // arbitrary value for testing
    let output_power = calculations::calculate_output_power(input_power, efficiency);
    assert_eq!(input_power.get(), output_power.get());
}

#[test]
fn test_case_c_efficiency_losses() {
    // Input:
    // - Input pulley radius: 0.10 m
    // - Output pulley radius: 0.20 m
    // - Input torque: 10 N·m
    // - Input speed: 120 RPM
    // - Efficiency: 0.90
    
    // Expected:
    // - Output speed: 60 RPM
    // - Ideal output torque: 20 N·m
    // - Efficiency-adjusted output torque: 18 N·m
    // - Output power is 90% of input power.
    // - Power loss is 10% of input power.
    
    let input_torque = Torque::new(10.0);
    let input_radius = 0.10;
    let output_radius = 0.20;
    let efficiency = Efficiency::new(0.90);
    
    let ideal_output_torque = calculations::calculate_ideal_output_torque(input_torque, input_radius, output_radius);
    assert_eq!(ideal_output_torque.get(), 20.0);
    
    let actual_output_torque = calculations::calculate_output_torque(input_torque, input_radius, output_radius, efficiency);
    assert_eq!(actual_output_torque.get(), 18.0);
    
    // Test power calculations
    let input_power = Power::new(100.0); // arbitrary value for testing
    let output_power = calculations::calculate_output_power(input_power, efficiency);
    assert_eq!(output_power.get(), 90.0);
    
    let power_loss = calculations::calculate_power_loss(input_power, efficiency);
    assert_eq!(power_loss.get(), 10.0);
}

#[test]
fn test_case_d_rotation_direction() {
    // Test open belt (same direction)
    let input_rpm = RotationalSpeed::new(120.0);
    let input_radius = 0.10;
    let output_radius = 0.20;
    
    let open_belt_output_rpm = calculations::calculate_output_rpm(input_rpm, input_radius, output_radius, BeltCategory::OpenBelt);
    assert_eq!(open_belt_output_rpm.get(), 60.0);
    
    // Test crossed belt (opposite direction)
    let crossed_belt_output_rpm = calculations::calculate_output_rpm(input_rpm, input_radius, output_radius, BeltCategory::CrossedBelt);
    // The RPM value should still be 60 (the sign only affects direction, not magnitude)
    assert_eq!(crossed_belt_output_rpm.get(), 60.0);
}

#[test]
fn test_case_e_invalid_configurations() {
    // Test zero pulley radius
    let config_zero_radius = BeltConfiguration::new(
        0.0, // zero radius
        1000.0,
        100.0,
        1000.0,
        0.95,
        BeltCategory::OpenBelt,
    );
    
    assert!(matches!(config_zero_radius.validate_configuration(), Err(BeltError::InvalidRadius)));
    
    // Test negative pulley radius
    let config_negative_radius = BeltConfiguration::new(
        -0.10, // negative radius
        1000.0,
        100.0,
        1000.0,
        0.95,
        BeltCategory::OpenBelt,
    );
    
    assert!(matches!(config_negative_radius.validate_configuration(), Err(BeltError::InvalidRadius)));
    
    // Test invalid efficiency
    let config_invalid_efficiency = BeltConfiguration::new(
        0.10,
        1000.0,
        100.0,
        1000.0,
        1.5, // invalid efficiency > 1.0
        BeltCategory::OpenBelt,
    );
    
    assert!(matches!(config_invalid_efficiency.validate_configuration(), Err(BeltError::InvalidEfficiency)));
}