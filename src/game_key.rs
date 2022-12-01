use glium::glutin::event::VirtualKeyCode;

pub enum GameKey {
	WalkNorth,
	WalkEast,
	WalkSouth,
	WalkWest,
	Count,
}

impl GameKey {
	pub const fn get_id(self) -> usize {
		match self {
			Self::WalkNorth => 0,
			Self::WalkEast => 1,
			Self::WalkSouth => 2,
			Self::WalkWest => 3,
			Self::Count => 4,
		}
	}

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