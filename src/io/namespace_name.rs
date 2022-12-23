pub enum NamespaceName {
	Tile,
}

impl NamespaceName {
	pub fn from_name(name: &String) -> Option<Self> {
		match name.as_str() {
			"tile" => Some(Self::Tile),
			_ => None,
		}
	}
}