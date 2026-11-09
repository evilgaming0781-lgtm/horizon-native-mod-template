//! Integration test to verify all network components work together.

#[cfg(test)]
mod integration_tests {
    use crate::mechanical::network::{MechanicalNetwork, NetworkTopology, NetworkRegistry};
    use crate::mechanical::core::identifiers::{NetworkId, NodeId, ComponentId, ConnectionId};
    use crate::mechanical::core::connections::Connection;
    use crate::mechanical::network::errors::NetworkError;
    use crate::mechanical::core::connections::ConnectionType;

    #[test]
    fn test_network_creation_and_basic_operations() {
        let network_id = NetworkId::new(1);
        let mut network = MechanicalNetwork::new(network_id);
        
        // Test basic network creation
        assert_eq!(network.id(), network_id);
        assert_eq!(network.node_count(), 0);
        assert_eq!(network.connection_count(), 0);
        
        // Test node registration
        let node_id = NodeId::new(10);
        assert!(network.register_node(node_id).is_ok());
        assert!(network.has_node(node_id));
        assert_eq!(network.node_count(), 1);
        
        // Test duplicate node rejection
        assert_eq!(network.register_node(node_id), Err(NetworkError::DuplicateNode));
        
        // Test node removal
        assert!(network.remove_node(node_id).is_ok());
        assert!(!network.has_node(node_id));
        assert_eq!(network.node_count(), 0);
    }

    #[test]
    fn test_network_topology_operations() {
        let mut topology = NetworkTopology::new();
        
        let node_a = NodeId::new(10);
        let node_b = NodeId::new(20);
        let node_c = NodeId::new(30);
        
        // Add nodes
        topology.add_node(node_a);
        topology.add_node(node_b);
        topology.add_node(node_c);
        
        // Test that topology is not empty
        assert!(!topology.is_empty());
        
        // Test connection operations (this will require a network for full test)
        let conn_id = ConnectionId::new(1);
        let component_a = ComponentId::new(10);
        let component_b = ComponentId::new(20);
        let connection = Connection::new(
            conn_id,
            component_a,
            component_b,
            ConnectionType::Rotational
        );
        
        // Note: This test focuses on the structure, not full integration
        assert_eq!(topology.graph.nodes().count(), 3);
    }

    #[test]
    fn test_registry_operations() {
        let mut registry = NetworkRegistry::new();
        
        let network_id = NetworkId::new(1);
        let network = MechanicalNetwork::new(network_id);
        
        // Test network registration
        assert!(registry.register_network(network).is_ok());
        assert!(registry.has_network(network_id));
        assert_eq!(registry.network_count(), 1);
        
        // Test network lookup
        let retrieved_network = registry.get_network(network_id);
        assert!(retrieved_network.is_some());
        
        // Test network removal
        let removed_network = registry.remove_network(network_id);
        assert!(removed_network.is_ok());
        assert_eq!(registry.network_count(), 0);
    }
}