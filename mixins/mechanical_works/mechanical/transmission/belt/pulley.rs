//! Pulley representation for belt transmission systems.
//!
//! This module provides the structure and behavior for individual pulleys in a belt system.

use std::fmt;
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power, AngularVelocity};
use crate::mechanical::transmission::belt::types::{BeltId, BeltConfiguration, BeltCategory};

/// A pulley component for belt transmission.
///
/// Pulleys are the rotating components that engage with a belt to transfer mechanical power.
#[derive(Debug, Clone)]
pub struct Pulley {
    /// Unique identifier for this pulley.
    pub id: BeltId,
    
    /// The configuration of this pulley.
    pub configuration: BeltConfiguration,
    
    /// Current rotational speed in RPM.
    pub current_rpm: RotationalSpeed,
    
    /// Current torque being transmitted.
    pub current_torque: Torque,
    
    /// Current power being transmitted.
    pub current_power: Power,
    
    /// Whether this pulley is enabled and operational.
    pub enabled: bool,
}

impl Pulley {
    /// Creates a new pulley with the specified parameters.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this pulley
    /// * `configuration` - Configuration settings for this pulley
    pub fn new(id: BeltId, configuration: BeltConfiguration) -> Self {
        Pulley {
            id,
            configuration,
            current_rpm: RotationalSpeed::new(0.0),
            current_torque: Torque::new(0.0),
            current_power: Power::new(0.0),
            enabled: configuration.enabled,
        }
    }
    
    /// Sets the current rotational speed of this pulley.
    ///
    /// # Arguments
    /// * `rpm` - Rotational speed in RPM
    pub fn set_rpm(&mut self, rpm: RotationalSpeed) {
        // Validate that RPM is within limits
        if rpm.get() > self.configuration.max_rpm {
            panic!("Rotational speed {} exceeds maximum of {}", rpm.get(), self.configuration.max_rpm);
        }
        self.current_rpm = rpm;
    }
    
    /// Sets the current torque on this pulley.
    ///
    /// # Arguments
    /// * `torque` - Torque in newton-metres
    pub fn set_torque(&mut self, torque: Torque) {
        // Validate that torque is within limits
        if torque.get() > self.configuration.max_torque {
            panic!("Torque {} exceeds maximum of {}", torque.get(), self.configuration.max_torque);
        }
        self.current_torque = torque;
    }
    
    /// Sets the current power on this pulley.
    ///
    /// # Arguments
    /// * `power` - Power in watts
    pub fn set_power(&mut self, power: Power) {
        // Validate that power is within limits
        if power.get() > self.configuration.max_power {
            panic!("Power {} exceeds maximum of {}", power.get(), self.configuration.max_power);
        }
        self.current_power = power;
    }
    
    /// Gets the effective radius of this pulley.
    pub fn radius(&self) -> f64 {
        self.configuration.radius
    }
    
    /// Gets the category of this pulley's belt.
    pub fn category(&self) -> BeltCategory {
        self.configuration.category
    }
    
    /// Checks if this pulley is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled && self.configuration.enabled
    }
    
    /// Enables or disables this pulley.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Calculates the linear velocity of the belt passing over this pulley.
    ///
    /// Linear velocity = ω × r
    ///
    /// # Returns
    /// The linear velocity in meters per second
    pub fn calculate_belt_linear_velocity(&self) -> f64 {
        let angular_velocity = self.current_rpm.to_rad_per_sec();
        angular_velocity.get() * self.radius()
    }
}

impl fmt::Display for Pulley {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pulley(id: {}, radius: {:.2}m, rpm: {}, torque: {}, power: {})", 
            self.id, self.radius(), self.current_rpm.get(), self.current_torque.get(), self.current_power.get())
    }
}