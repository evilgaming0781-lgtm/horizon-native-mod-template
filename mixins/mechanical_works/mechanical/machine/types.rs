//! Machine system foundational types for Horizon Mechanical Works.
//!
//! This module defines the core machine-related data types and structures used throughout
//! the mechanical automation system.

use crate::mechanical::power::types::*;
use std::collections::HashMap;

/// Unique identifier for a machine instance.
///
/// Machine IDs are globally unique within the mod to ensure proper tracking and management.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineId(pub String);

impl MachineId {
    /// Creates a new machine ID from a string.
    ///
    /// # Arguments
    /// * `id` - A unique identifier string for the machine instance
    ///
    /// # Returns
    /// A new MachineId instance
    pub fn new(id: String) -> Self {
        MachineId(id)
    }

    /// Gets the string representation of this machine ID.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MachineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier for a machine type.
///
/// Machine types define the category or class of machine (e.g., "water_wheel", "furnace").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineTypeId(pub String);

impl MachineTypeId {
    /// Creates a new machine type ID from a string.
    ///
    /// # Arguments
    /// * `type_id` - A unique identifier string for the machine type
    ///
    /// # Returns
    /// A new MachineTypeId instance
    pub fn new(type_id: String) -> Self {
        MachineTypeId(type_id)
    }

    /// Gets the string representation of this machine type ID.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MachineTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the specifications and capabilities of a machine type.
///
/// This structure defines the static characteristics of a machine type that don't change
/// during operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineSpecs {
    /// The unique identifier for this machine type.
    pub type_id: MachineTypeId,
    
    /// Human-readable name for this machine type.
    pub name: String,
    
    /// Description of what this machine does or represents.
    pub description: String,
    
    /// Maximum power consumption (in watts) that this machine can use.
    pub max_power_consumption: Watts,
    
    /// Minimum power required (in watts) to operate this machine.
    pub min_power_required: Watts,
    
    /// Maximum inventory slots this machine supports.
    pub max_inventory_slots: u32,
    
    /// Whether this machine requires power to operate.
    pub requires_power: bool,
}

impl MachineSpecs {
    /// Creates new machine specifications.
    ///
    /// # Arguments
    /// * `type_id` - Unique identifier for the machine type
    /// * `name` - Human-readable name
    /// * `description` - Description of the machine's function
    /// * `max_power_consumption` - Maximum power consumption in watts
    /// * `min_power_required` - Minimum power required to operate (must be <= max)
    /// * `max_inventory_slots` - Maximum inventory slots
    /// * `requires_power` - Whether this machine requires power to operate
    ///
    /// # Returns
    /// A new MachineSpecs instance or PowerError if validation fails
    pub fn new(
        type_id: MachineTypeId,
        name: String,
        description: String,
        max_power_consumption: Watts,
        min_power_required: Watts,
        max_inventory_slots: u32,
        requires_power: bool,
    ) -> Result<Self, PowerError> {
        if min_power_required.get() > max_power_consumption.get() {
            return Err(PowerError::InvalidValue);
        }
        
        Ok(MachineSpecs {
            type_id,
            name,
            description,
            max_power_consumption,
            min_power_required,
            max_inventory_slots,
            requires_power,
        })
    }

    /// Gets the maximum power consumption for this machine type.
    pub fn max_power_consumption(&self) -> Watts {
        self.max_power_consumption
    }

    /// Gets the minimum power required to operate this machine type.
    pub fn min_power_required(&self) -> Watts {
        self.min_power_required
    }

    /// Checks if this machine type requires power to operate.
    pub fn requires_power(&self) -> bool {
        self.requires_power
    }
}

/// Configuration structure for a specific machine instance.
///
/// This represents the runtime configuration that can be modified while the machine is active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineConfig {
    /// The unique identifier for this machine instance.
    pub id: MachineId,
    
    /// The type of machine this instance represents.
    pub type_id: MachineTypeId,
    
    /// Current power allocation (in watts) for this machine.
    pub power_allocation: Watts,
    
    /// Current inventory state for this machine.
    pub inventory: HashMap<String, u32>,
    
    /// Whether this machine is currently enabled and operational.
    pub enabled: bool,
}

impl MachineConfig {
    /// Creates a new machine configuration.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the machine instance
    /// * `type_id` - Type identifier for the machine
    /// * `power_allocation` - Initial power allocation (must be within limits)
    /// * `inventory` - Initial inventory state
    /// * `enabled` - Whether the machine is initially enabled
    ///
    /// # Returns
    /// A new MachineConfig instance or PowerError if validation fails
    pub fn new(
        id: MachineId,
        type_id: MachineTypeId,
        power_allocation: Watts,
        inventory: HashMap<String, u32>,
        enabled: bool,
    ) -> Result<Self, PowerError> {
        // Note: We don't validate power allocation here as it depends on the machine specs
        // which are not available in this context. Validation should happen at the machine level.
        
        Ok(MachineConfig {
            id,
            type_id,
            power_allocation,
            inventory,
            enabled,
        })
    }

    /// Gets the current power allocation for this machine.
    pub fn power_allocation(&self) -> Watts {
        self.power_allocation
    }

    /// Sets the power allocation for this machine.
    ///
    /// # Arguments
    /// * `allocation` - New power allocation in watts
    pub fn set_power_allocation(&mut self, allocation: Watts) {
        self.power_allocation = allocation;
    }

    /// Gets the current inventory state.
    pub fn inventory(&self) -> &HashMap<String, u32> {
        &self.inventory
    }

    /// Gets a mutable reference to the inventory state.
    pub fn inventory_mut(&mut self) -> &mut HashMap<String, u32> {
        &mut self.inventory
    }

    /// Checks if this machine is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables this machine.
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable (true) or disable (false) the machine
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Represents a machine instance with its specifications and configuration.
///
/// This is the complete representation of a machine in the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    /// The specifications that define this machine type.
    pub specs: MachineSpecs,
    
    /// The runtime configuration for this specific machine instance.
    pub config: MachineConfig,
}

impl Machine {
    /// Creates a new machine instance from specs and config.
    ///
    /// # Arguments
    /// * `specs` - The specifications for this machine type
    /// * `config` - The runtime configuration for this instance
    ///
    /// # Returns
    /// A new Machine instance
    pub fn new(specs: MachineSpecs, config: MachineConfig) -> Self {
        Machine { specs, config }
    }

    /// Gets the unique identifier for this machine instance.
    pub fn id(&self) -> &MachineId {
        &self.config.id
    }

    /// Gets the type identifier for this machine.
    pub fn type_id(&self) -> &MachineTypeId {
        &self.config.type_id
    }

    /// Gets the name of this machine type.
    pub fn name(&self) -> &str {
        &self.specs.name
    }

    /// Gets the description of this machine.
    pub fn description(&self) -> &str {
        &self.specs.description
    }

    /// Gets the maximum power consumption for this machine.
    pub fn max_power_consumption(&self) -> Watts {
        self.specs.max_power_consumption
    }

    /// Gets the minimum power required to operate this machine.
    pub fn min_power_required(&self) -> Watts {
        self.specs.min_power_required
    }

    /// Checks if this machine requires power to operate.
    pub fn requires_power(&self) -> bool {
        self.specs.requires_power
    }

    /// Gets the current power allocation for this machine.
    pub fn power_allocation(&self) -> Watts {
        self.config.power_allocation
    }

    /// Sets the power allocation for this machine.
    ///
    /// # Arguments
    /// * `allocation` - New power allocation in watts
    pub fn set_power_allocation(&mut self, allocation: Watts) {
        self.config.set_power_allocation(allocation);
    }

    /// Gets the current inventory state.
    pub fn inventory(&self) -> &HashMap<String, u32> {
        self.config.inventory()
    }

    /// Gets a mutable reference to the inventory state.
    pub fn inventory_mut(&mut self) -> &mut HashMap<String, u32> {
        self.config.inventory_mut()
    }

    /// Checks if this machine is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.is_enabled()
    }

    /// Enables or disables this machine.
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable (true) or disable (false) the machine
    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.set_enabled(enabled);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_id_creation() {
        let id = MachineId::new(String::from("machine_001"));
        assert_eq!(id.as_str(), "machine_001");
        assert_eq!(format!("{}", id), "machine_001");
    }

    #[test]
    fn test_machine_type_id_creation() {
        let type_id = MachineTypeId::new(String::from("water_wheel"));
        assert_eq!(type_id.as_str(), "water_wheel");
        assert_eq!(format!("{}", type_id), "water_wheel");
    }

    #[test]
    fn test_machine_specs_creation() {
        let specs = MachineSpecs::new(
            MachineTypeId::new(String::from("water_wheel")),
            String::from("Water Wheel"),
            String::from("Generates power from flowing water"),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        assert_eq!(specs.type_id.as_str(), "water_wheel");
        assert_eq!(specs.name, "Water Wheel");
        assert_eq!(specs.description, "Generates power from flowing water");
        assert_eq!(specs.max_power_consumption(), Watts::new(1000));
        assert_eq!(specs.min_power_required(), Watts::new(200));
        assert_eq!(specs.max_inventory_slots, 4);
        assert!(specs.requires_power());
    }

    #[test]
    fn test_machine_specs_invalid_power_requirements() {
        let result = MachineSpecs::new(
            MachineTypeId::new(String::from("water_wheel")),
            String::from("Water Wheel"),
            String::from("Generates power from flowing water"),
            Watts::new(200),  // Less than min
            Watts::new(1000),
            4,
            true,
        );
        
        assert!(matches!(result, Err(PowerError::InvalidValue)));
    }

    #[test]
    fn test_machine_config_creation() {
        let mut inventory = HashMap::new();
        inventory.insert(String::from("wood"), 5);
        
        let config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        assert_eq!(config.id.as_str(), "machine_001");
        assert_eq!(config.type_id.as_str(), "water_wheel");
        assert_eq!(config.power_allocation(), Watts::new(500));
        assert_eq!(config.inventory().get("wood"), Some(&5));
        assert!(config.is_enabled());
    }

    #[test]
    fn test_machine_config_power_allocation() {
        let mut inventory = HashMap::new();
        let mut config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        assert_eq!(config.power_allocation(), Watts::new(500));
        
        config.set_power_allocation(Watts::new(750));
        assert_eq!(config.power_allocation(), Watts::new(750));
    }

    #[test]
    fn test_machine_config_inventory() {
        let mut inventory = HashMap::new();
        inventory.insert(String::from("wood"), 5);
        
        let mut config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        assert_eq!(config.inventory().get("wood"), Some(&5));
        
        // Test mutable access
        config.inventory_mut().insert(String::from("stone"), 3);
        assert_eq!(config.inventory().get("stone"), Some(&3));
    }

    #[test]
    fn test_machine_config_enabled() {
        let inventory = HashMap::new();
        let mut config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        assert!(config.is_enabled());
        
        config.set_enabled(false);
        assert!(!config.is_enabled());
    }

    #[test]
    fn test_machine_creation() {
        let specs = MachineSpecs::new(
            MachineTypeId::new(String::from("water_wheel")),
            String::from("Water Wheel"),
            String::from("Generates power from flowing water"),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let mut inventory = HashMap::new();
        inventory.insert(String::from("wood"), 5);
        
        let config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        let machine = Machine::new(specs, config);
        
        assert_eq!(machine.id().as_str(), "machine_001");
        assert_eq!(machine.type_id().as_str(), "water_wheel");
        assert_eq!(machine.name(), "Water Wheel");
        assert_eq!(machine.description(), "Generates power from flowing water");
        assert_eq!(machine.max_power_consumption(), Watts::new(1000));
        assert_eq!(machine.min_power_required(), Watts::new(200));
        assert!(machine.requires_power());
        assert_eq!(machine.power_allocation(), Watts::new(500));
        assert_eq!(machine.inventory().get("wood"), Some(&5));
        assert!(machine.is_enabled());
    }

    #[test]
    fn test_machine_set_power_allocation() {
        let specs = MachineSpecs::new(
            MachineTypeId::new(String::from("water_wheel")),
            String::from("Water Wheel"),
            String::from("Generates power from flowing water"),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let mut inventory = HashMap::new();
        let config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        let mut machine = Machine::new(specs, config);
        
        assert_eq!(machine.power_allocation(), Watts::new(500));
        
        machine.set_power_allocation(Watts::new(750));
        assert_eq!(machine.power_allocation(), Watts::new(750));
    }

    #[test]
    fn test_machine_set_enabled() {
        let specs = MachineSpecs::new(
            MachineTypeId::new(String::from("water_wheel")),
            String::from("Water Wheel"),
            String::from("Generates power from flowing water"),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let inventory = HashMap::new();
        let mut config = MachineConfig::new(
            MachineId::new(String::from("machine_001")),
            MachineTypeId::new(String::from("water_wheel")),
            Watts::new(500),
            inventory,
            true,
        ).unwrap();
        
        let mut machine = Machine::new(specs, config);
        
        assert!(machine.is_enabled());
        
        machine.set_enabled(false);
        assert!(!machine.is_enabled());
    }
}