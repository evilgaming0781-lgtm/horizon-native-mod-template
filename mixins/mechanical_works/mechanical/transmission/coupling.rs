//! Mechanical coupling implementation.
//!
//! This module implements the foundation for connecting transmission components.

use std::fmt;
use crate::mechanical::core::identifiers::{ComponentId, NetworkId};
use crate::mechanical::transmission::types::{TransmissionCategory, TransmissionId};
use crate::mechanical::transmission::interfaces::RotationalTransmission;
use crate::mechanical::transmission::errors::{TransmissionError, TransmissionResult};
use crate::mechanical::transmission::integration::{IntegratedTransmission, SetTransmissionState, TransmissionState};

/// A mechanical coupling that connects two transmission components.
///
/// This structure represents a coupling component that can connect two 
/// rotational transmission components together.
#[derive(Debug, Clone)]
pub struct Coupling {
    /// Unique identifier for this coupling.
    id: TransmissionId,
    
    /// The underlying mechanical component ID.
    component_id: ComponentId,
    
    /// Network that this coupling belongs to.
    network_id: Option<NetworkId>,
    
    /// First transmission component connected to this coupling.
    component_a: Option<TransmissionId>,
    
    /// Second transmission component connected to this coupling.
    component_b: Option<TransmissionId>,
    
    /// Indicates if the coupling is currently enabled and operational.
    enabled: bool,
}

impl Coupling {
    /// Creates a new coupling with the specified identifier.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this coupling
    /// * `component_id` - The underlying mechanical component ID
    pub fn new(
        id: TransmissionId,
        component_id: ComponentId,
    ) -> Self {
        Coupling {
            id,
            component_id,
            network_id: None,
            component_a: None,
            component_b: None,
            enabled: true,
        }
    }
    
    /// Gets the unique identifier for this coupling.
    pub fn id(&self) -> TransmissionId {
        self.id
    }
    
    /// Gets the underlying mechanical component ID.
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
    
    /// Gets the network ID that this coupling belongs to.
    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id
    }
    
    /// Sets the network ID for this coupling.
    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id);
    }
    
    /// Gets the first transmission component connected to this coupling.
    pub fn component_a(&self) -> Option<TransmissionId> {
        self.component_a
    }
    
    /// Gets the second transmission component connected to this coupling.
    pub fn component_b(&self) -> Option<TransmissionId> {
        self.component_b
    }
    
    /// Attaches a transmission component to this coupling.
    ///
    /// # Arguments
    /// * `transmission_id` - The transmission component to attach
    ///
    /// # Returns
    /// * `Ok(())` if attachment succeeded
    /// * `Err(TransmissionError::InvalidAttachment)` if attachment is invalid
    pub fn attach_component(&mut self, transmission_id: TransmissionId) -> TransmissionResult<()> {
        // Check for self-connection (a coupling cannot connect to itself)
        if let Some(existing_component) = self.component_a {
            if existing_component == transmission_id {
                return Err(TransmissionError::SelfConnection);
            }
        }
        
        if let Some(existing_component) = self.component_b {
            if existing_component == transmission_id {
                return Err(TransmissionError::SelfConnection);
            }
        }
        
        // Check for duplicate attachment
        if self.component_a == Some(transmission_id) || self.component_b == Some(transmission_id) {
            return Err(TransmissionError::DuplicateAttachment);
        }
        
        // Attach to first available slot
        if self.component_a.is_none() {
            self.component_a = Some(transmission_id);
        } else if self.component_b.is_none() {
            self.component_b = Some(transmission_id);
        } else {
            return Err(TransmissionError::TooManyAttachments);
        }
        
        Ok(())
    }
    
    /// Detaches a transmission component from this coupling.
    ///
    /// # Arguments
    /// * `transmission_id` - The transmission component to detach
    ///
    /// # Returns
    /// * `Ok(())` if detachment succeeded
    /// * `Err(TransmissionError::MissingAttachment)` if the component was not attached
    pub fn detach_component(&mut self, transmission_id: TransmissionId) -> TransmissionResult<()> {
        if self.component_a == Some(transmission_id) {
            self.component_a = None;
        } else if self.component_b == Some(transmission_id) {
            self.component_b = None;
        } else {
            return Err(TransmissionError::MissingAttachment);
        }
        
        Ok(())
    }
    
    /// Checks if this coupling is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state of this coupling.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Checks if both components are connected to this coupling.
    pub fn is_connected(&self) -> bool {
        self.component_a.is_some() && self.component_b.is_some()
    }
    
    /// Validates that the coupling has two valid attachments.
    ///
    /// # Returns
    /// * `Ok(())` if coupling is valid
    /// * `Err(TransmissionError::InvalidCoupling)` if coupling is invalid
    pub fn validate(&self) -> TransmissionResult<()> {
        // A coupling must have exactly 2 components attached
        if self.component_a.is_none() || self.component_b.is_none() {
            return Err(TransmissionError::InvalidCoupling);
        }
        
        // Check for self-connections (should not happen in normal operation)
        if let (Some(a), Some(b)) = (self.component_a, self.component_b) {
            if a == b {
                return Err(TransmissionError::SelfConnection);
            }
        }
        
        Ok(())
    }
}

impl RotationalTransmission for Coupling {
    /// Gets the unique identifier for this transmission component.
    fn transmission_id(&self) -> TransmissionId {
        self.id
    }
    
    /// Checks if this transmission is currently enabled.
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl IntegratedTransmission for Coupling {
    fn input_state(&self) -> TransmissionState {
        // A coupling doesn't have a fixed input/output state - it depends on the connected components
        TransmissionState::new()
    }
    
    fn output_state(&self) -> TransmissionState {
        // A coupling doesn't have a fixed input/output state - it depends on the connected components
        TransmissionState::new()
    }
    
    /// Evaluates the coupling behavior given input conditions.
    ///
    /// For a coupling, this is essentially a pass-through with validation.
    fn evaluate(&self, input_state: TransmissionState) -> Result<TransmissionState, TransmissionError> {
        // Delegate to the centralized evaluation logic
        crate::mechanical::transmission::evaluation::evaluate_coupling(self, input_state)
    }
}

impl SetTransmissionState for Coupling {
    fn set_input_state(&mut self, _state: TransmissionState) -> Result<(), TransmissionError> {
        // A coupling doesn't maintain its own state - it just connects components
        Ok(())
    }
    
    fn set_output_state(&mut self, _state: TransmissionState) -> Result<(), TransmissionError> {
        // A coupling doesn't maintain its own state - it just connects components
        Ok(())
    }
}

impl fmt::Display for Coupling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coupling(id: {}, component_a: {:?}, component_b: {:?})", 
            self.id, self.component_a, self.component_b)
    }
}