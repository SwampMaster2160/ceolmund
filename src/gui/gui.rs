use crate::{render::vertex::Vertex, io::{io::IO, game_key::GameKey}, world::world::World};

use super::{gui_menu::GUIMenu, gui_menu_variant::GUIMenuVariant};

pub struct GUI {
	pub menus: Vec<GUIMenu>,
	pub should_close_game: bool,
}

impl GUI {
	pub fn render(&self, io: &IO) -> Vec<Vertex> {
		let mut vertices: Vec<Vertex> = Vec::new();
		for menu in &self.menus {
			menu.render(&mut vertices, io);
		}
		vertices
	}

	pub fn new() -> Self {
		Self {
			menus: vec![GUIMenu::new(GUIMenuVariant::Title)],
			should_close_game: false,
		}
	}

	pub fn tick(&mut self, world: &mut Option<World>, io: &mut IO) {
		if let Some(top_menu) = self.menus.last_mut() {
			for element in &mut top_menu.extra_elements {
				element.tick_mut_self(world, io);
			}
		}
		if let Some(top_menu) = self.menus.last_mut().cloned() {
			for element in top_menu.get_elements() {
				element.tick_mut_gui(self, world, io);
			}
		}
		if io.get_game_key_starting_now(GameKey::MenuOpenClose) {
			if let Some(top_menu) = self.menus.last_mut().cloned() {
				top_menu.menu_close_button_action(self, world, io);
			}
		}
		if let Some(top_menu) = self.menus.last_mut().cloned() {
			top_menu.tick(self, world, io, io.get_game_key(GameKey::CloseGame));
		}
	}

	pub fn does_menu_pause_game(&self) -> bool {
		self.menus.iter().any(|menu| menu.does_menu_pause_game())
	}
}