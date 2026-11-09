//! Network-specific error handling for mechanical systems.
//!
//! This module defines all error variants that can occur during network operations,
//! providing meaningful feedback for failed network management tasks.

use std::fmt;

/// A result type alias for network operations.
pub type NetworkResult<T> = Result<T, NetworkError>;

/// Errors that can occur when managing mechanical networks.
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkError {
    /// Indicates that a network with the specified ID already exists.
    DuplicateNetworkId,
    
    /// Indicates that a network with the specified ID does not exist.
    MissingNetwork,
    
    /// Indicates that a node with the specified ID already exists in the network.
    DuplicateNode,
    
    /// Indicates that a node with the specified ID does not exist in the network.
    MissingNode,
    
    /// Indicates that a connection with the specified ID already exists in the network.
    DuplicateConnection,
    
    /// Indicates that a connection with the specified ID does not exist in the network.
    MissingConnection,
    
    /// Indicates that one or both endpoints of a connection are invalid.
    InvalidConnectionEndpoints,
    
    /// Indicates that a node is attempting to be registered in a different network.
    ConflictingNetworkMembership,
    
    /// Indicates that a network topology is invalid.
    InvalidTopology,
    
    /// Indicates that a network operation failed due to an internal inconsistency.
    InternalError,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::DuplicateNetworkId => write!(f, "Network ID already exists"),
            NetworkError::MissingNetwork => write!(f, "Network does not exist"),
            NetworkError::DuplicateNode => write!(f, "Node already exists in network"),
            NetworkError::MissingNode => write!(f, "Node does not exist in network"),
            NetworkError::DuplicateConnection => write!(f, "Connection already exists in network"),
            NetworkError::MissingConnection => write!(f, "Connection does not exist in network"),
            NetworkError::InvalidConnectionEndpoints => write!(f, "Invalid connection endpoints"),
            NetworkError::ConflictingNetworkMembership => write!(f, "Node has conflicting network membership"),
            NetworkError::InvalidTopology => write!(f, "Network topology is invalid"),
            NetworkError::InternalError => write!(f, "Internal network error occurred"),
        }
    }
}

impl std::error::Error for NetworkError {}