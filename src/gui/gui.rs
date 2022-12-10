use crate::{render::{vertex::Vertex, render_data::RenderData}, io::input::Input, world::world::World};

use super::gui_menu::GUIMenu;

pub struct GUI {
	menus: Vec<GUIMenu>,
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
			menus: vec![GUIMenu::Test],
		}
	}

	pub fn tick(&mut self, world: &Option<World>, input: &Input, render_data: &RenderData) {
		if let Some(top_menu) = self.menus.last() {
			for element in top_menu.get_elements() {

			}
		}
	}
}