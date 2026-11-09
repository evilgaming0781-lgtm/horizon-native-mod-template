//! Belt transmission ratio calculations.
//!
//! This module provides the calculation and representation of belt transmission ratios.

use std::fmt;
use crate::mechanical::transmission::belt::types::BeltCategory;

/// Represents a belt transmission ratio.
///
/// The ratio is calculated as i = r_out / r_in where:
/// - r_in is the input pulley radius
/// - r_out is the output pulley radius
/// - i is the transmission ratio
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BeltRatio {
    /// Input pulley radius.
    pub input_radius: f64,
    
    /// Output pulley radius.
    pub output_radius: f64,
    
    /// The calculated ratio (r_out / r_in).
    pub ratio: f64,
    
    /// The belt arrangement type.
    pub category: BeltCategory,
}

impl BeltRatio {
    /// Creates a new belt ratio from input and output radii.
    ///
    /// # Arguments
    /// * `input_radius` - Radius of the input pulley in meters
    /// * `output_radius` - Radius of the output pulley in meters
    /// * `category` - The type of belt arrangement
    ///
    /// # Returns
    /// A new BeltRatio instance
    pub fn new(input_radius: f64, output_radius: f64, category: BeltCategory) -> Self {
        let ratio = if input_radius > 0.0 {
            output_radius / input_radius
        } else {
            0.0
        };
        
        BeltRatio {
            input_radius,
            output_radius,
            ratio,
            category,
        }
    }
    
    /// Gets the ratio value (r_out / r_in).
    pub fn get(&self) -> f64 {
        self.ratio
    }
    
    /// Checks if this is a speed reduction (output speed less than input speed).
    ///
    /// # Returns
    /// * `true` if the output speed is less than the input speed (ratio < 1.0)
    /// * `false` otherwise
    pub fn is_speed_reduction(&self) -> bool {
        self.ratio < 1.0
    }
    
    /// Checks if this is a speed increase (output speed greater than input speed).
    ///
    /// # Returns
    /// * `true` if the output speed is greater than the input speed (ratio > 1.0)
    /// * `false` otherwise
    pub fn is_speed_increase(&self) -> bool {
        self.ratio > 1.0
    }
    
    /// Checks if this is a neutral ratio (output speed equals input speed).
    ///
    /// # Returns
    /// * `true` if the output speed equals the input speed (ratio = 1.0)
    /// * `false` otherwise
    pub fn is_neutral(&self) -> bool {
        self.ratio == 1.0
    }
    
    /// Gets the direction of rotation change.
    ///
    /// # Returns
    /// * `true` if rotation direction changes (crossed belt)
    /// * `false` if rotation direction is preserved (open belt)
    pub fn rotation_direction_changes(&self) -> bool {
        match self.category {
            BeltCategory::OpenBelt => false,
            BeltCategory::CrossedBelt => true,
            _ => false, // Default to no change for other types
        }
    }
}

impl fmt::Display for BeltRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BeltRatio(input_r: {:.2}m, output_r: {:.2}m, ratio: {:.2})", 
            self.input_radius, self.output_radius, self.ratio)
    }
}