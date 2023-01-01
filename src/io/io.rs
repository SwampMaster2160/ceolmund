use crate::world::entity::entity_action_state::EntityActionStateVariant;
use crate::world::{entity::entity_type::EntityVariant, direction::Direction4};
use crate::world::item::item::ItemVariant;
use std::{path::PathBuf, fs::create_dir};
use crate::world::tile::tile::TileVariant;

use crc64::crc64;
use glium::glutin::{event::{KeyboardInput, ElementState, MouseButton}, dpi::PhysicalSize};
use home::home_dir;
use strum::{EnumCount, IntoEnumIterator};
use tokio::runtime::Runtime;

use super::{game_key::GameKey, formatted_file_writer::FormattedFileWriter};

pub const SERIALIZATION_VERSION: u32 = 1;

/// For everything hardware related.
pub struct IO {
	pub game_keys_keyboard: [bool; GameKey::COUNT],
	game_keys_gamepad: [bool; GameKey::COUNT],
	keys_pressed_last: [bool; GameKey::COUNT],
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
		let mut saving_namespace = FormattedFileWriter::new();
		// Add version to namespace
		let version = SERIALIZATION_VERSION.to_le_bytes();
		saving_namespace.body.extend(&version);
		// Add tile namespace
		let tile_name_ptr =  saving_namespace.push_string(&"tile".to_string()).unwrap();
		saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		for tile_variant in TileVariant::iter() {
			let tile_name_ptr =  saving_namespace.push_string(&tile_variant.get_name_id().to_string()).unwrap();
			saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		}
		saving_namespace.body.extend(0xFFFFFFFFu32.to_le_bytes());
		// Add item namespace
		let tile_name_ptr =  saving_namespace.push_string(&"item".to_string()).unwrap();
		saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		for tile_variant in ItemVariant::iter() {
			let tile_name_ptr =  saving_namespace.push_string(&tile_variant.get_name_id().to_string()).unwrap();
			saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		}
		saving_namespace.body.extend(0xFFFFFFFFu32.to_le_bytes());
		// Add entity namespace
		let tile_name_ptr =  saving_namespace.push_string(&"entity".to_string()).unwrap();
		saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		for tile_variant in EntityVariant::iter() {
			let tile_name_ptr =  saving_namespace.push_string(&tile_variant.get_name_id().to_string()).unwrap();
			saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		}
		saving_namespace.body.extend(0xFFFFFFFFu32.to_le_bytes());
		// Add direction 4 namespace
		let tile_name_ptr =  saving_namespace.push_string(&"direction_4".to_string()).unwrap();
		saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		for tile_variant in Direction4::iter() {
			let tile_name_ptr =  saving_namespace.push_string(&tile_variant.get_name_id().to_string()).unwrap();
			saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		}
		saving_namespace.body.extend(0xFFFFFFFFu32.to_le_bytes());
		// Add entity action state namespace
		let tile_name_ptr =  saving_namespace.push_string(&"entity_action_state".to_string()).unwrap();
		saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		for tile_variant in EntityActionStateVariant::iter() {
			let tile_name_ptr =  saving_namespace.push_string(&tile_variant.get_name_id().to_string()).unwrap();
			saving_namespace.body.extend(tile_name_ptr.to_le_bytes());
		}
		saving_namespace.body.extend(0xFFFFFFFFu32.to_le_bytes());
		// End namespaces
		saving_namespace.body.extend(0xFFFFFFFFu32.to_le_bytes());
		let saving_namespace = saving_namespace.write_to_vec().unwrap();
		// Get namespace hash
		let namespace_hash = crc64(0, saving_namespace.as_slice());

		Self {
			game_keys_keyboard: [false; GameKey::COUNT],
			game_keys_gamepad: [false; GameKey::COUNT],
			keys_pressed_last: [false; GameKey::COUNT],
			aspect_ratio: 0.,
			window_size: [0, 0],
			mouse_pos: [0, 0],
			key_chars: Vec::new(),
			home_path,
			worlds_path,
			char_widths,
			async_runtime: Runtime::new().unwrap(),
			saving_namespace: saving_namespace.clone(),
			saving_namespace_hash: namespace_hash,
		}
	}

	/// Set a game key as pressed or unpressed.
	pub fn key_press(&mut self, keyboard_input: &KeyboardInput) {
		for game_key in GameKey::from_key_code(keyboard_input.virtual_keycode) {
			self.game_keys_keyboard[game_key as usize] = keyboard_input.state == ElementState::Pressed;
		}
	}
	
	#[cfg(not(windows))]
	pub fn poll_gamepad(&mut self) {}

	#[cfg(windows)]
	pub fn poll_gamepad(&mut self) {
		let handle = rusty_xinput::XInputHandle::load_default().unwrap();
		for x in 0..4 {
			if let Ok(gamepad) = handle.get_state(x) {
				self.game_keys_gamepad[GameKey::WalkNorth as usize] = gamepad.arrow_up();
				self.game_keys_gamepad[GameKey::WalkEast as usize] = gamepad.arrow_right();
				self.game_keys_gamepad[GameKey::WalkSouth as usize] = gamepad.arrow_down();
				self.game_keys_gamepad[GameKey::WalkWest as usize] = gamepad.arrow_left();
			}
		}
	}

	/// Update weather the mouse is clicking or not.
	pub fn mouse_press(&mut self, state: ElementState, button: MouseButton) {
		if matches!(button, MouseButton::Left) {
			self.game_keys_keyboard[GameKey::GUIInteract as usize] = state == ElementState::Pressed;
		}
	}

	/// Get weather a game key is being pressed or not.
	pub fn get_game_key(&self, game_key: GameKey) -> bool {
		self.get_game_key_via_id(game_key as usize)
	}

	/// Get weather a game key (selected via it's id) is being pressed or not.
	pub fn get_game_key_via_id(&self, game_key: usize) -> bool {
		self.game_keys_keyboard[game_key] || self.game_keys_gamepad[game_key]
	}

	/// Get the weather a key is being pressed or not, starting since update_keys_pressed_last() was last called.
	pub fn get_game_key_starting_now(&self, game_key: GameKey) -> bool {
		let id = game_key as usize;
		self.get_game_key_via_id(id) & !self.keys_pressed_last[id]
	}

	pub fn get_mouse_pos_as_gui_pos(&self) -> [f32; 2] {
		[self.mouse_pos[0] as f32 * 256. / self.window_size[0] as f32 * self.aspect_ratio, self.mouse_pos[1] as f32 * 256. / self.window_size[1] as f32]
	}

	/// All keys pressed will be set as pressed last.
	pub fn update_keys_pressed_last(&mut self) {
		for x in 0..GameKey::COUNT {
			self.keys_pressed_last[x] = self.get_game_key_via_id(x);
		}
	}

	/// Updates window_size and aspect_ratio. Should be called when the game's window size changes.
	pub fn set_window_size(&mut self, size: &PhysicalSize<u32>) {
		self.window_size = [size.width, size.height];
		self.aspect_ratio = size.width as f32 / size.height as f32;
	}
}