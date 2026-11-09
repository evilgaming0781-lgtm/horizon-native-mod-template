//! Common transmission component representation.
//!
//! This module provides a reusable abstraction for transmission components.

use crate::mechanical::core::components::{ComponentIdentity, ComponentDescription, MechanicalComponent};
use crate::mechanical::core::identifiers::ComponentId;
use crate::mechanical::transmission::types::{TransmissionCategory, TransmissionId};

/// A transmission component that can be part of the mechanical system.
///
/// This structure represents a transmission component's identity, category,
/// and operational state within the system.
#[derive(Debug, Clone)]
pub struct TransmissionComponent {
    /// The unique identifier for this transmission component.
    pub id: TransmissionId,
    
    /// The underlying mechanical component identity.
    pub component_identity: ComponentIdentity,
    
    /// The category of this transmission component.
    pub category: TransmissionCategory,
    
    /// Indicates if the transmission is currently enabled and operational.
    pub enabled: bool,
}

impl TransmissionComponent {
    /// Creates a new transmission component.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the transmission component
    /// * `component_id` - The underlying mechanical component ID
    /// * `category` - The category of this transmission component
    pub fn new(
        id: TransmissionId,
        component_id: ComponentId,
        category: TransmissionCategory,
    ) -> Self {
        let component_identity = ComponentIdentity::new(component_id, crate::mechanical::core::components::ComponentCategory::Transmission);
        
        TransmissionComponent {
            id,
            component_identity,
            category,
            enabled: true,
        }
    }
    
    /// Gets the transmission ID.
    pub fn transmission_id(&self) -> TransmissionId {
        self.id
    }
    
    /// Gets the underlying mechanical component ID.
    pub fn component_id(&self) -> ComponentId {
        self.component_identity.id()
    }
    
    /// Gets the category of this transmission component.
    pub fn category(&self) -> TransmissionCategory {
        self.category
    }
    
    /// Checks if this transmission component is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state of this transmission component.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl MechanicalComponent for TransmissionComponent {
    /// Gets the identity of this component.
    fn identity(&self) -> &ComponentIdentity {
        &self.component_identity
    }
    
    /// Gets the description of this component.
    fn description(&self) -> &ComponentDescription {
        // This would typically be a more complex implementation that generates 
        // appropriate descriptions based on category, but for now we return a placeholder
        unimplemented!("Component description not yet implemented")
    }
    
    /// Checks if this component is compatible with another component.
    ///
    /// # Arguments
    /// * `other` - The other component to check compatibility with
    fn is_compatible_with(&self, _other: &dyn MechanicalComponent) -> bool {
        // For now, we'll assume all components are compatible - this will be refined later
        true
    }
}