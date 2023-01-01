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
	pub fn from_name(name: &String) -> Option<Self> {
		match name.as_str() {
			"tile" => Some(Self::Tile),
			"item" => Some(Self::Item),
			"entity" => Some(Self::Entity),
			"direction_4" => Some(Self::Direction4),
			"entity_action_state" => Some(Self::EntityActionStates),
			"difficulty" => Some(Self::Difficulty),
			_ => None,
		}
	}
}