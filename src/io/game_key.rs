use glium::glutin::event::VirtualKeyCode;
use strum_macros::EnumCount;

#[derive(Clone, Copy, EnumCount)]
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
	OpenTestMenu,
	InventoryUp,
	InventoryDown,
	InventoryLeft,
	InventoryRight,
	Interact,
}

impl GameKey {
	/// What real keys translate to what game keys.
	pub fn from_key_code(key_code: Option<VirtualKeyCode>) -> Vec<Self> {
		let key_code = match key_code {
			Some(key_code) => key_code,
			None => return Vec::new(),
		};
		match key_code {
			VirtualKeyCode::W => vec![Self::WalkNorth],
			VirtualKeyCode::A => vec![Self::WalkWest],
			VirtualKeyCode::S => vec![Self::WalkSouth],
			VirtualKeyCode::D => vec![Self::WalkEast],
			VirtualKeyCode::Escape => vec![Self::MenuOpenClose],
			VirtualKeyCode::Delete => vec![Self::DeleteTile],
			VirtualKeyCode::T => vec![Self::OpenTestMenu],
			VirtualKeyCode::Up => vec![Self::InventoryUp],
			VirtualKeyCode::Down => vec![Self::InventoryDown],
			VirtualKeyCode::Left => vec![Self::InventoryLeft],
			VirtualKeyCode::Right => vec![Self::InventoryRight],
			VirtualKeyCode::Return => vec![Self::Interact],
			_ => Vec::new(),
		}
	}
}