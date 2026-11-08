//! Block system foundational types for Horizon Mechanical Works.
//!
//! This module defines the core block-related data types used throughout
//! the mechanical automation system.

use std::fmt;

use crate::mechanical::machine::types::{MachineId, MachineTypeId};
use crate::mechanical::power::types::{Watts, PowerSupply, PowerDemand};

/// A unique identifier for mechanical blocks in the system.
///
/// This type is used to distinguish between different block types and ensure
/// type safety when working with block operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub u32);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block{}", self.0)
    }
}

impl Default for BlockId {
    fn default() -> Self {
        BlockId(0)
    }
}

impl BlockId {
    /// Creates a new block ID with the specified value.
    pub const fn new(id: u32) -> Self {
        BlockId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

/// Represents the orientation of a mechanical block in 3D space.
///
/// This defines how the block is oriented relative to its surroundings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockOrientation {
    /// Block is facing north (positive Z direction)
    North,
    
    /// Block is facing south (negative Z direction)
    South,
    
    /// Block is facing east (positive X direction)
    East,
    
    /// Block is facing west (negative X direction)
    West,
    
    /// Block is facing up (positive Y direction)
    Up,
    
    /// Block is facing down (negative Y direction)
    Down,
}

impl fmt::Display for BlockOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockOrientation::North => write!(f, "North"),
            BlockOrientation::South => write!(f, "South"),
            BlockOrientation::East => write!(f, "East"),
            BlockOrientation::West => write!(f, "West"),
            BlockOrientation::Up => write!(f, "Up"),
            BlockOrientation::Down => write!(f, "Down"),
        }
    }
}

/// Represents the facing direction of a mechanical block.
///
/// This defines which direction the block is "looking" or oriented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockFacing {
    /// Block faces north (positive Z direction)
    North,
    
    /// Block faces south (negative Z direction)
    South,
    
    /// Block faces east (positive X direction)
    East,
    
    /// Block faces west (negative X direction)
    West,
    
    /// Block faces up (positive Y direction)
    Up,
    
    /// Block faces down (negative Y direction)
    Down,
}

impl fmt::Display for BlockFacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockFacing::North => write!(f, "North"),
            BlockFacing::South => write!(f, "South"),
            BlockFacing::East => write!(f, "East"),
            BlockFacing::West => write!(f, "West"),
            BlockFacing::Up => write!(f, "Up"),
            BlockFacing::Down => write!(f, "Down"),
        }
    }
}

/// Represents the properties of a mechanical block.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockProperties {
    /// The unique identifier for this block type.
    pub block_id: BlockId,
    
    /// The machine type associated with this block (if any).
    pub machine_type: Option<MachineTypeId>,
    
    /// The power supply capability of this block (if it generates power).
    pub power_supply: Option<PowerSupply>,
    
    /// The power demand requirements of this block (if it consumes power).
    pub power_demand: Option<PowerDemand>,
    
    /// Whether this block is solid and can block movement.
    pub is_solid: bool,
    
    /// Whether this block can connect to other blocks mechanically.
    pub can_connect: bool,
    
    /// The maximum number of connections this block supports.
    pub max_connections: u32,
}

impl Default for BlockProperties {
    fn default() -> Self {
        BlockProperties {
            block_id: BlockId::default(),
            machine_type: None,
            power_supply: None,
            power_demand: None,
            is_solid: false,
            can_connect: false,
            max_connections: 0,
        }
    }
}

impl BlockProperties {
    /// Creates a new block properties with specified parameters.
    pub fn new(
        block_id: BlockId,
        machine_type: Option<MachineTypeId>,
        power_supply: Option<PowerSupply>,
        power_demand: Option<PowerDemand>,
        is_solid: bool,
        can_connect: bool,
        max_connections: u32,
    ) -> Self {
        BlockProperties {
            block_id,
            machine_type,
            power_supply,
            power_demand,
            is_solid,
            can_connect,
            max_connections,
        }
    }

    /// Checks if this block has a machine type.
    pub const fn has_machine(&self) -> bool {
        self.machine_type.is_some()
    }

    /// Checks if this block can generate power.
    pub const fn can_generate_power(&self) -> bool {
        self.power_supply.is_some()
    }

    /// Checks if this block requires power.
    pub const fn requires_power(&self) -> bool {
        self.power_demand.is_some()
    }
}

/// Represents mechanical connection properties between blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockConnection {
    /// The ID of the connected block.
    pub connected_block_id: BlockId,
    
    /// The orientation of the connection.
    pub orientation: BlockOrientation,
    
    /// The facing direction of the connection.
    pub facing: BlockFacing,
    
    /// Whether this is a valid connection.
    pub is_valid: bool,
}

impl BlockConnection {
    /// Creates a new block connection with specified parameters.
    pub fn new(
        connected_block_id: BlockId,
        orientation: BlockOrientation,
        facing: BlockFacing,
        is_valid: bool,
    ) -> Self {
        BlockConnection {
            connected_block_id,
            orientation,
            facing,
            is_valid,
        }
    }

    /// Creates a valid connection.
    pub fn valid(
        connected_block_id: BlockId,
        orientation: BlockOrientation,
        facing: BlockFacing,
    ) -> Self {
        BlockConnection::new(connected_block_id, orientation, facing, true)
    }

    /// Creates an invalid connection.
    pub fn invalid(
        connected_block_id: BlockId,
        orientation: BlockOrientation,
        facing: BlockFacing,
    ) -> Self {
        BlockConnection::new(connected_block_id, orientation, facing, false)
    }
}

/// Represents the context for placing a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlacementContext {
    /// The position where the block is being placed.
    pub position: [i32; 3],
    
    /// The orientation of the block being placed.
    pub orientation: BlockOrientation,
    
    /// The facing direction of the block being placed.
    pub facing: BlockFacing,
    
    /// Whether this is a replacement of an existing block.
    pub is_replacement: bool,
}

impl PlacementContext {
    /// Creates a new placement context.
    pub const fn new(
        position: [i32; 3],
        orientation: BlockOrientation,
        facing: BlockFacing,
        is_replacement: bool,
    ) -> Self {
        PlacementContext {
            position,
            orientation,
            facing,
            is_replacement,
        }
    }

    /// Creates a placement context for a new block.
    pub const fn new_block(
        position: [i32; 3],
        orientation: BlockOrientation,
        facing: BlockFacing,
    ) -> Self {
        PlacementContext::new(position, orientation, facing, false)
    }

    /// Creates a placement context for replacing an existing block.
    pub const fn replace_block(
        position: [i32; 3],
        orientation: BlockOrientation,
        facing: BlockFacing,
    ) -> Self {
        PlacementContext::new(position, orientation, facing, true)
    }
}

/// Represents the context for removing a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemovalContext {
    /// The position of the block being removed.
    pub position: [i32; 3],
    
    /// Whether this removal is forced (e.g., by explosion).
    pub is_forced: bool,
}

impl RemovalContext {
    /// Creates a new removal context.
    pub const fn new(position: [i32; 3], is_forced: bool) -> Self {
        RemovalContext {
            position,
            is_forced,
        }
    }

    /// Creates a normal removal context.
    pub const fn normal(position: [i32; 3]) -> Self {
        RemovalContext::new(position, false)
    }

    /// Creates a forced removal context.
    pub const fn forced(position: [i32; 3]) -> Self {
        RemovalContext::new(position, true)
    }
}

/// Represents a block configuration error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    /// Attempted to create a block with invalid properties.
    InvalidProperties,
    
    /// Attempted to connect blocks in an invalid orientation.
    InvalidConnection,
    
    /// Attempted to place a block at an invalid position.
    InvalidPosition,
    
    /// Attempted to remove a block that doesn't exist.
    BlockNotFound,
    
    /// Attempted to create a block with conflicting properties.
    ConflictingProperties,
    
    /// Attempted to exceed maximum connection limits.
    ConnectionLimitExceeded,
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockError::InvalidProperties => write!(f, "Invalid block properties"),
            BlockError::InvalidConnection => write!(f, "Invalid block connection"),
            BlockError::InvalidPosition => write!(f, "Invalid block position"),
            BlockError::BlockNotFound => write!(f, "Block not found"),
            BlockError::ConflictingProperties => write!(f, "Conflicting block properties"),
            BlockError::ConnectionLimitExceeded => write!(f, "Connection limit exceeded"),
        }
    }
}

impl std::error::Error for BlockError {}

/// Represents a block result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockResult {
    /// Operation completed successfully.
    Success,
    
    /// Operation failed with an error.
    Error(BlockError),
}

impl fmt::Display for BlockResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockResult::Success => write!(f, "Operation completed successfully"),
            BlockResult::Error(error) => write!(f, "Operation failed: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_id_creation() {
        let block_id = BlockId::new(42);
        assert_eq!(block_id.get(), 42);
        assert_eq!(format!("{}", block_id), "Block42");
        
        let default_id = BlockId::default();
        assert_eq!(default_id.get(), 0);
    }

    #[test]
    fn test_block_orientation_display() {
        assert_eq!(format!("{}", BlockOrientation::North), "North");
        assert_eq!(format!("{}", BlockOrientation::South), "South");
        assert_eq!(format!("{}", BlockOrientation::East), "East");
        assert_eq!(format!("{}", BlockOrientation::West), "West");
        assert_eq!(format!("{}", BlockOrientation::Up), "Up");
        assert_eq!(format!("{}", BlockOrientation::Down), "Down");
    }

    #[test]
    fn test_block_facing_display() {
        assert_eq!(format!("{}", BlockFacing::North), "North");
        assert_eq!(format!("{}", BlockFacing::South), "South");
        assert_eq!(format!("{}", BlockFacing::East), "East");
        assert_eq!(format!("{}", BlockFacing::West), "West");
        assert_eq!(format!("{}", BlockFacing::Up), "Up");
        assert_eq!(format!("{}", BlockFacing::Down), "Down");
    }

    #[test]
    fn test_block_properties_creation() {
        let block_id = BlockId::new(1);
        let machine_type = Some(MachineTypeId::new(10));
        let power_supply = Some(PowerSupply::new(Watts::new(1000)));
        let power_demand = Some(PowerDemand::new(Watts::new(500), Watts::new(1000)));
        
        let properties = BlockProperties::new(
            block_id,
            machine_type,
            power_supply,
            power_demand,
            true,
            true,
            4,
        );
        
        assert_eq!(properties.block_id, block_id);
        assert_eq!(properties.machine_type, machine_type);
        assert_eq!(properties.power_supply, power_supply);
        assert_eq!(properties.power_demand, power_demand);
        assert!(properties.is_solid);
        assert!(properties.can_connect);
        assert_eq!(properties.max_connections, 4);
    }

    #[test]
    fn test_block_properties_default() {
        let properties = BlockProperties::default();
        assert_eq!(properties.block_id, BlockId::default());
        assert!(properties.machine_type.is_none());
        assert!(properties.power_supply.is_none());
        assert!(properties.power_demand.is_none());
        assert!(!properties.is_solid);
        assert!(!properties.can_connect);
        assert_eq!(properties.max_connections, 0);
    }

    #[test]
    fn test_block_properties_checks() {
        let properties1 = BlockProperties::new(
            BlockId::new(1),
            Some(MachineTypeId::new(10)),
            None,
            None,
            true,
            false,
            0,
        );
        
        assert!(properties1.has_machine());
        assert!(!properties1.can_generate_power());
        assert!(!properties1.requires_power());
        
        let properties2 = BlockProperties::new(
            BlockId::new(2),
            None,
            Some(PowerSupply::new(Watts::new(1000))),
            Some(PowerDemand::new(Watts::new(500), Watts::new(1000))),
            true,
            false,
            0,
        );
        
        assert!(!properties2.has_machine());
        assert!(properties2.can_generate_power());
        assert!(properties2.requires_power());
    }

    #[test]
    fn test_block_connection_creation() {
        let block_id = BlockId::new(1);
        let orientation = BlockOrientation::North;
        let facing = BlockFacing::East;
        
        let connection = BlockConnection::new(block_id, orientation, facing, true);
        assert_eq!(connection.connected_block_id, block_id);
        assert_eq!(connection.orientation, orientation);
        assert_eq!(connection.facing, facing);
        assert!(connection.is_valid);
        
        let valid_connection = BlockConnection::valid(block_id, orientation, facing);
        assert!(valid_connection.is_valid);
        
        let invalid_connection = BlockConnection::invalid(block_id, orientation, facing);
        assert!(!invalid_connection.is_valid);
    }

    #[test]
    fn test_placement_context_creation() {
        let position = [10, 20, 30];
        let orientation = BlockOrientation::North;
        let facing = BlockFacing::East;
        
        let context = PlacementContext::new(position, orientation, facing, true);
        assert_eq!(context.position, position);
        assert_eq!(context.orientation, orientation);
        assert_eq!(context.facing, facing);
        assert!(context.is_replacement);
        
        let new_context = PlacementContext::new_block(position, orientation, facing);
        assert!(!new_context.is_replacement);
        
        let replace_context = PlacementContext::replace_block(position, orientation, facing);
        assert!(replace_context.is_replacement);
    }

    #[test]
    fn test_removal_context_creation() {
        let position = [10, 20, 30];
        
        let context = RemovalContext::new(position, true);
        assert_eq!(context.position, position);
        assert!(context.is_forced);
        
        let normal_context = RemovalContext::normal(position);
        assert!(!normal_context.is_forced);
        
        let forced_context = RemovalContext::forced(position);
        assert!(forced_context.is_forced);
    }

    #[test]
    fn test_block_error_display() {
        let error = BlockError::InvalidProperties;
        assert_eq!(format!("{}", error), "Invalid block properties");
        
        let error2 = BlockError::InvalidConnection;
        assert_eq!(format!("{}", error2), "Invalid block connection");
        
        let error3 = BlockError::InvalidPosition;
        assert_eq!(format!("{}", error3), "Invalid block position");
        
        let error4 = BlockError::BlockNotFound;
        assert_eq!(format!("{}", error4), "Block not found");
        
        let error5 = BlockError::ConflictingProperties;
        assert_eq!(format!("{}", error5), "Conflicting block properties");
        
        let error6 = BlockError::ConnectionLimitExceeded;
        assert_eq!(format!("{}", error6), "Connection limit exceeded");
    }

    #[test]
    fn test_block_result_display() {
        let result = BlockResult::Success;
        assert_eq!(format!("{}", result), "Operation completed successfully");
        
        let result2 = BlockResult::Error(BlockError::InvalidProperties);
        assert_eq!(format!("{}", result2), "Operation failed: Invalid block properties");
    }
}