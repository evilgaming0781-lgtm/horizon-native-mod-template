//! Transmission module for Horizon Mechanical Works.
//!
//! This module provides the foundation for mechanical transmission systems,
//! including shafts, couplings, and transmission-specific interfaces.

pub mod types;
pub mod component;
pub mod interfaces;
pub mod shaft;
pub mod coupling;
pub mod configuration;
pub mod validation;
pub mod errors;
pub mod belt;
pub mod evaluation;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod evaluation_tests;

/// Re-exports for convenient access to transmission-related types and functions.
pub use types::{TransmissionCategory, TransmissionId};
pub use component::TransmissionComponent;
pub use interfaces::{RotationalInput, RotationalOutput, RotationalTransmission};
pub use shaft::Shaft;
pub use coupling::Coupling;
pub use configuration::TransmissionConfiguration;
pub use validation::TransmissionValidation;
pub use errors::{TransmissionError, TransmissionResult};
pub use belt::{BeltId, BeltCategory, BeltConfiguration, Belt};
pub use evaluation::{evaluate_shaft, evaluate_coupling, evaluate_gear, evaluate_belt, evaluate_transmission};