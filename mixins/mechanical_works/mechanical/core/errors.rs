//! Shared error definitions for core mechanical operations.
//!
//! This module provides a consistent foundation for errors originating from 
//! core operations in the mechanical system.

use std::fmt;

/// A generic error type for core mechanical operations.
///
/// This enum represents various error conditions that can occur during
/// core mechanical system operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    /// An invalid identifier was provided.
    InvalidIdentifier,
    
    /// An invalid physical quantity was provided.
    InvalidPhysicalQuantity,
    
    /// An invalid connection was attempted.
    InvalidConnection,
    
    /// Components are incompatible for the specified operation.
    IncompatibleComponents,
    
    /// An invalid state transition was attempted.
    InvalidStateTransition,
    
    /// A mathematical or arithmetic error occurred (e.g., division by zero).
    ArithmeticError,
    
    /// A conversion error occurred between different units or types.
    ConversionError,
    
    /// A resource was not found.
    ResourceNotFound,
    
    /// An operation failed due to a system constraint.
    ConstraintViolation,
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::InvalidIdentifier => write!(f, "Invalid identifier provided"),
            CoreError::InvalidPhysicalQuantity => write!(f, "Invalid physical quantity provided"),
            CoreError::InvalidConnection => write!(f, "Invalid connection attempted"),
            CoreError::IncompatibleComponents => write!(f, "Components are incompatible"),
            CoreError::InvalidStateTransition => write!(f, "Invalid state transition attempted"),
            CoreError::ArithmeticError => write!(f, "Mathematical or arithmetic error occurred"),
            CoreError::ConversionError => write!(f, "Conversion error occurred"),
            CoreError::ResourceNotFound => write!(f, "Required resource was not found"),
            CoreError::ConstraintViolation => write!(f, "Operation violated system constraints"),
        }
    }
}

impl std::error::Error for CoreError {}

/// A specialized error for identifier-related issues.
#[derive(Debug, Clone, PartialEq)]
pub enum IdentifierError {
    /// The component ID is invalid (e.g., zero or out of range).
    InvalidComponentId,
    
    /// The network ID is invalid.
    InvalidNetworkId,
    
    /// The connection ID is invalid.
    InvalidConnectionId,
    
    /// The node ID is invalid.
    InvalidNodeId,
    
    /// An identifier was not found in the system.
    IdentifierNotFound,
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentifierError::InvalidComponentId => write!(f, "Invalid component ID"),
            IdentifierError::InvalidNetworkId => write!(f, "Invalid network ID"),
            IdentifierError::InvalidConnectionId => write!(f, "Invalid connection ID"),
            IdentifierError::InvalidNodeId => write!(f, "Invalid node ID"),
            IdentifierError::IdentifierNotFound => write!(f, "Identifier not found in system"),
        }
    }
}

impl std::error::Error for IdentifierError {}

/// A specialized error for physical quantity-related issues.
#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalQuantityError {
    /// The torque value is invalid (e.g., negative or NaN).
    InvalidTorque,
    
    /// The angular velocity value is invalid.
    InvalidAngularVelocity,
    
    /// The rotational speed value is invalid.
    InvalidRotationalSpeed,
    
    /// The power value is invalid.
    InvalidPower,
    
    /// The energy value is invalid.
    InvalidEnergy,
    
    /// The efficiency value is outside the valid range.
    InvalidEfficiency,
    
    /// A calculation involving physical quantities failed.
    CalculationError,
}

impl fmt::Display for PhysicalQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicalQuantityError::InvalidTorque => write!(f, "Invalid torque value"),
            PhysicalQuantityError::InvalidAngularVelocity => write!(f, "Invalid angular velocity value"),
            PhysicalQuantityError::InvalidRotationalSpeed => write!(f, "Invalid rotational speed value"),
            PhysicalQuantityError::InvalidPower => write!(f, "Invalid power value"),
            PhysicalQuantityError::InvalidEnergy => write!(f, "Invalid energy value"),
            PhysicalQuantityError::InvalidEfficiency => write!(f, "Invalid efficiency value"),
            PhysicalQuantityError::CalculationError => write!(f, "Physical quantity calculation error"),
        }
    }
}

impl std::error::Error for PhysicalQuantityError {}

/// A specialized error for connection-related issues.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionError {
    /// The connection is invalid (e.g., self-connection).
    InvalidConnection,
    
    /// Components are incompatible for the specified connection type.
    IncompatibleComponents,
    
    /// The connection already exists in the system.
    ConnectionExists,
    
    /// The connection was not found in the system.
    ConnectionNotFound,
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::InvalidConnection => write!(f, "Invalid connection specified"),
            ConnectionError::IncompatibleComponents => write!(f, "Components are incompatible for this connection"),
            ConnectionError::ConnectionExists => write!(f, "Connection already exists in system"),
            ConnectionError::ConnectionNotFound => write!(f, "Connection not found in system"),
        }
    }
}

impl std::error::Error for ConnectionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_error_display() {
        assert_eq!(format!("{}", CoreError::InvalidIdentifier), "Invalid identifier provided");
        assert_eq!(format!("{}", CoreError::InvalidPhysicalQuantity), "Invalid physical quantity provided");
        assert_eq!(format!("{}", CoreError::InvalidConnection), "Invalid connection attempted");
        assert_eq!(format!("{}", CoreError::IncompatibleComponents), "Components are incompatible");
        assert_eq!(format!("{}", CoreError::InvalidStateTransition), "Invalid state transition attempted");
        assert_eq!(format!("{}", CoreError::ArithmeticError), "Mathematical or arithmetic error occurred");
        assert_eq!(format!("{}", CoreError::ConversionError), "Conversion error occurred");
        assert_eq!(format!("{}", CoreError::ResourceNotFound), "Required resource was not found");
        assert_eq!(format!("{}", CoreError::ConstraintViolation), "Operation violated system constraints");
    }

    #[test]
    fn test_identifier_error_display() {
        assert_eq!(format!("{}", IdentifierError::InvalidComponentId), "Invalid component ID");
        assert_eq!(format!("{}", IdentifierError::InvalidNetworkId), "Invalid network ID");
        assert_eq!(format!("{}", IdentifierError::InvalidConnectionId), "Invalid connection ID");
        assert_eq!(format!("{}", IdentifierError::InvalidNodeId), "Invalid node ID");
        assert_eq!(format!("{}", IdentifierError::IdentifierNotFound), "Identifier not found in system");
    }

    #[test]
    fn test_physical_quantity_error_display() {
        assert_eq!(format!("{}", PhysicalQuantityError::InvalidTorque), "Invalid torque value");
        assert_eq!(format!("{}", PhysicalQuantityError::InvalidAngularVelocity), "Invalid angular velocity value");
        assert_eq!(format!("{}", PhysicalQuantityError::InvalidRotationalSpeed), "Invalid rotational speed value");
        assert_eq!(format!("{}", PhysicalQuantityError::InvalidPower), "Invalid power value");
        assert_eq!(format!("{}", PhysicalQuantityError::InvalidEnergy), "Invalid energy value");
        assert_eq!(format!("{}", PhysicalQuantityError::InvalidEfficiency), "Invalid efficiency value");
        assert_eq!(format!("{}", PhysicalQuantityError::CalculationError), "Physical quantity calculation error");
    }

    #[test]
    fn test_connection_error_display() {
        assert_eq!(format!("{}", ConnectionError::InvalidConnection), "Invalid connection specified");
        assert_eq!(format!("{}", ConnectionError::IncompatibleComponents), "Components are incompatible for this connection");
        assert_eq!(format!("{}", ConnectionError::ConnectionExists), "Connection already exists in system");
        assert_eq!(format!("{}", ConnectionError::ConnectionNotFound), "Connection not found in system");
    }
}