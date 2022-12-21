use std::{path::PathBuf, fs::create_dir};
use crate::world::tile::tile::TileVariant;

use crc64::crc64;
use glium::glutin::event::{KeyboardInput, ElementState, MouseButton};
use home::home_dir;
use tokio::runtime::Runtime;

use super::{game_key::GameKey, formatted_file_writer::FormattedFileWriter};

/// For everything hardware related.
pub struct IO {
	pub game_keys_keyboard: [bool; GameKey::Count.get_id()],
	game_keys_gamepad: [bool; GameKey::Count.get_id()],
	keys_pressed_last: [bool; GameKey::Count.get_id()],
	pub aspect_ratio: f32,
	pub window_size: [u32; 2],
	pub mouse_pos: [u32; 2],
	pub key_chars: Vec<char>,
	pub home_path: PathBuf,
	pub worlds_path: PathBuf,
	pub char_widths: Vec<u8>,
	pub async_runtime: Runtime,
	pub saving_namespace: Vec<u8>,
	pub saving_namespace_hash: u64,
}

impl IO {
	pub fn new() -> Self {
		// Get and create paths
		let mut home_path = home_dir().unwrap();
		home_path.push(".ceolmund");
		let mut worlds_path = home_path.clone();
		worlds_path.push("worlds");
		create_dir(&home_path).ok();
		create_dir(&worlds_path).ok();
		// Get the widths of chars in the gui.
		let mut char_widths = Vec::new();
		char_widths.extend(include_bytes!("../asset/render_width/0.cwt"));
		char_widths.extend(include_bytes!("../asset/render_width/1.cwt"));
		char_widths.extend(include_bytes!("../asset/render_width/2.cwt"));
		// Create namespace for saving worlds
		let mut saving_namespace = FormattedFileWriter::new(0);
		// Add tile namespace
		let tile_name_ptr =  saving_namespace.push_string(&"tile".to_string()).unwrap();
		saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		for tile_variant in TileVariant::get_variant_array() {
			let tile_name_ptr =  saving_namespace.push_string(&tile_variant.get_name_id().to_string()).unwrap();
			saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		}
		saving_namespace.body.extend(0u32.to_le_bytes());

		saving_namespace.body.extend(0u32.to_le_bytes());
		let saving_namespace = saving_namespace.write_to_vec().unwrap();

		Self {
			game_keys_keyboard: [false; GameKey::Count.get_id()],
			game_keys_gamepad: [false; GameKey::Count.get_id()],
			keys_pressed_last: [false; GameKey::Count.get_id()],
			aspect_ratio: 0.,
			window_size: [0, 0],
			mouse_pos: [0, 0],
			key_chars: Vec::new(),
			home_path,
			worlds_path,
			char_widths,
			async_runtime: Runtime::new().unwrap(),
			saving_namespace: saving_namespace.clone(),
			saving_namespace_hash: crc64(0, saving_namespace.as_slice()),
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