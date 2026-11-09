//! Integrated transmission system for Horizon Mechanical Works.
//!
//! This module provides the common interface and integration for all transmission components,
//! ensuring they can work together coherently while maintaining their individual functionality.

use std::fmt;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity};
use crate::mechanical::transmission::types::{TransmissionId, TransmissionCategory};
use crate::mechanical::transmission::interfaces::{RotationalInput, RotationalOutput, RotationalTransmission};
use crate::mechanical::transmission::errors::TransmissionError;
use crate::mechanical::transmission::component::TransmissionComponent;
use crate::mechanical::transmission::evaluation;

/// Common state for rotational transmission inputs and outputs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransmissionState {
    /// Current torque being transmitted.
    pub torque: Torque,
    
    /// Current rotational speed being transmitted.
    pub rpm: RotationalSpeed,
    
    /// Current power being transmitted.
    pub power: Power,
    
    /// Indicates if the transmission is enabled.
    pub enabled: bool,
}

impl TransmissionState {
    /// Creates a new transmission state with default values.
    pub fn new() -> Self {
        TransmissionState {
            torque: Torque::new(0.0),
            rpm: RotationalSpeed::new(0.0),
            power: Power::new(0.0),
            enabled: true,
        }
    }
    
    /// Creates a new transmission state with specified values.
    pub fn with_values(torque: Torque, rpm: RotationalSpeed, power: Power, enabled: bool) -> Self {
        TransmissionState {
            torque,
            rpm,
            power,
            enabled,
        }
    }
}

impl Default for TransmissionState {
    fn default() -> Self {
        TransmissionState::new()
    }
}

/// A trait representing a complete rotational transmission with input/output state.
///
/// This trait provides a unified interface for all transmission components to 
/// expose their current state and support the common execution model.
pub trait IntegratedTransmission: RotationalTransmission {
    /// Gets the current input state of this transmission.
    fn input_state(&self) -> TransmissionState;
    
    /// Gets the current output state of this transmission.
    fn output_state(&self) -> TransmissionState;
    
    /// Evaluates the transmission behavior given input conditions.
    ///
    /// # Arguments
    /// * `input_state` - The input state to evaluate against
    ///
    /// # Returns
    /// * `Ok(TransmissionState)` - The resulting output state if evaluation succeeds
    /// * `Err(TransmissionError)` - If the evaluation fails due to invalid inputs or constraints
    fn evaluate(&self, input_state: TransmissionState) -> Result<TransmissionState, TransmissionError>;
}

/// Trait for validating transmission configurations.
pub trait TransmissionValidation {
    /// Validates that this transmission component is properly configured.
    ///
    /// # Returns
    /// * `Ok(())` if the configuration is valid
    /// * `Err(TransmissionError)` if the configuration is invalid
    fn validate_config(&self) -> Result<(), TransmissionError>;
}

/// Trait for setting the current state of a transmission.
pub trait SetTransmissionState {
    /// Sets the input state of this transmission.
    ///
    /// # Arguments
    /// * `state` - The new input state to set
    fn set_input_state(&mut self, state: TransmissionState) -> Result<(), TransmissionError>;
    
    /// Sets the output state of this transmission.
    ///
    /// # Arguments
    /// * `state` - The new output state to set
    fn set_output_state(&mut self, state: TransmissionState) -> Result<(), TransmissionError>;
}

/// A common implementation for transmission components that can be integrated.
#[derive(Debug, Clone)]
pub struct IntegratedTransmissionComponent {
    /// The underlying transmission component.
    pub component: TransmissionComponent,
    
    /// Current input state of the transmission.
    pub input_state: TransmissionState,
    
    /// Current output state of the transmission.
    pub output_state: TransmissionState,
}

impl IntegratedTransmissionComponent {
    /// Creates a new integrated transmission component.
    ///
    /// # Arguments
    /// * `component` - The underlying transmission component
    pub fn new(component: TransmissionComponent) -> Self {
        IntegratedTransmissionComponent {
            component,
            input_state: TransmissionState::new(),
            output_state: TransmissionState::new(),
        }
    }
    
    /// Gets the underlying transmission component.
    pub fn component(&self) -> &TransmissionComponent {
        &self.component
    }
    
    /// Gets a mutable reference to the underlying transmission component.
    pub fn component_mut(&mut self) -> &mut TransmissionComponent {
        &mut self.component
    }
}

impl RotationalTransmission for IntegratedTransmissionComponent {
    fn transmission_id(&self) -> TransmissionId {
        self.component.transmission_id()
    }
    
    fn is_enabled(&self) -> bool {
        self.component.is_enabled()
    }
}

impl IntegratedTransmission for IntegratedTransmissionComponent {
    fn input_state(&self) -> TransmissionState {
        self.input_state
    }
    
    fn output_state(&self) -> TransmissionState {
        self.output_state
    }
    
    fn evaluate(&self, input_state: TransmissionState) -> Result<TransmissionState, TransmissionError> {
        // Default implementation - delegates to evaluation module
        evaluation::evaluate_transmission(
            self.component.transmission_id(),
            self.component.category(),
            input_state,
            self
        )
    }
}

impl SetTransmissionState for IntegratedTransmissionComponent {
    fn set_input_state(&mut self, state: TransmissionState) -> Result<(), TransmissionError> {
        self.input_state = state;
        Ok(())
    }
    
    fn set_output_state(&mut self, state: TransmissionState) -> Result<(), TransmissionError> {
        self.output_state = state;
        Ok(())
    }
}