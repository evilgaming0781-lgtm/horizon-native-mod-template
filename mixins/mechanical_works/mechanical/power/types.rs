//! Power system foundational types for Horizon Mechanical Works.
//!
//! This module defines the core power-related data types and constants used throughout
//! the mechanical automation system.

use std::fmt;

/// The standard unit for measuring power in the HMW system.
/// 
/// All power values are represented in watts (W) internally for consistency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Watts(pub u32);

impl fmt::Display for Watts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}W", self.0)
    }
}

impl Default for Watts {
    fn default() -> Self {
        Watts(0)
    }
}

impl Watts {
    /// Creates a new power value in watts.
    pub const fn new(watts: u32) -> Self {
        Watts(watts)
    }

    /// Returns the power value as u32.
    pub const fn get(&self) -> u32 {
        self.0
    }

    /// Checks if the power value is zero.
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Adds two power values, saturating at the maximum value.
    pub fn saturating_add(&self, other: Self) -> Self {
        Watts(self.0.saturating_add(other.0))
    }

    /// Subtracts two power values, saturating at zero.
    pub fn saturating_sub(&self, other: Self) -> Self {
        Watts(self.0.saturating_sub(other.0))
    }
}

/// Represents a power supply capability or demand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerSupply {
    /// The maximum power output (in watts) that can be supplied.
    pub max_output: Watts,
    
    /// The current power output (in watts) being supplied.
    pub current_output: Watts,
}

impl Default for PowerSupply {
    fn default() -> Self {
        PowerSupply {
            max_output: Watts::new(0),
            current_output: Watts::new(0),
        }
    }
}

impl PowerSupply {
    /// Creates a new power supply with specified maximum output.
    pub const fn new(max_output: Watts) -> Self {
        PowerSupply {
            max_output,
            current_output: Watts::new(0),
        }
    }

    /// Sets the current output of this power supply.
    pub fn set_current_output(&mut self, output: Watts) {
        self.current_output = output;
    }

    /// Checks if the power supply is active (has non-zero output).
    pub const fn is_active(&self) -> bool {
        !self.current_output.is_zero()
    }

    /// Checks if the power supply has sufficient capacity.
    pub const fn has_capacity(&self, required: Watts) -> bool {
        self.max_output.get() >= required.get()
    }
}

/// Represents a power demand from a consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerDemand {
    /// The minimum power requirement (in watts).
    pub min_required: Watts,
    
    /// The maximum power that can be consumed (in watts).
    pub max_consumed: Watts,
}

impl Default for PowerDemand {
    fn default() -> Self {
        PowerDemand {
            min_required: Watts::new(0),
            max_consumed: Watts::new(0),
        }
    }
}

impl PowerDemand {
    /// Creates a new power demand with specified requirements.
    pub const fn new(min_required: Watts, max_consumed: Watts) -> Self {
        PowerDemand {
            min_required,
            max_consumed,
        }
    }

    /// Checks if the power demand can be satisfied.
    pub const fn is_satisfied(&self, available: Watts) -> bool {
        available.get() >= self.min_required.get()
    }

    /// Calculates the actual power consumed based on available power.
    pub fn consume(&self, available: Watts) -> Watts {
        // Consume up to the maximum allowed or available, whichever is less
        if available.get() > self.max_consumed.get() {
            self.max_consumed
        } else {
            available
        }
    }
}

/// Represents a power capacity limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerCapacity {
    /// The total capacity (in watts).
    pub total: Watts,
    
    /// The current usage (in watts).
    pub used: Watts,
}

impl Default for PowerCapacity {
    fn default() -> Self {
        PowerCapacity {
            total: Watts::new(0),
            used: Watts::new(0),
        }
    }
}

impl PowerCapacity {
    /// Creates a new power capacity with specified total.
    pub const fn new(total: Watts) -> Self {
        PowerCapacity {
            total,
            used: Watts::new(0),
        }
    }

    /// Sets the current usage of this capacity.
    pub fn set_used(&mut self, used: Watts) {
        self.used = used;
    }

    /// Calculates available capacity.
    pub fn available(&self) -> Watts {
        self.total.saturating_sub(self.used)
    }

    /// Checks if there is sufficient capacity.
    pub fn has_capacity(&self, required: Watts) -> bool {
        self.available().get() >= required.get()
    }
}

/// Represents a power system error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerError {
    /// Attempted to use more power than available.
    InsufficientPower,
    
    /// Attempted to exceed maximum power limits.
    OverLimit,
    
    /// Invalid power value (negative or zero when positive required).
    InvalidValue,
}

impl fmt::Display for PowerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerError::InsufficientPower => write!(f, "Insufficient power"),
            PowerError::OverLimit => write!(f, "Power limit exceeded"),
            PowerError::InvalidValue => write!(f, "Invalid power value"),
        }
    }
}

impl std::error::Error for PowerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watts_creation() {
        let watts = Watts::new(100);
        assert_eq!(watts.get(), 100);
        assert!(!watts.is_zero());
        
        let zero_watts = Watts::new(0);
        assert!(zero_watts.is_zero());
    }

    #[test]
    fn test_watts_display() {
        let watts = Watts::new(500);
        assert_eq!(format!("{}", watts), "500W");
    }

    #[test]
    fn test_watts_default() {
        let default_watts = Watts::default();
        assert_eq!(default_watts.get(), 0);
    }

    #[test]
    fn test_watts_saturating_add() {
        let watts1 = Watts::new(u32::MAX - 10);
        let watts2 = Watts::new(20);
        let result = watts1.saturating_add(watts2);
        assert_eq!(result.get(), u32::MAX); // Should saturate

        let watts3 = Watts::new(50);
        let watts4 = Watts::new(30);
        let result2 = watts3.saturating_add(watts4);
        assert_eq!(result2.get(), 80);
    }

    #[test]
    fn test_watts_saturating_sub() {
        let watts1 = Watts::new(10);
        let watts2 = Watts::new(5);
        let result = watts1.saturating_sub(watts2);
        assert_eq!(result.get(), 5);

        let watts3 = Watts::new(5);
        let watts4 = Watts::new(10);
        let result2 = watts3.saturating_sub(watts4);
        assert_eq!(result2.get(), 0); // Should saturate to zero
    }

    #[test]
    fn test_power_supply_creation() {
        let supply = PowerSupply::new(Watts::new(1000));
        assert_eq!(supply.max_output.get(), 1000);
        assert_eq!(supply.current_output.get(), 0);
        assert!(!supply.is_active());
    }

    #[test]
    fn test_power_supply_set_current_output() {
        let mut supply = PowerSupply::new(Watts::new(1000));
        supply.set_current_output(Watts::new(500));
        assert_eq!(supply.current_output.get(), 500);
        assert!(supply.is_active());
    }

    #[test]
    fn test_power_supply_has_capacity() {
        let supply = PowerSupply::new(Watts::new(1000));
        assert!(supply.has_capacity(Watts::new(500)));
        assert!(!supply.has_capacity(Watts::new(1500)));
    }

    #[test]
    fn test_power_demand_creation() {
        let demand = PowerDemand::new(Watts::new(200), Watts::new(800));
        assert_eq!(demand.min_required.get(), 200);
        assert_eq!(demand.max_consumed.get(), 800);
    }

    #[test]
    fn test_power_demand_is_satisfied() {
        let demand = PowerDemand::new(Watts::new(200), Watts::new(800));
        assert!(demand.is_satisfied(Watts::new(300)));
        assert!(!demand.is_satisfied(Watts::new(100)));
    }

    #[test]
    fn test_power_demand_consume() {
        let demand = PowerDemand::new(Watts::new(200), Watts::new(800));
        
        // Should consume up to max_consumed
        let consumed = demand.consume(Watts::new(1000));
        assert_eq!(consumed.get(), 800);
        
        // Should consume available power if less than max
        let consumed2 = demand.consume(Watts::new(500));
        assert_eq!(consumed2.get(), 500);
    }

    #[test]
    fn test_power_capacity_creation() {
        let capacity = PowerCapacity::new(Watts::new(1000));
        assert_eq!(capacity.total.get(), 1000);
        assert_eq!(capacity.used.get(), 0);
    }

    #[test]
    fn test_power_capacity_set_used() {
        let mut capacity = PowerCapacity::new(Watts::new(1000));
        capacity.set_used(Watts::new(300));
        assert_eq!(capacity.used.get(), 300);
    }

    #[test]
    fn test_power_capacity_available() {
        let mut capacity = PowerCapacity::new(Watts::new(1000));
        capacity.set_used(Watts::new(300));
        assert_eq!(capacity.available().get(), 700);
    }

    #[test]
    fn test_power_capacity_has_capacity() {
        let mut capacity = PowerCapacity::new(Watts::new(1000));
        capacity.set_used(Watts::new(300));
        assert!(capacity.has_capacity(Watts::new(500)));
        assert!(!capacity.has_capacity(Watts::new(800)));
    }

    #[test]
    fn test_power_error_display() {
        let error = PowerError::InsufficientPower;
        assert_eq!(format!("{}", error), "Insufficient power");

        let error2 = PowerError::OverLimit;
        assert_eq!(format!("{}", error2), "Power limit exceeded");

        let error3 = PowerError::InvalidValue;
        assert_eq!(format!("{}", error3), "Invalid power value");
    }
}