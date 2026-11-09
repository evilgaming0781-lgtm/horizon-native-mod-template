//! Gear-specific types.
//!
//! This module defines types used specifically for gear transmission components.

use std::fmt;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};
use crate::mechanical::transmission::types::TransmissionId;

/// Unique identifier for a gear component.
///
/// Gear IDs are used to track specific gear components within the mechanical system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GearId(pub u64);

impl fmt::Display for GearId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gear{}", self.0)
    }
}

impl Default for GearId {
    fn default() -> Self {
        GearId(0)
    }
}

impl GearId {
    /// Creates a new gear ID with the specified value.
    pub const fn new(id: u64) -> Self {
        GearId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// The category or type of a gear component.
///
/// Gear categories help identify what kind of gear functionality a component provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GearCategory {
    /// A standard spur gear with straight teeth.
    Spur,
    
    /// A helical gear with angled teeth.
    Helical,
    
    /// A bevel gear used for intersecting shafts.
    Bevel,
    
    /// A worm gear for large speed reductions.
    Worm,
    
    /// A planetary gear system.
    Planetary,
}

impl fmt::Display for GearCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearCategory::Spur => write!(f, "Spur"),
            GearCategory::Helical => write!(f, "Helical"),
            GearCategory::Bevel => write!(f, "Bevel"),
            GearCategory::Worm => write!(f, "Worm"),
            GearCategory::Planetary => write!(f, "Planetary"),
        }
    }
}

/// Gear configuration settings.
///
/// This structure holds the configuration parameters that affect how 
/// gear components operate and what limits they enforce.
#[derive(Debug, Clone)]
pub struct GearConfiguration {
    /// Number of teeth on the gear.
    pub tooth_count: u32,
    
    /// Maximum rotational speed in RPM that this gear can handle.
    pub max_rpm: f64,
    
    /// Maximum torque in newton-metres that this gear can transmit.
    pub max_torque: f64,
    
    /// Maximum power in watts that this gear can handle.
    pub max_power: f64,
    
    /// Gear efficiency as a fraction (0.0 to 1.0).
    pub efficiency: f64,
    
    /// Indicates if the gear is currently enabled and operational.
    pub enabled: bool,
}

impl GearConfiguration {
    /// Creates a new gear configuration with default values.
    ///
    /// # Arguments
    /// * `tooth_count` - Number of teeth on the gear
    /// * `max_rpm` - Maximum rotational speed in RPM
    /// * `max_torque` - Maximum torque in newton-metres  
    /// * `max_power` - Maximum power in watts
    /// * `efficiency` - Gear efficiency as a fraction (0.0 to 1.0)
    pub fn new(
        tooth_count: u32,
        max_rpm: f64,
        max_torque: f64,
        max_power: f64,
        efficiency: f64,
    ) -> Self {
        GearConfiguration {
            tooth_count,
            max_rpm,
            max_torque,
            max_power,
            efficiency,
            enabled: true,
        }
    }
    
    /// Creates a new gear configuration with default values.
    ///
    /// # Arguments
    /// * `tooth_count` - Number of teeth on the gear
    /// * `max_rpm` - Maximum rotational speed in RPM
    /// * `max_torque` - Maximum torque in newton-metres  
    /// * `max_power` - Maximum power in watts
    /// * `efficiency` - Gear efficiency as a fraction (0.0 to 1.0)
    /// * `enabled` - Whether the gear is enabled by default
    pub fn with_enabled(
        tooth_count: u32,
        max_rpm: f64,
        max_torque: f64,
        max_power: f64,
        efficiency: f64,
        enabled: bool,
    ) -> Self {
        GearConfiguration {
            tooth_count,
            max_rpm,
            max_torque,
            max_power,
            efficiency,
            enabled,
        }
    }
    
    /// Validates that the configuration values are valid.
    ///
    /// # Returns
    /// * `Ok(())` if all configuration values are valid
    /// * `Err(GearError)` if any configuration value is invalid
    pub fn validate(&self) -> Result<(), GearError> {
        // Validate tooth count (should be positive)
        if self.tooth_count == 0 {
            return Err(GearError::InvalidToothCount);
        }
        
        // Validate maximum RPM
        if self.max_rpm < 0.0 {
            return Err(GearError::InvalidConfiguration);
        }
        
        // Validate maximum torque
        if self.max_torque < 0.0 {
            return Err(GearError::InvalidConfiguration);
        }
        
        // Validate maximum power
        if self.max_power < 0.0 {
            return Err(GearError::InvalidConfiguration);
        }
        
        // Validate efficiency (should be between 0 and 1)
        if self.efficiency < 0.0 || self.efficiency > 1.0 {
            return Err(GearError::InvalidEfficiency);
        }
        
        Ok(())
    }
    
    /// Gets the tooth count.
    pub fn tooth_count(&self) -> u32 {
        self.tooth_count
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
    
    /// Checks if the gear is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl fmt::Display for GearConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GearConfiguration(tooth_count: {}, rpm: {}, torque: {}, power: {}, efficiency: {})", 
            self.tooth_count, self.max_rpm, self.max_torque, self.max_power, self.efficiency)
    }
}