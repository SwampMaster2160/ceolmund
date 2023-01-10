use crate::{io::{file_writer::FileWriter, file_reader::FileReader, namespace::Namespace}};

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