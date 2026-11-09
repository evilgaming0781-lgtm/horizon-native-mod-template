//! Rotational shaft implementation.
//!
//! This module implements the first actual transmission component - a rotational shaft.

use std::fmt;
use crate::mechanical::core::identifiers::{ComponentId, NetworkId};
use crate::mechanical::core::units::{Torque, AngularVelocity, RotationalSpeed, Power};
use crate::mechanical::transmission::types::{TransmissionCategory, TransmissionId, TransmissionConfiguration};
use crate::mechanical::transmission::interfaces::{RotationalInput, RotationalOutput, RotationalTransmission};
use crate::mechanical::transmission::errors::{TransmissionError, TransmissionResult};
use crate::mechanical::transmission::integration::{IntegratedTransmission, SetTransmissionState, TransmissionState};

/// A rotational shaft that transfers mechanical power.
///
/// This structure represents a shaft component within the transmission system,
/// capable of transferring rotational motion between connected components.
#[derive(Debug, Clone)]
pub struct Shaft {
    /// Unique identifier for this shaft.
    id: TransmissionId,
    
    /// The underlying mechanical component ID.
    component_id: ComponentId,
    
    /// Network that this shaft belongs to.
    network_id: Option<NetworkId>,
    
    /// Configuration settings for this shaft.
    configuration: TransmissionConfiguration,
    
    /// Current torque being transmitted.
    current_torque: Torque,
    
    /// Current rotational speed being transmitted.
    current_rpm: RotationalSpeed,
    
    /// Current power being transmitted.
    current_power: Power,
    
    /// Indicates if the shaft is currently enabled and operational.
    enabled: bool,
}

impl Shaft {
    /// Creates a new shaft with the specified configuration.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this shaft
    /// * `component_id` - The underlying mechanical component ID
    /// * `configuration` - Configuration settings for this shaft
    pub fn new(
        id: TransmissionId,
        component_id: ComponentId,
        configuration: TransmissionConfiguration,
    ) -> Self {
        Shaft {
            id,
            component_id,
            network_id: None,
            configuration,
            current_torque: Torque::new(0.0),
            current_rpm: RotationalSpeed::new(0.0),
            current_power: Power::new(0.0),
            enabled: configuration.enabled,
        }
    }
    
    /// Gets the unique identifier for this shaft.
    pub fn id(&self) -> TransmissionId {
        self.id
    }
    
    /// Gets the underlying mechanical component ID.
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
    
    /// Gets the network ID that this shaft belongs to.
    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id
    }
    
    /// Sets the network ID for this shaft.
    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id);
    }
    
    /// Gets the configuration of this shaft.
    pub fn configuration(&self) -> &TransmissionConfiguration {
        &self.configuration
    }
    
    /// Gets the current torque being transmitted.
    pub fn current_torque(&self) -> Torque {
        self.current_torque
    }
    
    /// Gets the current rotational speed being transmitted.
    pub fn current_rpm(&self) -> RotationalSpeed {
        self.current_rpm
    }
    
    /// Gets the current power being transmitted.
    pub fn current_power(&self) -> Power {
        self.current_power
    }
    
    /// Sets the current torque being transmitted.
    ///
    /// # Arguments
    /// * `torque` - The torque to set
    ///
    /// # Returns
    /// * `Ok(())` if the torque is within limits
    /// * `Err(TransmissionError::ExceededTorque)` if torque exceeds maximum
    pub fn set_torque(&mut self, torque: Torque) -> TransmissionResult<()> {
        if torque > Torque::new(self.configuration.max_torque) {
            return Err(TransmissionError::ExceededTorque);
        }
        self.current_torque = torque;
        Ok(())
    }
    
    /// Sets the current rotational speed being transmitted.
    ///
    /// # Arguments
    /// * `rpm` - The rotational speed to set
    ///
    /// # Returns
    /// * `Ok(())` if the RPM is within limits
    /// * `Err(TransmissionError::ExceededRPM)` if RPM exceeds maximum
    pub fn set_rpm(&mut self, rpm: RotationalSpeed) -> TransmissionResult<()> {
        if rpm > RotationalSpeed::new(self.configuration.max_rpm) {
            return Err(TransmissionError::ExceededRPM);
        }
        self.current_rpm = rpm;
        Ok(())
    }
    
    /// Sets the current power being transmitted.
    ///
    /// # Arguments
    /// * `power` - The power to set
    ///
    /// # Returns
    /// * `Ok(())` if the power is within limits
    /// * `Err(TransmissionError::ExceededPower)` if power exceeds maximum
    pub fn set_power(&mut self, power: Power) -> TransmissionResult<()> {
        if power > Power::new(self.configuration.max_power) {
            return Err(TransmissionError::ExceededPower);
        }
        self.current_power = power;
        Ok(())
    }
    
    /// Checks if this shaft is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state of this shaft.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl RotationalInput for Shaft {
    /// Gets the maximum torque this input can handle.
    fn max_input_torque(&self) -> Torque {
        Torque::new(self.configuration.max_torque)
    }
    
    /// Gets the maximum rotational speed this input can handle.
    fn max_input_rpm(&self) -> RotationalSpeed {
        RotationalSpeed::new(self.configuration.max_rpm)
    }
    
    /// Checks if this input is compatible with the given output.
    ///
    /// # Arguments
    /// * `output` - The rotational output to check compatibility with
    fn is_compatible_with_output(&self, _output: &dyn RotationalOutput) -> bool {
        // For now, all outputs are compatible with shaft inputs
        // This will be expanded in later implementations
        true
    }
}

impl RotationalOutput for Shaft {
    /// Gets the current torque being delivered.
    fn current_torque(&self) -> Torque {
        self.current_torque
    }
    
    /// Gets the current rotational speed being delivered.
    fn current_rpm(&self) -> RotationalSpeed {
        self.current_rpm
    }
    
    /// Gets the current power being delivered.
    fn current_power(&self) -> Power {
        self.current_power
    }
    
    /// Checks if this output is compatible with the given input.
    ///
    /// # Arguments
    /// * `input` - The rotational input to check compatibility with
    fn is_compatible_with_input(&self, _input: &dyn RotationalInput) -> bool {
        // For now, all inputs are compatible with shaft outputs
        // This will be expanded in later implementations
        true
    }
}

impl RotationalTransmission for Shaft {
    /// Gets the unique identifier for this transmission component.
    fn transmission_id(&self) -> TransmissionId {
        self.id
    }
    
    /// Checks if this transmission is currently enabled.
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl IntegratedTransmission for Shaft {
    fn input_state(&self) -> TransmissionState {
        TransmissionState::with_values(
            self.current_torque(),
            self.current_rpm(),
            self.current_power(),
            self.is_enabled()
        )
    }
    
    fn output_state(&self) -> TransmissionState {
        TransmissionState::with_values(
            self.current_torque(),
            self.current_rpm(),
            self.current_power(),
            self.is_enabled()
        )
    }
    
    /// Evaluates the shaft behavior given input conditions.
    ///
    /// For a shaft, this is a simple pass-through with validation.
    fn evaluate(&self, input_state: TransmissionState) -> Result<TransmissionState, TransmissionError> {
        // Delegate to the centralized evaluation logic
        crate::mechanical::transmission::evaluation::evaluate_shaft(self, input_state)
    }
}

impl SetTransmissionState for Shaft {
    fn set_input_state(&mut self, state: TransmissionState) -> Result<(), TransmissionError> {
        self.current_torque = state.torque;
        self.current_rpm = state.rpm;
        self.current_power = state.power;
        self.enabled = state.enabled;
        Ok(())
    }
    
    fn set_output_state(&mut self, state: TransmissionState) -> Result<(), TransmissionError> {
        self.current_torque = state.torque;
        self.current_rpm = state.rpm;
        self.current_power = state.power;
        self.enabled = state.enabled;
        Ok(())
    }
}

impl fmt::Display for Shaft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shaft(id: {}, component_id: {}, rpm: {}, torque: {})", 
            self.id, self.component_id, self.current_rpm, self.current_torque)
    }
}