//! Unit tests for the mechanical transmission subsystem.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::transmission::{
        types::{TransmissionId, TransmissionCategory, TransmissionConfiguration},
        component::TransmissionComponent,
        interfaces::{RotationalInput, RotationalOutput, RotationalTransmission},
        integration::{IntegratedTransmission, SetTransmissionState, TransmissionState},
        errors::TransmissionError,
        shaft::Shaft,
        coupling::Coupling,
    };
    use crate::mechanical::transmission::gear::{
        gear::Gear,
        types::{GearId, GearCategory, GearConfiguration},
    };
    use crate::mechanical::core::identifiers::ComponentId;
    use crate::mechanical::core::units::{Torque, RotationalSpeed, Power};

    #[test]
    fn test_shaft_creation() {
        let component_id = ComponentId::new(1);
        let transmission_id = TransmissionId::new(1);
        
        let config = TransmissionConfiguration {
            max_rpm: 1000.0,
            max_torque: 50.0,
            max_power: 5000.0,
            enabled: true,
        };
        
        let shaft = Shaft::new(transmission_id, component_id, config);
        
        // Test that it implements the required traits
        assert_eq!(shaft.transmission_id(), transmission_id);
        assert!(shaft.is_enabled());
    }

    #[test]
    fn test_coupling_creation() {
        let component_id = ComponentId::new(2);
        let transmission_id = TransmissionId::new(2);
        
        let coupling = Coupling::new(transmission_id, component_id);
        
        // Test that it implements the required traits
        assert_eq!(coupling.transmission_id(), transmission_id);
        assert!(coupling.is_enabled());
    }
    
    #[test]
    fn test_transmission_module_imports() {
        // This is just a placeholder to ensure the module compiles correctly
        assert!(true);
    }
}

#[cfg(test)]
mod evaluation_tests {
    use super::*;
    // Tests are in their own file now
}