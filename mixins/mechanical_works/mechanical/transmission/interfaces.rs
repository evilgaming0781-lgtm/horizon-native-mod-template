//! Transmission input/output interfaces.
//!
//! This module defines the contracts for rotational input and output interfaces.

use crate::mechanical::core::units::{Torque, AngularVelocity, RotationalSpeed, Power};
use crate::mechanical::transmission::types::TransmissionId;

/// A rotational input interface.
///
/// This trait represents a component that can receive rotational motion.
pub trait RotationalInput {
    /// Gets the maximum torque this input can handle.
    fn max_input_torque(&self) -> Torque;
    
    /// Gets the maximum rotational speed this input can handle.
    fn max_input_rpm(&self) -> RotationalSpeed;
    
    /// Checks if this input is compatible with the given output.
    ///
    /// # Arguments
    /// * `output` - The rotational output to check compatibility with
    fn is_compatible_with_output(&self, output: &dyn RotationalOutput) -> bool;
}

/// A rotational output interface.
///
/// This trait represents a component that can deliver rotational motion.
pub trait RotationalOutput {
    /// Gets the current torque being delivered.
    fn current_torque(&self) -> Torque;
    
    /// Gets the current rotational speed being delivered.
    fn current_rpm(&self) -> RotationalSpeed;
    
    /// Gets the current power being delivered.
    fn current_power(&self) -> Power;
    
    /// Checks if this output is compatible with the given input.
    ///
    /// # Arguments
    /// * `input` - The rotational input to check compatibility with
    fn is_compatible_with_input(&self, input: &dyn RotationalInput) -> bool;
}

/// A rotational transmission interface.
///
/// This trait combines both input and output capabilities for transmission components.
pub trait RotationalTransmission: RotationalInput + RotationalOutput {
    /// Gets the unique identifier for this transmission component.
    fn transmission_id(&self) -> TransmissionId;
    
    /// Checks if this transmission is currently enabled.
    fn is_enabled(&self) -> bool;
}