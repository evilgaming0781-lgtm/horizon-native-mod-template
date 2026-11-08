//! Inventory management system for Horizon Mechanical Works.
//!
//! This module provides core inventory operations including insertion, removal,
//! extraction, and transfer of item stacks between inventories.

use std::fmt;

use super::types::{ItemId, ItemStack, InventorySlot, InventoryCapacity, InventoryError};

/// Represents an inventory with a fixed number of slots and maximum stack sizes.
#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    /// The capacity limits for this inventory.
    pub capacity: InventoryCapacity,
    
    /// The slots in this inventory.
    pub slots: Vec<InventorySlot>,
}

impl Inventory {
    /// Creates a new inventory with the specified capacity.
    pub fn new(capacity: InventoryCapacity) -> Self {
        let mut slots = Vec::with_capacity(capacity.slot_count as usize);
        for _ in 0..capacity.slot_count {
            slots.push(InventorySlot::empty());
        }
        
        Inventory {
            capacity,
            slots,
        }
    }

    /// Creates a new inventory with the specified number of slots and default max stack size.
    pub fn with_slots(slot_count: u32) -> Self {
        Inventory::new(InventoryCapacity::new(slot_count, 64))
    }

    /// Gets the total number of slots in this inventory.
    pub const fn slot_count(&self) -> u32 {
        self.capacity.slot_count
    }

    /// Gets the maximum stack size for items in this inventory.
    pub const fn max_stack_size(&self) -> u32 {
        self.capacity.max_stack_size
    }

    /// Checks if the inventory is full (no empty slots).
    pub fn is_full(&self) -> bool {
        self.slots.iter().all(|slot| !slot.is_empty())
    }

    /// Checks if the inventory is empty (all slots are empty).
    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|slot| slot.is_empty())
    }

    /// Gets the number of occupied slots.
    pub fn occupied_slots(&self) -> u32 {
        self.slots.iter().filter(|slot| !slot.is_empty()).count() as u32
    }

    /// Gets the number of available slots.
    pub fn available_slots(&self) -> u32 {
        self.slots.iter().filter(|slot| slot.is_empty()).count() as u32
    }

    /// Gets the total available capacity (sum of all empty slot capacities).
    pub fn available_capacity(&self) -> u32 {
        self.slots.iter()
            .filter(|slot| slot.is_empty())
            .count() as u32 * self.capacity.max_stack_size
    }

    /// Inserts an item stack into the inventory.
    ///
    /// Attempts to insert the item stack by:
    /// 1. Finding existing stacks of the same item type and adding to them
    /// 2. Finding empty slots to place new stacks
    ///
    /// Returns the number of items successfully inserted.
    pub fn insert_stack(&mut self, mut stack: ItemStack) -> Result<u32, InventoryError> {
        if stack.quantity == 0 {
            return Ok(0);
        }

        // First try to add to existing stacks of the same item type
        let mut remaining_quantity = stack.quantity;
        for slot in &mut self.slots {
            if let Some(existing_stack) = slot.get_item_stack_mut() {
                if existing_stack.item_id == stack.item_id {
                    let available_space = self.capacity.max_stack_size - existing_stack.quantity;
                    if available_space > 0 {
                        let to_add = std::cmp::min(remaining_quantity, available_space);
                        existing_stack.add_quantity(to_add, self.capacity.max_stack_size)
                            .map_err(|_| InventoryError::StackOverflow)?;
                        remaining_quantity -= to_add;
                        
                        if remaining_quantity == 0 {
                            return Ok(stack.quantity);
                        }
                    }
                }
            }
        }

        // If there are still items left, try to place them in empty slots
        for slot in &mut self.slots {
            if slot.is_empty() {
                let available_space = self.capacity.max_stack_size;
                let to_place = std::cmp::min(remaining_quantity, available_space);
                
                let new_stack = ItemStack::new(stack.item_id, to_place);
                slot.set_item_stack(Some(new_stack));
                remaining_quantity -= to_place;
                
                if remaining_quantity == 0 {
                    return Ok(stack.quantity);
                }
            }
        }

        // If we get here, there's not enough space
        let inserted = stack.quantity - remaining_quantity;
        Ok(inserted)
    }

    /// Removes a specific quantity of items from the inventory.
    ///
    /// Returns an error if there aren't enough items available.
    pub fn remove_items(&mut self, item_id: ItemId, quantity: u32) -> Result<u32, InventoryError> {
        if quantity == 0 {
            return Ok(0);
        }

        let mut remaining_quantity = quantity;
        let mut total_removed = 0;

        // Remove from existing stacks
        for slot in &mut self.slots {
            if let Some(stack) = slot.get_item_stack_mut() {
                if stack.item_id == item_id {
                    let to_remove = std::cmp::min(remaining_quantity, stack.quantity);
                    stack.remove_quantity(to_remove)
                        .map_err(|_| InventoryError::InsufficientItems)?;
                    remaining_quantity -= to_remove;
                    total_removed += to_remove;

                    // If the stack is now empty, remove it
                    if stack.quantity == 0 {
                        slot.set_item_stack(None);
                    }

                    if remaining_quantity == 0 {
                        return Ok(total_removed);
                    }
                }
            }
        }

        // If we get here, there weren't enough items
        Err(InventoryError::InsufficientItems)
    }

    /// Extracts a complete item stack from the inventory.
    ///
    /// Returns the extracted stack or an error if the item isn't found.
    pub fn extract_stack(&mut self, item_id: ItemId) -> Result<ItemStack, InventoryError> {
        for slot in &mut self.slots {
            if let Some(stack) = slot.get_item_stack_mut() {
                if stack.item_id == item_id {
                    return Ok(slot.remove_item_stack().unwrap());
                }
            }
        }
        
        Err(InventoryError::EmptySlot)
    }

    /// Transfers items from this inventory to another inventory.
    ///
    /// Returns the number of items successfully transferred.
    pub fn transfer_to(&mut self, other: &mut Inventory, item_id: ItemId, quantity: u32) -> Result<u32, InventoryError> {
        if quantity == 0 {
            return Ok(0);
        }

        // First check if we have enough items
        let mut available_quantity = 0u32;
        for slot in &self.slots {
            if let Some(stack) = slot.get_item_stack() {
                if stack.item_id == item_id {
                    available_quantity += stack.quantity;
                }
            }
        }

        if available_quantity < quantity {
            return Err(InventoryError::InsufficientItems);
        }

        // Transfer items
        let mut remaining_quantity = quantity;
        let mut total_transferred = 0;

        // Remove items from this inventory first
        for slot in &mut self.slots {
            if let Some(stack) = slot.get_item_stack_mut() {
                if stack.item_id == item_id {
                    let to_transfer = std::cmp::min(remaining_quantity, stack.quantity);
                    
                    // Add to the other inventory
                    let transfer_stack = ItemStack::new(item_id, to_transfer);
                    let inserted = other.insert_stack(transfer_stack)
                        .map_err(|_| InventoryError::StackOverflow)?;
                    
                    // Remove from this inventory
                    stack.remove_quantity(to_transfer)
                        .map_err(|_| InventoryError::InsufficientItems)?;
                    
                    if stack.quantity == 0 {
                        slot.set_item_stack(None);
                    }
                    
                    remaining_quantity -= to_transfer;
                    total_transferred += inserted;
                    
                    if remaining_quantity == 0 {
                        return Ok(total_transferred);
                    }
                }
            }
        }

        // Should not reach here due to the initial check, but just in case
        Ok(total_transferred)
    }

    /// Gets a reference to a slot at the specified index.
    ///
    /// Returns an error if the index is out of bounds.
    pub fn get_slot(&self, index: usize) -> Result<&InventorySlot, InventoryError> {
        self.slots.get(index)
            .ok_or(InventoryError::InvalidCapacity)
    }

    /// Gets a mutable reference to a slot at the specified index.
    ///
    /// Returns an error if the index is out of bounds.
    pub fn get_slot_mut(&mut self, index: usize) -> Result<&mut InventorySlot, InventoryError> {
        self.slots.get_mut(index)
            .ok_or(InventoryError::InvalidCapacity)
    }
}

/// Represents the result of an inventory operation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InventoryOperationResult {
    /// Operation completed successfully with the specified number of items processed.
    Success(u32),
    
    /// Operation failed with a specific error.
    Error(InventoryError),
}

impl fmt::Display for InventoryOperationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryOperationResult::Success(count) => write!(f, "Success: {} items processed", count),
            InventoryOperationResult::Error(error) => write!(f, "Error: {}", error),
        }
    }
}

impl From<Result<u32, InventoryError>> for InventoryOperationResult {
    fn from(result: Result<u32, InventoryError>) -> Self {
        match result {
            Ok(count) => InventoryOperationResult::Success(count),
            Err(error) => InventoryOperationResult::Error(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_creation() {
        let capacity = InventoryCapacity::new(5, 64);
        let inventory = Inventory::new(capacity);
        
        assert_eq!(inventory.slot_count(), 5);
        assert_eq!(inventory.max_stack_size(), 64);
        assert!(inventory.is_empty());
        assert_eq!(inventory.occupied_slots(), 0);
        assert_eq!(inventory.available_slots(), 5);
    }

    #[test]
    fn test_inventory_with_slots() {
        let inventory = Inventory::with_slots(3);
        
        assert_eq!(inventory.slot_count(), 3);
        assert_eq!(inventory.max_stack_size(), 64);
    }

    #[test]
    fn test_is_full_and_is_empty() {
        let mut inventory = Inventory::with_slots(2);
        
        // Should be empty initially
        assert!(inventory.is_empty());
        assert!(!inventory.is_full());
        
        // Insert items to fill one slot
        let stack1 = ItemStack::new(ItemId::new(1), 32);
        inventory.insert_stack(stack1).unwrap();
        
        assert!(!inventory.is_empty());
        assert!(!inventory.is_full());
        
        // Fill the second slot
        let stack2 = ItemStack::new(ItemId::new(2), 64);
        inventory.insert_stack(stack2).unwrap();
        
        assert!(!inventory.is_empty());
        assert!(inventory.is_full());
    }

    #[test]
    fn test_occupied_and_available_slots() {
        let mut inventory = Inventory::with_slots(3);
        
        // All slots should be available initially
        assert_eq!(inventory.occupied_slots(), 0);
        assert_eq!(inventory.available_slots(), 3);
        
        // Insert items
        let stack1 = ItemStack::new(ItemId::new(1), 10);
        inventory.insert_stack(stack1).unwrap();
        
        assert_eq!(inventory.occupied_slots(), 1);
        assert_eq!(inventory.available_slots(), 2);
    }

    #[test]
    fn test_insert_stack() {
        let mut inventory = Inventory::with_slots(2);
        
        // Insert first stack
        let stack1 = ItemStack::new(ItemId::new(1), 30);
        let inserted = inventory.insert_stack(stack1).unwrap();
        assert_eq!(inserted, 30);
        
        // Insert second stack of different type
        let stack2 = ItemStack::new(ItemId::new(2), 40);
        let inserted2 = inventory.insert_stack(stack2).unwrap();
        assert_eq!(inserted2, 40);
        
        // Try to insert more than available capacity
        let stack3 = ItemStack::new(ItemId::new(3), 100);
        let inserted3 = inventory.insert_stack(stack3).unwrap();
        assert_eq!(inserted3, 64); // Only 64 can fit in remaining slot
        
        assert_eq!(inventory.occupied_slots(), 2);
    }

    #[test]
    fn test_insert_stack_same_item() {
        let mut inventory = Inventory::with_slots(2);
        
        // Insert first stack
        let stack1 = ItemStack::new(ItemId::new(1), 30);
        let inserted = inventory.insert_stack(stack1).unwrap();
        assert_eq!(inserted, 30);
        
        // Insert more of the same item type - should combine
        let stack2 = ItemStack::new(ItemId::new(1), 40);
        let inserted2 = inventory.insert_stack(stack2).unwrap();
        assert_eq!(inserted2, 40); // All 40 should fit in existing stack
        
        // Check that we have 70 items total
        let mut count = 0;
        for slot in &inventory.slots {
            if let Some(stack) = slot.get_item_stack() {
                if stack.item_id == ItemId::new(1) {
                    count += stack.quantity;
                }
            }
        }
        assert_eq!(count, 70);
    }

    #[test]
    fn test_remove_items() {
        let mut inventory = Inventory::with_slots(2);
        
        // Insert some items
        let stack1 = ItemStack::new(ItemId::new(1), 50);
        let stack2 = ItemStack::new(ItemId::new(2), 30);
        inventory.insert_stack(stack1).unwrap();
        inventory.insert_stack(stack2).unwrap();
        
        // Remove some items
        let removed = inventory.remove_items(ItemId::new(1), 20).unwrap();
        assert_eq!(removed, 20);
        
        // Try to remove more than available
        let result = inventory.remove_items(ItemId::new(1), 40);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_stack() {
        let mut inventory = Inventory::with_slots(2);
        
        // Insert some items
        let stack1 = ItemStack::new(ItemId::new(1), 50);
        inventory.insert_stack(stack1).unwrap();
        
        // Extract the stack
        let extracted = inventory.extract_stack(ItemId::new(1)).unwrap();
        assert_eq!(extracted.item_id, ItemId::new(1));
        assert_eq!(extracted.quantity, 50);
        
        // Try to extract non-existent item
        let result = inventory.extract_stack(ItemId::new(2));
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_to() {
        let mut inv1 = Inventory::with_slots(2);
        let mut inv2 = Inventory::with_slots(2);
        
        // Insert items in first inventory
        let stack1 = ItemStack::new(ItemId::new(1), 50);
        inv1.insert_stack(stack1).unwrap();
        
        // Transfer to second inventory
        let transferred = inv1.transfer_to(&mut inv2, ItemId::new(1), 30).unwrap();
        assert_eq!(transferred, 30);
        
        // Check that items were moved correctly
        let mut count1 = 0;
        for slot in &inv1.slots {
            if let Some(stack) = slot.get_item_stack() {
                if stack.item_id == ItemId::new(1) {
                    count1 += stack.quantity;
                }
            }
        }
        assert_eq!(count1, 20); // 50 - 30 = 20
        
        let mut count2 = 0;
        for slot in &inv2.slots {
            if let Some(stack) = slot.get_item_stack() {
                if stack.item_id == ItemId::new(1) {
                    count2 += stack.quantity;
                }
            }
        }
        assert_eq!(count2, 30);
        
        // Try to transfer more than available
        let result = inv1.transfer_to(&mut inv2, ItemId::new(1), 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_slot() {
        let inventory = Inventory::with_slots(3);
        
        // Get valid slot
        let slot = inventory.get_slot(1).unwrap();
        assert!(slot.is_empty());
        
        // Try to get invalid slot
        let result = inventory.get_slot(5);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_slot_mut() {
        let mut inventory = Inventory::with_slots(3);
        
        // Get mutable reference to valid slot
        let slot = inventory.get_slot_mut(1).unwrap();
        assert!(slot.is_empty());
        
        // Try to get invalid slot
        let result = inventory.get_slot_mut(5);
        assert!(result.is_err());
    }

    #[test]
    fn test_inventory_operation_result() {
        let success_result: InventoryOperationResult = Ok(10).into();
        assert_eq!(format!("{}", success_result), "Success: 10 items processed");
        
        let error_result: InventoryOperationResult = Err(InventoryError::InsufficientItems).into();
        assert_eq!(format!("{}", error_result), "Error: Insufficient items in inventory");
    }
}