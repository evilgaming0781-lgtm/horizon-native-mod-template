//! Belt-specific types.
//!
//! This module defines types used specifically for belt transmission components.

use std::fmt;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};
use crate::mechanical::transmission::types::TransmissionId;

/// Unique identifier for a belt component.
///
/// Belt IDs are used to track specific belt components within the mechanical system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BeltId(pub u64);

impl fmt::Display for BeltId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Belt{}", self.0)
    }
}

impl Default for BeltId {
    fn default() -> Self {
        BeltId(0)
    }
}

impl BeltId {
    /// Creates a new belt ID with the specified value.
    pub const fn new(id: u64) -> Self {
        BeltId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// The category or type of a belt component.
///
/// Belt categories help identify what kind of belt functionality a component provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeltCategory {
    /// A standard open belt transmission.
    OpenBelt,
    
    /// A crossed belt transmission.
    CrossedBelt,
    
    /// A timing belt for precise synchronization.
    TimingBelt,
    
    /// A flat belt for power transmission.
    FlatBelt,
}

impl fmt::Display for BeltCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeltCategory::OpenBelt => write!(f, "OpenBelt"),
            BeltCategory::CrossedBelt => write!(f, "CrossedBelt"),
            BeltCategory::TimingBelt => write!(f, "TimingBelt"),
            BeltCategory::FlatBelt => write!(f, "FlatBelt"),
        }
    }
}

/// Belt configuration settings.
///
/// This structure holds the configuration parameters that affect how 
/// belt components operate and what limits they enforce.
#[derive(Debug, Clone)]
pub struct BeltConfiguration {
    /// The effective radius of the pulley in meters.
    pub radius: f64,
    
    /// Maximum rotational speed in RPM that this belt can handle.
    pub max_rpm: f64,
    
    /// Maximum torque in newton-metres that this belt can transmit.
    pub max_torque: f64,
    
    /// Maximum power in watts that this belt can handle.
    pub max_power: f64,
    
    /// Belt efficiency as a fraction (0.0 to 1.0).
    pub efficiency: f64,
    
    /// Indicates if the belt is currently enabled and operational.
    pub enabled: bool,
    
    /// The type of belt arrangement (open or crossed).
    pub category: BeltCategory,
}

impl BeltConfiguration {
    /// Creates a new belt configuration with default values.
    ///
    /// # Arguments
    /// * `radius` - Effective radius of the pulley in meters
    /// * `max_rpm` - Maximum rotational speed in RPM
    /// * `max_torque` - Maximum torque in newton-metres  
    /// * `max_power` - Maximum power in watts
    /// * `efficiency` - Belt efficiency as a fraction (0.0 to 1.0)
    /// * `category` - The type of belt arrangement
    pub fn new(
        radius: f64,
        max_rpm: f64,
        max_torque: f64,
        max_power: f64,
        efficiency: f64,
        category: BeltCategory,
    ) -> Self {
        BeltConfiguration {
            radius,
            max_rpm,
            max_torque,
            max_power,
            efficiency,
            enabled: true,
            category,
        }
    }
    
    /// Creates a new belt configuration with default values.
    ///
    /// # Arguments
    /// * `radius` - Effective radius of the pulley in meters
    /// * `max_rpm` - Maximum rotational speed in RPM
    /// * `max_torque` - Maximum torque in newton-metres  
    /// * `max_power` - Maximum power in watts
    /// * `efficiency` - Belt efficiency as a fraction (0.0 to 1.0)
    /// * `enabled` - Whether the belt is enabled by default
    /// * `category` - The type of belt arrangement
    pub fn with_enabled(
        radius: f64,
        max_rpm: f64,
        max_torque: f64,
        max_power: f64,
        efficiency: f64,
        enabled: bool,
        category: BeltCategory,
    ) -> Self {
        BeltConfiguration {
            radius,
            max_rpm,
            max_torque,
            max_power,
            efficiency,
            enabled,
            category,
        }
    }
    
    /// Validates that the configuration values are valid.
    ///
    /// # Returns
    /// * `Ok(())` if all configuration values are valid
    /// * `Err(BeltError)` if any configuration value is invalid
    pub fn validate(&self) -> Result<(), BeltError> {
        // Validate radius (should be positive)
        if self.radius <= 0.0 {
            return Err(BeltError::InvalidRadius);
        }
        
        // Validate maximum RPM
        if self.max_rpm < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate maximum torque
        if self.max_torque < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate maximum power
        if self.max_power < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate efficiency (should be between 0 and 1)
        if self.efficiency < 0.0 || self.efficiency > 1.0 {
            return Err(BeltError::InvalidEfficiency);
        }
        
        Ok(())
    }
    
    /// Gets the radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    /// Gets the maximum RPM.
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
    
    /// Gets the efficiency.
    pub fn efficiency(&self) -> f64 {
        self.efficiency
    }
    
    /// Checks if the belt is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Gets the belt category.
    pub fn category(&self) -> BeltCategory {
        self.category
    }
}

impl fmt::Display for BeltConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BeltConfiguration(radius: {:.2}m, rpm: {}, torque: {}, power: {}, efficiency: {})", 
            self.radius, self.max_rpm, self.max_torque, self.max_power, self.efficiency)
    }
}