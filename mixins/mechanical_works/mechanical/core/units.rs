//! Shared physical measurement types for mechanical systems.
//!
//! This module provides type-safe physical quantities to prevent mixing of 
//! different units and ensure accurate mechanical calculations.

use std::fmt;

/// Torque measured in newton-metres (N·m).
///
/// Torque represents the rotational force applied to a component.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Torque(pub f64);

impl fmt::Display for Torque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} N·m", self.0)
    }
}

impl Default for Torque {
    fn default() -> Self {
        Torque(0.0)
    }
}

impl Torque {
    /// Creates a new torque value in newton-metres.
    pub fn new(torque: f64) -> Self {
        if torque.is_nan() || torque.is_infinite() {
            panic!("Torque cannot be NaN or infinite");
        }
        Torque(torque)
    }

    /// Returns the torque value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the torque value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Angular velocity measured in radians per second (rad/s).
///
/// Angular velocity represents the rate of change of angular position.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AngularVelocity(pub f64);

impl fmt::Display for AngularVelocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} rad/s", self.0)
    }
}

impl Default for AngularVelocity {
    fn default() -> Self {
        AngularVelocity(0.0)
    }
}

impl AngularVelocity {
    /// Creates a new angular velocity value in radians per second.
    pub fn new(velocity: f64) -> Self {
        if velocity.is_nan() || velocity.is_infinite() {
            panic!("Angular velocity cannot be NaN or infinite");
        }
        AngularVelocity(velocity)
    }

    /// Returns the angular velocity value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the angular velocity value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Rotational speed measured in revolutions per minute (RPM).
///
/// RPM represents the number of complete rotations per minute.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RotationalSpeed(pub f64);

impl fmt::Display for RotationalSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} RPM", self.0)
    }
}

impl Default for RotationalSpeed {
    fn default() -> Self {
        RotationalSpeed(0.0)
    }
}

impl RotationalSpeed {
    /// Creates a new rotational speed value in revolutions per minute.
    pub fn new(speed: f64) -> Self {
        if speed.is_nan() || speed.is_infinite() || speed < 0.0 {
            panic!("Rotational speed cannot be negative, NaN or infinite");
        }
        RotationalSpeed(speed)
    }

    /// Returns the rotational speed value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Converts RPM to radians per second.
    ///
    /// Formula: ω = rpm × 2π / 60
    pub fn to_rad_per_sec(&self) -> AngularVelocity {
        let rad_per_sec = self.0 * 2.0 * std::f64::consts::PI / 60.0;
        AngularVelocity::new(rad_per_sec)
    }

    /// Checks if the rotational speed value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Mechanical power measured in watts (W).
///
/// Power represents the rate at which work is done or energy is transferred.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Power(pub f64);

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} W", self.0)
    }
}

impl Default for Power {
    fn default() -> Self {
        Power(0.0)
    }
}

impl Power {
    /// Creates a new power value in watts.
    pub fn new(power: f64) -> Self {
        if power.is_nan() || power.is_infinite() {
            panic!("Power cannot be NaN or infinite");
        }
        Power(power)
    }

    /// Returns the power value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the power value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Energy measured in joules (J).
///
/// Energy represents the capacity for doing work.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy(pub f64);

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} J", self.0)
    }
}

impl Default for Energy {
    fn default() -> Self {
        Energy(0.0)
    }
}

impl Energy {
    /// Creates a new energy value in joules.
    pub fn new(energy: f64) -> Self {
        if energy.is_nan() || energy.is_infinite() {
            panic!("Energy cannot be NaN or infinite");
        }
        Energy(energy)
    }

    /// Returns the energy value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the energy value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }
}

/// Efficiency represented as a percentage (0.0 to 1.0).
///
/// Efficiency represents the ratio of useful output to total input.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Efficiency(pub f64);

impl fmt::Display for Efficiency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
    }
}

impl Default for Efficiency {
    fn default() -> Self {
        Efficiency(1.0) // 100% efficiency by default
    }
}

impl Efficiency {
    /// Creates a new efficiency value.
    ///
    /// # Arguments
    /// * `efficiency` - Value between 0.0 (0%) and 1.0 (100%)
    ///
    /// # Panics
    /// Panics if the efficiency is outside the valid range [0.0, 1.0]
    pub fn new(efficiency: f64) -> Self {
        if efficiency.is_nan() || efficiency.is_infinite() || efficiency < 0.0 || efficiency > 1.0 {
            panic!("Efficiency must be between 0.0 and 1.0");
        }
        Efficiency(efficiency)
    }

    /// Returns the efficiency value as f64.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Checks if the efficiency is zero.
    pub fn is_zero(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }

    /// Checks if the efficiency is maximum (100%).
    pub fn is_maximum(&self) -> bool {
        (self.0 - 1.0).abs() < f64::EPSILON
    }
}

/// Calculates mechanical power using the relationship P = τ × ω.
///
/// # Arguments
/// * `torque` - The torque in newton-metres
/// * `angular_velocity` - The angular velocity in radians per second
///
/// # Returns
/// The mechanical power in watts
pub fn calculate_power(torque: Torque, angular_velocity: AngularVelocity) -> Power {
    let power = torque.get() * angular_velocity.get();
    Power::new(power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torque_creation() {
        let torque = Torque::new(10.0);
        assert_eq!(torque.get(), 10.0);
    }

    #[test]
    #[should_panic(expected = "Torque cannot be NaN or infinite")]
    fn test_torque_invalid_value() {
        let _ = Torque::new(f64::NAN);
    }

    #[test]
    fn test_angular_velocity_creation() {
        let velocity = AngularVelocity::new(5.0);
        assert_eq!(velocity.get(), 5.0);
    }

    #[test]
    #[should_panic(expected = "Angular velocity cannot be NaN or infinite")]
    fn test_angular_velocity_invalid_value() {
        let _ = AngularVelocity::new(f64::INFINITY);
    }

    #[test]
    fn test_rotational_speed_creation() {
        let speed = RotationalSpeed::new(120.0);
        assert_eq!(speed.get(), 120.0);
    }

    #[test]
    #[should_panic(expected = "Rotational speed cannot be negative, NaN or infinite")]
    fn test_rotational_speed_invalid_value() {
        let _ = RotationalSpeed::new(-10.0);
    }

    #[test]
    fn test_rpm_to_rad_per_sec_conversion() {
        let rpm = RotationalSpeed::new(60.0); // 60 RPM
        let rad_per_sec = rpm.to_rad_per_sec();
        assert!((rad_per_sec.get() - 2.0 * std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_power_creation() {
        let power = Power::new(100.0);
        assert_eq!(power.get(), 100.0);
    }

    #[test]
    #[should_panic(expected = "Power cannot be NaN or infinite")]
    fn test_power_invalid_value() {
        let _ = Power::new(f64::INFINITY);
    }

    #[test]
    fn test_energy_creation() {
        let energy = Energy::new(50.0);
        assert_eq!(energy.get(), 50.0);
    }

    #[test]
    #[should_panic(expected = "Energy cannot be NaN or infinite")]
    fn test_energy_invalid_value() {
        let _ = Energy::new(f64::NAN);
    }

    #[test]
    fn test_efficiency_creation() {
        let efficiency = Efficiency::new(0.85);
        assert_eq!(efficiency.get(), 0.85);
    }

    #[test]
    #[should_panic(expected = "Efficiency must be between 0.0 and 1.0")]
    fn test_efficiency_invalid_value() {
        let _ = Efficiency::new(1.5);
    }

    #[test]
    fn test_power_calculation() {
        let torque = Torque::new(10.0);
        let angular_velocity = AngularVelocity::new(2.0);
        let power = calculate_power(torque, angular_velocity);
        assert_eq!(power.get(), 20.0);
    }
}