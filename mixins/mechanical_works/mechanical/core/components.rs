//! Shared mechanical component contracts.
//!
//! This module defines the foundational representation of mechanical components
//! that can be used across different subsystems.

use std::fmt;

/// The category or type of a mechanical component.
///
/// Component categories help identify what kind of mechanical functionality
/// a component provides within the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentCategory {
    /// A generator that produces mechanical power.
    Generator,
    
    /// A consumer that uses mechanical power.
    Consumer,
    
    /// A transmission component like a shaft or belt.
    Transmission,
    
    /// A machine that performs specific mechanical operations.
    Machine,
    
    /// A structural component that connects other components.
    Structural,
}

impl fmt::Display for ComponentCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentCategory::Generator => write!(f, "Generator"),
            ComponentCategory::Consumer => write!(f, "Consumer"),
            ComponentCategory::Transmission => write!(f, "Transmission"),
            ComponentCategory::Machine => write!(f, "Machine"),
            ComponentCategory::Structural => write!(f, "Structural"),
        }
    }
}

/// A mechanical component identity.
///
/// This structure represents a component's unique identity including its ID
/// and category within the mechanical system.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentIdentity {
    /// Unique identifier for this component.
    pub id: crate::mechanical::core::identifiers::ComponentId,
    
    /// Category of this component.
    pub category: ComponentCategory,
}

impl ComponentIdentity {
    /// Creates a new component identity.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the component
    /// * `category` - The category of the component
    pub fn new(id: crate::mechanical::core::identifiers::ComponentId, category: ComponentCategory) -> Self {
        ComponentIdentity { id, category }
    }
    
    /// Gets the component ID.
    pub fn id(&self) -> crate::mechanical::core::identifiers::ComponentId {
        self.id
    }
    
    /// Gets the component category.
    pub fn category(&self) -> ComponentCategory {
        self.category
    }
}

/// A mechanical component description or metadata.
///
/// This structure provides general information about a component that can be
/// used for display, categorization, and system management purposes.
#[derive(Debug, Clone)]
pub struct ComponentDescription {
    /// A human-readable name for the component.
    pub name: String,
    
    /// A brief description of what the component does.
    pub description: String,
    
    /// The category of the component.
    pub category: ComponentCategory,
    
    /// Optional manufacturer or creator information.
    pub manufacturer: Option<String>,
    
    /// Optional model or version information.
    pub model: Option<String>,
}

impl ComponentDescription {
    /// Creates a new component description.
    ///
    /// # Arguments
    /// * `name` - Human-readable name for the component
    /// * `description` - Brief description of what the component does
    /// * `category` - The category of the component
    pub fn new(
        name: String, 
        description: String, 
        category: ComponentCategory,
    ) -> Self {
        ComponentDescription {
            name,
            description,
            category,
            manufacturer: None,
            model: None,
        }
    }
    
    /// Sets the manufacturer information.
    pub fn with_manufacturer(mut self, manufacturer: String) -> Self {
        self.manufacturer = Some(manufacturer);
        self
    }
    
    /// Sets the model information.
    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }
}

/// A mechanical component that can be part of the system.
///
/// This trait defines the basic interface that all mechanical components must implement.
pub trait MechanicalComponent {
    /// Gets the identity of this component.
    fn identity(&self) -> &ComponentIdentity;
    
    /// Gets the description of this component.
    fn description(&self) -> &ComponentDescription;
    
    /// Checks if this component is compatible with another component.
    ///
    /// # Arguments
    /// * `other` - The other component to check compatibility with
    fn is_compatible_with(&self, other: &dyn MechanicalComponent) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_category_display() {
        assert_eq!(format!("{}", ComponentCategory::Generator), "Generator");
        assert_eq!(format!("{}", ComponentCategory::Consumer), "Consumer");
        assert_eq!(format!("{}", ComponentCategory::Transmission), "Transmission");
        assert_eq!(format!("{}", ComponentCategory::Machine), "Machine");
        assert_eq!(format!("{}", ComponentCategory::Structural), "Structural");
    }

    #[test]
    fn test_component_identity_creation() {
        let id = crate::mechanical::core::identifiers::ComponentId::new(123);
        let identity = ComponentIdentity::new(id, ComponentCategory::Generator);
        
        assert_eq!(identity.id(), id);
        assert_eq!(identity.category(), ComponentCategory::Generator);
    }

    #[test]
    fn test_component_description_creation() {
        let description = ComponentDescription::new(
            "Water Wheel".to_string(),
            "A water-driven mechanical generator".to_string(),
            ComponentCategory::Generator,
        );
        
        assert_eq!(description.name, "Water Wheel");
        assert_eq!(description.description, "A water-driven mechanical generator");
        assert_eq!(description.category, ComponentCategory::Generator);
    }

    #[test]
    fn test_component_description_with_manufacturer_and_model() {
        let description = ComponentDescription::new(
            "Shaft".to_string(),
            "Rotational transmission component".to_string(),
            ComponentCategory::Transmission,
        )
        .with_manufacturer("MechanicalWorks Inc.".to_string())
        .with_model("MW-2023".to_string());
        
        assert_eq!(description.manufacturer, Some("MechanicalWorks Inc.".to_string()));
        assert_eq!(description.model, Some("MW-2023".to_string()));
    }
}