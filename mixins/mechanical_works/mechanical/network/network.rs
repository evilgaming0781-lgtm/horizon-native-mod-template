//! Representation of an individual mechanical network.
//!
//! This module defines the structure and operations for managing a single mechanical network,
//! including node registration, connection management, and network metadata.

use std::collections::{HashMap, HashSet};
use crate::mechanical::core::identifiers::{NetworkId, NodeId, ComponentId, ConnectionId};
use crate::mechanical::core::connections::Connection;
use crate::mechanical::network::errors::{NetworkError, NetworkResult};

/// A mechanical network that manages interconnected components.
///
/// This structure represents a single mechanical network containing nodes (components)
/// and connections between them. Each network has a unique identifier and maintains
/// its own set of registered nodes and connections.
#[derive(Debug, Clone)]
pub struct MechanicalNetwork {
    /// Unique identifier for this network.
    id: NetworkId,
    
    /// Registered nodes in this network.
    nodes: HashSet<NodeId>,
    
    /// Registered connections in this network.
    connections: HashMap<ConnectionId, Connection>,
}

impl MechanicalNetwork {
    /// Creates a new mechanical network with the specified ID.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the new network
    ///
    /// # Returns
    /// A new `MechanicalNetwork` instance
    pub fn new(id: NetworkId) -> Self {
        MechanicalNetwork {
            id,
            nodes: HashSet::new(),
            connections: HashMap::new(),
        }
    }
    
    /// Gets the unique identifier for this network.
    ///
    /// # Returns
    /// The `NetworkId` of this network
    pub fn id(&self) -> NetworkId {
        self.id
    }
    
    /// Registers a node in this network.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to register
    ///
    /// # Returns
    /// * `Ok(())` if registration succeeded
    /// * `Err(NetworkError::DuplicateNode)` if the node already exists
    pub fn register_node(&mut self, node_id: NodeId) -> NetworkResult<()> {
        if self.nodes.contains(&node_id) {
            return Err(NetworkError::DuplicateNode);
        }
        
        self.nodes.insert(node_id);
        Ok(())
    }
    
    /// Removes a node from this network.
    ///
    /// This operation also removes any connections that involve the node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to remove
    ///
    /// # Returns
    /// * `Ok(())` if removal succeeded
    /// * `Err(NetworkError::MissingNode)` if the node does not exist
    pub fn remove_node(&mut self, node_id: NodeId) -> NetworkResult<()> {
        if !self.nodes.contains(&node_id) {
            return Err(NetworkError::MissingNode);
        }
        
        // Remove all connections that involve this node
        let connection_ids_to_remove: Vec<ConnectionId> = self.connections
            .keys()
            .filter(|&conn_id| {
                if let Some(conn) = self.connections.get(&conn_id) {
                    conn.component_a() == ComponentId::new(node_id.get()) || 
                    conn.component_b() == ComponentId::new(node_id.get())
                } else {
                    false
                }
            })
            .cloned()
            .collect();
            
        for conn_id in connection_ids_to_remove {
            self.connections.remove(&conn_id);
        }
        
        self.nodes.remove(&node_id);
        Ok(())
    }
    
    /// Registers a connection in this network.
    ///
    /// # Arguments
    /// * `connection` - The connection to register
    ///
    /// # Returns
    /// * `Ok(())` if registration succeeded
    /// * `Err(NetworkError::DuplicateConnection)` if the connection already exists
    /// * `Err(NetworkError::InvalidConnectionEndpoints)` if endpoints are invalid
    pub fn register_connection(&mut self, connection: Connection) -> NetworkResult<()> {
        // Check if connection already exists
        if self.connections.contains_key(&connection.id()) {
            return Err(NetworkError::DuplicateConnection);
        }
        
        // Validate that both endpoints exist in this network
        let component_a = connection.component_a();
        let component_b = connection.component_b();
        
        // Convert component IDs to node IDs (assuming ComponentId and NodeId are equivalent)
        let node_a = NodeId::new(component_a.get());
        let node_b = NodeId::new(component_b.get());
        
        if !self.nodes.contains(&node_a) || !self.nodes.contains(&node_b) {
            return Err(NetworkError::InvalidConnectionEndpoints);
        }
        
        self.connections.insert(connection.id(), connection);
        Ok(())
    }
    
    /// Removes a connection from this network.
    ///
    /// # Arguments
    /// * `connection_id` - The ID of the connection to remove
    ///
    /// # Returns
    /// * `Ok(())` if removal succeeded
    /// * `Err(NetworkError::MissingConnection)` if the connection does not exist
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> NetworkResult<()> {
        if !self.connections.contains_key(&connection_id) {
            return Err(NetworkError::MissingConnection);
        }
        
        self.connections.remove(&connection_id);
        Ok(())
    }
    
    /// Checks if a node exists in this network.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to check
    ///
    /// # Returns
    /// True if the node exists in this network, false otherwise
    pub fn has_node(&self, node_id: NodeId) -> bool {
        self.nodes.contains(&node_id)
    }
    
    /// Checks if a connection exists in this network.
    ///
    /// # Arguments
    /// * `connection_id` - The ID of the connection to check
    ///
    /// # Returns
    /// True if the connection exists in this network, false otherwise
    pub fn has_connection(&self, connection_id: ConnectionId) -> bool {
        self.connections.contains_key(&connection_id)
    }
    
    /// Gets all nodes registered in this network.
    ///
    /// # Returns
    /// An iterator over all node IDs in the network
    pub fn nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.iter()
    }
    
    /// Gets all connections registered in this network.
    ///
    /// # Returns
    /// An iterator over all connections in the network
    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.values()
    }
    
    /// Gets the number of nodes in this network.
    ///
    /// # Returns
    /// The count of nodes in the network
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Gets the number of connections in this network.
    ///
    /// # Returns
    /// The count of connections in the network
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::core::identifiers::{ComponentId, ConnectionId};
    use crate::mechanical::core::connections::ConnectionType;

    #[test]
    fn test_network_creation() {
        let network_id = NetworkId::new(1);
        let network = MechanicalNetwork::new(network_id);
        
        assert_eq!(network.id(), network_id);
        assert_eq!(network.node_count(), 0);
        assert_eq!(network.connection_count(), 0);
    }

    #[test]
    fn test_node_registration() {
        let network_id = NetworkId::new(1);
        let mut network = MechanicalNetwork::new(network_id);
        
        let node_id = NodeId::new(10);
        assert!(network.register_node(node_id).is_ok());
        assert!(network.has_node(node_id));
        assert_eq!(network.node_count(), 1);
        
        // Try to register the same node again - should fail
        assert_eq!(network.register_node(node_id), Err(NetworkError::DuplicateNode));
    }

    #[test]
    fn test_node_removal() {
        let network_id = NetworkId::new(1);
        let mut network = MechanicalNetwork::new(network_id);
        
        let node_id = NodeId::new(10);
        assert!(network.register_node(node_id).is_ok());
        assert_eq!(network.node_count(), 1);
        
        // Remove the node
        assert!(network.remove_node(node_id).is_ok());
        assert!(!network.has_node(node_id));
        assert_eq!(network.node_count(), 0);
        
        // Try to remove a non-existent node - should fail
        assert_eq!(network.remove_node(node_id), Err(NetworkError::MissingNode));
    }

    #[test]
    fn test_connection_registration() {
        let network_id = NetworkId::new(1);
        let mut network = MechanicalNetwork::new(network_id);
        
        // Register nodes first
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        assert!(network.register_node(node_a).is_ok());
        assert!(network.register_node(node_b).is_ok());
        
        // Create a connection
        let conn_id = ConnectionId::new(1);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        
        // Register the connection - should succeed
        assert!(network.register_connection(connection).is_ok());
        assert!(network.has_connection(conn_id));
        assert_eq!(network.connection_count(), 1);
        
        // Try to register the same connection again - should fail
        let duplicate_connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        assert_eq!(
            network.register_connection(duplicate_connection),
            Err(NetworkError::DuplicateConnection)
        );
        
        // Try to register a connection with non-existent nodes - should fail
        let invalid_node = NodeId::new(30);
        let invalid_conn_id = ConnectionId::new(2);
        let invalid_component_a = ComponentId::new(10);
        let invalid_component_b = ComponentId::new(30); // Non-existent node
        let invalid_connection = Connection::new(
            invalid_conn_id,
            invalid_component_a,
            invalid_component_b,
            ConnectionType::Rotational
        );
        
        assert_eq!(
            network.register_connection(invalid_connection),
            Err(NetworkError::InvalidConnectionEndpoints)
        );
    }

    #[test]
    fn test_connection_removal() {
        let network_id = NetworkId::new(1);
        let mut network = MechanicalNetwork::new(network_id);
        
        // Register nodes first
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        assert!(network.register_node(node_a).is_ok());
        assert!(network.register_node(node_b).is_ok());
        
        // Create and register a connection
        let conn_id = ConnectionId::new(1);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        
        assert!(network.register_connection(connection).is_ok());
        assert_eq!(network.connection_count(), 1);
        
        // Remove the connection
        assert!(network.remove_connection(conn_id).is_ok());
        assert!(!network.has_connection(conn_id));
        assert_eq!(network.connection_count(), 0);
        
        // Try to remove a non-existent connection - should fail
        assert_eq!(
            network.remove_connection(conn_id),
            Err(NetworkError::MissingConnection)
        );
    }

    #[test]
    fn test_node_removal_with_connections() {
        let network_id = NetworkId::new(1);
        let mut network = MechanicalNetwork::new(network_id);
        
        // Register nodes
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        assert!(network.register_node(node_a).is_ok());
        assert!(network.register_node(node_b).is_ok());
        assert!(network.register_node(node_c).is_ok());
        
        // Create and register connections
        let conn1_id = ConnectionId::new(1);
        let conn2_id = ConnectionId::new(2);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let component_c = ComponentId::new(20); // Connects node B to node C
        
        let connection1 = Connection::new(
            conn1_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        let connection2 = Connection::new(
            conn2_id,
            component_b,
            component_c,
            ConnectionType::Rotational
        );
        
        assert!(network.register_connection(connection1).is_ok());
        assert!(network.register_connection(connection2).is_ok());
        assert_eq!(network.connection_count(), 2);
        
        // Remove node B (which should remove both connections)
        assert!(network.remove_node(node_b).is_ok());
        assert!(!network.has_node(node_b));
        assert_eq!(network.node_count(), 2); // Should have nodes A and C
        assert_eq!(network.connection_count(), 0); // Both connections should be removed
    }
}