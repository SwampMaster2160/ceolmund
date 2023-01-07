use crate::{render::vertex::Vertex, io::{io::IO, game_key::GameKey}, world::world::World};

use super::{gui_menu::GUIMenu, gui_menu_variant::GUIMenuVariant};

/// The main GUI struct containing all the GUI layers.
pub struct GUI {
	pub menus: Vec<GUIMenu>,
	pub should_close_game: bool,
}

impl GUI {
	/// Render the GUI
	pub fn render(&self, io: &IO, world: &Option<World>) -> Vec<Vertex> {
		let mut vertices: Vec<Vertex> = Vec::new();
		for menu in &self.menus {
			menu.render(&mut vertices, io, world);
		}
		vertices
	}

	pub fn new() -> Self {
		Self {
			menus: vec![GUIMenu::new(GUIMenuVariant::Title)],
			should_close_game: false,
		}
	}

	/// Tick the GUI
	pub fn tick(&mut self, world: &mut Option<World>, io: &mut IO) {
		// Tick GUI elements of the top layer.
		if let Some(top_menu) = self.menus.last_mut() {
			for element in &mut top_menu.extra_elements {
				element.tick_mut_self(world, io, [0, 0]);
			}
		}
		if let Some(top_menu) = self.menus.last_mut().cloned() {
			for element in top_menu.get_elements(world) {
				element.tick_mut_gui(self, world, io);
			}
		}
		// When Esc is pressed, call on top menu.
		if io.get_game_key_starting_now(GameKey::MenuOpenClose) {
			if let Some(top_menu) = self.menus.last_mut().cloned() {
				top_menu.menu_close_button_action(self, world, io);
			}
		}
		// Tick the top menu
		if let Some(top_menu) = self.menus.last_mut().cloned() {
			top_menu.tick(self, world, io, io.get_game_key(GameKey::CloseGame));
		}
	}

	/// Weather the menu will pause the game or not.
	pub fn does_menu_pause_game(&self) -> bool {
		self.menus.iter().any(|menu| menu.does_menu_pause_game())
	}
}