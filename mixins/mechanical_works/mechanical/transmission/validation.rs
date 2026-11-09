//! Transmission validation utilities.
//!
//! This module provides validation logic for transmission components and connections.

use crate::mechanical::transmission::types::{TransmissionId, TransmissionCategory};
use crate::mechanical::transmission::component::TransmissionComponent;
use crate::mechanical::transmission::shaft::Shaft;
use crate::mechanical::transmission::coupling::Coupling;
use crate::mechanical::transmission::interfaces::RotationalTransmission;
use crate::mechanical::transmission::errors::{TransmissionError, TransmissionResult};

/// Validation results for transmission components.
#[derive(Debug, Clone, PartialEq)]
pub struct TransmissionValidationResult {
    /// Indicates if the validation passed.
    pub is_valid: bool,
    
    /// List of validation errors found.
    pub errors: Vec<TransmissionError>,
    
    /// List of validation warnings or observations.
    pub warnings: Vec<String>,
}

impl TransmissionValidationResult {
    /// Creates a new empty validation result.
    pub fn new() -> Self {
        TransmissionValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    /// Adds an error to the validation result.
    pub fn add_error(&mut self, error: TransmissionError) {
        self.is_valid = false;
        self.errors.push(error);
    }
    
    /// Adds a warning to the validation result.
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// Validation utilities for transmission components.
///
/// This structure provides methods for validating transmission state and detecting
/// structural issues that may affect transmission operation.
#[derive(Debug, Clone)]
pub struct TransmissionValidation;

impl TransmissionValidation {
    /// Validates a transmission component.
    ///
    /// # Arguments
    /// * `component` - The component to validate
    ///
    /// # Returns
    /// A `TransmissionValidationResult` indicating the validity of the component and any issues found
    pub fn validate_component(component: &TransmissionComponent) -> TransmissionValidationResult {
        let mut result = TransmissionValidationResult::new();
        
        // Check if the component is enabled (disabled components should not be used)
        if !component.is_enabled() {
            result.add_warning("Transmission component is disabled".to_string());
        }
        
        result
    }
    
    /// Validates a shaft.
    ///
    /// # Arguments
    /// * `shaft` - The shaft to validate
    ///
    /// # Returns
    /// A `TransmissionValidationResult` indicating the validity of the shaft and any issues found
    pub fn validate_shaft(shaft: &Shaft) -> TransmissionValidationResult {
        let mut result = TransmissionValidationResult::new();
        
        // Check if the shaft is enabled
        if !shaft.is_enabled() {
            result.add_warning("Shaft is disabled".to_string());
        }
        
        // Check for valid configuration (should be done in the component creation)
        // but we can add additional checks here if needed
        
        result
    }
    
    /// Validates a coupling.
    ///
    /// # Arguments
    /// * `coupling` - The coupling to validate
    ///
    /// # Returns
    /// A `TransmissionValidationResult` indicating the validity of the coupling and any issues found
    pub fn validate_coupling(coupling: &Coupling) -> TransmissionValidationResult {
        let mut result = TransmissionValidationResult::new();
        
        // Check if the coupling is enabled
        if !coupling.is_enabled() {
            result.add_warning("Coupling is disabled".to_string());
        }
        
        // Validate that the coupling has valid attachments
        if let Err(error) = coupling.validate() {
            result.add_error(error);
        }
        
        result
    }
    
    /// Validates that two transmission components are compatible.
    ///
    /// # Arguments
    /// * `component_a` - First component to check
    /// * `component_b` - Second component to check
    ///
    /// # Returns
    /// A `TransmissionValidationResult` indicating compatibility and any issues found
    pub fn validate_compatibility(
        component_a: &dyn RotationalTransmission,
        component_b: &dyn RotationalTransmission,
    ) -> TransmissionValidationResult {
        let mut result = TransmissionValidationResult::new();
        
        // Check if both components are enabled
        if !component_a.is_enabled() {
            result.add_error(TransmissionError::DisabledComponent);
        }
        
        if !component_b.is_enabled() {
            result.add_error(TransmissionError::DisabledComponent);
        }
        
        // Additional compatibility checks would go here
        // For now, we assume all components are compatible
        
        result
    }
    
    /// Validates that a transmission configuration is valid.
    ///
    /// # Arguments
    /// * `config` - The configuration to validate
    ///
    /// # Returns
    /// A `TransmissionValidationResult` indicating the validity of the configuration and any issues found
    pub fn validate_configuration(config: &crate::mechanical::transmission::configuration::TransmissionConfiguration) -> TransmissionValidationResult {
        let mut result = TransmissionValidationResult::new();
        
        // Validate configuration values
        if let Err(error) = config.validate() {
            result.add_error(error);
        }
        
        result
    }
}