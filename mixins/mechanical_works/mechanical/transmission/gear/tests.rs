//! Tests for the gear transmission system.

use crate::mechanical::core::units::{Torque, RotationalSpeed, AngularVelocity, Power, Efficiency};
use crate::mechanical::transmission::gear::{Gear, GearId, GearCategory, GearConfiguration, GearPair, GearRatio};
use crate::mechanical::core::identifiers::ComponentId;
use crate::mechanical::transmission::gear::errors::GearError;

#[test]
fn test_gear_creation() {
    let configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    let gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    assert_eq!(gear.id(), GearId::new(1));
    assert_eq!(gear.component_id(), ComponentId::new(100));
    assert_eq!(gear.category(), GearCategory::Spur);
    assert_eq!(gear.configuration().tooth_count(), 20);
    assert!(gear.is_enabled());
}

#[test]
fn test_gear_creation_invalid_tooth_count() {
    let configuration = GearConfiguration::new(0, 1000.0, 50.0, 5000.0, 0.95);
    let gear_result = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    );
    
    assert!(matches!(gear_result, Err(GearError::InvalidToothCount)));
}

#[test]
fn test_gear_torque_setting() {
    let mut configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    let mut gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    let torque = Torque::new(25.0);
    assert!(gear.set_torque(torque).is_ok());
    assert_eq!(gear.current_torque(), torque);
}

#[test]
fn test_gear_torque_exceeded() {
    let configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    let mut gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    let torque = Torque::new(100.0); // Exceeds max_torque of 50.0
    assert!(matches!(gear.set_torque(torque), Err(GearError::ExceededTorque)));
}

#[test]
fn test_gear_rpm_setting() {
    let mut configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    let mut gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    let rpm = RotationalSpeed::new(500.0);
    assert!(gear.set_rpm(rpm).is_ok());
    assert_eq!(gear.current_rpm(), rpm);
}

#[test]
fn test_gear_rpm_exceeded() {
    let configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    let mut gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    let rpm = RotationalSpeed::new(2000.0); // Exceeds max_rpm of 1000.0
    assert!(matches!(gear.set_rpm(rpm), Err(GearError::ExceededRPM)));
}

#[test]
fn test_gear_pair_creation() {
    let gear_pair = GearPair::new(1, GearId::new(1), GearId::new(2)).unwrap();
    
    assert_eq!(gear_pair.id(), 1);
    assert_eq!(gear_pair.input_gear(), GearId::new(1));
    assert_eq!(gear_pair.output_gear(), GearId::new(2));
    assert!(gear_pair.is_enabled());
}

#[test]
fn test_gear_pair_self_pairing() {
    let gear_pair = GearPair::new(1, GearId::new(1), GearId::new(1));
    
    assert!(matches!(gear_pair, Err(GearError::SelfPairing)));
}

#[test]
fn test_gear_ratio_creation() {
    let gear_ratio = GearRatio::new(20, 40).unwrap();
    
    assert_eq!(gear_ratio.input_teeth(), 20);
    assert_eq!(gear_ratio.output_teeth(), 40);
    assert_eq!(gear_ratio.ratio(), 2.0); // 40/20 = 2
    assert!(gear_ratio.is_reversed());
}

#[test]
fn test_gear_ratio_speed_reduction() {
    let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
    
    assert!(gear_ratio.is_speed_reduction());
    assert!(!gear_ratio.is_speed_increase());
    assert!(!gear_ratio.is_neutral());
}

#[test]
fn test_gear_ratio_speed_increase() {
    let gear_ratio = GearRatio::new(40, 20).unwrap(); // 1:2 ratio (speed increase)
    
    assert!(!gear_ratio.is_speed_reduction());
    assert!(gear_ratio.is_speed_increase());
    assert!(!gear_ratio.is_neutral());
}

#[test]
fn test_gear_ratio_neutral() {
    let gear_ratio = GearRatio::new(20, 20).unwrap(); // 1:1 ratio (neutral)
    
    assert!(!gear_ratio.is_speed_reduction());
    assert!(!gear_ratio.is_speed_increase());
    assert!(gear_ratio.is_neutral());
}

#[test]
fn test_gear_ratio_invalid_teeth() {
    let gear_ratio = GearRatio::new(0, 40);
    
    assert!(matches!(gear_ratio, Err(GearError::InvalidToothCount)));
}

#[test]
fn test_output_rpm_calculation() {
    let input_rpm = RotationalSpeed::new(120.0); // 120 RPM
    let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
    
    let output_rpm = gear_ratio.calculate_output_rpm(input_rpm);
    assert_eq!(output_rpm.get(), 60.0); // Should be 120 / 2 = 60 RPM
}

#[test]
fn test_output_angular_velocity_calculation() {
    let input_angular_velocity = AngularVelocity::new(10.0); // 10 rad/s
    let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
    
    let output_angular_velocity = gear_ratio.calculate_output_angular_velocity(input_angular_velocity);
    assert_eq!(output_angular_velocity.get(), 5.0); // Should be 10 / 2 = 5 rad/s
}

#[test]
fn test_output_torque_ideal() {
    let input_torque = Torque::new(10.0); // 10 N·m
    let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
    
    let output_torque = crate::mechanical::transmission::gear::calculations::calculate_output_torque_ideal(input_torque, &gear_ratio);
    assert_eq!(output_torque.get(), 20.0); // Should be 10 * 2 = 20 N·m
}

#[test]
fn test_output_torque_with_efficiency() {
    let input_torque = Torque::new(10.0); // 10 N·m
    let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
    let efficiency = Efficiency::new(0.9); // 90% efficiency
    
    let output_torque = crate::mechanical::transmission::gear::calculations::calculate_output_torque_with_efficiency(input_torque, &gear_ratio, efficiency);
    assert_eq!(output_torque.get(), 18.0); // Should be 10 * 2 * 0.9 = 18 N·m
}

#[test]
fn test_power_conservation() {
    let input_power = Power::new(100.0); // 100 W
    let efficiency = Efficiency::new(0.9); // 90% efficiency
    
    let output_power = crate::mechanical::transmission::gear::calculations::calculate_output_power(input_power, efficiency);
    assert_eq!(output_power.get(), 90.0); // Should be 100 * 0.9 = 90 W
    
    let power_loss = crate::mechanical::transmission::gear::calculations::calculate_power_loss(input_power, efficiency);
    assert_eq!(power_loss.get(), 10.0); // Should be 100 * (1 - 0.9) = 10 W
}

#[test]
fn test_rotation_direction() {
    let gear_ratio = GearRatio::new(20, 40).unwrap(); // 2:1 ratio (speed reduction)
    
    // Test with CW input (+1)
    let output_direction = crate::mechanical::transmission::gear::calculations::calculate_rotation_direction(&gear_ratio, 1);
    assert_eq!(output_direction, -1); // Should be CCW (-1) for external meshing
    
    // Test with CCW input (-1)
    let output_direction = crate::mechanical::transmission::gear::calculations::calculate_rotation_direction(&gear_ratio, -1);
    assert_eq!(output_direction, 1); // Should be CW (+1) for external meshing
}

#[test]
fn test_gear_validation_enabled() {
    let configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    let gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    // Should pass validation for enabled gear
    assert!(gear.validate_gear_enabled().is_ok());
}

#[test]
fn test_gear_validation_disabled() {
    let mut configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
    configuration.set_enabled(false);
    
    let gear = Gear::new(
        GearId::new(1),
        ComponentId::new(100),
        GearCategory::Spur,
        configuration
    ).unwrap();
    
    // Should fail validation for disabled gear
    assert!(matches!(gear.validate_gear_enabled(), Err(GearError::DisabledGear)));
}

#[test]
fn test_gear_pair_validation_self_pairing() {
    let gear_pair = GearPair::new(1, GearId::new(1), GearId::new(1));
    
    // Should fail validation for self-pairing
    assert!(matches!(gear_pair, Err(GearError::SelfPairing)));
}

#[test]
fn test_gear_pair_validation_valid() {
    let gear_pair = GearPair::new(1, GearId::new(1), GearId::new(2));
    
    // Should pass validation for valid pair
    assert!(gear_pair.unwrap().validate_gear_pair().is_ok());
}

#[test]
fn test_gear_ratio_validation_valid() {
    let gear_ratio = GearRatio::new(20, 40).unwrap();
    
    // Should pass validation for valid ratio
    assert!(gear_ratio.validate_gear_ratio().is_ok());
}

#[test]
fn test_gear_ratio_validation_invalid_teeth() {
    let result = GearRatio::new(0, 40);
    
    // Should fail validation for invalid tooth count
    assert!(matches!(result, Err(GearError::InvalidToothCount)));
}