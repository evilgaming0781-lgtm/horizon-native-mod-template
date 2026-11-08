//! Shared mechanical connection definitions.
//!
//! This module provides the foundation for connections between mechanical components.

use std::fmt;

/// A connection type that defines compatibility between components.
///
/// Connection types determine what kinds of components can be connected to each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionType {
    /// A rotational connection that transmits torque and angular velocity.
    Rotational,
    
    /// A linear connection that transmits force and displacement.
    Linear,
    
    /// An electrical connection for power transmission.
    Electrical,
    
    /// A fluid connection for liquid or gas transfer.
    Fluid,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionType::Rotational => write!(f, "Rotational"),
            ConnectionType::Linear => write!(f, "Linear"),
            ConnectionType::Electrical => write!(f, "Electrical"),
            ConnectionType::Fluid => write!(f, "Fluid"),
        }
    }
}

/// A connection between two mechanical components.
///
/// This structure represents a specific connection in the mechanical system,
/// tracking which components are connected and what type of connection exists.
#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    /// Unique identifier for this connection.
    pub id: crate::mechanical::core::identifiers::ConnectionId,
    
    /// The first component in the connection.
    pub component_a: crate::mechanical::core::identifiers::ComponentId,
    
    /// The second component in the connection.
    pub component_b: crate::mechanical::core::identifiers::ComponentId,
    
    /// The type of connection between the components.
    pub connection_type: ConnectionType,
    
    /// Optional description of this connection.
    pub description: Option<String>,
}

impl Connection {
    /// Creates a new connection between two components.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this connection
    /// * `component_a` - First component in the connection
    /// * `component_b` - Second component in the connection
    /// * `connection_type` - Type of connection between components
    pub fn new(
        id: crate::mechanical::core::identifiers::ConnectionId,
        component_a: crate::mechanical::core::identifiers::ComponentId,
        component_b: crate::mechanical::core::identifiers::ComponentId,
        connection_type: ConnectionType,
    ) -> Self {
        Connection {
            id,
            component_a,
            component_b,
            connection_type,
            description: None,
        }
    }
    
    /// Creates a new connection with an optional description.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this connection
    /// * `component_a` - First component in the connection
    /// * `component_b` - Second component in the connection
    /// * `connection_type` - Type of connection between components
    /// * `description` - Optional description of this connection
    pub fn with_description(
        id: crate::mechanical::core::identifiers::ConnectionId,
        component_a: crate::mechanical::core::identifiers::ComponentId,
        component_b: crate::mechanical::core::identifiers::ComponentId,
        connection_type: ConnectionType,
        description: String,
    ) -> Self {
        Connection {
            id,
            component_a,
            component_b,
            connection_type,
            description: Some(description),
        }
    }
    
    /// Checks if this connection is valid (components are different and not self-referential).
    ///
    /// # Returns
    /// True if the connection is valid, false otherwise
    pub fn is_valid(&self) -> bool {
        // A valid connection must have two different components
        self.component_a != self.component_b
    }
    
    /// Gets the first component in this connection.
    pub fn component_a(&self) -> crate::mechanical::core::identifiers::ComponentId {
        self.component_a
    }
    
    /// Gets the second component in this connection.
    pub fn component_b(&self) -> crate::mechanical::core::identifiers::ComponentId {
        self.component_b
    }
    
    /// Gets the connection type.
    pub fn connection_type(&self) -> ConnectionType {
        self.connection_type
    }
    
    /// Gets the connection ID.
    pub fn id(&self) -> crate::mechanical::core::identifiers::ConnectionId {
        self.id
    }
}

/// A connection validation error.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionError {
    /// Indicates that a connection was attempted between the same component (self-connection).
    SelfConnection,
    
    /// Indicates that components are incompatible for the specified connection type.
    IncompatibleComponents,
    
    /// Indicates an invalid connection configuration.
    InvalidConfiguration,
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::SelfConnection => write!(f, "Cannot connect a component to itself"),
            ConnectionError::IncompatibleComponents => write!(f, "Components are incompatible for this connection type"),
            ConnectionError::InvalidConfiguration => write!(f, "Invalid connection configuration"),
        }
    }
}

impl std::error::Error for ConnectionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_type_display() {
        assert_eq!(format!("{}", ConnectionType::Rotational), "Rotational");
        assert_eq!(format!("{}", ConnectionType::Linear), "Linear");
        assert_eq!(format!("{}", ConnectionType::Electrical), "Electrical");
        assert_eq!(format!("{}", ConnectionType::Fluid), "Fluid");
    }

    #[test]
    fn test_connection_creation() {
        let id = crate::mechanical::core::identifiers::ConnectionId::new(1);
        let component_a = crate::mechanical::core::identifiers::ComponentId::new(10);
        let component_b = crate::mechanical::core::identifiers::ComponentId::new(20);
        
        let connection = Connection::new(
            id,
            component_a,
            component_b,
            ConnectionType::Rotational,
        );
        
        assert_eq!(connection.id(), id);
        assert_eq!(connection.component_a(), component_a);
        assert_eq!(connection.component_b(), component_b);
        assert_eq!(connection.connection_type(), ConnectionType::Rotational);
        assert_eq!(connection.description, None);
    }

    #[test]
    fn test_connection_with_description() {
        let id = crate::mechanical::core::identifiers::ConnectionId::new(1);
        let component_a = crate::mechanical::core::identifiers::ComponentId::new(10);
        let component_b = crate::mechanical::core::identifiers::ComponentId::new(20);
        
        let connection = Connection::with_description(
            id,
            component_a,
            component_b,
            ConnectionType::Rotational,
            "Main transmission shaft".to_string(),
        );
        
        assert_eq!(connection.description, Some("Main transmission shaft".to_string()));
    }

    #[test]
    fn test_connection_validity() {
        let id = crate::mechanical::core::identifiers::ConnectionId::new(1);
        let component_a = crate::mechanical::core::identifiers::ComponentId::new(10);
        let component_b = crate::mechanical::core::identifiers::ComponentId::new(20);
        
        let valid_connection = Connection::new(
            id,
            component_a,
            component_b,
            ConnectionType::Rotational,
        );
        
        assert!(valid_connection.is_valid());
        
        // Test invalid connection (self-connection)
        let self_connection = Connection::new(
            id,
            component_a,
            component_a,
            ConnectionType::Rotational,
        );
        
        assert!(!self_connection.is_valid());
    }

    #[test]
    fn test_connection_error_display() {
        let error = ConnectionError::SelfConnection;
        assert_eq!(format!("{}", error), "Cannot connect a component to itself");
        
        let error = ConnectionError::IncompatibleComponents;
        assert_eq!(format!("{}", error), "Components are incompatible for this connection type");
        
        let error = ConnectionError::InvalidConfiguration;
        assert_eq!(format!("{}", error), "Invalid connection configuration");
    }
}