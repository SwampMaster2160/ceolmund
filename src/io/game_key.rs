use glium::glutin::event::VirtualKeyCode;

#[derive(Clone, Copy)]
/// A game key that can be translated from multiple real keys/buttons and represents an action like walking up/down ect..
pub enum GameKey {
	WalkNorth,
	WalkEast,
	WalkSouth,
	WalkWest,
	GUIInteract,
	MenuOpenClose,
	CloseGame,
	DeleteTile,
	Build1,
	Build2,
	Build3,
	Build4,
	Build5,
	Build6,
	Build7,
	Build8,
	Build9,
	Build0,
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
			Self::MenuOpenClose => 5,
			Self::CloseGame => 6,
			Self::DeleteTile => 7,
			Self::Build1 => 8,
			Self::Build2 => 9,
			Self::Build3 => 10,
			Self::Build4 => 11,
			Self::Build5 => 12,
			Self::Build6 => 13,
			Self::Build7 => 14,
			Self::Build8 => 15,
			Self::Build9 => 16,
			Self::Build0 => 17,
			Self::Count => 18, // Total amount of game keys excluding the count value
		}
	}

	/// What real keys translate to what game keys.
	pub fn from_key_code(key_code: VirtualKeyCode) -> Vec<Self> {
		match key_code {
			VirtualKeyCode::W => vec![Self::WalkNorth],
			VirtualKeyCode::A => vec![Self::WalkWest],
			VirtualKeyCode::S => vec![Self::WalkSouth],
			VirtualKeyCode::D => vec![Self::WalkEast],
			VirtualKeyCode::Escape => vec![Self::MenuOpenClose],
			VirtualKeyCode::Delete => vec![Self::DeleteTile],
			VirtualKeyCode::Key0 => vec![Self::Build0],
			VirtualKeyCode::Key1 => vec![Self::Build1],
			VirtualKeyCode::Key2 => vec![Self::Build2],
			VirtualKeyCode::Key3 => vec![Self::Build3],
			VirtualKeyCode::Key4 => vec![Self::Build4],
			VirtualKeyCode::Key5 => vec![Self::Build5],
			VirtualKeyCode::Key6 => vec![Self::Build6],
			VirtualKeyCode::Key7 => vec![Self::Build7],
			VirtualKeyCode::Key8 => vec![Self::Build8],
			VirtualKeyCode::Key9 => vec![Self::Build9],
			_ => Vec::new(),
		}
	}
}