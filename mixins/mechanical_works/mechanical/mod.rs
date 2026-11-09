//! Mechanical system module for Horizon Mechanical Works.
//!
//! This module provides the core mechanical automation functionality including:
//! - Power management systems
//! - Machine operations
//! - Component interactions
//! - Rotational physics foundation
//! - Network topology management
//! - Transmission systems

pub mod power;
pub mod machine;
pub mod inventory;
pub mod interaction;
pub mod block;
pub mod core;
pub mod physics;
pub mod network;
pub mod transmission;

/// Initializes the mechanical system components.
///
/// This function establishes clean integration boundaries between all mechanical subsystems
/// and ensures shared types are properly reused without duplication.
pub fn initialize_mechanical_system() {
    // System initialization is currently minimal as we're focused on internal integration
    // All subsystems are properly declared and exported through their respective modules
}