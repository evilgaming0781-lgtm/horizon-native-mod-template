//! Belt pulley pair representation.
//!
//! This module provides the structure for connecting two pulleys in a belt transmission.

use std::fmt;
use crate::mechanical::transmission::belt::{pulley::Pulley, ratio::BeltRatio};
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};

/// A pair of pulleys connected by a belt.
///
/// This represents the connection between an input pulley and output pulley
/// in a belt transmission system.
#[derive(Debug, Clone)]
pub struct BeltPair {
    /// The input pulley (driving pulley).
    pub input_pulley: Pulley,
    
    /// The output pulley (driven pulley).
    pub output_pulley: Pulley,
    
    /// The calculated ratio between the pulleys.
    pub ratio: BeltRatio,
}

impl BeltPair {
    /// Creates a new belt pair with the specified pulleys.
    ///
    /// # Arguments
    /// * `input_pulley` - The driving pulley
    /// * `output_pulley` - The driven pulley
    ///
    /// # Returns
    /// A new BeltPair instance
    pub fn new(input_pulley: Pulley, output_pulley: Pulley) -> Self {
        let ratio = BeltRatio::new(
            input_pulley.radius(),
            output_pulley.radius(),
            input_pulley.category()
        );
        
        BeltPair {
            input_pulley,
            output_pulley,
            ratio,
        }
    }
    
    /// Gets the input pulley.
    pub fn input(&self) -> &Pulley {
        &self.input_pulley
    }
    
    /// Gets the output pulley.
    pub fn output(&self) -> &Pulley {
        &self.output_pulley
    }
    
    /// Gets a mutable reference to the input pulley.
    pub fn input_mut(&mut self) -> &mut Pulley {
        &mut self.input_pulley
    }
    
    /// Gets a mutable reference to the output pulley.
    pub fn output_mut(&mut self) -> &mut Pulley {
        &mut self.output_pulley
    }
    
    /// Checks if this pair is valid (input and output pulleys are different).
    ///
    /// # Returns
    /// * `true` if the pulleys are different (valid pair)
    /// * `false` if both pulleys are the same (invalid pair)
    pub fn is_valid(&self) -> bool {
        self.input_pulley.id != self.output_pulley.id
    }
    
    /// Checks if this pair is enabled (both pulleys are enabled).
    ///
    /// # Returns
    /// * `true` if both pulleys are enabled
    /// * `false` if either pulley is disabled
    pub fn is_enabled(&self) -> bool {
        self.input_pulley.is_enabled() && self.output_pulley.is_enabled()
    }
    
    /// Sets the input RPM for this pair.
    ///
    /// # Arguments
    /// * `rpm` - Input rotational speed in RPM
    pub fn set_input_rpm(&mut self, rpm: RotationalSpeed) {
        self.input_pulley.set_rpm(rpm);
    }
    
    /// Sets the output RPM for this pair.
    ///
    /// # Arguments
    /// * `rpm` - Output rotational speed in RPM
    pub fn set_output_rpm(&mut self, rpm: RotationalSpeed) {
        self.output_pulley.set_rpm(rpm);
    }
    
    /// Sets the input torque for this pair.
    ///
    /// # Arguments
    /// * `torque` - Input torque in newton-metres
    pub fn set_input_torque(&mut self, torque: Torque) {
        self.input_pulley.set_torque(torque);
    }
    
    /// Sets the output torque for this pair.
    ///
    /// # Arguments
    /// * `torque` - Output torque in newton-metres
    pub fn set_output_torque(&mut self, torque: Torque) {
        self.output_pulley.set_torque(torque);
    }
    
    /// Sets the input power for this pair.
    ///
    /// # Arguments
    /// * `power` - Input power in watts
    pub fn set_input_power(&mut self, power: Power) {
        self.input_pulley.set_power(power);
    }
    
    /// Sets the output power for this pair.
    ///
    /// # Arguments
    /// * `power` - Output power in watts
    pub fn set_output_power(&mut self, power: Power) {
        self.output_pulley.set_power(power);
    }
}

impl fmt::Display for BeltPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BeltPair(input: {}, output: {})", 
            self.input_pulley.id, self.output_pulley.id)
    }
}