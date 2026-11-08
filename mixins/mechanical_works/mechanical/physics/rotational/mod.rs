//! Rotational physics subsystem for Horizon Mechanical Works.
//!
//! This module provides the core rotational mechanics calculations including:
//! - Kinematics (motion parameters)
//! - Dynamics (forces and torques)
//! - Energy calculations
//! - Efficiency modeling
//! - Operating limits

pub mod kinematics;
pub mod dynamics;
pub mod energy;
pub mod efficiency;
pub mod limits;
pub mod errors;

// Re-export key types for easier access
pub use kinematics::{AngularAcceleration, rpm_to_rad_per_sec, rad_per_sec_to_rpm};
pub use dynamics::{MomentOfInertia};
pub use energy::Power;
pub use efficiency::Efficiency;
pub use limits::RotationalLimits;