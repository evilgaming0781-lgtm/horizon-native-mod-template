//! Belt transmission module for Horizon Mechanical Works.
//!
//! This module provides the foundation for mechanical belt systems,
//! including belt components, pulleys, and transmission calculations.

pub mod types;
pub mod belt;
pub mod pulley;
pub mod pair;
pub mod ratio;
pub mod calculations;
pub mod validation;
pub mod errors;
pub mod example;

#[cfg(test)]
mod tests;

/// Re-exports for convenient access to belt-related types and functions.
pub use types::{BeltId, BeltCategory, BeltConfiguration};
pub use belt::Belt;
pub use pulley::Pulley;
pub use pair::BeltPair;
pub use ratio::BeltRatio;
pub use calculations::{
    calculate_output_angular_velocity, 
    calculate_output_rpm, 
    calculate_output_torque, 
    calculate_output_power, 
    calculate_power_loss,
    calculate_belt_linear_velocity
};
pub use validation::BeltValidation;
pub use errors::{BeltError, BeltResult};