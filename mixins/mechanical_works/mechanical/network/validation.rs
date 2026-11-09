//! Network validation for mechanical systems.
//!
//! This module provides validation logic for ensuring mechanical network integrity,
//! checking for duplicate entries, missing references, and other structural issues.

use std::collections::{HashMap, HashSet};
use crate::mechanical::core::identifiers::{NetworkId, NodeId, ComponentId, ConnectionId};
use crate::mechanical::core::connections::Connection;
use crate::mechanical::network::network::MechanicalNetwork;
use crate::mechanical::network::errors::{NetworkError, NetworkResult};

/// Validation results for mechanical networks.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    /// Indicates if the network is valid.
    pub is_valid: bool,
    
    /// List of validation errors found.
    pub errors: Vec<NetworkError>,
    
    /// List of validation warnings or observations.
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Creates a new empty validation result.
    pub fn new() -> Self {
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    /// Adds an error to the validation result.
    pub fn add_error(&mut self, error: NetworkError) {
        self.is_valid = false;
        self.errors.push(error);
    }
    
    /// Adds a warning to the validation result.
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// Validation utilities for mechanical networks.
///
/// This structure provides methods for validating network state and detecting
/// structural issues that may affect network operation.
#[derive(Debug, Clone)]
pub struct NetworkValidation;

impl NetworkValidation {
    /// Validates a single mechanical network.
    ///
    /// # Arguments
    /// * `network` - The network to validate
    ///
    /// # Returns
    /// A `ValidationResult` indicating the validity of the network and any issues found
    pub fn validate_network(network: &MechanicalNetwork) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check for duplicate nodes (shouldn't happen with HashSet, but good to verify)
        let mut seen_nodes = HashSet::new();
        for node_id in network.nodes() {
            if !seen_nodes.insert(node_id) {
                result.add_error(NetworkError::DuplicateNode);
            }
        }
        
        // Check for duplicate connections
        let mut seen_connections = HashSet::new();
        for connection in network.connections() {
            if !seen_connections.insert(connection.id()) {
                result.add_error(NetworkError::DuplicateConnection);
            }
        }
        
        // Validate each connection's endpoints
        for connection in network.connections() {
            let component_a = connection.component_a();
            let component_b = connection.component_b();
            
            // Check for self-connections (should not happen with valid connections)
            if component_a == component_b {
                result.add_error(NetworkError::InvalidConnectionEndpoints);
            }
            
            // Validate that endpoints exist in the network
            let node_a = NodeId::new(component_a.get());
            let node_b = NodeId::new(component_b.get());
            
            if !network.has_node(node_a) || !network.has_node(node_b) {
                result.add_error(NetworkError::InvalidConnectionEndpoints);
            }
        }
        
        // Check for orphaned connections (connections that reference non-existent endpoints)
        // This is already handled above, but we check explicitly here for completeness
        let mut connection_endpoints: HashSet<(NodeId, NodeId)> = HashSet::new();
        for connection in network.connections() {
            let component_a = connection.component_a();
            let component_b = connection.component_b();
            let node_a = NodeId::new(component_a.get());
            let node_b = NodeId::new(component_b.get());
            
            // Check if both endpoints exist
            if !network.has_node(node_a) || !network.has_node(node_b) {
                result.add_error(NetworkError::InvalidConnectionEndpoints);
            }
            
            connection_endpoints.insert((node_a, node_b));
        }
        
        result
    }
    
    /// Validates the consistency between a network and its registry.
    ///
    /// # Arguments
    /// * `network` - The network to validate
    /// * `registry` - The registry containing this network
    ///
    /// # Returns
    /// A `ValidationResult` indicating the validity of the network-registry relationship
    pub fn validate_network_registry_consistency(
        network: &MechanicalNetwork,
        registry: &crate::mechanical::network::registry::NetworkRegistry,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Verify that the network exists in the registry
        if !registry.has_network(network.id()) {
            result.add_error(NetworkError::MissingNetwork);
        }
        
        // Check for node membership consistency
        for node_id in network.nodes() {
            if let Some(node_network) = registry.get_node_network(*node_id) {
                if node_network != network.id() {
                    result.add_error(NetworkError::ConflictingNetworkMembership);
                }
            } else {
                result.add_error(NetworkError::MissingNode);
            }
        }
        
        // Check that all nodes in the registry belong to this network
        for (node_id, &network_id) in registry.nodes() {
            if network_id == network.id() && !network.has_node(*node_id) {
                result.add_error(NetworkError::MissingNode);
            }
        }
        
        result
    }
    
    /// Validates that a collection of networks maintains proper integrity.
    ///
    /// # Arguments
    /// * `networks` - Collection of networks to validate
    ///
    /// # Returns
    /// A `ValidationResult` indicating the validity of all networks
    pub fn validate_network_collection(networks: &[&MechanicalNetwork]) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check for duplicate network IDs
        let mut seen_network_ids = HashSet::new();
        for network in networks {
            if !seen_network_ids.insert(network.id()) {
                result.add_error(NetworkError::DuplicateNetworkId);
            }
        }
        
        // Validate each individual network
        for network in networks {
            let network_result = Self::validate_network(network);
            if !network_result.is_valid {
                result.is_valid = false;
                result.errors.extend(network_result.errors);
                result.warnings.extend(network_result.warnings);
            }
        }
        
        result
    }
    
    /// Validates that a node can be safely registered in a network.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to validate
    /// * `network_id` - The ID of the network it would be registered in
    /// * `registry` - The registry containing all networks
    ///
    /// # Returns
    /// A `ValidationResult` indicating if the registration is valid
    pub fn validate_node_registration(
        node_id: NodeId,
        network_id: NetworkId,
        registry: &crate::mechanical::network::registry::NetworkRegistry,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check that the network exists
        if !registry.has_network(network_id) {
            result.add_error(NetworkError::MissingNetwork);
        }
        
        // Check for conflicting membership
        if let Some(existing_network) = registry.get_node_network(node_id) {
            if existing_network != network_id {
                result.add_error(NetworkError::ConflictingNetworkMembership);
            }
        }
        
        result
    }
    
    /// Validates that a connection can be safely registered in a network.
    ///
    /// # Arguments
    /// * `connection` - The connection to validate
    /// * `network` - The network it would be registered in
    ///
    /// # Returns
    /// A `ValidationResult` indicating if the registration is valid
    pub fn validate_connection_registration(
        connection: &Connection,
        network: &MechanicalNetwork,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check for duplicate connection IDs
        if network.has_connection(connection.id()) {
            result.add_error(NetworkError::DuplicateConnection);
        }
        
        // Validate that both endpoints exist in the network
        let component_a = connection.component_a();
        let component_b = connection.component_b();
        
        let node_a = NodeId::new(component_a.get());
        let node_b = NodeId::new(component_b.get());
        
        if !network.has_node(node_a) || !network.has_node(node_b) {
            result.add_error(NetworkError::InvalidConnectionEndpoints);
        }
        
        // Check for self-connections (though this should be prevented at the connection level)
        if component_a == component_b {
            result.add_error(NetworkError::InvalidConnectionEndpoints);
        }
        
        result
    }
}