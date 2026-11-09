//! Transmission configuration management.
//!
//! This module provides configurable parameters for transmission components.

use std::fmt;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};
use crate::mechanical::transmission::errors::{TransmissionError, TransmissionResult};

/// Configuration settings for transmission components.
///
/// This structure holds the configuration parameters that affect how 
/// transmission components operate and what limits they enforce.
#[derive(Debug, Clone)]
pub struct TransmissionConfiguration {
    /// Maximum rotational speed in RPM that this component can handle.
    pub max_rpm: f64,
    
    /// Maximum torque in newton-metres that this component can transmit.
    pub max_torque: f64,
    
    /// Maximum power in watts that this component can handle.
    pub max_power: f64,
    
    /// Indicates if the transmission is currently enabled and operational.
    pub enabled: bool,
}

impl TransmissionConfiguration {
    /// Creates a new transmission configuration with default values.
    ///
    /// # Arguments
    /// * `max_rpm` - Maximum rotational speed in RPM
    /// * `max_torque` - Maximum torque in newton-metres  
    /// * `max_power` - Maximum power in watts
    pub fn new(max_rpm: f64, max_torque: f64, max_power: f64) -> Self {
        TransmissionConfiguration {
            max_rpm,
            max_torque,
            max_power,
            enabled: true,
        }
    }
    
    /// Creates a new transmission configuration with default values.
    ///
    /// # Arguments
    /// * `max_rpm` - Maximum rotational speed in RPM
    /// * `max_torque` - Maximum torque in newton-metres  
    /// * `max_power` - Maximum power in watts
    /// * `enabled` - Whether the component is enabled by default
    pub fn with_enabled(max_rpm: f64, max_torque: f64, max_power: f64, enabled: bool) -> Self {
        TransmissionConfiguration {
            max_rpm,
            max_torque,
            max_power,
            enabled,
        }
    }
    
    /// Validates that the configuration values are valid.
    ///
    /// # Returns
    /// * `Ok(())` if all configuration values are valid
    /// * `Err(TransmissionError)` if any configuration value is invalid
    pub fn validate(&self) -> TransmissionResult<()> {
        // Validate maximum RPM
        if self.max_rpm < 0.0 {
            return Err(TransmissionError::InvalidConfiguration);
        }
        
        // Validate maximum torque
        if self.max_torque < 0.0 {
            return Err(TransmissionError::InvalidConfiguration);
        }
        
        // Validate maximum power
        if self.max_power < 0.0 {
            return Err(TransmissionError::InvalidConfiguration);
        }
        
        Ok(())
    }
    
    /// Gets the maximum rotational speed.
    pub fn max_rpm(&self) -> f64 {
        self.max_rpm
    }
    
    /// Gets the maximum torque.
    pub fn max_torque(&self) -> f64 {
        self.max_torque
    }
    
    /// Gets the maximum power.
    pub fn max_power(&self) -> f64 {
        self.max_power
    }
    
    /// Checks if the transmission is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl fmt::Display for TransmissionConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TransmissionConfiguration(rpm: {}, torque: {}, power: {})", 
            self.max_rpm, self.max_torque, self.max_power)
    }
}