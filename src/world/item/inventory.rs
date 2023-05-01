use crate::{io::{file_writer::FileWriter, file_reader::FileReader, namespace::Namespace}, error::Error};

use super::{item::Item, item_category::ItemCategory};

/// An inventory generic with a slot count constant argument. Contains that amount of item slots.
#[derive(Clone)]
pub struct Inventory<const SLOT_COUNT: usize> {
	pub items: Box<[(Item, u8); SLOT_COUNT]>,
}

impl<const SLOT_COUNT: usize> Inventory<SLOT_COUNT> {
	/// Creates a new blank inventory.
	pub fn new() -> Self {
		Self {
			items: Box::new([(); SLOT_COUNT].map(|_| (Item::None, 0)))
		}
	}

	// Add an am,ount of an item to the inventory, get back what could not be added.
	pub fn add_items(&mut self, to_add: (Item, u16)) -> (Item, u16) {
		let item_to_add = to_add.0;
		let mut amount_left_to_add = to_add.1;
		// Return if the item is none.
		if item_to_add.is_none() {
			return (Item::None, 0);
		}
		// Add to existing stacks.
		for (stack_item, stack_amount) in self.items.iter_mut() {
			// Skip stacks of a diffrent item.
			if item_to_add != *stack_item {
				continue;
			}
			// Add item to the stack and take from the amount left to add.
			let amount_to_add_to_stack = 99u16.saturating_sub(*stack_amount as u16).min(amount_left_to_add) as u8;
			*stack_amount += amount_to_add_to_stack;
			amount_left_to_add -= amount_to_add_to_stack as u16;
			// Return is there is nothing left to add.
			if amount_left_to_add == 0 {
				return (Item::None, 0);
			}
		}
		// Create new stacks.
		for (stack_item, stack_amount) in self.items.iter_mut() {
			// Skip stacks that contain items.
			if !stack_item.is_none() {
				continue;
			}
			// Add item to the stack and take from the amount left to add.
			*stack_item = item_to_add.clone();
			let to_add = amount_left_to_add.min(99) as u8;
			*stack_amount = to_add;
			amount_left_to_add -= to_add as u16;
			// Return is there is nothing left to add.
			if amount_left_to_add == 0 {
				return (Item::None, 0);
			}
		}
		// Return leftover.
		(item_to_add, amount_left_to_add)
	}

	pub fn swap_items(&mut self, a: usize, b: usize) -> Option<()> {
		let item_stack_a = self.items.get(a)?.clone();
		let item_stack_b = self.items.get(b)?.clone();
		*self.items.get_mut(a)? = item_stack_b;
		*self.items.get_mut(b)? = item_stack_a;
		Some(())
	}

	/// Count how many of an item are in the stack saturating at the u16 limit.
	pub fn count_items(&self, item_category: &ItemCategory) -> u16 {
		self.items.iter().filter(|stack| item_category.has_item(&stack.0)).fold(0, |count, stack| count.saturating_add(stack.1 as u16))
	}

	/// Try remove the items and return false if it failed. There should not be two item stacks of the same item.
	pub fn try_remove_items(&mut self, to_remove: Vec<(ItemCategory, u16)>) -> bool {
		// Check if we have the items to remove.
		if to_remove.iter().any(|stack| self.count_items(&stack.0) < stack.1) {
			return false;
		}
		// Remove items
		for (item_to_remove, amount_to_remove) in to_remove {
			let mut amount_left_to_remove = amount_to_remove;
			for (stack_item, stack_amount) in self.items.iter_mut() {
				// Only remove items that match.
				if !item_to_remove.has_item(stack_item) {
					continue;
				}
				// Calculate how many items to remove from the stack.
				let amount_to_remove_u8 = amount_left_to_remove.try_into().unwrap_or(u8::MAX);
				let to_remove = (*stack_amount).min(amount_to_remove_u8);
				// Remove item.
				amount_left_to_remove -= to_remove as u16;
				*stack_amount -= to_remove;
				// If there are no items left on the stack then set the item to none.
				if *stack_amount == 0 {
					*stack_item = Item::None;
				}
				// Break the loop if there are no items left to remove.
				if amount_left_to_remove == 0 {
					break;
				}
			}
		}
		// Success
		true
	}

	pub fn serialize(&self, file: &mut FileWriter) {
		// Serialize each item stack and its stack amount.
		for (stack_item, stack_amount) in self.items.iter() {
			stack_item.serialize(file);
			file.push_u8(*stack_amount);
		}
	}

	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32) -> Result<Self, Error> {
		// Create blank.
		let mut inventory = Self::new();
		// Read the item type and id for each slot.
		for (stack_item, stack_amount) in inventory.items.iter_mut() {
			*stack_item = Item::deserialize(file, namespace, version)?;
			*stack_amount = file.read_u8()?;
		}
		
		Ok(inventory)
	}
}