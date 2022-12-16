use glium::glutin::event::{KeyboardInput, ElementState, MouseButton};

use super::game_key::GameKey;

pub struct Input {
	pub game_keys_keyboard: [bool; GameKey::Count.get_id()],
	game_keys_gamepad: [bool; GameKey::Count.get_id()],
	keys_pressed_last: [bool; GameKey::Count.get_id()],
	pub aspect_ratio: f32,
	pub window_size: [u32; 2],
	pub mouse_pos: [u32; 2],
	pub key_chars: Vec<char>,
}

impl Input {
	pub fn new() -> Self {
		Self {
			game_keys_keyboard: [false; GameKey::Count.get_id()],
			game_keys_gamepad: [false; GameKey::Count.get_id()],
			keys_pressed_last: [false; GameKey::Count.get_id()],
			aspect_ratio: 0.,
			window_size: [0, 0],
			mouse_pos: [0, 0],
			key_chars: Vec::new(),
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

	pub fn mouse_press(&mut self, state: ElementState, button: MouseButton) {
		if matches!(button, MouseButton::Left) {
			self.game_keys_keyboard[GameKey::GUIInteract.get_id()] = match state {
				ElementState::Pressed => true,
				ElementState::Released => false,
			}
		}
	}

	pub fn get_game_key(&self, game_key: GameKey) -> bool {
		self.get_game_key_via_id(game_key.get_id())
	}

	pub fn get_game_key_via_id(&self, game_key: usize) -> bool {
		self.game_keys_keyboard[game_key] || self.game_keys_gamepad[game_key]
	}

	pub fn get_game_key_starting_now(&self, game_key: GameKey) -> bool {
		let id = game_key.get_id();
		self.get_game_key_via_id(id) & !self.keys_pressed_last[id]
	}

	pub fn get_mouse_pos_as_gui_pos(&self) -> [f32; 2] {
		[self.mouse_pos[0] as f32 * 256. / self.window_size[0] as f32 * self.aspect_ratio, self.mouse_pos[1] as f32 * 256. / self.window_size[1] as f32]
	}

	pub fn update_keys_pressed_last(&mut self) {
		for x in 0..GameKey::Count.get_id() {
			self.keys_pressed_last[x] = self.get_game_key_via_id(x);
		}
	}
}