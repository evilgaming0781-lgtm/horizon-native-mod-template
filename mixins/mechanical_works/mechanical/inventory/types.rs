//! Inventory system foundational types for Horizon Mechanical Works.
//!
//! This module defines the core inventory-related data types used throughout
//! the mechanical automation system.

use std::fmt;

/// A unique identifier for items in the inventory system.
///
/// This type is used to distinguish between different item types and ensure
/// type safety when working with inventory operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(pub u32);

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item{}", self.0)
    }
}

impl Default for ItemId {
    fn default() -> Self {
        ItemId(0)
    }
}

impl ItemId {
    /// Creates a new item ID with the specified value.
    pub const fn new(id: u32) -> Self {
        ItemId(id)
    }

    /// Returns the underlying ID value.
    pub const fn get(&self) -> u32 {
        self.0
    }
}

/// Represents an item stack in the inventory system.
///
/// An item stack contains a specific item type and a quantity of that item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    /// The type of item in this stack.
    pub item_id: ItemId,
    
    /// The quantity of this item in the stack.
    pub quantity: u32,
}

impl Default for ItemStack {
    fn default() -> Self {
        ItemStack {
            item_id: ItemId::default(),
            quantity: 0,
        }
    }
}

impl ItemStack {
    /// Creates a new item stack with the specified item ID and quantity.
    pub const fn new(item_id: ItemId, quantity: u32) -> Self {
        ItemStack {
            item_id,
            quantity,
        }
    }

    /// Creates an empty item stack (zero quantity).
    pub const fn empty() -> Self {
        ItemStack {
            item_id: ItemId::new(0),
            quantity: 0,
        }
    }

    /// Checks if the item stack is empty (quantity is zero).
    pub const fn is_empty(&self) -> bool {
        self.quantity == 0
    }

    /// Sets the quantity of this item stack.
    ///
    /// Returns an error if the quantity exceeds the maximum allowed stack size.
    pub fn set_quantity(&mut self, quantity: u32, max_stack_size: u32) -> Result<(), InventoryError> {
        if quantity > max_stack_size {
            return Err(InventoryError::StackOverflow);
        }
        self.quantity = quantity;
        Ok(())
    }

    /// Adds the specified quantity to this item stack.
    ///
    /// Returns an error if the resulting quantity exceeds the maximum allowed stack size.
    pub fn add_quantity(&mut self, quantity: u32, max_stack_size: u32) -> Result<(), InventoryError> {
        let new_quantity = self.quantity.saturating_add(quantity);
        if new_quantity > max_stack_size {
            return Err(InventoryError::StackOverflow);
        }
        self.quantity = new_quantity;
        Ok(())
    }

    /// Removes the specified quantity from this item stack.
    ///
    /// Returns an error if the requested quantity exceeds what's available.
    pub fn remove_quantity(&mut self, quantity: u32) -> Result<(), InventoryError> {
        if quantity > self.quantity {
            return Err(InventoryError::InsufficientItems);
        }
        self.quantity -= quantity;
        Ok(())
    }

    /// Gets the maximum stack size for this item type.
    ///
    /// This is a placeholder implementation - in a real system, this would be
    /// determined by the item definition or configuration.
    pub fn max_stack_size(&self) -> u32 {
        // Default maximum stack size for items
        64
    }
}

/// Represents an inventory slot.
///
/// An inventory slot can either contain an item stack or be empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventorySlot {
    /// The item stack contained in this slot, or None if the slot is empty.
    pub item_stack: Option<ItemStack>,
}

impl Default for InventorySlot {
    fn default() -> Self {
        InventorySlot {
            item_stack: None,
        }
    }
}

impl InventorySlot {
    /// Creates a new inventory slot with an empty stack.
    pub const fn empty() -> Self {
        InventorySlot {
            item_stack: None,
        }
    }

    /// Creates a new inventory slot with the specified item stack.
    pub fn new(item_stack: ItemStack) -> Self {
        InventorySlot {
            item_stack: Some(item_stack),
        }
    }

    /// Checks if this slot is empty.
    pub const fn is_empty(&self) -> bool {
        self.item_stack.is_none()
    }

    /// Gets the item stack in this slot, or None if empty.
    pub const fn get_item_stack(&self) -> Option<&ItemStack> {
        self.item_stack.as_ref()
    }

    /// Gets a mutable reference to the item stack in this slot, or None if empty.
    pub fn get_item_stack_mut(&mut self) -> Option<&mut ItemStack> {
        self.item_stack.as_mut()
    }

    /// Sets the item stack in this slot.
    ///
    /// If `item_stack` is `None`, the slot becomes empty.
    pub fn set_item_stack(&mut self, item_stack: Option<ItemStack>) {
        self.item_stack = item_stack;
    }

    /// Adds an item stack to this slot.
    ///
    /// If the slot is empty, the entire stack is placed in the slot.
    /// If the slot already contains a stack of the same item type, the quantities are combined.
    /// Returns an error if the resulting stack exceeds the maximum stack size.
    pub fn add_item_stack(&mut self, item_stack: ItemStack, max_stack_size: u32) -> Result<(), InventoryError> {
        match &mut self.item_stack {
            Some(existing_stack) => {
                // If existing stack is of same item type, combine quantities
                if existing_stack.item_id == item_stack.item_id {
                    existing_stack.add_quantity(item_stack.quantity, max_stack_size)?;
                } else {
                    // Different item types - can't combine
                    return Err(InventoryError::IncompatibleItems);
                }
            }
            None => {
                // Slot is empty, just set the new stack
                if item_stack.quantity > max_stack_size {
                    return Err(InventoryError::StackOverflow);
                }
                self.item_stack = Some(item_stack);
            }
        }
        Ok(())
    }

    /// Removes an item stack from this slot.
    ///
    /// Returns the removed stack, or None if the slot was empty.
    pub fn remove_item_stack(&mut self) -> Option<ItemStack> {
        self.item_stack.take()
    }

    /// Removes a specific quantity of items from this slot.
    ///
    /// Returns an error if there aren't enough items available.
    pub fn remove_quantity(&mut self, quantity: u32) -> Result<(), InventoryError> {
        match &mut self.item_stack {
            Some(stack) => {
                if quantity > stack.quantity {
                    return Err(InventoryError::InsufficientItems);
                }
                stack.quantity -= quantity;
                // If quantity becomes zero, remove the stack
                if stack.quantity == 0 {
                    self.item_stack = None;
                }
                Ok(())
            }
            None => Err(InventoryError::EmptySlot),
        }
    }
}

/// Represents inventory capacity limits.
///
/// Defines the maximum number of slots and maximum stack size for an inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryCapacity {
    /// The total number of slots in the inventory.
    pub slot_count: u32,
    
    /// The maximum stack size for any item in the inventory.
    pub max_stack_size: u32,
}

impl Default for InventoryCapacity {
    fn default() -> Self {
        InventoryCapacity {
            slot_count: 0,
            max_stack_size: 64,
        }
    }
}

impl InventoryCapacity {
    /// Creates a new inventory capacity with specified limits.
    pub const fn new(slot_count: u32, max_stack_size: u32) -> Self {
        InventoryCapacity {
            slot_count,
            max_stack_size,
        }
    }

    /// Checks if the inventory has enough slots for the given number of items.
    pub const fn has_slots(&self, slot_count: u32) -> bool {
        self.slot_count >= slot_count
    }

    /// Checks if the stack size is within limits.
    pub const fn is_valid_stack_size(&self, stack_size: u32) -> bool {
        self.max_stack_size >= stack_size
    }
}

/// Represents an inventory error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryError {
    /// Attempted to exceed maximum stack size.
    StackOverflow,
    
    /// Attempted to remove more items than are available.
    InsufficientItems,
    
    /// Attempted to perform an operation on an empty slot.
    EmptySlot,
    
    /// Attempted to combine incompatible item types.
    IncompatibleItems,
    
    /// Invalid capacity value (negative or zero when positive required).
    InvalidCapacity,
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::StackOverflow => write!(f, "Stack overflow - item count exceeds maximum"),
            InventoryError::InsufficientItems => write!(f, "Insufficient items in inventory"),
            InventoryError::EmptySlot => write!(f, "Slot is empty"),
            InventoryError::IncompatibleItems => write!(f, "Cannot combine incompatible item types"),
            InventoryError::InvalidCapacity => write!(f, "Invalid capacity value"),
        }
    }
}

impl std::error::Error for InventoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id_creation() {
        let item_id = ItemId::new(42);
        assert_eq!(item_id.get(), 42);
        assert_eq!(format!("{}", item_id), "Item42");
        
        let default_id = ItemId::default();
        assert_eq!(default_id.get(), 0);
    }

    #[test]
    fn test_item_stack_creation() {
        let item_id = ItemId::new(100);
        let stack = ItemStack::new(item_id, 50);
        assert_eq!(stack.item_id.get(), 100);
        assert_eq!(stack.quantity, 50);
        
        let empty_stack = ItemStack::empty();
        assert!(empty_stack.is_empty());
        assert_eq!(empty_stack.quantity, 0);
    }

    #[test]
    fn test_item_stack_is_empty() {
        let stack = ItemStack::new(ItemId::new(1), 0);
        assert!(stack.is_empty());
        
        let stack2 = ItemStack::new(ItemId::new(1), 1);
        assert!(!stack2.is_empty());
    }

    #[test]
    fn test_item_stack_set_quantity() {
        let mut stack = ItemStack::new(ItemId::new(1), 10);
        let max_size = 64;
        
        // Valid quantity
        assert!(stack.set_quantity(30, max_size).is_ok());
        assert_eq!(stack.quantity, 30);
        
        // Invalid quantity (exceeds max)
        assert!(stack.set_quantity(100, max_size).is_err());
    }

    #[test]
    fn test_item_stack_add_quantity() {
        let mut stack = ItemStack::new(ItemId::new(1), 10);
        let max_size = 64;
        
        // Valid addition
        assert!(stack.add_quantity(20, max_size).is_ok());
        assert_eq!(stack.quantity, 30);
        
        // Invalid addition (exceeds max)
        assert!(stack.add_quantity(50, max_size).is_err());
    }

    #[test]
    fn test_item_stack_remove_quantity() {
        let mut stack = ItemStack::new(ItemId::new(1), 10);
        
        // Valid removal
        assert!(stack.remove_quantity(5).is_ok());
        assert_eq!(stack.quantity, 5);
        
        // Invalid removal (exceeds available)
        assert!(stack.remove_quantity(10).is_err());
    }

    #[test]
    fn test_inventory_slot_creation() {
        let slot = InventorySlot::empty();
        assert!(slot.is_empty());
        
        let stack = ItemStack::new(ItemId::new(1), 5);
        let slot_with_stack = InventorySlot::new(stack.clone());
        assert!(!slot_with_stack.is_empty());
        assert_eq!(slot_with_stack.get_item_stack().unwrap(), &stack);
    }

    #[test]
    fn test_inventory_slot_is_empty() {
        let empty_slot = InventorySlot::empty();
        assert!(empty_slot.is_empty());
        
        let slot_with_item = InventorySlot::new(ItemStack::new(ItemId::new(1), 5));
        assert!(!slot_with_item.is_empty());
    }

    #[test]
    fn test_inventory_slot_set_item_stack() {
        let mut slot = InventorySlot::empty();
        assert!(slot.is_empty());
        
        let stack = ItemStack::new(ItemId::new(1), 5);
        slot.set_item_stack(Some(stack.clone()));
        assert!(!slot.is_empty());
        assert_eq!(slot.get_item_stack().unwrap(), &stack);
        
        slot.set_item_stack(None);
        assert!(slot.is_empty());
    }

    #[test]
    fn test_inventory_slot_add_item_stack() {
        let max_size = 64;
        let mut slot = InventorySlot::empty();
        
        // Add to empty slot
        let stack1 = ItemStack::new(ItemId::new(1), 10);
        assert!(slot.add_item_stack(stack1, max_size).is_ok());
        assert_eq!(slot.get_item_stack().unwrap().quantity, 10);
        
        // Add same item type to existing slot
        let stack2 = ItemStack::new(ItemId::new(1), 5);
        assert!(slot.add_item_stack(stack2, max_size).is_ok());
        assert_eq!(slot.get_item_stack().unwrap().quantity, 15);
        
        // Add different item type (should fail)
        let stack3 = ItemStack::new(ItemId::new(2), 5);
        assert!(slot.add_item_stack(stack3, max_size).is_err());
    }

    #[test]
    fn test_inventory_slot_remove_item_stack() {
        let mut slot = InventorySlot::new(ItemStack::new(ItemId::new(1), 5));
        let removed = slot.remove_item_stack();
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().quantity, 5);
        assert!(slot.is_empty());
    }

    #[test]
    fn test_inventory_slot_remove_quantity() {
        let mut slot = InventorySlot::new(ItemStack::new(ItemId::new(1), 5));
        
        // Valid removal
        assert!(slot.remove_quantity(2).is_ok());
        assert_eq!(slot.get_item_stack().unwrap().quantity, 3);
        
        // Remove all items
        assert!(slot.remove_quantity(3).is_ok());
        assert!(slot.is_empty());
        
        // Try to remove from empty slot
        assert!(slot.remove_quantity(1).is_err());
    }

    #[test]
    fn test_inventory_capacity_creation() {
        let capacity = InventoryCapacity::new(10, 64);
        assert_eq!(capacity.slot_count, 10);
        assert_eq!(capacity.max_stack_size, 64);
        
        let default_capacity = InventoryCapacity::default();
        assert_eq!(default_capacity.slot_count, 0);
        assert_eq!(default_capacity.max_stack_size, 64);
    }

    #[test]
    fn test_inventory_capacity_has_slots() {
        let capacity = InventoryCapacity::new(10, 64);
        assert!(capacity.has_slots(5));
        assert!(!capacity.has_slots(15));
    }

    #[test]
    fn test_inventory_capacity_is_valid_stack_size() {
        let capacity = InventoryCapacity::new(10, 64);
        assert!(capacity.is_valid_stack_size(32));
        assert!(!capacity.is_valid_stack_size(100));
    }

    #[test]
    fn test_inventory_error_display() {
        let error = InventoryError::StackOverflow;
        assert_eq!(format!("{}", error), "Stack overflow - item count exceeds maximum");
        
        let error2 = InventoryError::InsufficientItems;
        assert_eq!(format!("{}", error2), "Insufficient items in inventory");
        
        let error3 = InventoryError::EmptySlot;
        assert_eq!(format!("{}", error3), "Slot is empty");
        
        let error4 = InventoryError::IncompatibleItems;
        assert_eq!(format!("{}", error4), "Cannot combine incompatible item types");
        
        let error5 = InventoryError::InvalidCapacity;
        assert_eq!(format!("{}", error5), "Invalid capacity value");
    }
}