//! Network topology management for mechanical systems.
//!
//! This module provides graph operations for managing the connectivity and structure
//! of mechanical networks, including node and connection management.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::mechanical::core::identifiers::{NodeId, ComponentId, ConnectionId};
use crate::mechanical::core::connections::Connection;
use crate::mechanical::network::errors::{NetworkError, NetworkResult};

/// A representation of the network graph structure.
///
/// This structure maintains adjacency information for nodes in a mechanical network,
/// enabling efficient graph operations like neighbor lookup and reachability analysis.
#[derive(Debug, Clone)]
pub struct NetworkGraph {
    /// Adjacency list mapping each node to its neighbors.
    adjacency_list: HashMap<NodeId, HashSet<NodeId>>,
    
    /// Map of connections to their endpoints.
    connection_map: HashMap<ConnectionId, (NodeId, NodeId)>,
}

impl NetworkGraph {
    /// Creates a new empty network graph.
    pub fn new() -> Self {
        NetworkGraph {
            adjacency_list: HashMap::new(),
            connection_map: HashMap::new(),
        }
    }
    
    /// Adds a node to the graph.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to add
    pub fn add_node(&mut self, node_id: NodeId) {
        self.adjacency_list.entry(node_id).or_insert_with(HashSet::new);
    }
    
    /// Removes a node from the graph.
    ///
    /// This operation also removes all connections involving this node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to remove
    pub fn remove_node(&mut self, node_id: NodeId) {
        // Remove all connections that involve this node
        let connection_ids_to_remove: Vec<ConnectionId> = self.connection_map
            .keys()
            .filter(|&conn_id| {
                if let Some((node_a, node_b)) = self.connection_map.get(&conn_id) {
                    *node_a == node_id || *node_b == node_id
                } else {
                    false
                }
            })
            .cloned()
            .collect();
            
        for conn_id in connection_ids_to_remove {
            self.connection_map.remove(&conn_id);
        }
        
        // Remove the node from adjacency list and all other nodes' neighbor lists
        self.adjacency_list.remove(&node_id);
        for neighbors in self.adjacency_list.values_mut() {
            neighbors.remove(&node_id);
        }
    }
    
    /// Adds a connection between two nodes.
    ///
    /// # Arguments
    /// * `connection` - The connection to add
    ///
    /// # Returns
    /// * `Ok(())` if the connection was added successfully
    /// * `Err(NetworkError::InvalidConnectionEndpoints)` if endpoints are invalid
    pub fn add_connection(&mut self, connection: &Connection) -> NetworkResult<()> {
        let component_a = connection.component_a();
        let component_b = connection.component_b();
        
        // Convert to node IDs (assuming ComponentId and NodeId are equivalent)
        let node_a = NodeId::new(component_a.get());
        let node_b = NodeId::new(component_b.get());
        
        // Check if both endpoints exist in the graph
        if !self.adjacency_list.contains_key(&node_a) || !self.adjacency_list.contains_key(&node_b) {
            return Err(NetworkError::InvalidConnectionEndpoints);
        }
        
        // Add connection to map
        self.connection_map.insert(connection.id(), (node_a, node_b));
        
        // Update adjacency list for both nodes
        self.adjacency_list.entry(node_a).or_insert_with(HashSet::new).insert(node_b);
        self.adjacency_list.entry(node_b).or_insert_with(HashSet::new).insert(node_a);
        
        Ok(())
    }
    
    /// Removes a connection from the graph.
    ///
    /// # Arguments
    /// * `connection_id` - The ID of the connection to remove
    ///
    /// # Returns
    /// * `Ok(())` if the connection was removed successfully
    /// * `Err(NetworkError::MissingConnection)` if the connection doesn't exist
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> NetworkResult<()> {
        if let Some((node_a, node_b)) = self.connection_map.remove(&connection_id) {
            // Remove from adjacency lists
            if let Some(neighbors_a) = self.adjacency_list.get_mut(&node_a) {
                neighbors_a.remove(&node_b);
            }
            if let Some(neighbors_b) = self.adjacency_list.get_mut(&node_b) {
                neighbors_b.remove(&node_a);
            }
            Ok(())
        } else {
            Err(NetworkError::MissingConnection)
        }
    }
    
    /// Checks if two nodes are directly connected.
    ///
    /// # Arguments
    /// * `node_a` - First node ID
    /// * `node_b` - Second node ID
    ///
    /// # Returns
    /// True if the nodes are directly connected, false otherwise
    pub fn are_connected(&self, node_a: NodeId, node_b: NodeId) -> bool {
        self.adjacency_list.get(&node_a)
            .map_or(false, |neighbors| neighbors.contains(&node_b))
    }
    
    /// Gets all neighbors of a given node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to get neighbors for
    ///
    /// # Returns
    /// An iterator over neighbor node IDs
    pub fn neighbors(&self, node_id: NodeId) -> impl Iterator<Item = &NodeId> {
        self.adjacency_list.get(&node_id)
            .map(|neighbors| neighbors.iter())
            .unwrap_or_else(|| [].iter())
    }
    
    /// Finds all nodes reachable from a starting node (breadth-first search).
    ///
    /// # Arguments
    /// * `start_node` - The node to start the search from
    ///
    /// # Returns
    /// A HashSet containing all reachable node IDs
    pub fn find_reachable(&self, start_node: NodeId) -> HashSet<NodeId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        if self.adjacency_list.contains_key(&start_node) {
            visited.insert(start_node);
            queue.push_back(start_node);
        }
        
        while let Some(current_node) = queue.pop_front() {
            if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        
        visited
    }
    
    /// Finds all connected components in the graph.
    ///
    /// # Returns
    /// A vector of sets, where each set contains node IDs belonging to a connected component
    pub fn find_connected_components(&self) -> Vec<HashSet<NodeId>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        
        for node_id in self.adjacency_list.keys() {
            if !visited.contains(node_id) {
                let component = self.find_reachable(*node_id);
                visited.extend(&component);
                components.push(component);
            }
        }
        
        components
    }
    
    /// Checks if two nodes belong to the same connected component.
    ///
    /// # Arguments
    /// * `node_a` - First node ID
    /// * `node_b` - Second node ID
    ///
    /// # Returns
    /// True if both nodes are in the same connected component, false otherwise
    pub fn same_component(&self, node_a: NodeId, node_b: NodeId) -> bool {
        // If either node doesn't exist, they can't be in the same component
        if !self.adjacency_list.contains_key(&node_a) || !self.adjacency_list.contains_key(&node_b) {
            return false;
        }
        
        // If both nodes are the same, they're definitely in the same component
        if node_a == node_b {
            return true;
        }
        
        let reachable_from_a = self.find_reachable(node_a);
        reachable_from_a.contains(&node_b)
    }
    
    /// Checks if a node is isolated (has no connections).
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to check
    ///
    /// # Returns
    /// True if the node has no connections, false otherwise
    pub fn is_isolated(&self, node_id: NodeId) -> bool {
        self.adjacency_list.get(&node_id)
            .map_or(true, |neighbors| neighbors.is_empty())
    }
    
    /// Checks if the graph is empty.
    ///
    /// # Returns
    /// True if there are no nodes in the graph, false otherwise
    pub fn is_empty(&self) -> bool {
        self.adjacency_list.is_empty()
    }
    
    /// Gets all nodes in the graph.
    ///
    /// # Returns
    /// An iterator over all node IDs in the graph
    pub fn nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.adjacency_list.keys()
    }
    
    /// Gets all connections in the graph.
    ///
    /// # Returns
    /// An iterator over all connection IDs in the graph
    pub fn connections(&self) -> impl Iterator<Item = &ConnectionId> {
        self.connection_map.keys()
    }
}

/// Topology management for mechanical networks.
///
/// This structure provides high-level operations for managing network connectivity,
/// including adding/removing nodes and connections, checking relationships, and
/// performing graph traversals.
#[derive(Debug, Clone)]
pub struct NetworkTopology {
    /// The underlying graph representation.
    pub graph: NetworkGraph,
}

impl NetworkTopology {
    /// Creates a new empty network topology.
    pub fn new() -> Self {
        NetworkTopology {
            graph: NetworkGraph::new(),
        }
    }
    
    /// Adds a node to the network topology.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to add
    pub fn add_node(&mut self, node_id: NodeId) {
        self.graph.add_node(node_id);
    }
    
    /// Removes a node from the network topology.
    ///
    /// This operation also removes all connections involving this node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to remove
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.graph.remove_node(node_id);
    }
    
    /// Adds a connection between two nodes in the network topology.
    ///
    /// # Arguments
    /// * `connection` - The connection to add
    ///
    /// # Returns
    /// * `Ok(())` if the connection was added successfully
    /// * `Err(NetworkError::InvalidConnectionEndpoints)` if endpoints are invalid
    pub fn add_connection(&mut self, connection: &Connection) -> NetworkResult<()> {
        self.graph.add_connection(connection)
    }
    
    /// Removes a connection from the network topology.
    ///
    /// # Arguments
    /// * `connection_id` - The ID of the connection to remove
    ///
    /// # Returns
    /// * `Ok(())` if the connection was removed successfully
    /// * `Err(NetworkError::MissingConnection)` if the connection doesn't exist
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> NetworkResult<()> {
        self.graph.remove_connection(connection_id)
    }
    
    /// Checks if two nodes are directly connected.
    ///
    /// # Arguments
    /// * `node_a` - First node ID
    /// * `node_b` - Second node ID
    ///
    /// # Returns
    /// True if the nodes are directly connected, false otherwise
    pub fn are_connected(&self, node_a: NodeId, node_b: NodeId) -> bool {
        self.graph.are_connected(node_a, node_b)
    }
    
    /// Gets all neighbors of a given node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to get neighbors for
    ///
    /// # Returns
    /// An iterator over neighbor node IDs
    pub fn neighbors(&self, node_id: NodeId) -> impl Iterator<Item = &NodeId> {
        self.graph.neighbors(node_id)
    }
    
    /// Finds all nodes reachable from a starting node.
    ///
    /// # Arguments
    /// * `start_node` - The node to start the search from
    ///
    /// # Returns
    /// A HashSet containing all reachable node IDs
    pub fn find_reachable(&self, start_node: NodeId) -> HashSet<NodeId> {
        self.graph.find_reachable(start_node)
    }
    
    /// Finds all connected components in the network.
    ///
    /// # Returns
    /// A vector of sets, where each set contains node IDs belonging to a connected component
    pub fn find_connected_components(&self) -> Vec<HashSet<NodeId>> {
        self.graph.find_connected_components()
    }
    
    /// Checks if two nodes belong to the same connected component.
    ///
    /// # Arguments
    /// * `node_a` - First node ID
    /// * `node_b` - Second node ID
    ///
    /// # Returns
    /// True if both nodes are in the same connected component, false otherwise
    pub fn same_component(&self, node_a: NodeId, node_b: NodeId) -> bool {
        self.graph.same_component(node_a, node_b)
    }
    
    /// Checks if a node is isolated (has no connections).
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to check
    ///
    /// # Returns
    /// True if the node has no connections, false otherwise
    pub fn is_isolated(&self, node_id: NodeId) -> bool {
        self.graph.is_isolated(node_id)
    }
    
    /// Checks if the network is empty.
    ///
    /// # Returns
    /// True if there are no nodes in the network, false otherwise
    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::core::identifiers::{ComponentId, ConnectionId};
    use crate::mechanical::core::connections::ConnectionType;

    #[test]
    fn test_graph_creation() {
        let graph = NetworkGraph::new();
        assert!(graph.is_empty());
    }

    #[test]
    fn test_node_addition_and_removal() {
        let mut graph = NetworkGraph::new();
        
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        
        // Add nodes
        graph.add_node(node_a);
        graph.add_node(node_b);
        
        assert!(!graph.is_empty());
        assert!(graph.nodes().any(|&n| n == node_a));
        assert!(graph.nodes().any(|&n| n == node_b));
        
        // Remove one node
        graph.remove_node(node_a);
        assert!(!graph.nodes().any(|&n| n == node_a));
        assert!(graph.nodes().any(|&n| n == node_b));
    }

    #[test]
    fn test_connection_operations() {
        let mut graph = NetworkGraph::new();
        
        // Add nodes first
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        
        graph.add_node(node_a);
        graph.add_node(node_b);
        graph.add_node(node_c);
        
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
        
        // Add the connection
        assert!(graph.add_connection(&connection).is_ok());
        
        // Check if they're connected
        assert!(graph.are_connected(node_a, node_b));
        assert!(!graph.are_connected(node_a, node_c));
        
        // Check neighbors
        let neighbors: Vec<NodeId> = graph.neighbors(node_a).cloned().collect();
        assert_eq!(neighbors.len(), 1);
        assert!(neighbors.contains(&node_b));
        
        // Remove the connection
        assert!(graph.remove_connection(conn_id).is_ok());
        assert!(!graph.are_connected(node_a, node_b));
    }

    #[test]
    fn test_invalid_connection_endpoints() {
        let mut graph = NetworkGraph::new();
        
        // Add only one node
        let node_a = NodeId::new(10);
        graph.add_node(node_a);
        
        // Try to add connection with non-existent node
        let conn_id = ConnectionId::new(1);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20); // Non-existent node
        let connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        
        assert_eq!(
            graph.add_connection(&connection),
            Err(NetworkError::InvalidConnectionEndpoints)
        );
    }

    #[test]
    fn test_reachability() {
        let mut graph = NetworkGraph::new();
        
        // Create a chain: A - B - C
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        
        graph.add_node(node_a);
        graph.add_node(node_b);
        graph.add_node(node_c);
        
        // Add connections
        let conn1_id = ConnectionId::new(1);
        let conn2_id = ConnectionId::new(2);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let component_c = ComponentId::new(30);
        
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
        
        graph.add_connection(&connection1).unwrap();
        graph.add_connection(&connection2).unwrap();
        
        // Test reachability from node A
        let reachable = graph.find_reachable(node_a);
        assert!(reachable.contains(&node_a));
        assert!(reachable.contains(&node_b));
        assert!(reachable.contains(&node_c));
        
        // Test reachability from node B
        let reachable = graph.find_reachable(node_b);
        assert!(reachable.contains(&node_a));
        assert!(reachable.contains(&node_b));
        assert!(reachable.contains(&node_c));
    }

    #[test]
    fn test_connected_components() {
        let mut graph = NetworkGraph::new();
        
        // Create two disconnected components: A - B and C - D
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        let node_d = NodeId::new(40);
        
        graph.add_node(node_a);
        graph.add_node(node_b);
        graph.add_node(node_c);
        graph.add_node(node_d);
        
        // Add connections within components
        let conn1_id = ConnectionId::new(1);
        let conn2_id = ConnectionId::new(2);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let component_c = ComponentId::new(30);
        let component_d = ComponentId::new(40);
        
        let connection1 = Connection::new(
            conn1_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        let connection2 = Connection::new(
            conn2_id,
            component_c,
            component_d,
            ConnectionType::Rotational
        );
        
        graph.add_connection(&connection1).unwrap();
        graph.add_connection(&connection2).unwrap();
        
        // Find connected components
        let components = graph.find_connected_components();
        assert_eq!(components.len(), 2);
        
        // Check that each component contains the right nodes
        let mut found_component_a = false;
        let mut found_component_b = false;
        
        for component in &components {
            if component.contains(&node_a) && component.contains(&node_b) {
                found_component_a = true;
            }
            if component.contains(&node_c) && component.contains(&node_d) {
                found_component_b = true;
            }
        }
        
        assert!(found_component_a);
        assert!(found_component_b);
    }

    #[test]
    fn test_isolated_nodes() {
        let mut graph = NetworkGraph::new();
        
        // Add nodes
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        
        graph.add_node(node_a);
        graph.add_node(node_b);
        graph.add_node(node_c);
        
        // Check that all are isolated
        assert!(graph.is_isolated(node_a));
        assert!(graph.is_isolated(node_b));
        assert!(graph.is_isolated(node_c));
        
        // Connect two nodes
        let conn_id = ConnectionId::new(1);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        
        graph.add_connection(&connection).unwrap();
        
        // Check isolation after connection
        assert!(!graph.is_isolated(node_a));
        assert!(!graph.is_isolated(node_b));
        assert!(graph.is_isolated(node_c));
    }

    #[test]
    fn test_topology_operations() {
        let mut topology = NetworkTopology::new();
        
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        
        // Add nodes via topology
        topology.add_node(node_a);
        topology.add_node(node_b);
        topology.add_node(node_c);
        
        assert!(!topology.is_empty());
        
        // Add connection
        let conn_id = ConnectionId::new(1);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        
        assert!(topology.add_connection(&connection).is_ok());
        assert!(topology.are_connected(node_a, node_b));
        assert!(!topology.are_connected(node_a, node_c));
        
        // Test same component
        assert!(topology.same_component(node_a, node_b));
        assert!(!topology.same_component(node_a, node_c));
        
        // Remove node and test that connections are also removed
        topology.remove_node(node_b);
        assert!(!topology.are_connected(node_a, node_b));
    }
}