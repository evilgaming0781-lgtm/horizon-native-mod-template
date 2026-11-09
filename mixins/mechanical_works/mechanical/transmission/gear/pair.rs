//! Gear pair representation.
//!
//! This module implements a gear-pair component for connecting two gears.

use std::fmt;
use crate::mechanical::transmission::gear::{Gear, GearId};
use crate::mechanical::transmission::gear::errors::GearError;

/// A gear pair that connects two compatible gears.
///
/// This structure represents a connection between two gears where one serves as
/// the input gear and the other as the output gear in a transmission system.
#[derive(Debug, Clone)]
pub struct GearPair {
    /// Unique identifier for this gear pair.
    id: u64,
    
    /// The input gear in this pair.
    input_gear: GearId,
    
    /// The output gear in this pair.
    output_gear: GearId,
    
    /// Indicates if the gear pair is currently enabled and operational.
    enabled: bool,
}

impl GearPair {
    /// Creates a new gear pair with the specified gears.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this gear pair
    /// * `input_gear_id` - The ID of the input gear
    /// * `output_gear_id` - The ID of the output gear
    ///
    /// # Returns
    /// * `Ok(GearPair)` if creation succeeded
    /// * `Err(GearError)` if configuration is invalid
    pub fn new(
        id: u64,
        input_gear_id: GearId,
        output_gear_id: GearId,
    ) -> Result<Self, GearError> {
        // Check for self-pairing (a gear cannot be paired with itself)
        if input_gear_id == output_gear_id {
            return Err(GearError::SelfPairing);
        }
        
        Ok(GearPair {
            id,
            input_gear: input_gear_id,
            output_gear: output_gear_id,
            enabled: true,
        })
    }
    
    /// Gets the unique identifier for this gear pair.
    pub fn id(&self) -> u64 {
        self.id
    }
    
    /// Gets the input gear ID.
    pub fn input_gear(&self) -> GearId {
        self.input_gear
    }
    
    /// Gets the output gear ID.
    pub fn output_gear(&self) -> GearId {
        self.output_gear
    }
    
    /// Checks if this gear pair is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Sets the enabled state of this gear pair.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Validates that this gear pair has valid configuration.
    ///
    /// # Returns
    /// * `Ok(())` if the gear pair is valid
    /// * `Err(GearError)` if the gear pair has invalid configuration
    pub fn validate(&self) -> Result<(), GearError> {
        // A valid gear pair must have two different gears
        if self.input_gear == self.output_gear {
            return Err(GearError::SelfPairing);
        }
        
        Ok(())
    }
    
    /// Checks if the specified gear is part of this gear pair.
    ///
    /// # Arguments
    /// * `gear_id` - The ID of the gear to check
    ///
    /// # Returns
    /// True if the gear is part of this pair, false otherwise
    pub fn contains_gear(&self, gear_id: GearId) -> bool {
        self.input_gear == gear_id || self.output_gear == gear_id
    }
}

impl fmt::Display for GearPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GearPair(id: {}, input: {}, output: {})", 
            self.id, self.input_gear, self.output_gear)
    }
}