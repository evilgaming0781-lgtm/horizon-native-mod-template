//! Shared, strongly typed identifiers for mechanical components and systems.
//!
//! This module provides type-safe identifiers to prevent accidental mixing of 
//! different identifier types throughout the mechanical system.

use std::fmt;

/// Unique identifier for a mechanical component.
///
/// Component IDs are globally unique within the mod to ensure proper tracking
/// and management of all mechanical components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentId(pub u64);

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Component{}", self.0)
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        ComponentId(0)
    }
}

impl ComponentId {
    /// Creates a new component ID with the specified value.
    pub const fn new(id: u64) -> Self {
        ComponentId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// Unique identifier for a mechanical network.
///
/// Network IDs are used to track and manage interconnected mechanical systems
/// that operate together as a single unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetworkId(pub u64);

impl fmt::Display for NetworkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Network{}", self.0)
    }
}

impl Default for NetworkId {
    fn default() -> Self {
        NetworkId(0)
    }
}

impl NetworkId {
    /// Creates a new network ID with the specified value.
    pub const fn new(id: u64) -> Self {
        NetworkId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// Unique identifier for a mechanical connection.
///
/// Connection IDs are used to track specific connections between components
/// within the mechanical system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectionId(pub u64);

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connection{}", self.0)
    }
}

impl Default for ConnectionId {
    fn default() -> Self {
        ConnectionId(0)
    }
}

impl ConnectionId {
    /// Creates a new connection ID with the specified value.
    pub const fn new(id: u64) -> Self {
        ConnectionId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// Unique identifier for a mechanical node.
///
/// Node IDs represent specific points in the mechanical system where components
/// can connect or interact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u64);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node{}", self.0)
    }
}

impl Default for NodeId {
    fn default() -> Self {
        NodeId(0)
    }
}

impl NodeId {
    /// Creates a new node ID with the specified value.
    pub const fn new(id: u64) -> Self {
        NodeId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u64 {
        self.0
    }
}