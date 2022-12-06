use glium::glutin::event::{KeyboardInput, ElementState};

use super::game_key::GameKey;

pub struct Input {
	game_keys_keyboard: [bool; GameKey::Count.get_id()],
	game_keys_gamepad: [bool; GameKey::Count.get_id()],
}

impl Input {
	pub fn new() -> Self {
		Self {
			game_keys_keyboard: [false; GameKey::Count.get_id()],
			game_keys_gamepad: [false; GameKey::Count.get_id()],
		}
	}

	pub fn key_press(&mut self, keyboard_input: &KeyboardInput) {
		if let Some(keycode) = keyboard_input.virtual_keycode {
			for game_key in GameKey::from_key_code(keycode) {
				self.game_keys_keyboard[game_key.get_id()] = match keyboard_input.state {
					ElementState::Pressed => true,
					ElementState::Released => false,
				}
			}
		}
	}
	
	#[cfg(not(windows))]
	pub fn poll_gamepad(&mut self) {}

	#[cfg(windows)]
	pub fn poll_gamepad(&mut self) {
		let handle = rusty_xinput::XInputHandle::load_default().unwrap();
		for x in 0..4 {
			if let Ok(gamepad) = handle.get_state(x) {
				self.game_keys_gamepad[GameKey::WalkNorth.get_id()] = gamepad.arrow_up();
				self.game_keys_gamepad[GameKey::WalkEast.get_id()] = gamepad.arrow_right();
				self.game_keys_gamepad[GameKey::WalkSouth.get_id()] = gamepad.arrow_down();
				self.game_keys_gamepad[GameKey::WalkWest.get_id()] = gamepad.arrow_left();
			}
		}
	}

	pub fn get_game_key(&self, game_key: GameKey) -> bool {
		self.game_keys_keyboard[game_key.get_id()] || self.game_keys_gamepad[game_key.get_id()]
	}
}