//! Machine registry system for Horizon Mechanical Works.
//!
//! This module provides a centralized registry for machine type definitions,
//! allowing registration, retrieval, and management of machine specifications.

use crate::mechanical::machine::types::*;
use std::collections::HashMap;

/// Represents the result of a machine registry operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryResult<T> {
    /// Whether the operation was successful.
    pub success: bool,
    
    /// The result value if the operation succeeded.
    pub value: Option<T>,
    
    /// Optional message describing the result or reason for failure.
    pub message: String,
}

impl<T> RegistryResult<T> {
    /// Creates a new successful registry result.
    pub fn success(value: T, message: String) -> Self {
        RegistryResult {
            success: true,
            value: Some(value),
            message,
        }
    }

    /// Creates a new failed registry result.
    pub fn failure(message: String) -> Self {
        RegistryResult {
            success: false,
            value: None,
            message,
        }
    }
}

/// Error type for machine registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Attempted to register a machine type that already exists.
    DuplicateRegistration {
        type_id: String,
        message: String,
    },
    
    /// Attempted to retrieve a machine type that does not exist.
    NotFound {
        type_id: String,
        message: String,
    },
    
    /// Invalid machine specifications provided for registration.
    InvalidSpecifications {
        type_id: String,
        reason: String,
    },
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::DuplicateRegistration { type_id, message } => {
                write!(f, "Duplicate registration for machine type '{}': {}", type_id, message)
            }
            RegistryError::NotFound { type_id, message } => {
                write!(f, "Machine type '{}' not found: {}", type_id, message)
            }
            RegistryError::InvalidSpecifications { type_id, reason } => {
                write!(f, "Invalid specifications for machine type '{}': {}", type_id, reason)
            }
        }
    }
}

impl std::error::Error for RegistryError {}

/// Centralized registry for machine type definitions.
///
/// This struct provides a thread-safe way to register, retrieve, and manage
/// machine type specifications throughout the system.
pub struct MachineRegistry {
    /// Map of registered machine types by their unique type identifiers.
    machine_types: HashMap<MachineTypeId, MachineSpecs>,
}

impl MachineRegistry {
    /// Creates a new empty machine registry.
    ///
    /// # Returns
    /// A new MachineRegistry instance
    pub fn new() -> Self {
        MachineRegistry {
            machine_types: HashMap::new(),
        }
    }

    /// Registers a new machine type definition.
    ///
    /// # Arguments
    /// * `specs` - The machine specifications to register
    ///
    /// # Returns
    /// A RegistryResult indicating success or failure of the registration
    pub fn register_machine_type(&mut self, specs: MachineSpecs) -> RegistryResult<MachineTypeId> {
        let type_id = specs.type_id.clone();
        
        // Check if this machine type is already registered
        if self.machine_types.contains_key(&type_id) {
            return RegistryResult::failure(format!(
                "Machine type '{}' is already registered",
                type_id.as_str()
            ));
        }
        
        // Validate the specifications before registration
        match Self::validate_specs(&specs) {
            Ok(_) => {
                // Register the machine type
                self.machine_types.insert(type_id.clone(), specs);
                
                RegistryResult::success(
                    type_id,
                    format!("Successfully registered machine type '{}'", type_id.as_str()),
                )
            }
            Err(validation_error) => {
                RegistryResult::failure(format!(
                    "Failed to register machine type '{}': {}",
                    type_id.as_str(),
                    validation_error
                ))
            }
        }
    }

    /// Retrieves a machine type definition by its unique identifier.
    ///
    /// # Arguments
    /// * `type_id` - The unique identifier of the machine type to retrieve
    ///
    /// # Returns
    /// A RegistryResult containing the machine specifications if found,
    /// or an error if not found
    pub fn get_machine_type(&self, type_id: &MachineTypeId) -> RegistryResult<MachineSpecs> {
        match self.machine_types.get(type_id) {
            Some(specs) => RegistryResult::success(
                specs.clone(),
                format!("Successfully retrieved machine type '{}'", type_id.as_str()),
            ),
            None => RegistryResult::failure(format!(
                "Machine type '{}' not found in registry",
                type_id.as_str()
            )),
        }
    }

    /// Removes a machine type definition from the registry.
    ///
    /// # Arguments
    /// * `type_id` - The unique identifier of the machine type to remove
    ///
    /// # Returns
    /// A RegistryResult indicating success or failure of the removal
    pub fn remove_machine_type(&mut self, type_id: &MachineTypeId) -> RegistryResult<MachineTypeId> {
        match self.machine_types.remove(type_id) {
            Some(_) => RegistryResult::success(
                type_id.clone(),
                format!("Successfully removed machine type '{}'", type_id.as_str()),
            ),
            None => RegistryResult::failure(format!(
                "Cannot remove machine type '{}': not found in registry",
                type_id.as_str()
            )),
        }
    }

    /// Checks if a machine type is registered.
    ///
    /// # Arguments
    /// * `type_id` - The unique identifier of the machine type to check
    ///
    /// # Returns
    /// `true` if the machine type is registered, `false` otherwise
    pub fn contains_machine_type(&self, type_id: &MachineTypeId) -> bool {
        self.machine_types.contains_key(type_id)
    }

    /// Gets the count of registered machine types.
    ///
    /// # Returns
    /// The number of registered machine types
    pub fn len(&self) -> usize {
        self.machine_types.len()
    }

    /// Checks if the registry is empty.
    ///
    /// # Returns
    /// `true` if no machine types are registered, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.machine_types.is_empty()
    }

    /// Gets an iterator over all registered machine type IDs.
    ///
    /// # Returns
    /// An iterator over the registered machine type IDs
    pub fn machine_type_ids(&self) -> impl Iterator<Item = &MachineTypeId> {
        self.machine_types.keys()
    }

    /// Validates machine specifications before registration.
    ///
    /// # Arguments
    /// * `specs` - The machine specifications to validate
    ///
    /// # Returns
    /// `Ok(())` if validation passes, or `RegistryError` if it fails
    fn validate_specs(specs: &MachineSpecs) -> Result<(), String> {
        // Check that type_id is not empty
        if specs.type_id.as_str().is_empty() {
            return Err("Machine type ID cannot be empty".to_string());
        }
        
        // Check that name is not empty
        if specs.name.is_empty() {
            return Err("Machine name cannot be empty".to_string());
        }
        
        // Check that description is not empty
        if specs.description.is_empty() {
        return Err("Machine description cannot be empty".to_string());
        }
        
        // Validate power requirements
        if specs.min_power_required.get() > specs.max_power_consumption.get() {
            return Err(
                "Minimum power required cannot exceed maximum power consumption".to_string()
            );
        }
        
        // Validate inventory slots
        if specs.max_inventory_slots == 0 {
            return Err("Maximum inventory slots must be greater than zero".to_string());
        }
        
        Ok(())
    }
}

impl Default for MachineRegistry {
    /// Creates a new empty machine registry with default settings.
    ///
    /// This is primarily for testing and should not be used in production.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_result_creation() {
        let success_result = RegistryResult::success(MachineTypeId::new("test".to_string()), "Test message".to_string());
        assert!(success_result.success);
        assert_eq!(success_result.value, Some(MachineTypeId::new("test".to_string())));
        assert_eq!(success_result.message, "Test message");

        let failure_result = RegistryResult::failure("Error message".to_string());
        assert!(!failure_result.success);
        assert_eq!(failure_result.value, None);
        assert_eq!(failure_result.message, "Error message");
    }

    #[test]
    fn test_registry_error_display() {
        let error = RegistryError::DuplicateRegistration {
            type_id: "test_machine".to_string(),
            message: "Already registered".to_string(),
        };
        assert_eq!(format!("{}", error), "Duplicate registration for machine type 'test_machine': Already registered");

        let error2 = RegistryError::NotFound {
            type_id: "missing_machine".to_string(),
            message: "Not found".to_string(),
        };
        assert_eq!(format!("{}", error2), "Machine type 'missing_machine' not found: Not found");

        let error3 = RegistryError::InvalidSpecifications {
            type_id: "invalid_machine".to_string(),
            reason: "Power requirements invalid".to_string(),
        };
        assert_eq!(format!("{}", error3), "Invalid specifications for machine type 'invalid_machine': Power requirements invalid");
    }

    #[test]
    fn test_registry_creation() {
        let registry = MachineRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_register_machine_type_success() {
        let mut registry = MachineRegistry::new();
        
        let specs = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel".to_string(),
            "Generates power from flowing water".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let result = registry.register_machine_type(specs.clone());
        assert!(result.success);
        assert_eq!(result.value, Some(MachineTypeId::new("water_wheel".to_string())));
        assert_eq!(result.message, "Successfully registered machine type 'water_wheel'");
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
        
        // Verify we can retrieve it
        let retrieval = registry.get_machine_type(&MachineTypeId::new("water_wheel".to_string()));
        assert!(retrieval.success);
        assert_eq!(retrieval.value, Some(specs));
    }

    #[test]
    fn test_register_machine_type_duplicate() {
        let mut registry = MachineRegistry::new();
        
        let specs1 = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel".to_string(),
            "Generates power from flowing water".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let specs2 = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel Clone".to_string(), // Different name
            "Another water wheel".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        // Register first specs
        let result1 = registry.register_machine_type(specs1);
        assert!(result1.success);
        
        // Try to register duplicate - should fail
        let result2 = registry.register_machine_type(specs2);
        assert!(!result2.success);
        assert_eq!(result2.message, "Machine type 'water_wheel' is already registered");
        assert_eq!(registry.len(), 1); // Still only one registered
    }

    #[test]
    fn test_get_machine_type_success() {
        let mut registry = MachineRegistry::new();
        
        let specs = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel".to_string(),
            "Generates power from flowing water".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        // Register the specs
        let _ = registry.register_machine_type(specs.clone());
        
        // Retrieve the specs
        let result = registry.get_machine_type(&MachineTypeId::new("water_wheel".to_string()));
        assert!(result.success);
        assert_eq!(result.value, Some(specs));
        assert_eq!(result.message, "Successfully retrieved machine type 'water_wheel'");
    }

    #[test]
    fn test_get_machine_type_not_found() {
        let registry = MachineRegistry::new();
        
        let result = registry.get_machine_type(&MachineTypeId::new("nonexistent".to_string()));
        assert!(!result.success);
        assert_eq!(result.message, "Machine type 'nonexistent' not found in registry");
    }

    #[test]
    fn test_remove_machine_type_success() {
        let mut registry = MachineRegistry::new();
        
        let specs = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel".to_string(),
            "Generates power from flowing water".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        // Register the specs
        let _ = registry.register_machine_type(specs);
        
        // Remove the specs
        let result = registry.remove_machine_type(&MachineTypeId::new("water_wheel".to_string()));
        assert!(result.success);
        assert_eq!(result.value, Some(MachineTypeId::new("water_wheel".to_string())));
        assert_eq!(result.message, "Successfully removed machine type 'water_wheel'");
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
        
        // Verify it's no longer found
        let retrieval = registry.get_machine_type(&MachineTypeId::new("water_wheel".to_string()));
        assert!(!retrieval.success);
    }

    #[test]
    fn test_remove_machine_type_not_found() {
        let mut registry = MachineRegistry::new();
        
        let result = registry.remove_machine_type(&MachineTypeId::new("nonexistent".to_string()));
        assert!(!result.success);
        assert_eq!(result.message, "Cannot remove machine type 'nonexistent': not found in registry");
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_contains_machine_type() {
        let mut registry = MachineRegistry::new();
        
        let specs = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel".to_string(),
            "Generates power from flowing water".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        // Initially should not contain the type
        assert!(!registry.contains_machine_type(&MachineTypeId::new("water_wheel".to_string())));
        
        // After registration should contain it
        let _ = registry.register_machine_type(specs);
        assert!(registry.contains_machine_type(&MachineTypeId::new("water_wheel".to_string())));
        
        // After removal should not contain it
        let _ = registry.remove_machine_type(&MachineTypeId::new("water_wheel".to_string()));
        assert!(!registry.contains_machine_type(&MachineTypeId::new("water_wheel".to_string())));
    }

    #[test]
    fn test_machine_type_ids_iterator() {
        let mut registry = MachineRegistry::new();
        
        let specs1 = MachineSpecs::new(
            MachineTypeId::new("water_wheel".to_string()),
            "Water Wheel".to_string(),
            "Generates power from flowing water".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let specs2 = MachineSpecs::new(
            MachineTypeId::new("furnace".to_string()),
            "Furnace".to_string(),
            "Produces heat for smelting".to_string(),
            Watts::new(2000),
            Watts::new(500),
            6,
            true,
        ).unwrap();
        
        // Register two specs
        let _ = registry.register_machine_type(specs1);
        let _ = registry.register_machine_type(specs2);
        
        // Check that we can iterate over the IDs
        let ids: Vec<_> = registry.machine_type_ids().collect();
        assert_eq!(ids.len(), 2);
        let id_strings: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        assert!(id_strings.contains(&"water_wheel".to_string()));
        assert!(id_strings.contains(&"furnace".to_string()));
    }

    #[test]
    fn test_validate_specs_success() {
        let specs = MachineSpecs::new(
            MachineTypeId::new("valid_machine".to_string()),
            "Valid Machine".to_string(),
            "A valid machine specification".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let result = MachineRegistry::validate_specs(&specs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_specs_empty_type_id() {
        let specs = MachineSpecs::new(
            MachineTypeId::new("".to_string()),
            "Valid Machine".to_string(),
            "A valid machine specification".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let result = MachineRegistry::validate_specs(&specs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Machine type ID cannot be empty");
    }

    #[test]
    fn test_validate_specs_empty_name() {
        let specs = MachineSpecs::new(
            MachineTypeId::new("valid_machine".to_string()),
            "".to_string(),
            "A valid machine specification".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let result = MachineRegistry::validate_specs(&specs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Machine name cannot be empty");
    }

    #[test]
    fn test_validate_specs_empty_description() {
        let specs = MachineSpecs::new(
            MachineTypeId::new("valid_machine".to_string()),
            "Valid Machine".to_string(),
            "".to_string(),
            Watts::new(1000),
            Watts::new(200),
            4,
            true,
        ).unwrap();
        
        let result = MachineRegistry::validate_specs(&specs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Machine description cannot be empty");
    }

    #[test]
    fn test_validate_specs_invalid_power_requirements() {
        let specs = MachineSpecs::new(
            MachineTypeId::new("valid_machine".to_string()),
            "Valid Machine".to_string(),
            "A valid machine specification".to_string(),
            Watts::new(200), // Less than min
            Watts::new(1000),
            4,
            true,
        ).unwrap();
        
        let result = MachineRegistry::validate_specs(&specs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Minimum power required cannot exceed maximum power consumption");
    }

    #[test]
    fn test_validate_specs_zero_inventory_slots() {
        let specs = MachineSpecs::new(
            MachineTypeId::new("valid_machine".to_string()),
            "Valid Machine".to_string(),
            "A valid machine specification".to_string(),
            Watts::new(1000),
            Watts::new(200),
            0, // Zero inventory slots
            true,
        ).unwrap();
        
        let result = MachineRegistry::validate_specs(&specs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Maximum inventory slots must be greater than zero");
    }

    #[test]
    fn test_registry_default() {
        let registry = MachineRegistry::default();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}