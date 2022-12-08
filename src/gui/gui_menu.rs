use crate::{render::{vertex::Vertex, render::{gui_pos_to_screen_pos, render_gui_rect}}, io::input::Input};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement};

pub enum GUIMenu {
	Test,
}

impl GUIMenu {
	pub const fn get_const_elements(&self) -> &'static [GUIElement] {
		match self {
			Self::Test => &[
				GUIElement::Rect { pos: [10, 10], size: [10, 10], alignment: GUIAlignment::Left, color: [255, 0, 0, 255] },
				GUIElement::Button { pos: [30, 20], size: [30, 15], alignment: GUIAlignment::Left },
			],
		}
	}

	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &Input) {
		for element in self.get_const_elements() {
			element.render(vertices, input);
		}
	}
}