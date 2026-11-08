//! Consumer subsystem for Horizon Mechanical Works.
//!
//! This module provides the foundational consumer functionality including:
//! - Power consumption management
//! - Demand specification
//! - Allocation and usage tracking

use crate::mechanical::power::types::*;

/// Represents a mechanical consumer component.
///
/// Consumers request and use power based on their operational requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consumer {
    /// The unique identifier for this consumer instance.
    id: String,
    
    /// The minimum power requirement (in watts) for this consumer to operate.
    min_required: Watts,
    
    /// The maximum power that can be consumed (in watts) by this consumer.
    max_consumed: Watts,
    
    /// The allocated power (in watts) that this consumer has been assigned.
    allocated_power: Watts,
    
    /// Whether the consumer is currently enabled and operational.
    enabled: bool,
}

impl Consumer {
    /// Creates a new consumer with the specified ID and power requirements.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for this consumer instance
    /// * `min_required` - The minimum power required to operate (must be >= 0)
    /// * `max_consumed` - The maximum power that can be consumed (must be >= min_required)
    ///
    /// # Returns
    /// A new Consumer instance with the specified configuration
    ///
    /// # Errors
    /// Returns `PowerError::InvalidValue` if:
    /// - min_required is negative (not possible with u32, but checked for consistency)
    /// - max_consumed is less than min_required
    pub fn new(id: String, min_required: Watts, max_consumed: Watts) -> Result<Self, PowerError> {
        if min_required.get() > max_consumed.get() {
            return Err(PowerError::InvalidValue);
        }
        
        Ok(Consumer {
            id,
            min_required,
            max_consumed,
            allocated_power: Watts::new(0),
            enabled: false,
        })
    }

    /// Gets the unique identifier for this consumer.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the minimum power requirement for this consumer.
    pub fn min_required(&self) -> Watts {
        self.min_required
    }

    /// Gets the maximum power that can be consumed by this consumer.
    pub fn max_consumed(&self) -> Watts {
        self.max_consumed
    }

    /// Gets the allocated power assigned to this consumer.
    pub fn allocated_power(&self) -> Watts {
        self.allocated_power
    }

    /// Checks if this consumer is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables the consumer.
    ///
    /// When enabled, the consumer can request and use power up to its maximum capacity.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the consumer.
    ///
    /// When disabled, the consumer cannot consume power regardless of allocation.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.allocated_power = Watts::new(0);
    }

    /// Allocates power to this consumer.
    ///
    /// # Arguments
    /// * `power` - The amount of power (in watts) to allocate
    ///
    /// # Returns
    /// `Ok(())` if the allocation was successful, or `PowerError::OverLimit` if 
    /// the requested allocation exceeds maximum consumption capacity
    pub fn allocate_power(&mut self, power: Watts) -> Result<(), PowerError> {
        if power.get() > self.max_consumed.get() {
            return Err(PowerError::OverLimit);
        }
        
        self.allocated_power = power;
        Ok(())
    }

    /// Consumes power from the allocated amount.
    ///
    /// # Arguments
    /// * `available` - The available power (in watts) that can be consumed
    ///
    /// # Returns
    /// The actual power consumed (0 if disabled, insufficient, or not enough allocated)
    pub fn consume(&mut self, available: Watts) -> Watts {
        // If consumer is disabled or no power allocated, nothing to consume
        if !self.enabled || self.allocated_power.is_zero() {
            return Watts::new(0);
        }
        
        // Check if we have sufficient allocated power
        if self.allocated_power.get() < self.min_required.get() {
            return Watts::new(0); // Not enough allocated for minimum requirement
        }
        
        // Consume up to what's available and within maximum capacity
        let actual_consumed = self.max_consumed.consume(available);
        
        // Ensure we don't consume more than allocated
        if actual_consumed.get() > self.allocated_power.get() {
            self.allocated_power = Watts::new(0); // Reset allocation after consumption
            return self.allocated_power;
        }
        
        self.allocated_power = self.allocated_power.saturating_sub(actual_consumed);
        actual_consumed
    }

    /// Checks if this consumer can be satisfied with the given available power.
    ///
    /// # Arguments
    /// * `available` - The available power (in watts) to check against requirements
    ///
    /// # Returns
    /// `true` if the consumer's minimum requirement can be met, `false` otherwise
    pub fn is_satisfied(&self, available: Watts) -> bool {
        self.min_required.is_zero() || available.get() >= self.min_required.get()
    }

    /// Gets the remaining power that can still be consumed.
    ///
    /// # Returns
    /// The remaining power (in watts) that can be consumed by this consumer
    pub fn remaining_power(&self) -> Watts {
        self.allocated_power
    }
}

impl Default for Consumer {
    /// Creates a default consumer with zero requirements.
    ///
    /// This is primarily for testing and should not be used in production.
    fn default() -> Self {
        Consumer {
            id: String::from("default"),
            min_required: Watts::new(0),
            max_consumed: Watts::new(0),
            allocated_power: Watts::new(0),
            enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_creation() {
        let consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        assert_eq!(consumer.id(), "consumer1");
        assert_eq!(consumer.min_required(), Watts::new(200));
        assert_eq!(consumer.max_consumed(), Watts::new(800));
        assert_eq!(consumer.allocated_power(), Watts::new(0));
        assert!(!consumer.is_enabled());
    }

    #[test]
    fn test_consumer_creation_invalid_requirements() {
        // max_consumed less than min_required
        let result = Consumer::new(String::from("consumer1"), Watts::new(800), Watts::new(200));
        assert!(matches!(result, Err(PowerError::InvalidValue)));
    }

    #[test]
    fn test_consumer_enable_disable() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Initially disabled
        assert!(!consumer.is_enabled());
        assert_eq!(consumer.allocated_power(), Watts::new(0));
        
        // Enable the consumer
        consumer.enable();
        assert!(consumer.is_enabled());
        
        // Disable the consumer
        consumer.disable();
        assert!(!consumer.is_enabled());
        assert_eq!(consumer.allocated_power(), Watts::new(0));
    }

    #[test]
    fn test_consumer_allocate_power() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Valid allocation
        let result = consumer.allocate_power(Watts::new(500));
        assert!(result.is_ok());
        assert_eq!(consumer.allocated_power(), Watts::new(500));
        
        // Invalid allocation (exceeds maximum consumption)
        let result2 = consumer.allocate_power(Watts::new(1000));
        assert!(matches!(result2, Err(PowerError::OverLimit)));
        // Allocation should remain unchanged
        assert_eq!(consumer.allocated_power(), Watts::new(500));
    }

    #[test]
    fn test_consumer_consume_disabled() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Allocate some power
        let _ = consumer.allocate_power(Watts::new(500));
        assert_eq!(consumer.allocated_power(), Watts::new(500));
        
        // Consume when disabled - should return 0
        let consumed = consumer.consume(Watts::new(1000));
        assert_eq!(consumed, Watts::new(0));
        assert_eq!(consumer.allocated_power(), Watts::new(500)); // Should remain unchanged
    }

    #[test]
    fn test_consumer_consume_insufficient_allocated() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Enable and allocate less than minimum required
        consumer.enable();
        let _ = consumer.allocate_power(Watts::new(100)); // Less than min_required of 200
        assert_eq!(consumer.allocated_power(), Watts::new(100));
        
        // Try to consume - should return 0 since not enough allocated for minimum
        let consumed = consumer.consume(Watts::new(1000));
        assert_eq!(consumed, Watts::new(0));
        assert_eq!(consumer.allocated_power(), Watts::new(100)); // Should remain unchanged
    }

    #[test]
    fn test_consumer_consume_sufficient_allocated() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Enable and allocate sufficient power
        consumer.enable();
        let _ = consumer.allocate_power(Watts::new(500));
        assert_eq!(consumer.allocated_power(), Watts::new(500));
        
        // Consume available power - should consume up to max_consumed or available
        let consumed = consumer.consume(Watts::new(1000));
        assert_eq!(consumed, Watts::new(500)); // Consumes all allocated
        assert_eq!(consumer.allocated_power(), Watts::new(0)); // All consumed
    }

    #[test]
    fn test_consumer_consume_limited_by_max_consumption() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Enable and allocate more than max_consumed
        consumer.enable();
        let _ = consumer.allocate_power(Watts::new(1500)); // More than max_consumed of 800
        assert_eq!(consumer.allocated_power(), Watts::new(1500));
        
        // Consume available power - should be limited by max_consumed
        let consumed = consumer.consume(Watts::new(2000));
        assert_eq!(consumed, Watts::new(800)); // Limited by max_consumed
        assert_eq!(consumer.allocated_power(), Watts::new(700)); // Remaining after consumption
    }

    #[test]
    fn test_consumer_is_satisfied() {
        let consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Should be satisfied with enough power
        assert!(consumer.is_satisfied(Watts::new(300)));
        
        // Should not be satisfied with insufficient power
        assert!(!consumer.is_satisfied(Watts::new(100)));
        
        // Zero minimum requirement should always be satisfied
        let consumer_zero = Consumer::new(String::from("consumer2"), Watts::new(0), Watts::new(800)).unwrap();
        assert!(consumer_zero.is_satisfied(Watts::new(0)));
        assert!(consumer_zero.is_satisfied(Watts::new(100)));
    }

    #[test]
    fn test_consumer_remaining_power() {
        let mut consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        
        // Initially no allocated power
        assert_eq!(consumer.remaining_power(), Watts::new(0));
        
        // Allocate some power
        let _ = consumer.allocate_power(Watts::new(500));
        assert_eq!(consumer.remaining_power(), Watts::new(500));
        
        // Consume some power
        consumer.enable();
        let consumed = consumer.consume(Watts::new(300));
        assert_eq!(consumed, Watts::new(300));
        assert_eq!(consumer.remaining_power(), Watts::new(200));
    }

    #[test]
    fn test_consumer_default() {
        let consumer = Consumer::default();
        assert_eq!(consumer.id(), "default");
        assert_eq!(consumer.min_required(), Watts::new(0));
        assert_eq!(consumer.max_consumed(), Watts::new(0));
        assert_eq!(consumer.allocated_power(), Watts::new(0));
        assert!(!consumer.is_enabled());
    }
}