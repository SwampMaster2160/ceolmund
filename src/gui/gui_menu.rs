use crate::{render::{vertex::Vertex, render::{render_gui_string}, render_data::RenderData}, io::input::Input, world::world::World};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement};

const RECT_COLOR: [u8; 4] = [31, 31, 31, 255];

#[derive(Clone)]
pub enum GUIMenu {
	Test,
}

impl GUIMenu {
	pub fn get_const_elements(&self) -> Vec<GUIElement> {
		match self {
			Self::Test => vec![
				GUIElement::Rect { pos: [10, 10], size: [10, 10], alignment: GUIAlignment::Left, color: RECT_COLOR },
				GUIElement::Button { pos: [30, 20], size: [30, 15], alignment: GUIAlignment::Left, text: "Hi" },
				GUIElement::Text { text: "Hello", pos: [50, 40], alignment: GUIAlignment::Left, text_alignment: GUIAlignment::Left },
			],
		}
	}

	pub fn get_elements(&self) -> Vec<GUIElement> {
		self.get_const_elements()
	}

	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &Input, render_data: &RenderData) {
		for element in self.get_elements() {
			element.render(vertices, input, render_data);
		}
	}
}