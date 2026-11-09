//! Gear transmission module for Horizon Mechanical Works.
//!
//! This module provides the foundation for mechanical gear systems,
//! including gear representations, ratios, and calculations.

pub mod types;
pub mod gear;
pub mod pair;
pub mod ratio;
pub mod calculations;
pub mod validation;
pub mod errors;

#[cfg(test)]
mod tests;

/// Re-exports for convenient access to gear-related types and functions.
pub use types::{GearId, GearCategory, GearConfiguration};
pub use gear::Gear;
pub use pair::GearPair;
pub use ratio::GearRatio;
pub use calculations::{calculate_output_angular_velocity, calculate_output_rpm, calculate_output_torque, calculate_output_power, calculate_power_loss};
pub use validation::GearValidation;
pub use errors::{GearError, GearResult};