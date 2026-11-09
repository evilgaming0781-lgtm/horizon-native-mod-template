//! Shared transmission-specific types.
//!
//! This module defines common types used throughout the transmission subsystem.

use std::fmt;
use crate::mechanical::core::identifiers::{ComponentId, NetworkId};

/// Unique identifier for a transmission component.
///
/// Transmission IDs are used to track specific transmission components
/// within the mechanical system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmissionId(pub u64);

impl fmt::Display for TransmissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Transmission{}", self.0)
    }
}

impl Default for TransmissionId {
    fn default() -> Self {
        TransmissionId(0)
    }
}

impl TransmissionId {
    /// Creates a new transmission ID with the specified value.
    pub const fn new(id: u64) -> Self {
        TransmissionId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// The category or type of a transmission component.
///
/// Transmission categories help identify what kind of transmission functionality
/// a component provides within the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransmissionCategory {
    /// A rotational shaft that transfers mechanical power.
    Shaft,
    
    /// A coupling that connects two transmission components.
    Coupling,
    
    /// A gear that changes rotational speed and torque.
    Gear,
    
    /// A belt or chain for power transfer.
    Belt,
    
    /// A clutch mechanism.
    Clutch,
}

impl fmt::Display for TransmissionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransmissionCategory::Shaft => write!(f, "Shaft"),
            TransmissionCategory::Coupling => write!(f, "Coupling"),
            TransmissionCategory::Gear => write!(f, "Gear"),
            TransmissionCategory::Belt => write!(f, "Belt"),
            TransmissionCategory::Clutch => write!(f, "Clutch"),
        }
    }
}

/// Transmission operating configuration.
///
/// This structure holds the configuration settings for transmission components
/// that affect their operational behavior and limits.
#[derive(Debug, Clone)]
pub struct TransmissionConfiguration {
    /// Maximum rotational speed in RPM that this component can handle.
    pub max_rpm: f64,
    
    /// Maximum torque in newton-metres that this component can transmit.
    pub max_torque: f64,
    
    /// Maximum power in watts that this component can handle.
    pub max_power: f64,
    
    /// Indicates if the transmission is currently enabled and operational.
    pub enabled: bool,
}