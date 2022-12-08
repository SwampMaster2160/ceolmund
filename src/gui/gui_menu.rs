use crate::{render::{vertex::Vertex, render::{render_gui_char, render_gui_string}, render_data::RenderData}, io::input::Input};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement};

pub enum GUIMenu {
	Test,
}

impl GUIMenu {
	pub fn get_const_elements(&self) -> Vec<GUIElement> {
		match self {
			Self::Test => vec![
				GUIElement::Rect { pos: [10, 10], size: [10, 10], alignment: GUIAlignment::Left, color: [255, 0, 0, 255] },
				GUIElement::Button { pos: [30, 20], size: [30, 15], alignment: GUIAlignment::Left },
				GUIElement::Text { string: "Hello", pos: [50, 40], alignment: GUIAlignment::Left, text_alignment: GUIAlignment::Left },
			],
		}
	}

	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &Input, render_data: &RenderData) {
		for element in self.get_const_elements() {
			element.render(vertices, input, render_data);
		}
		//let (ver, width) = render_gui_char('A', [120, 100], GUIAlignment::Left, input, render_data);
		//vertices.extend(ver);
		//render_gui_string("Hello", [256, 100], GUIAlignment::Right, GUIAlignment::Right, input, render_data, vertices);
	}
}