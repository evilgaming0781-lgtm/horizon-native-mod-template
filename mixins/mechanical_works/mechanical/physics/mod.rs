//! Rotational physics module for Horizon Mechanical Works.
//!
//! This module provides mathematical foundations for rotational mechanics including:
//! - Kinematics (motion calculations)
//! - Dynamics (force and torque calculations)
//! - Energy transfer calculations
//! - Efficiency modeling
//! - Operating limits validation

pub mod rotational;

// Re-export key types from rotational module
pub use rotational::{KinematicsError, DynamicsError, EnergyError, EfficiencyError, LimitError};