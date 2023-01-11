use crate::io::{file_writer::FileWriter, file_reader::FileReader, namespace::Namespace};

use super::item::Item;

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
			let amount_to_add_to_stack = 100u16.saturating_sub(*stack_amount as u16).min(amount_left_to_add) as u8;
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
			let to_add = amount_left_to_add.min(100) as u8;
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

	pub fn serialize(&self, file: &mut FileWriter) {
		for (item, stack_amount) in self.items.iter() {
			item.serialize(file);
			file.push_u8(*stack_amount);
		}
	}

	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32) -> Option<Self> {
		let mut inventory = Self::new();
		for x in 0..SLOT_COUNT {
			let item = Item::deserialize(file, namespace, version)?;
			let amount = file.read_u8()?;
			inventory.items[x] = (item, amount);
		}
		Some(inventory)
	}
}