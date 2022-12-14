use crate::{render::{vertex::Vertex, render_data::RenderData}, io::{input::Input, game_key::GameKey}, world::world::World};

use super::{gui_menu::GUIMenu};

pub struct GUI {
	pub menus: Vec<GUIMenu>,
	pub should_close_game: bool,
}

impl GUI {
	pub fn render(&self, input: &Input, render_data: &RenderData) -> Vec<Vertex> {
		let mut vertices: Vec<Vertex> = Vec::new();
		for menu in &self.menus {
			menu.render(&mut vertices, input, render_data);
		}
		vertices
	}

	pub fn new() -> Self {
		Self {
			menus: vec![GUIMenu::Title],
			should_close_game: false,
		}
	}

	pub fn tick(&mut self, world: &mut Option<World>, input: &mut Input, render_data: &RenderData) {
		if let Some(top_menu) = self.menus.last_mut() {
			for element in top_menu.get_elements().iter_mut() {
				element.tick_mut_self(world, input, render_data);
			}
		}
		if let Some(top_menu) = self.menus.last_mut().cloned() {
			for element in top_menu.get_elements() {
				element.tick_mut_gui(self, world, input, render_data);
			}
		}
		if input.get_game_key_starting_now(GameKey::MenuOpenClose) {
			if let Some(top_menu) = self.menus.last_mut().cloned() {
				top_menu.menu_close_button_action(self, world, input, render_data);
			}
		}
		if let Some(top_menu) = self.menus.last_mut().cloned() {
			top_menu.tick(self, world, input, render_data, input.get_game_key(GameKey::CloseGame));
		}
	}

	pub fn does_menu_pause_game(&self) -> bool {
		self.menus.iter().any(|menu| menu.does_menu_pause_game())
	}
}