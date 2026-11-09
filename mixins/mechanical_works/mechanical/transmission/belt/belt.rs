//! Belt transmission component.
//!
//! This module provides the main belt transmission component that implements 
//! the rotational transmission interface.

use std::fmt;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity};
use crate::mechanical::transmission::belt::{pair::BeltPair, types::{BeltId, BeltConfiguration, BeltCategory}};
use crate::mechanical::transmission::interfaces::{RotationalInput, RotationalOutput, RotationalTransmission};
use crate::mechanical::transmission::integration::{IntegratedTransmission, SetTransmissionState, TransmissionState};

/// A belt transmission component.
///
/// This represents a complete belt transmission system that can transfer
/// rotational motion between two pulleys.
#[derive(Debug)]
pub struct Belt {
    /// Unique identifier for this belt.
    pub id: BeltId,
    
    /// The configuration of this belt.
    pub configuration: BeltConfiguration,
    
    /// The pair of pulleys connected by this belt.
    pub pair: BeltPair,
    
    /// Whether this belt is currently enabled and operational.
    pub enabled: bool,
}

impl Belt {
    /// Creates a new belt with the specified parameters.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this belt
    /// * `configuration` - Configuration settings for this belt
    /// * `input_pulley` - The input (driving) pulley
    /// * `output_pulley` - The output (driven) pulley
    ///
    /// # Returns
    /// A new Belt instance
    pub fn new(
        id: BeltId, 
        configuration: BeltConfiguration, 
        input_pulley: crate::mechanical::transmission::belt::pulley::Pulley,
        output_pulley: crate::mechanical::transmission::belt::pulley::Pulley,
    ) -> Self {
        let pair = BeltPair::new(input_pulley, output_pulley);
        
        Belt {
            id,
            configuration,
            pair,
            enabled: configuration.enabled,
        }
    }
    
    /// Gets the current ratio of this belt.
    pub fn ratio(&self) -> &crate::mechanical::transmission::belt::ratio::BeltRatio {
        &self.pair.ratio
    }
    
    /// Checks if this belt is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled && self.configuration.enabled && self.pair.is_enabled()
    }
    
    /// Enables or disables this belt.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Sets the input RPM for this belt.
    ///
    /// # Arguments
    /// * `rpm` - Input rotational speed in RPM
    pub fn set_input_rpm(&mut self, rpm: RotationalSpeed) {
        self.pair.set_input_rpm(rpm);
    }
    
    /// Sets the output RPM for this belt.
    ///
    /// # Arguments
    /// * `rpm` - Output rotational speed in RPM
    pub fn set_output_rpm(&mut self, rpm: RotationalSpeed) {
        self.pair.set_output_rpm(rpm);
    }
    
    /// Sets the input torque for this belt.
    ///
    /// # Arguments
    /// * `torque` - Input torque in newton-metres
    pub fn set_input_torque(&mut self, torque: Torque) {
        self.pair.set_input_torque(torque);
    }
    
    /// Sets the output torque for this belt.
    ///
    /// # Arguments
    /// * `torque` - Output torque in newton-metres
    pub fn set_output_torque(&mut self, torque: Torque) {
        self.pair.set_output_torque(torque);
    }
    
    /// Sets the input power for this belt.
    ///
    /// # Arguments
    /// * `power` - Input power in watts
    pub fn set_input_power(&mut self, power: Power) {
        self.pair.set_input_power(power);
    }
    
    /// Sets the output power for this belt.
    ///
    /// # Arguments
    /// * `power` - Output power in watts
    pub fn set_output_power(&mut self, power: Power) {
        self.pair.set_output_power(power);
    }
}

impl fmt::Display for Belt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Belt(id: {}, ratio: {})", self.id, self.pair.ratio)
    }
}

// Implement the RotationalInput trait
impl RotationalInput for Belt {
    fn max_input_torque(&self) -> Torque {
        self.configuration.max_torque.into()
    }
    
    fn max_input_rpm(&self) -> RotationalSpeed {
        self.configuration.max_rpm.into()
    }
    
    fn is_compatible_with_output(&self, _output: &dyn RotationalOutput) -> bool {
        // Basic compatibility check - in a real implementation this would be more complex
        true
    }
}

// Implement the RotationalOutput trait
impl RotationalOutput for Belt {
    fn current_torque(&self) -> Torque {
        self.pair.output_pulley.current_torque
    }
    
    fn current_rpm(&self) -> RotationalSpeed {
        self.pair.output_pulley.current_rpm
    }
    
    fn current_power(&self) -> Power {
        self.pair.output_pulley.current_power
    }
    
    fn is_compatible_with_input(&self, _input: &dyn RotationalInput) -> bool {
        // Basic compatibility check - in a real implementation this would be more complex
        true
    }
}

// Implement the RotationalTransmission trait
impl RotationalTransmission for Belt {
    fn transmission_id(&self) -> crate::mechanical::transmission::types::TransmissionId {
        crate::mechanical::transmission::types::TransmissionId::new(self.id.get())
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled && self.configuration.enabled && self.pair.is_enabled()
    }
}

impl IntegratedTransmission for Belt {
    fn input_state(&self) -> TransmissionState {
        TransmissionState::with_values(
            self.pair.input_pulley.current_torque,
            self.pair.input_pulley.current_rpm,
            self.pair.input_pulley.current_power,
            self.is_enabled()
        )
    }
    
    fn output_state(&self) -> TransmissionState {
        TransmissionState::with_values(
            self.pair.output_pulley.current_torque,
            self.pair.output_pulley.current_rpm,
            self.pair.output_pulley.current_power,
            self.is_enabled()
        )
    }
    
    /// Evaluates the belt behavior given input conditions.
    ///
    /// This method implements the belt ratio calculations and power conservation.
    fn evaluate(&self, input_state: TransmissionState) -> Result<TransmissionState, crate::mechanical::transmission::errors::TransmissionError> {
        // Delegate to the centralized evaluation logic
        crate::mechanical::transmission::evaluation::evaluate_belt(self, input_state)
    }
}

impl SetTransmissionState for Belt {
    fn set_input_state(&mut self, state: TransmissionState) -> Result<(), crate::mechanical::transmission::errors::TransmissionError> {
        self.pair.input_pulley.current_torque = state.torque;
        self.pair.input_pulley.current_rpm = state.rpm;
        self.pair.input_pulley.current_power = state.power;
        self.enabled = state.enabled;
        Ok(())
    }
    
    fn set_output_state(&mut self, state: TransmissionState) -> Result<(), crate::mechanical::transmission::errors::TransmissionError> {
        self.pair.output_pulley.current_torque = state.torque;
        self.pair.output_pulley.current_rpm = state.rpm;
        self.pair.output_pulley.current_power = state.power;
        self.enabled = state.enabled;
        Ok(())
    }
}