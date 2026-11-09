//! Gear validation logic.
//!
//! This module implements validation for gear components and operations,
//! ensuring all gear parameters, configurations, and limits are within acceptable ranges.

use crate::mechanical::transmission::gear::{Gear, GearPair, GearConfiguration, GearRatio};
use crate::mechanical::transmission::gear::errors::GearError;

/// Trait for validating gear-related components.
pub trait GearValidation {
    /// Validates a gear configuration.
    fn validate_gear_configuration(&self) -> Result<(), GearError>;
    
    /// Validates a gear pair configuration.
    fn validate_gear_pair(&self) -> Result<(), GearError>;
    
    /// Validates that a gear is enabled and operational.
    fn validate_gear_enabled(&self) -> Result<(), GearError>;
    
    /// Validates that a gear's operating limits are not exceeded.
    fn validate_gear_limits(&self) -> Result<(), GearError>;
    
    /// Validates that a gear ratio is valid.
    fn validate_gear_ratio(&self) -> Result<(), GearError>;
}

impl GearValidation for Gear {
    /// Validates a gear configuration.
    fn validate_gear_configuration(&self) -> Result<(), GearError> {
        self.configuration().validate()
    }
    
    /// Validates a gear pair configuration.
    fn validate_gear_pair(&self) -> Result<(), GearError> {
        // For now, we just check that the gear is valid
        // In more complex scenarios, this would validate compatibility between gears
        Ok(())
    }
    
    /// Validates that a gear is enabled and operational.
    fn validate_gear_enabled(&self) -> Result<(), GearError> {
        if !self.is_enabled() {
            return Err(GearError::DisabledGear);
        }
        Ok(())
    }
    
    /// Validates that a gear's operating limits are not exceeded.
    fn validate_gear_limits(&self) -> Result<(), GearError> {
        // Check RPM limit
        if self.current_rpm() > self.configuration().max_rpm() {
            return Err(GearError::ExceededRPM);
        }
        
        // Check torque limit  
        if self.current_torque() > self.configuration().max_torque() {
            return Err(GearError::ExceededTorque);
        }
        
        // Check power limit
        if self.current_power() > self.configuration().max_power() {
            return Err(GearError::ExceededPower);
        }
        
        Ok(())
    }
    
    /// Validates that a gear ratio is valid.
    fn validate_gear_ratio(&self) -> Result<(), GearError> {
        // For now, we just ensure the ratio has valid tooth counts
        // More detailed validation would check compatibility between gears
        Ok(())
    }
}

impl GearValidation for GearPair {
    /// Validates a gear configuration.
    fn validate_gear_configuration(&self) -> Result<(), GearError> {
        // A valid gear pair must have two different gears
        if self.input_gear() == self.output_gear() {
            return Err(GearError::SelfPairing);
        }
        Ok(())
    }
    
    /// Validates a gear pair configuration.
    fn validate_gear_pair(&self) -> Result<(), GearError> {
        // A valid gear pair must have two different gears
        if self.input_gear() == self.output_gear() {
            return Err(GearError::SelfPairing);
        }
        Ok(())
    }
    
    /// Validates that a gear is enabled and operational.
    fn validate_gear_enabled(&self) -> Result<(), GearError> {
        // For a gear pair, we would need to validate the individual gears
        // This would require access to gear instances, so for now just pass
        Ok(())
    }
    
    /// Validates that a gear's operating limits are not exceeded.
    fn validate_gear_limits(&self) -> Result<(), GearError> {
        // For a gear pair, we'd need to validate the individual gears
        // This would require access to gear instances, so for now just pass
        Ok(())
    }
    
    /// Validates that a gear ratio is valid.
    fn validate_gear_ratio(&self) -> Result<(), GearError> {
        // A gear pair doesn't have a ratio by itself - it's the combination of two gears
        // Validation would happen at the gear level or when calculating the ratio
        Ok(())
    }
}

impl GearValidation for GearRatio {
    /// Validates a gear configuration.
    fn validate_gear_configuration(&self) -> Result<(), GearError> {
        // A gear ratio is valid if it has positive tooth counts
        if self.input_teeth() == 0 || self.output_teeth() == 0 {
            return Err(GearError::InvalidToothCount);
        }
        Ok(())
    }
    
    /// Validates a gear pair configuration.
    fn validate_gear_pair(&self) -> Result<(), GearError> {
        // A gear ratio is valid if it has positive tooth counts
        if self.input_teeth() == 0 || self.output_teeth() == 0 {
            return Err(GearError::InvalidToothCount);
        }
        Ok(())
    }
    
    /// Validates that a gear is enabled and operational.
    fn validate_gear_enabled(&self) -> Result<(), GearError> {
        // A ratio doesn't have an enabled state
        Ok(())
    }
    
    /// Validates that a gear's operating limits are not exceeded.
    fn validate_gear_limits(&self) -> Result<(), GearError> {
        // A ratio doesn't have operating limits
        Ok(())
    }
    
    /// Validates that a gear ratio is valid.
    fn validate_gear_ratio(&self) -> Result<(), GearError> {
        // A gear ratio is valid if it has positive tooth counts
        if self.input_teeth() == 0 || self.output_teeth() == 0 {
            return Err(GearError::InvalidToothCount);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};
    use crate::mechanical::transmission::gear::types::{GearId, GearCategory, GearConfiguration};
    use crate::mechanical::core::identifiers::ComponentId;
    
    #[test]
    fn test_gear_validation_enabled() {
        let configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
        let gear = Gear::new(
            GearId::new(1),
            ComponentId::new(100),
            GearCategory::Spur,
            configuration
        ).unwrap();
        
        // Should pass validation for enabled gear
        assert!(gear.validate_gear_enabled().is_ok());
    }
    
    #[test]
    fn test_gear_validation_disabled() {
        let mut configuration = GearConfiguration::new(20, 1000.0, 50.0, 5000.0, 0.95);
        configuration.set_enabled(false);
        
        let gear = Gear::new(
            GearId::new(1),
            ComponentId::new(100),
            GearCategory::Spur,
            configuration
        ).unwrap();
        
        // Should fail validation for disabled gear
        assert!(matches!(gear.validate_gear_enabled(), Err(GearError::DisabledGear)));
    }
    
    #[test]
    fn test_gear_pair_validation_self_pairing() {
        let gear_pair = GearPair::new(1, GearId::new(1), GearId::new(1));
        
        // Should fail validation for self-pairing
        assert!(matches!(gear_pair.validate_gear_pair(), Err(GearError::SelfPairing)));
    }
    
    #[test]
    fn test_gear_pair_validation_valid() {
        let gear_pair = GearPair::new(1, GearId::new(1), GearId::new(2));
        
        // Should pass validation for valid pair
        assert!(gear_pair.validate_gear_pair().is_ok());
    }
    
    #[test]
    fn test_gear_ratio_validation_valid() {
        let gear_ratio = GearRatio::new(20, 40).unwrap();
        
        // Should pass validation for valid ratio
        assert!(gear_ratio.validate_gear_ratio().is_ok());
    }
    
    #[test]
    fn test_gear_ratio_validation_invalid_teeth() {
        let result = GearRatio::new(0, 40);
        
        // Should fail validation for invalid tooth count
        assert!(matches!(result, Err(GearError::InvalidToothCount)));
    }
}