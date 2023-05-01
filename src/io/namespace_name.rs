use crate::error::Error;

#[derive(Debug)]
pub enum NamespaceName {
	Tile,
	Item,
	Entity,
	Direction4,
	EntityActionStates,
	Difficulty,
}

impl NamespaceName {
	pub fn from_name(name: &String) -> Result<Self, Error> {
		match name.as_str() {
			"tile" => Ok(Self::Tile),
			"item" => Ok(Self::Item),
			"entity" => Ok(Self::Entity),
			"direction_4" => Ok(Self::Direction4),
			"entity_action_state" => Ok(Self::EntityActionStates),
			"difficulty" => Ok(Self::Difficulty),
			_ => Err(Error::IDOutOfNamespaceBounds),
		}
	}
}