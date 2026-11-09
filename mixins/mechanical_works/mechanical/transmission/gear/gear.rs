//! Individual gear representation.
//!
//! This module implements an individual gear component with configuration and properties.

use std::fmt;
use crate::mechanical::core::identifiers::ComponentId;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};
use crate::mechanical::transmission::types::{TransmissionCategory, TransmissionId};
use crate::mechanical::transmission::component::TransmissionComponent;
use crate::mechanical::transmission::interfaces::{RotationalInput, RotationalOutput, RotationalTransmission};
use crate::mechanical::transmission::errors::TransmissionError;
use crate::mechanical::transmission::gear::types::{GearId, GearCategory, GearConfiguration};
use crate::mechanical::transmission::gear::errors::GearError;
use crate::mechanical::transmission::integration::{IntegratedTransmission, SetTransmissionState, TransmissionState};

/// A gear component that can be part of the mechanical system.
///
/// This structure represents a gear component within the transmission system,
/// capable of transferring rotational motion with specific gear ratio properties.
#[derive(Debug, Clone)]
pub struct Gear {
    /// Unique identifier for this gear.
    id: GearId,
    
    /// The underlying transmission component identity.
    transmission_component: TransmissionComponent,
    
    /// The category of this gear.
    category: GearCategory,
    
    /// Configuration settings for this gear.
    configuration: GearConfiguration,
    
    /// Current torque being transmitted.
    current_torque: Torque,
    
    /// Current rotational speed being transmitted.
    current_rpm: RotationalSpeed,
    
    /// Current power being transmitted.
    current_power: Power,
    
    /// Indicates if the gear is currently enabled and operational.
    enabled: bool,
}

impl Gear {
    /// Creates a new gear with the specified configuration.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this gear
    /// * `component_id` - The underlying mechanical component ID
    /// * `category` - The category of this gear
    /// * `configuration` - Configuration settings for this gear
    ///
    /// # Returns
    /// * `Ok(Gear)` if creation succeeded
    /// * `Err(GearError)` if configuration is invalid
    pub fn new(
        id: GearId,
        component_id: ComponentId,
        category: GearCategory,
        configuration: GearConfiguration,
    ) -> Result<Self, GearError> {
        // Validate the configuration
        configuration.validate()?;
        
        let transmission_component = TransmissionComponent::new(
            TransmissionId::new(id.get()),
            component_id,
            TransmissionCategory::Gear,
        );
        
        Ok(Gear {
            id,
            transmission_component,
            category,
            configuration,
            current_torque: Torque::new(0.0),
            current_rpm: RotationalSpeed::new(0.0),
            current_power: Power::new(0.0),
            enabled: configuration.enabled,
        })
    }
    
    /// Gets the unique identifier for this gear.
    pub fn id(&self) -> GearId {
        self.id
    }
    
    /// Gets the underlying mechanical component ID.
    pub fn component_id(&self) -> ComponentId {
        self.transmission_component.component_id()
    }
    
    /// Gets the gear category.
    pub fn category(&self) -> GearCategory {
        self.category
    }
    
    /// Gets the configuration of this gear.
    pub fn configuration(&self) -> &GearConfiguration {
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
    /// * `Err(GearError::ExceededTorque)` if torque exceeds maximum
    pub fn set_torque(&mut self, torque: Torque) -> Result<(), GearError> {
        if torque > Torque::new(self.configuration.max_torque) {
            return Err(GearError::ExceededTorque);
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
    /// * `Err(GearError::ExceededRPM)` if RPM exceeds maximum
    pub fn set_rpm(&mut self, rpm: RotationalSpeed) -> Result<(), GearError> {
        if rpm > RotationalSpeed::new(self.configuration.max_rpm) {
            return Err(GearError::ExceededRPM);
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
    /// * `Err(GearError::ExceededPower)` if power exceeds maximum
    pub fn set_power(&mut self, power: Power) -> Result<(), GearError> {
        if power > Power::new(self.configuration.max_power) {
            return Err(GearError::ExceededPower);
        }
        self.current_power = power;
        Ok(())
    }
    
    /// Checks if this gear is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state of this gear.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.transmission_component.set_enabled(enabled);
    }
    
    /// Validates that this gear's configuration is valid.
    ///
    /// # Returns
    /// * `Ok(())` if the gear is valid
    /// * `Err(GearError)` if the gear has invalid configuration
    pub fn validate(&self) -> Result<(), GearError> {
        self.configuration.validate()
    }
}

impl RotationalInput for Gear {
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
        // For now, all outputs are compatible with gear inputs
        // This will be expanded in later implementations
        true
    }
}

impl RotationalOutput for Gear {
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
        // For now, all inputs are compatible with gear outputs
        // This will be expanded in later implementations
        true
    }
}

impl RotationalTransmission for Gear {
    /// Gets the unique identifier for this transmission component.
    fn transmission_id(&self) -> TransmissionId {
        self.transmission_component.transmission_id()
    }
    
    /// Checks if this transmission is currently enabled.
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl IntegratedTransmission for Gear {
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
    
    /// Evaluates the gear behavior given input conditions.
    ///
    /// This method implements the gear ratio calculations and power conservation.
    fn evaluate(&self, input_state: TransmissionState) -> Result<TransmissionState, TransmissionError> {
        // Delegate to the centralized evaluation logic
        crate::mechanical::transmission::evaluation::evaluate_gear(self, input_state)
    }
}

impl SetTransmissionState for Gear {
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

impl fmt::Display for Gear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gear(id: {}, category: {}, rpm: {}, torque: {})", 
            self.id, self.category, self.current_rpm, self.current_torque)
    }
}