use glium::glutin::event::{KeyboardInput, ElementState};

use super::game_key::GameKey;

pub struct Input {
	game_keys: [bool; GameKey::Count.get_id()]
}

impl Input {
	pub fn new() -> Self {
		Self {
			game_keys: [false; GameKey::Count.get_id()]
		}
	}

	pub fn key_press(&mut self, keyboard_input: &KeyboardInput) {
		if let Some(keycode) = keyboard_input.virtual_keycode {
			for game_key in GameKey::from_key_code(keycode) {
				self.game_keys[game_key.get_id()] = match keyboard_input.state {
					ElementState::Pressed => true,
					ElementState::Released => false,
				}
			}
		}
	}

	pub fn get_game_key(&self, game_key: GameKey) -> bool {
		self.game_keys[game_key.get_id()]
	}
}