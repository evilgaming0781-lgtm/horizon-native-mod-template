//! Generator subsystem for Horizon Mechanical Works.
//!
//! This module provides the foundational generator functionality including:
//! - Generator state management
//! - Power production capabilities
//! - Configuration and capacity handling

use super::types::*;

/// Represents a mechanical generator component.
///
/// Generators can produce power based on their configuration and operational state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generator {
    /// The unique identifier for this generator instance.
    id: String,
    
    /// The maximum power output (in watts) that this generator can produce.
    max_output: Watts,
    
    /// The current power output (in watts) being produced.
    current_output: Watts,
    
    /// Whether the generator is currently enabled and operational.
    enabled: bool,
}

impl Generator {
    /// Creates a new generator with the specified ID and maximum output capacity.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for this generator instance
    /// * `max_output` - The maximum power output in watts (must be > 0)
    ///
    /// # Returns
    /// A new Generator instance with the specified configuration
    ///
    /// # Errors
    /// Returns `PowerError::InvalidValue` if max_output is zero
    pub fn new(id: String, max_output: Watts) -> Result<Self, PowerError> {
        if max_output.is_zero() {
            return Err(PowerError::InvalidValue);
        }
        
        Ok(Generator {
            id,
            max_output,
            current_output: Watts::new(0),
            enabled: false,
        })
    }

    /// Gets the unique identifier for this generator.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the maximum power output capacity of this generator.
    pub fn max_output(&self) -> Watts {
        self.max_output
    }

    /// Gets the current power output being produced by this generator.
    pub fn current_output(&self) -> Watts {
        self.current_output
    }

    /// Checks if this generator is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables the generator.
    ///
    /// When enabled, the generator can produce power up to its maximum capacity.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the generator.
    ///
    /// When disabled, the generator produces no power regardless of its configuration.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.current_output = Watts::new(0);
    }

    /// Sets the current power output for this generator.
    ///
    /// # Arguments
    /// * `output` - The desired power output in watts
    ///
    /// # Returns
    /// `Ok(())` if the output was set successfully, or `PowerError::OverLimit` if 
    /// the requested output exceeds the maximum capacity
    pub fn set_output(&mut self, output: Watts) -> Result<(), PowerError> {
        if output.get() > self.max_output.get() {
            return Err(PowerError::OverLimit);
        }
        
        self.current_output = output;
        Ok(())
    }

    /// Produces power based on the generator's configuration and enabled state.
    ///
    /// If the generator is enabled, it will produce up to its maximum capacity.
    /// If disabled, no power is produced.
    ///
    /// # Returns
    /// The actual power output produced (0 if disabled or insufficient capacity)
    pub fn produce(&mut self) -> Watts {
        if !self.enabled {
            self.current_output = Watts::new(0);
            return self.current_output;
        }
        
        // Production cannot exceed maximum capacity
        if self.current_output.get() > self.max_output.get() {
            self.current_output = self.max_output;
        }
        
        self.current_output
    }

    /// Attempts to increase the generator's output by a specified amount.
    ///
    /// # Arguments
    /// * `amount` - The additional power to produce in watts
    ///
    /// # Returns
    /// `Ok(())` if the output was increased successfully, or `PowerError::OverLimit`
    /// if the new output would exceed maximum capacity
    pub fn increase_output(&mut self, amount: Watts) -> Result<(), PowerError> {
        let new_output = self.current_output.saturating_add(amount);
        
        if new_output.get() > self.max_output.get() {
            return Err(PowerError::OverLimit);
        }
        
        self.current_output = new_output;
        Ok(())
    }

    /// Attempts to decrease the generator's output by a specified amount.
    ///
    /// # Arguments
    /// * `amount` - The reduction in power to produce in watts
    ///
    /// # Returns
    /// `Ok(())` if the output was decreased successfully, or `PowerError::InvalidValue`
    /// if the requested reduction would make output negative
    pub fn decrease_output(&mut self, amount: Watts) -> Result<(), PowerError> {
        if amount.get() > self.current_output.get() {
            return Err(PowerError::InvalidValue);
        }
        
        self.current_output = self.current_output.saturating_sub(amount);
        Ok(())
    }
}

impl Default for Generator {
    /// Creates a default generator with zero capacity.
    ///
    /// This is primarily for testing and should not be used in production.
    fn default() -> Self {
        Generator {
            id: String::from("default"),
            max_output: Watts::new(0),
            current_output: Watts::new(0),
            enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        assert_eq!(generator.id(), "gen1");
        assert_eq!(generator.max_output(), Watts::new(1000));
        assert_eq!(generator.current_output(), Watts::new(0));
        assert!(!generator.is_enabled());
    }

    #[test]
    fn test_generator_creation_zero_output() {
        let result = Generator::new(String::from("gen1"), Watts::new(0));
        assert!(matches!(result, Err(PowerError::InvalidValue)));
    }

    #[test]
    fn test_generator_enable_disable() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Initially disabled
        assert!(!generator.is_enabled());
        assert_eq!(generator.current_output(), Watts::new(0));
        
        // Enable the generator
        generator.enable();
        assert!(generator.is_enabled());
        
        // Disable the generator
        generator.disable();
        assert!(!generator.is_enabled());
        assert_eq!(generator.current_output(), Watts::new(0));
    }

    #[test]
    fn test_generator_set_output() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Valid output setting
        let result = generator.set_output(Watts::new(500));
        assert!(result.is_ok());
        assert_eq!(generator.current_output(), Watts::new(500));
        
        // Invalid output (exceeds capacity)
        let result2 = generator.set_output(Watts::new(1500));
        assert!(matches!(result2, Err(PowerError::OverLimit)));
        // Output should remain unchanged
        assert_eq!(generator.current_output(), Watts::new(500));
    }

    #[test]
    fn test_generator_produce_disabled() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Set output before enabling
        let _ = generator.set_output(Watts::new(500));
        assert_eq!(generator.current_output(), Watts::new(500));
        
        // Produce when disabled - should return 0
        let produced = generator.produce();
        assert_eq!(produced, Watts::new(0));
        assert_eq!(generator.current_output(), Watts::new(0));
    }

    #[test]
    fn test_generator_produce_enabled() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Enable and set output
        generator.enable();
        let _ = generator.set_output(Watts::new(500));
        
        // Produce when enabled - should return current output
        let produced = generator.produce();
        assert_eq!(produced, Watts::new(500));
        assert_eq!(generator.current_output(), Watts::new(500));
    }

    #[test]
    fn test_generator_produce_exceeds_capacity() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Enable and set output above capacity
        generator.enable();
        let _ = generator.set_output(Watts::new(1500)); // This should be capped
        
        // Produce - should cap at max output
        let produced = generator.produce();
        assert_eq!(produced, Watts::new(1000)); // Should be capped at max capacity
        assert_eq!(generator.current_output(), Watts::new(1000));
    }

    #[test]
    fn test_generator_increase_output() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Valid increase
        let result = generator.increase_output(Watts::new(200));
        assert!(result.is_ok());
        assert_eq!(generator.current_output(), Watts::new(200));
        
        // Increase that exceeds capacity
        let result2 = generator.increase_output(Watts::new(900));
        assert!(matches!(result2, Err(PowerError::OverLimit)));
        // Output should remain unchanged
        assert_eq!(generator.current_output(), Watts::new(200));
    }

    #[test]
    fn test_generator_decrease_output() {
        let mut generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        
        // Set initial output
        let _ = generator.set_output(Watts::new(500));
        
        // Valid decrease
        let result = generator.decrease_output(Watts::new(200));
        assert!(result.is_ok());
        assert_eq!(generator.current_output(), Watts::new(300));
        
        // Decrease that would make output negative
        let result2 = generator.decrease_output(Watts::new(400));
        assert!(matches!(result2, Err(PowerError::InvalidValue)));
        // Output should remain unchanged
        assert_eq!(generator.current_output(), Watts::new(300));
    }

    #[test]
    fn test_generator_default() {
        let generator = Generator::default();
        assert_eq!(generator.id(), "default");
        assert_eq!(generator.max_output(), Watts::new(0));
        assert_eq!(generator.current_output(), Watts::new(0));
        assert!(!generator.is_enabled());
    }
}