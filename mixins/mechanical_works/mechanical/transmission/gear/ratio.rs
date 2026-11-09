//! Gear ratio calculations.
//!
//! This module implements gear ratio calculations and related operations.

use std::fmt;
use crate::mechanical::transmission::gear::{GearId, Gear};
use crate::mechanical::core::units::{RotationalSpeed, AngularVelocity};
use crate::mechanical::transmission::gear::errors::GearError;

/// A gear ratio that represents the relationship between two gears.
///
/// This structure encapsulates the mathematical relationship between input and output
/// gears in a gear pair, including the ratio and direction information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GearRatio {
    /// The input gear tooth count.
    pub input_teeth: u32,
    
    /// The output gear tooth count.  
    pub output_teeth: u32,
    
    /// The gear ratio (output/input).
    pub ratio: f64,
    
    /// Indicates if the rotation direction is reversed (true for external meshing).
    pub reversed_direction: bool,
}

impl GearRatio {
    /// Creates a new gear ratio from input and output tooth counts.
    ///
    /// # Arguments
    /// * `input_teeth` - Tooth count of the input gear
    /// * `output_teeth` - Tooth count of the output gear
    ///
    /// # Returns
    /// * `Ok(GearRatio)` if creation succeeded
    /// * `Err(GearError)` if tooth counts are invalid
    pub fn new(input_teeth: u32, output_teeth: u32) -> Result<Self, GearError> {
        // Validate tooth counts (should be positive)
        if input_teeth == 0 || output_teeth == 0 {
            return Err(GearError::InvalidToothCount);
        }
        
        // Calculate the ratio
        let ratio = output_teeth as f64 / input_teeth as f64;
        
        // For external meshing gears, the direction is reversed (when output > input, speed reduction)
        let reversed_direction = output_teeth > input_teeth;
        
        Ok(GearRatio {
            input_teeth,
            output_teeth,
            ratio,
            reversed_direction,
        })
    }
    
    /// Gets the input tooth count.
    pub fn input_teeth(&self) -> u32 {
        self.input_teeth
    }
    
    /// Gets the output tooth count.
    pub fn output_teeth(&self) -> u32 {
        self.output_teeth
    }
    
    /// Gets the gear ratio (output/input).
    pub fn ratio(&self) -> f64 {
        self.ratio
    }
    
    /// Checks if rotation direction is reversed.
    pub fn is_reversed(&self) -> bool {
        self.reversed_direction
    }
    
    /// Calculates the output RPM based on input RPM and gear ratio.
    ///
    /// # Arguments
    /// * `input_rpm` - Input rotational speed in RPM
    ///
    /// # Returns
    /// Output rotational speed in RPM
    pub fn calculate_output_rpm(&self, input_rpm: RotationalSpeed) -> RotationalSpeed {
        let output_rpm = input_rpm.get() / self.ratio;
        RotationalSpeed::new(output_rpm)
    }
    
    /// Calculates the output angular velocity based on input angular velocity and gear ratio.
    ///
    /// # Arguments
    /// * `input_angular_velocity` - Input angular velocity in rad/s
    ///
    /// # Returns
    /// Output angular velocity in rad/s
    pub fn calculate_output_angular_velocity(&self, input_angular_velocity: AngularVelocity) -> AngularVelocity {
        let output_angular_velocity = input_angular_velocity.get() / self.ratio;
        AngularVelocity::new(output_angular_velocity)
    }
    
    /// Checks if this gear ratio represents a speed reduction.
    ///
    /// # Returns
    /// True if the output speed is less than input speed, false otherwise
    pub fn is_speed_reduction(&self) -> bool {
        self.ratio < 1.0
    }
    
    /// Checks if this gear ratio represents a speed increase.
    ///
    /// # Returns
    /// True if the output speed is greater than input speed, false otherwise
    pub fn is_speed_increase(&self) -> bool {
        self.ratio > 1.0
    }
    
    /// Checks if this gear ratio is neutral (1:1).
    ///
    /// # Returns
    /// True if input and output speeds are equal, false otherwise
    pub fn is_neutral(&self) -> bool {
        self.ratio == 1.0
    }
}

impl fmt::Display for GearRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GearRatio({}:{}, ratio: {:.2}, reversed: {})", 
            self.input_teeth, self.output_teeth, self.ratio, self.reversed_direction)
    }
}