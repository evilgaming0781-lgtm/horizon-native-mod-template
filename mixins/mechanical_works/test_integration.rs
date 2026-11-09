//! Simple integration test for the transmission evaluation system.

use horizon_mechanical_works::mechanical::transmission::{
    types::{TransmissionId, TransmissionCategory, TransmissionConfiguration},
    shaft::Shaft,
    coupling::Coupling,
    gear::Gear,
    belt::Belt,
    component::TransmissionComponent,
    interfaces::{RotationalInput, RotationalOutput, RotationalTransmission},
    integration::{IntegratedTransmission, SetTransmissionState, TransmissionState},
    errors::TransmissionError,
};
use horizon_mechanical_works::mechanical::transmission::gear::{
    types::{GearId, GearCategory, GearConfiguration},
};
use horizon_mechanical_works::mechanical::transmission::belt::types::{BeltId, BeltConfiguration};
use horizon_mechanical_works::mechanical::core::identifiers::ComponentId;
use horizon_mechanical_works::mechanical::core::units::{Torque, RotationalSpeed, Power};

#[test]
fn test_shaft_evaluation() {
    let component_id = ComponentId::new(1);
    let transmission_id = TransmissionId::new(1);
    
    let config = TransmissionConfiguration {
        max_rpm: 1000.0,
        max_torque: 50.0,
        max_power: 5000.0,
        enabled: true,
    };
    
    let shaft = Shaft::new(transmission_id, component_id, config);
    
    // Test ideal pass-through behavior
    let input_state = TransmissionState::with_values(
        Torque::new(25.0),
        RotationalSpeed::new(500.0),
        Power::new(2500.0),
        true,
    );
    
    let output_state = shaft.evaluate(input_state).unwrap();
    assert_eq!(output_state.torque, Torque::new(25.0));
    assert_eq!(output_state.rpm, RotationalSpeed::new(500.0));
    assert_eq!(output_state.power, Power::new(2500.0));
    assert_eq!(output_state.enabled, true);
}

#[test]
fn test_gear_evaluation() {
    // Create a gear with 2:1 ratio (speed reduction, torque increase)
    let component_id = ComponentId::new(2);
    let gear_id = GearId::new(1);
    
    let gear_config = GearConfiguration {
        ratio: horizon_mechanical_works::mechanical::transmission::gear::ratio::GearRatio::new(20, 40).unwrap(), // 2:1 ratio
        max_rpm: 1000.0,
        max_torque: 50.0,
        max_power: 5000.0,
        efficiency: horizon_mechanical_works::mechanical::core::units::Efficiency::new(0.9), // 90% efficiency
        enabled: true,
    };
    
    let gear = Gear::new(gear_id, component_id, GearCategory::SpurGear, gear_config).unwrap();
    
    // Input state with 1000 RPM and 10 N·m torque
    let input_state = TransmissionState::with_values(
        Torque::new(10.0),
        RotationalSpeed::new(1000.0),
        Power::new(10000.0),
        true,
    );
    
    let output_state = gear.evaluate(input_state).unwrap();
    
    // Expected: 
    // - RPM should be reduced by 2x (1000 / 2 = 500 RPM)
    // - Torque should be increased by 2x (10 * 2 = 20 N·m) but with efficiency applied
    // - Power should be reduced by efficiency (10000 * 0.9 = 9000 W)
    
    assert_eq!(output_state.rpm.get(), 500.0); // 1000 / 2 = 500
    assert_eq!(output_state.torque.get(), 18.0); // 10 * 2 * 0.9 = 18 (efficiency applied)
    assert_eq!(output_state.power.get(), 9000.0); // 10000 * 0.9 = 9000
    assert_eq!(output_state.enabled, true);
}

#[test]
fn test_belt_evaluation() {
    // Create a belt with open belt arrangement
    let component_id = ComponentId::new(3);
    let belt_id = BeltId::new(1);
    
    let input_pulley = horizon_mechanical_works::mechanical::transmission::belt::pulley::Pulley::new(
        0.10, // radius in meters
        Torque::new(10.0),
        RotationalSpeed::new(1000.0),
        Power::new(10000.0),
    );
    
    let output_pulley = horizon_mechanical_works::mechanical::transmission::belt::pulley::Pulley::new(
        0.20, // radius in meters (double the input)
        Torque::new(0.0),
        RotationalSpeed::new(0.0),
        Power::new(0.0),
    );
    
    let belt_config = BeltConfiguration {
        max_rpm: 1000.0,
        max_torque: 50.0,
        max_power: 5000.0,
        efficiency: horizon_mechanical_works::mechanical::core::units::Efficiency::new(0.9), // 90% efficiency
        category: horizon_mechanical_works::mechanical::transmission::belt::types::BeltCategory::OpenBelt,
        enabled: true,
    };
    
    let belt = Belt::new(belt_id, belt_config, input_pulley, output_pulley);
    
    // Input state with 1000 RPM and 10 N·m torque
    let input_state = TransmissionState::with_values(
        Torque::new(10.0),
        RotationalSpeed::new(1000.0),
        Power::new(10000.0),
        true,
    );
    
    let output_state = belt.evaluate(input_state).unwrap();
    
    // Expected: 
    // - RPM should be reduced by 2x (1000 / 2 = 500 RPM) for open belt
    // - Torque should be increased by 2x (10 * 2 = 20 N·m) but with efficiency applied  
    // - Power should be reduced by efficiency (10000 * 0.9 = 9000 W)
    
    assert_eq!(output_state.rpm.get(), 500.0); // 1000 / 2 = 500
    assert_eq!(output_state.torque.get(), 18.0); // 10 * 2 * 0.9 = 18
    assert_eq!(output_state.power.get(), 9000.0); // 10000 * 0.9 = 9000
    assert_eq!(output_state.enabled, true);
}