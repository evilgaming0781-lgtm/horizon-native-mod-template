//! Example usage of the belt transmission system.
//!
//! This module demonstrates how to use the belt transmission components.

use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity, Efficiency};
use crate::mechanical::transmission::belt::{
    types::{BeltId, BeltCategory, BeltConfiguration},
    pulley::Pulley,
    pair::BeltPair,
    ratio::BeltRatio,
    belt::Belt,
    calculations,
};

/// Example demonstrating a simple belt transmission.
pub fn example_belt_transmission() {
    // Create configuration for input pulley
    let input_config = BeltConfiguration::new(
        0.10, // radius in meters
        1000.0, // max RPM
        100.0, // max torque in N·m
        1000.0, // max power in W
        0.95, // efficiency
        BeltCategory::OpenBelt,
    );
    
    // Create configuration for output pulley
    let output_config = BeltConfiguration::new(
        0.20, // radius in meters (larger - speed reduction)
        1000.0, // max RPM
        100.0, // max torque in N·m
        1000.0, // max power in W
        0.95, // efficiency
        BeltCategory::OpenBelt,
    );
    
    // Create pulleys
    let input_pulley = Pulley::new(BeltId::new(1), input_config);
    let output_pulley = Pulley::new(BeltId::new(2), output_config);
    
    // Create belt pair
    let belt_pair = BeltPair::new(input_pulley, output_pulley);
    
    // Create a complete belt transmission
    let belt = Belt::new(
        BeltId::new(3),
        BeltConfiguration::new(
            0.15, // belt radius
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        ),
        belt_pair.input_pulley.clone(),
        belt_pair.output_pulley.clone()
    );
    
    // Calculate transmission parameters
    let input_rpm = RotationalSpeed::new(120.0);
    let output_rpm = calculations::calculate_output_rpm(
        input_rpm,
        belt_pair.input().radius(),
        belt_pair.output().radius(),
        belt_pair.input().category()
    );
    
    println!("Input RPM: {}", input_rpm.get());
    println!("Output RPM: {}", output_rpm.get());
    println!("Belt Ratio: {}", belt.ratio().get());
    println!("Belt pair is valid: {}", belt_pair.is_valid());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_compilation() {
        example_belt_transmission();
        assert!(true); // If we get here, compilation worked
    }
}