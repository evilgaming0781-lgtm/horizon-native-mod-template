//! Validation functions for belt transmission systems.
//!
//! This module provides validation logic for belt configurations, pulleys, and transmissions.

use crate::mechanical::transmission::belt::{pair::BeltPair, pulley::Pulley, types::BeltConfiguration};
use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};

/// Trait for validating belt components.
pub trait BeltValidation {
    /// Validates a belt configuration.
    fn validate_configuration(&self) -> Result<(), BeltError>;
    
    /// Validates a pulley.
    fn validate_pulley(&self) -> Result<(), BeltError>;
    
    /// Validates a belt pair.
    fn validate_pair(&self) -> Result<(), BeltError>;
}

/// Errors that can occur during belt validation.
#[derive(Debug, Clone, PartialEq)]
pub enum BeltError {
    /// Invalid pulley radius (zero or negative).
    InvalidRadius,
    
    /// Invalid efficiency value (outside 0.0 to 1.0 range).
    InvalidEfficiency,
    
    /// Invalid configuration values.
    InvalidConfiguration,
    
    /// Invalid torque value.
    InvalidTorque,
    
    /// Invalid RPM value.
    InvalidRpm,
    
    /// Invalid power value.
    InvalidPower,
    
    /// Invalid ratio calculation.
    InvalidRatio,
    
    /// Invalid pulley pair (same pulley used for input and output).
    InvalidPair,
    
    /// Belt is disabled.
    DisabledBelt,
}

impl std::fmt::Display for BeltError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeltError::InvalidRadius => write!(f, "Invalid pulley radius"),
            BeltError::InvalidEfficiency => write!(f, "Invalid efficiency value"),
            BeltError::InvalidConfiguration => write!(f, "Invalid configuration values"),
            BeltError::InvalidTorque => write!(f, "Invalid torque value"),
            BeltError::InvalidRpm => write!(f, "Invalid RPM value"),
            BeltError::InvalidPower => write!(f, "Invalid power value"),
            BeltError::InvalidRatio => write!(f, "Invalid ratio calculation"),
            BeltError::InvalidPair => write!(f, "Invalid pulley pair (same pulley used for input and output)"),
            BeltError::DisabledBelt => write!(f, "Belt is disabled"),
        }
    }
}

impl std::error::Error for BeltError {}

/// Result type for belt operations.
pub type BeltResult<T> = Result<T, BeltError>;

impl BeltValidation for BeltConfiguration {
    fn validate_configuration(&self) -> Result<(), BeltError> {
        // Validate radius (should be positive)
        if self.radius <= 0.0 {
            return Err(BeltError::InvalidRadius);
        }
        
        // Validate maximum RPM
        if self.max_rpm < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate maximum torque
        if self.max_torque < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate maximum power
        if self.max_power < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate efficiency (should be between 0 and 1)
        if self.efficiency < 0.0 || self.efficiency > 1.0 {
            return Err(BeltError::InvalidEfficiency);
        }
        
        Ok(())
    }
}

impl BeltValidation for Pulley {
    fn validate_pulley(&self) -> Result<(), BeltError> {
        // Validate radius (should be positive)
        if self.configuration.radius <= 0.0 {
            return Err(BeltError::InvalidRadius);
        }
        
        // Validate maximum RPM
        if self.configuration.max_rpm < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate maximum torque
        if self.configuration.max_torque < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate maximum power
        if self.configuration.max_power < 0.0 {
            return Err(BeltError::InvalidConfiguration);
        }
        
        // Validate efficiency (should be between 0 and 1)
        if self.configuration.efficiency < 0.0 || self.configuration.efficiency > 1.0 {
            return Err(BeltError::InvalidEfficiency);
        }
        
        Ok(())
    }
}

impl BeltValidation for BeltPair {
    fn validate_pair(&self) -> Result<(), BeltError> {
        // Check that input and output pulleys are different
        if self.input_pulley.id == self.output_pulley.id {
            return Err(BeltError::InvalidPair);
        }
        
        // Validate both pulleys
        self.input_pulley.validate_pulley()?;
        self.output_pulley.validate_pulley()?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::transmission::belt::types::{BeltId, BeltCategory};

    #[test]
    fn test_valid_configuration_validation() {
        let config = BeltConfiguration::new(
            0.10, // radius
            1000.0, // max_rpm
            100.0, // max_torque
            1000.0, // max_power
            0.95, // efficiency
            BeltCategory::OpenBelt,
        );
        
        assert!(config.validate_configuration().is_ok());
    }

    #[test]
    fn test_invalid_radius_validation() {
        let config = BeltConfiguration::new(
            -0.10, // invalid negative radius
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        );
        
        assert!(matches!(config.validate_configuration(), Err(BeltError::InvalidRadius)));
    }

    #[test]
    fn test_invalid_efficiency_validation() {
        let config = BeltConfiguration::new(
            0.10,
            1000.0,
            100.0,
            1000.0,
            1.5, // invalid efficiency > 1.0
            BeltCategory::OpenBelt,
        );
        
        assert!(matches!(config.validate_configuration(), Err(BeltError::InvalidEfficiency)));
    }

    #[test]
    fn test_valid_pulley_validation() {
        let config = BeltConfiguration::new(
            0.10,
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        );
        
        let pulley = Pulley::new(BeltId::new(1), config);
        assert!(pulley.validate_pulley().is_ok());
    }

    #[test]
    fn test_invalid_pulley_radius_validation() {
        let config = BeltConfiguration::new(
            -0.10, // invalid negative radius
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        );
        
        let pulley = Pulley::new(BeltId::new(1), config);
        assert!(matches!(pulley.validate_pulley(), Err(BeltError::InvalidRadius)));
    }

    #[test]
    fn test_valid_pair_validation() {
        let input_config = BeltConfiguration::new(
            0.10,
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        );
        
        let output_config = BeltConfiguration::new(
            0.20,
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        );
        
        let input_pulley = Pulley::new(BeltId::new(1), input_config);
        let output_pulley = Pulley::new(BeltId::new(2), output_config);
        let pair = BeltPair::new(input_pulley, output_pulley);
        
        assert!(pair.validate_pair().is_ok());
    }

    #[test]
    fn test_invalid_pair_validation() {
        let config = BeltConfiguration::new(
            0.10,
            1000.0,
            100.0,
            1000.0,
            0.95,
            BeltCategory::OpenBelt,
        );
        
        let pulley = Pulley::new(BeltId::new(1), config);
        let pair = BeltPair::new(pulley.clone(), pulley);
        
        assert!(matches!(pair.validate_pair(), Err(BeltError::InvalidPair)));
    }
}