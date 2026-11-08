//! Integration tests for core mechanical components.

#[cfg(test)]
mod tests {
    use crate::mechanical::core::{identifiers, units, components, connections, state, errors};

    #[test]
    fn test_core_module_imports() {
        // Test that all modules can be imported and used
        let _ = identifiers::ComponentId::new(1);
        let _ = units::Torque::new(10.0);
        let _ = components::ComponentCategory::Generator;
        let _ = connections::ConnectionType::Rotational;
        let _ = state::OperationalState::Idle;
        let _ = errors::CoreError::InvalidIdentifier;
    }
}