//! Network module for Horizon Mechanical Works.
//!
//! This module provides the foundation for mechanical network topology and connectivity,
//! managing how mechanical components are connected and organized within the system.

pub mod network;
pub mod topology;
pub mod registry;
pub mod validation;
pub mod errors;

#[cfg(test)]
mod tests;

/// Re-exports for convenient access to network-related types and functions.
pub use network::MechanicalNetwork;
pub use topology::{NetworkTopology, NetworkGraph};
pub use registry::NetworkRegistry;
pub use validation::NetworkValidation;
pub use errors::{NetworkError, NetworkResult};