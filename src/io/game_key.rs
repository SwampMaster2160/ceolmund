use glium::glutin::event::VirtualKeyCode;

#[derive(Clone, Copy)]
/// A game key that can be translated from multiple real keys/buttons and represents an action like walking up/down ect..
pub enum GameKey {
	WalkNorth,
	WalkEast,
	WalkSouth,
	WalkWest,
	GUIInteract,
	Count, // A count, not to be used as an actual game key
}

impl GameKey {
	pub const fn get_id(self) -> usize {
		match self {
			Self::WalkNorth => 0,
			Self::WalkEast => 1,
			Self::WalkSouth => 2,
			Self::WalkWest => 3,
			Self::GUIInteract => 4,
			Self::Count => 5, // Total amount of game keys excluding the count value
		}
	}

	/// What real keys translate to what game keys.
	pub fn from_key_code(key_code: VirtualKeyCode) -> Vec<Self> {
		match key_code {
			VirtualKeyCode::W => vec![Self::WalkNorth],
			VirtualKeyCode::A => vec![Self::WalkWest],
			VirtualKeyCode::S => vec![Self::WalkSouth],
			VirtualKeyCode::D => vec![Self::WalkEast],
			_ => Vec::new(),
		}
	}
}