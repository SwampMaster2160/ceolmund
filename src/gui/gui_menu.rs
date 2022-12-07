use crate::{render::{vertex::Vertex, render::gui_pos_to_screen_pos}, io::input::Input};

use super::gui_alignment::GUIAlignment;

pub enum GUIMenu {
	Test,
}

impl GUIMenu {
	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &Input) {
		/*vertices.extend([
			Vertex { color: [1., 0., 0., 1.], position: gui_pos_to_screen_pos([0, 0], GUIAlignment::Right, input), texture_position: [0., 0.] },
			Vertex { color: [1., 0., 0., 1.], position: gui_pos_to_screen_pos([256, 0], GUIAlignment::Right, input), texture_position: [0., 0.] },
			Vertex { color: [1., 0., 0., 1.], position: gui_pos_to_screen_pos([0, 256], GUIAlignment::Right, input), texture_position: [0., 0.] },
		]);*/
	}
}