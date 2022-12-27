pub enum NamespaceName {
	Tile,
	Item,
	Entity,
	Direction4,
}

impl NamespaceName {
	pub fn from_name(name: &String) -> Option<Self> {
		match name.as_str() {
			"tile" => Some(Self::Tile),
			"item" => Some(Self::Item),
			"Entity" => Some(Self::Entity),
			"direction_4" => Some(Self::Direction4),
			_ => None,
		}
	}
}