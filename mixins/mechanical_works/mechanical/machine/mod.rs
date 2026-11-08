//! Machine system module for Horizon Mechanical Works.
//!
//! This module provides the core mechanical automation functionality including:
//! - Power management systems
//! - Machine operations
//! - Component interactions
//! - Machine type definitions
//! - State management
//! - Control layer
//! - Registry system

pub mod types;
pub mod state;
pub mod control;
pub mod registry;
pub mod power;
pub mod machine;
pub mod inventory;
pub mod interaction;
pub mod block;