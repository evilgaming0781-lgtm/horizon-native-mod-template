//! Power grid subsystem for Horizon Mechanical Works.
//!
//! This module provides the foundational power grid functionality including:
//! - Grid management of generators and consumers
//! - Power allocation and distribution
//! - State tracking and recalculation

use crate::mechanical::power::{generator::Generator, consumer::Consumer, types::*};
use std::collections::HashMap;

/// Represents a power grid that manages generators and consumers.
///
/// The grid handles power distribution by allocating available generation to consumers
/// based on their requirements and priorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerGrid {
    /// Unique identifier for this power grid.
    id: String,
    
    /// Map of registered generators by their IDs.
    generators: HashMap<String, Generator>,
    
    /// Map of registered consumers by their IDs.
    consumers: HashMap<String, Consumer>,
    
    /// Total available power from all enabled generators.
    total_generation: Watts,
    
    /// Total power demand from all enabled consumers.
    total_demand: Watts,
}

impl PowerGrid {
    /// Creates a new power grid with the specified ID.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for this power grid instance
    ///
    /// # Returns
    /// A new PowerGrid instance
    pub fn new(id: String) -> Self {
        PowerGrid {
            id,
            generators: HashMap::new(),
            consumers: HashMap::new(),
            total_generation: Watts::new(0),
            total_demand: Watts::new(0),
        }
    }

    /// Gets the unique identifier for this power grid.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Registers a new generator with the power grid.
    ///
    /// # Arguments
    /// * `generator` - The generator to register
    ///
    /// # Returns
    /// `Ok(())` if registration was successful, or `PowerError::InvalidValue` 
    /// if a generator with the same ID already exists
    pub fn add_generator(&mut self, generator: Generator) -> Result<(), PowerError> {
        if self.generators.contains_key(generator.id()) {
            return Err(PowerError::InvalidValue);
        }
        
        self.generators.insert(generator.id().to_string(), generator);
        Ok(())
    }

    /// Removes a generator from the power grid.
    ///
    /// # Arguments
    /// * `id` - The ID of the generator to remove
    ///
    /// # Returns
    /// `Ok(())` if removal was successful, or `PowerError::InvalidValue` 
    /// if no generator with that ID exists
    pub fn remove_generator(&mut self, id: &str) -> Result<(), PowerError> {
        if !self.generators.contains_key(id) {
            return Err(PowerError::InvalidValue);
        }
        
        self.generators.remove(id);
        Ok(())
    }

    /// Registers a new consumer with the power grid.
    ///
    /// # Arguments
    /// * `consumer` - The consumer to register
    ///
    /// # Returns
    /// `Ok(())` if registration was successful, or `PowerError::InvalidValue` 
    /// if a consumer with the same ID already exists
    pub fn add_consumer(&mut self, consumer: Consumer) -> Result<(), PowerError> {
        if self.consumers.contains_key(consumer.id()) {
            return Err(PowerError::InvalidValue);
        }
        
        self.consumers.insert(consumer.id().to_string(), consumer);
        Ok(())
    }

    /// Removes a consumer from the power grid.
    ///
    /// # Arguments
    /// * `id` - The ID of the consumer to remove
    ///
    /// # Returns
    /// `Ok(())` if removal was successful, or `PowerError::InvalidValue` 
    /// if no consumer with that ID exists
    pub fn remove_consumer(&mut self, id: &str) -> Result<(), PowerError> {
        if !self.consumers.contains_key(id) {
            return Err(PowerError::InvalidValue);
        }
        
        self.consumers.remove(id);
        Ok(())
    }

    /// Gets a reference to a registered generator.
    ///
    /// # Arguments
    /// * `id` - The ID of the generator to retrieve
    ///
    /// # Returns
    /// `Some(&Generator)` if found, or `None` if not found
    pub fn get_generator(&self, id: &str) -> Option<&Generator> {
        self.generators.get(id)
    }

    /// Gets a mutable reference to a registered generator.
    ///
    /// # Arguments
    /// * `id` - The ID of the generator to retrieve
    ///
    /// # Returns
    /// `Some(&mut Generator)` if found, or `None` if not found
    pub fn get_generator_mut(&mut self, id: &str) -> Option<&mut Generator> {
        self.generators.get_mut(id)
    }

    /// Gets a reference to a registered consumer.
    ///
    /// # Arguments
    /// * `id` - The ID of the consumer to retrieve
    ///
    /// # Returns
    /// `Some(&Consumer)` if found, or `None` if not found
    pub fn get_consumer(&self, id: &str) -> Option<&Consumer> {
        self.consumers.get(id)
    }

    /// Gets a mutable reference to a registered consumer.
    ///
    /// # Arguments
    /// * `id` - The ID of the consumer to retrieve
    ///
    /// # Returns
    /// `Some(&mut Consumer)` if found, or `None` if not found
    pub fn get_consumer_mut(&mut self, id: &str) -> Option<&mut Consumer> {
        self.consumers.get_mut(id)
    }

    /// Gets the total available power generation from all enabled generators.
    pub fn total_generation(&self) -> Watts {
        self.total_generation
    }

    /// Gets the total power demand from all enabled consumers.
    pub fn total_demand(&self) -> Watts {
        self.total_demand
    }

    /// Calculates and performs power allocation for all registered components.
    ///
    /// This method:
    /// 1. Recalculates total generation from enabled generators
    /// 2. Recalculates total demand from enabled consumers  
    /// 3. Allocates available power to consumers based on their requirements
    /// 4. Handles insufficient power by prioritizing consumers with higher minimum requirements
    ///
    /// # Returns
    /// The actual power allocated (which may be less than total demand if insufficient generation)
    pub fn allocate_power(&mut self) -> Watts {
        // Recalculate total generation from enabled generators
        self.total_generation = self.calculate_total_generation();
        
        // Recalculate total demand from enabled consumers
        self.total_demand = self.calculate_total_demand();
        
        // If no generation, nothing to allocate
        if self.total_generation.is_zero() {
            // Reset all allocations to zero
            for consumer in self.consumers.values_mut() {
                if consumer.is_enabled() {
                    let _ = consumer.allocate_power(Watts::new(0));
                }
            }
            return Watts::new(0);
        }
        
        // If no demand, nothing to allocate
        if self.total_demand.is_zero() {
            return Watts::new(0);
        }
        
        // If generation is less than demand, we need to distribute proportionally
        if self.total_generation.get() < self.total_demand.get() {
            return self.allocate_power_proportional();
        }
        
        // If generation meets or exceeds demand, allocate full demand
        self.allocate_power_full()
    }

    /// Calculates total power generation from all enabled generators.
    ///
    /// # Returns
    /// The total available power (in watts) from enabled generators
    fn calculate_total_generation(&self) -> Watts {
        let mut total = Watts::new(0);
        
        for generator in self.generators.values() {
            if generator.is_enabled() {
                total = total.saturating_add(generator.produce());
            }
        }
        
        total
    }

    /// Calculates total power demand from all enabled consumers.
    ///
    /// # Returns
    /// The total required power (in watts) from enabled consumers
    fn calculate_total_demand(&self) -> Watts {
        let mut total = Watts::new(0);
        
        for consumer in self.consumers.values() {
            if consumer.is_enabled() {
                // Use minimum requirement as the basis for demand calculation
                total = total.saturating_add(consumer.min_required());
            }
        }
        
        total
    }

    /// Allocates power proportionally when generation is insufficient.
    ///
    /// This method distributes available power among consumers based on their 
    /// minimum requirements, giving priority to consumers with higher minimum demands.
    ///
    /// # Returns
    /// The actual power allocated (which may be less than total demand)
    fn allocate_power_proportional(&mut self) -> Watts {
        // Create a list of enabled consumers sorted by minimum requirement (descending)
        let mut sorted_consumers: Vec<&mut Consumer> = self.consumers
            .values_mut()
            .filter(|c| c.is_enabled())
            .collect();
        
        sorted_consumers.sort_by(|a, b| b.min_required().get().cmp(&a.min_required().get()));
        
        let mut allocated_power = Watts::new(0);
        let available_generation = self.total_generation;
        
        // Distribute power to consumers in order of priority
        for consumer in sorted_consumers {
            // Calculate how much this consumer can get based on their minimum requirement
            let consumer_min = consumer.min_required();
            
            // If we don't have enough power for this consumer's minimum, give them what we can
            if available_generation.get() >= consumer_min.get() {
                // Allocate the full minimum requirement
                let _ = consumer.allocate_power(consumer_min);
                allocated_power = allocated_power.saturating_add(consumer_min);
            } else {
                // Give what we have to this consumer (if any)
                if !available_generation.is_zero() {
                    let _ = consumer.allocate_power(available_generation);
                    allocated_power = allocated_power.saturating_add(available_generation);
                }
                break; // No more power to distribute
            }
        }
        
        allocated_power
    }

    /// Allocates full power when generation meets or exceeds demand.
    ///
    /// This method allocates the maximum possible power to consumers based on their 
    /// maximum consumption limits.
    ///
    /// # Returns
    /// The actual power allocated (which equals total demand)
    fn allocate_power_full(&mut self) -> Watts {
        let mut allocated_power = Watts::new(0);
        
        // For simplicity, we'll distribute the full generation to consumers based on their max capacity
        // In a more sophisticated system, this would be distributed according to priority or other factors
        
        for consumer in self.consumers.values_mut() {
            if consumer.is_enabled() {
                let max_consumed = consumer.max_consumed();
                let _ = consumer.allocate_power(max_consumed);
                allocated_power = allocated_power.saturating_add(max_consumed);
            }
        }
        
        allocated_power
    }

    /// Gets the number of registered generators.
    pub fn generator_count(&self) -> usize {
        self.generators.len()
    }

    /// Gets the number of registered consumers.
    pub fn consumer_count(&self) -> usize {
        self.consumers.len()
    }

    /// Checks if a generator with the specified ID exists.
    pub fn has_generator(&self, id: &str) -> bool {
        self.generators.contains_key(id)
    }

    /// Checks if a consumer with the specified ID exists.
    pub fn has_consumer(&self, id: &str) -> bool {
        self.consumers.contains_key(id)
    }
}

impl Default for PowerGrid {
    /// Creates a default power grid with a "default" ID.
    ///
    /// This is primarily for testing and should not be used in production.
    fn default() -> Self {
        PowerGrid::new(String::from("default"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_grid_creation() {
        let grid = PowerGrid::new(String::from("grid1"));
        assert_eq!(grid.id(), "grid1");
        assert_eq!(grid.total_generation(), Watts::new(0));
        assert_eq!(grid.total_demand(), Watts::new(0));
        assert_eq!(grid.generator_count(), 0);
        assert_eq!(grid.consumer_count(), 0);
    }

    #[test]
    fn test_power_grid_add_generator() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        let generator1 = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        let result = grid.add_generator(generator1);
        assert!(result.is_ok());
        assert_eq!(grid.generator_count(), 1);
        assert!(grid.has_generator("gen1"));
        
        // Try to add duplicate generator
        let generator2 = Generator::new(String::from("gen1"), Watts::new(500)).unwrap();
        let result2 = grid.add_generator(generator2);
        assert!(matches!(result2, Err(PowerError::InvalidValue)));
        assert_eq!(grid.generator_count(), 1); // Should remain unchanged
    }

    #[test]
    fn test_power_grid_remove_generator() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        let generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        let _ = grid.add_generator(generator);
        assert_eq!(grid.generator_count(), 1);
        
        // Remove existing generator
        let result = grid.remove_generator("gen1");
        assert!(result.is_ok());
        assert_eq!(grid.generator_count(), 0);
        assert!(!grid.has_generator("gen1"));
        
        // Try to remove non-existent generator
        let result2 = grid.remove_generator("gen1");
        assert!(matches!(result2, Err(PowerError::InvalidValue)));
    }

    #[test]
    fn test_power_grid_add_consumer() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        let consumer1 = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let result = grid.add_consumer(consumer1);
        assert!(result.is_ok());
        assert_eq!(grid.consumer_count(), 1);
        assert!(grid.has_consumer("consumer1"));
        
        // Try to add duplicate consumer
        let consumer2 = Consumer::new(String::from("consumer1"), Watts::new(300), Watts::new(900)).unwrap();
        let result2 = grid.add_consumer(consumer2);
        assert!(matches!(result2, Err(PowerError::InvalidValue)));
        assert_eq!(grid.consumer_count(), 1); // Should remain unchanged
    }

    #[test]
    fn test_power_grid_remove_consumer() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        let consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let _ = grid.add_consumer(consumer);
        assert_eq!(grid.consumer_count(), 1);
        
        // Remove existing consumer
        let result = grid.remove_consumer("consumer1");
        assert!(result.is_ok());
        assert_eq!(grid.consumer_count(), 0);
        assert!(!grid.has_consumer("consumer1"));
        
        // Try to remove non-existent consumer
        let result2 = grid.remove_consumer("consumer1");
        assert!(matches!(result2, Err(PowerError::InvalidValue)));
    }

    #[test]
    fn test_power_grid_get_generator() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        let generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        let _ = grid.add_generator(generator);
        
        // Get existing generator
        let gen_opt = grid.get_generator("gen1");
        assert!(gen_opt.is_some());
        assert_eq!(gen_opt.unwrap().id(), "gen1");
        
        // Try to get non-existent generator
        let gen_opt2 = grid.get_generator("nonexistent");
        assert!(gen_opt2.is_none());
    }

    #[test]
    fn test_power_grid_get_consumer() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        let consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let _ = grid.add_consumer(consumer);
        
        // Get existing consumer
        let cons_opt = grid.get_consumer("consumer1");
        assert!(cons_opt.is_some());
        assert_eq!(cons_opt.unwrap().id(), "consumer1");
        
        // Try to get non-existent consumer
        let cons_opt2 = grid.get_consumer("nonexistent");
        assert!(cons_opt2.is_none());
    }

    #[test]
    fn test_power_grid_allocate_power_no_generation() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        // Add a consumer without any generators
        let consumer = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let _ = grid.add_consumer(consumer);
        
        // Enable the consumer
        let mut consumer_ref = grid.get_consumer_mut("consumer1").unwrap();
        consumer_ref.enable();
        
        // Allocate power - should return 0 since no generation
        let allocated = grid.allocate_power();
        assert_eq!(allocated, Watts::new(0));
        assert_eq!(grid.total_generation(), Watts::new(0));
        assert_eq!(grid.total_demand(), Watts::new(200));
    }

    #[test]
    fn test_power_grid_allocate_power_no_demand() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        // Add a generator without any consumers
        let generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        let _ = grid.add_generator(generator);
        
        // Enable the generator
        let mut gen_ref = grid.get_generator_mut("gen1").unwrap();
        gen_ref.enable();
        
        // Allocate power - should return 0 since no demand
        let allocated = grid.allocate_power();
        assert_eq!(allocated, Watts::new(0));
        assert_eq!(grid.total_generation(), Watts::new(1000));
        assert_eq!(grid.total_demand(), Watts::new(0));
    }

    #[test]
    fn test_power_grid_allocate_power_sufficient() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        // Add a generator
        let generator = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        let _ = grid.add_generator(generator);
        
        // Enable the generator
        let mut gen_ref = grid.get_generator_mut("gen1").unwrap();
        gen_ref.enable();
        
        // Add consumers
        let consumer1 = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let consumer2 = Consumer::new(String::from("consumer2"), Watts::new(300), Watts::new(600)).unwrap();
        let _ = grid.add_consumer(consumer1);
        let _ = grid.add_consumer(consumer2);
        
        // Enable consumers
        let mut cons_ref1 = grid.get_consumer_mut("consumer1").unwrap();
        let mut cons_ref2 = grid.get_consumer_mut("consumer2").unwrap();
        cons_ref1.enable();
        cons_ref2.enable();
        
        // Allocate power - should allocate full demand since generation is sufficient
        let allocated = grid.allocate_power();
        assert_eq!(allocated, Watts::new(1400)); // 800 + 600 (max consumption)
        assert_eq!(grid.total_generation(), Watts::new(1000));
        assert_eq!(grid.total_demand(), Watts::new(500)); // 200 + 300 (min requirements)
    }

    #[test]
    fn test_power_grid_allocate_power_insufficient() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        // Add a generator with limited capacity
        let generator = Generator::new(String::from("gen1"), Watts::new(500)).unwrap();
        let _ = grid.add_generator(generator);
        
        // Enable the generator
        let mut gen_ref = grid.get_generator_mut("gen1").unwrap();
        gen_ref.enable();
        
        // Add consumers with high demand
        let consumer1 = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let consumer2 = Consumer::new(String::from("consumer2"), Watts::new(300), Watts::new(600)).unwrap();
        let _ = grid.add_consumer(consumer1);
        let _ = grid.add_consumer(consumer2);
        
        // Enable consumers
        let mut cons_ref1 = grid.get_consumer_mut("consumer1").unwrap();
        let mut cons_ref2 = grid.get_consumer_mut("consumer2").unwrap();
        cons_ref1.enable();
        cons_ref2.enable();
        
        // Allocate power - should distribute proportionally based on minimum requirements
        let allocated = grid.allocate_power();
        assert_eq!(allocated, Watts::new(500)); // All available generation
        assert_eq!(grid.total_generation(), Watts::new(500));
        assert_eq!(grid.total_demand(), Watts::new(500)); // 200 + 300 (min requirements)
    }

    #[test]
    fn test_power_grid_calculate_total_generation() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        // Add generators
        let generator1 = Generator::new(String::from("gen1"), Watts::new(1000)).unwrap();
        let generator2 = Generator::new(String::from("gen2"), Watts::new(1500)).unwrap();
        let _ = grid.add_generator(generator1);
        let _ = grid.add_generator(generator2);
        
        // Enable one generator
        let mut gen_ref1 = grid.get_generator_mut("gen1").unwrap();
        let mut gen_ref2 = grid.get_generator_mut("gen2").unwrap();
        gen_ref1.enable();
        // gen2 remains disabled
        
        // Calculate total generation - should only include enabled generators
        let total = grid.calculate_total_generation();
        assert_eq!(total, Watts::new(1000));
    }

    #[test]
    fn test_power_grid_calculate_total_demand() {
        let mut grid = PowerGrid::new(String::from("grid1"));
        
        // Add consumers
        let consumer1 = Consumer::new(String::from("consumer1"), Watts::new(200), Watts::new(800)).unwrap();
        let consumer2 = Consumer::new(String::from("consumer2"), Watts::new(300), Watts::new(600)).unwrap();
        let _ = grid.add_consumer(consumer1);
        let _ = grid.add_consumer(consumer2);
        
        // Enable one consumer
        let mut cons_ref1 = grid.get_consumer_mut("consumer1").unwrap();
        let mut cons_ref2 = grid.get_consumer_mut("consumer2").unwrap();
        cons_ref1.enable();
        // cons2 remains disabled
        
        // Calculate total demand - should only include enabled consumers
        let total = grid.calculate_total_demand();
        assert_eq!(total, Watts::new(200));
    }

    #[test]
    fn test_power_grid_default() {
        let grid = PowerGrid::default();
        assert_eq!(grid.id(), "default");
        assert_eq!(grid.generator_count(), 0);
        assert_eq!(grid.consumer_count(), 0);
    }
}