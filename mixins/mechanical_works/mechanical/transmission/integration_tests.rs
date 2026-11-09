//! Integration tests for the mechanical transmission subsystem.
//!
//! These tests verify that all transmission components integrate properly 
//! with the common interfaces and execution model.

#[cfg(test)]
mod integration_tests {
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
    fn test_shaft_integration() {
        let component_id = ComponentId::new(1);
        let transmission_id = TransmissionId::new(1);
        
        let config = TransmissionConfiguration {
            max_rpm: 1000.0,
            max_torque: 50.0,
            max_power: 5000.0,
            enabled: true,
        };
        
        let mut shaft = Shaft::new(transmission_id, component_id, config);
        
        // Test that it implements the required traits
        assert_eq!(shaft.transmission_id(), transmission_id);
        assert!(shaft.is_enabled());
        
        // Test setting and getting values
        shaft.set_torque(Torque::new(25.0)).unwrap();
        shaft.set_rpm(RotationalSpeed::new(500.0)).unwrap();
        shaft.set_power(Power::new(2500.0)).unwrap();
        
        assert_eq!(shaft.current_torque(), Torque::new(25.0));
        assert_eq!(shaft.current_rpm(), RotationalSpeed::new(500.0));
        assert_eq!(shaft.current_power(), Power::new(2500.0));
        
        // Test input/output traits
        assert_eq!(shaft.max_input_torque(), Torque::new(50.0));
        assert_eq!(shaft.max_input_rpm(), RotationalSpeed::new(1000.0));
        
        // Test integrated transmission evaluation
        let input_state = TransmissionState::with_values(
            Torque::new(25.0),
            RotationalSpeed::new(500.0),
            Power::new(2500.0),
            true,
        );
        
        let output_state = shaft.evaluate(input_state).unwrap();
        assert_eq!(output_state.torque, Torque::new(25.0));
        assert_eq!(output_state.rpm, RotationalSpeed::new(500.0));
        assert_eq!(output_state.power, Power::new(2500.0));
        assert_eq!(output_state.enabled, true);
    }

    #[test]
    fn test_coupling_integration() {
        let component_id = ComponentId::new(2);
        let transmission_id = TransmissionId::new(2);
        
        let mut coupling = Coupling::new(transmission_id, component_id);
        
        // Test that it implements the required traits
        assert_eq!(coupling.transmission_id(), transmission_id);
        assert!(coupling.is_enabled());
        
        // Test basic functionality
        assert!(!coupling.is_connected());
        
        // Test setting and getting values
        coupling.set_enabled(false);
        assert!(!coupling.is_enabled());
    }

    #[test]
    fn test_disabled_transmission() {
        let component_id = ComponentId::new(3);
        let transmission_id = TransmissionId::new(3);
        
        let config = TransmissionConfiguration {
            max_rpm: 1000.0,
            max_torque: 50.0,
            max_power: 5000.0,
            enabled: false, // Disabled
        };
        
        let mut shaft = Shaft::new(transmission_id, component_id, config);
        
        // Test that it's disabled
        assert!(!shaft.is_enabled());
        
        // Test evaluation of disabled transmission should fail
        let input_state = TransmissionState::with_values(
            Torque::new(25.0),
            RotationalSpeed::new(500.0),
            Power::new(2500.0),
            false,
        );
        
        let result = shaft.evaluate(input_state);
        assert!(matches!(result, Err(TransmissionError::DisabledComponent)));
    }

    #[test]
    fn test_invalid_input_conditions() {
        let component_id = ComponentId::new(4);
        let transmission_id = TransmissionId::new(4);
        
        let config = TransmissionConfiguration {
            max_rpm: 1000.0,
            max_torque: 50.0,
            max_power: 5000.0,
            enabled: true,
        };
        
        let mut shaft = Shaft::new(transmission_id, component_id, config);
        
        // Test exceeding torque limit
        let input_state = TransmissionState::with_values(
            Torque::new(100.0), // Exceeds max_torque of 50.0
            RotationalSpeed::new(500.0),
            Power::new(2500.0),
            true,
        );
        
        let result = shaft.evaluate(input_state);
        assert!(matches!(result, Err(TransmissionError::ExceededTorque)));
    }

    #[test]
    fn test_component_identity() {
        let component_id = ComponentId::new(5);
        let transmission_id = TransmissionId::new(5);
        
        let config = TransmissionConfiguration {
            max_rpm: 1000.0,
            max_torque: 50.0,
            max_power: 5000.0,
            enabled: true,
        };
        
        let shaft = Shaft::new(transmission_id, component_id, config);
        
        // Test identity methods
        assert_eq!(shaft.transmission_id(), transmission_id);
        assert_eq!(shaft.component_id(), component_id);
    }
}