//! Network registry for managing multiple mechanical networks.
//!
//! This module provides a centralized registry for tracking all mechanical networks
//! in the system, enabling efficient lookup and management of network state.

use std::collections::{HashMap, HashSet};
use crate::mechanical::core::identifiers::{NetworkId, NodeId, ComponentId};
use crate::mechanical::network::network::MechanicalNetwork;
use crate::mechanical::network::errors::{NetworkError, NetworkResult};

/// A registry for managing multiple mechanical networks.
///
/// This structure maintains a collection of all active mechanical networks,
/// providing operations to register, lookup, and remove networks while
/// enforcing constraints on network membership and uniqueness.
#[derive(Debug, Clone)]
pub struct NetworkRegistry {
    /// Map of network IDs to their corresponding networks.
    networks: HashMap<NetworkId, MechanicalNetwork>,
    
    /// Map of node IDs to the network they belong to.
    node_network_map: HashMap<NodeId, NetworkId>,
}

impl NetworkRegistry {
    /// Creates a new empty network registry.
    pub fn new() -> Self {
        NetworkRegistry {
            networks: HashMap::new(),
            node_network_map: HashMap::new(),
        }
    }
    
    /// Registers a new network in the registry.
    ///
    /// # Arguments
    /// * `network` - The network to register
    ///
    /// # Returns
    /// * `Ok(())` if registration succeeded
    /// * `Err(NetworkError::DuplicateNetworkId)` if a network with this ID already exists
    pub fn register_network(&mut self, network: MechanicalNetwork) -> NetworkResult<()> {
        let network_id = network.id();
        
        if self.networks.contains_key(&network_id) {
            return Err(NetworkError::DuplicateNetworkId);
        }
        
        self.networks.insert(network_id, network);
        Ok(())
    }
    
    /// Looks up a network by its ID.
    ///
    /// # Arguments
    /// * `network_id` - The ID of the network to look up
    ///
    /// # Returns
    /// * `Some(&MechanicalNetwork)` if the network exists
    /// * `None` if no network with this ID exists
    pub fn get_network(&self, network_id: NetworkId) -> Option<&MechanicalNetwork> {
        self.networks.get(&network_id)
    }
    
    /// Mutably looks up a network by its ID.
    ///
    /// # Arguments
    /// * `network_id` - The ID of the network to look up
    ///
    /// # Returns
    /// * `Some(&mut MechanicalNetwork)` if the network exists
    /// * `None` if no network with this ID exists
    pub fn get_network_mut(&mut self, network_id: NetworkId) -> Option<&mut MechanicalNetwork> {
        self.networks.get_mut(&network_id)
    }
    
    /// Removes a network from the registry.
    ///
    /// # Arguments
    /// * `network_id` - The ID of the network to remove
    ///
    /// # Returns
    /// * `Ok(MechanicalNetwork)` if removal succeeded
    /// * `Err(NetworkError::MissingNetwork)` if no network with this ID exists
    pub fn remove_network(&mut self, network_id: NetworkId) -> NetworkResult<MechanicalNetwork> {
        if let Some(network) = self.networks.remove(&network_id) {
            // Remove all node mappings for this network
            let nodes_to_remove: Vec<NodeId> = self.node_network_map
                .keys()
                .filter(|&node_id| self.node_network_map[&node_id] == network_id)
                .cloned()
                .collect();
                
            for node_id in nodes_to_remove {
                self.node_network_map.remove(&node_id);
            }
            
            Ok(network)
        } else {
            Err(NetworkError::MissingNetwork)
        }
    }
    
    /// Registers a node in a specific network.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to register
    /// * `network_id` - The ID of the network to register the node in
    ///
    /// # Returns
    /// * `Ok(())` if registration succeeded
    /// * `Err(NetworkError::MissingNetwork)` if the network doesn't exist
    /// * `Err(NetworkError::ConflictingNetworkMembership)` if the node is already registered in a different network
    pub fn register_node(&mut self, node_id: NodeId, network_id: NetworkId) -> NetworkResult<()> {
        // Check if the network exists
        if !self.networks.contains_key(&network_id) {
            return Err(NetworkError::MissingNetwork);
        }
        
        // Check if node is already registered in a different network
        if let Some(existing_network_id) = self.node_network_map.get(&node_id) {
            if *existing_network_id != network_id {
                return Err(NetworkError::ConflictingNetworkMembership);
            }
        }
        
        // Register the node in the network
        if let Some(network) = self.networks.get_mut(&network_id) {
            network.register_node(node_id)?;
            self.node_network_map.insert(node_id, network_id);
            Ok(())
        } else {
            Err(NetworkError::MissingNetwork)
        }
    }
    
    /// Removes a node from its network.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to remove
    ///
    /// # Returns
    /// * `Ok(())` if removal succeeded
    /// * `Err(NetworkError::MissingNode)` if the node is not registered in any network
    pub fn remove_node(&mut self, node_id: NodeId) -> NetworkResult<()> {
        if let Some(network_id) = self.node_network_map.get(&node_id) {
            // Remove the node from its network
            if let Some(network) = self.networks.get_mut(network_id) {
                network.remove_node(node_id)?;
                self.node_network_map.remove(&node_id);
                Ok(())
            } else {
                Err(NetworkError::MissingNetwork)
            }
        } else {
            Err(NetworkError::MissingNode)
        }
    }
    
    /// Gets the network that a node belongs to.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to look up
    ///
    /// # Returns
    /// * `Some(NetworkId)` if the node is registered in a network
    /// * `None` if the node is not registered in any network
    pub fn get_node_network(&self, node_id: NodeId) -> Option<NetworkId> {
        self.node_network_map.get(&node_id).copied()
    }
    
    /// Checks if a network exists.
    ///
    /// # Arguments
    /// * `network_id` - The ID of the network to check
    ///
    /// # Returns
    /// True if the network exists, false otherwise
    pub fn has_network(&self, network_id: NetworkId) -> bool {
        self.networks.contains_key(&network_id)
    }
    
    /// Checks if a node is registered in any network.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the node to check
    ///
    /// # Returns
    /// True if the node is registered in a network, false otherwise
    pub fn has_node(&self, node_id: NodeId) -> bool {
        self.node_network_map.contains_key(&node_id)
    }
    
    /// Gets all registered networks.
    ///
    /// # Returns
    /// An iterator over all network IDs and their corresponding networks
    pub fn networks(&self) -> impl Iterator<Item = (&NetworkId, &MechanicalNetwork)> {
        self.networks.iter()
    }
    
    /// Gets all registered nodes.
    ///
    /// # Returns
    /// An iterator over all node IDs and their corresponding network IDs
    pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &NetworkId)> {
        self.node_network_map.iter()
    }
    
    /// Gets the number of registered networks.
    ///
    /// # Returns
    /// The count of networks in the registry
    pub fn network_count(&self) -> usize {
        self.networks.len()
    }
    
    /// Gets the number of registered nodes.
    ///
    /// # Returns
    /// The count of nodes in the registry
    pub fn node_count(&self) -> usize {
        self.node_network_map.len()
    }
}