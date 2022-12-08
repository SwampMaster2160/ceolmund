use crate::{io::input::Input, render::{vertex::Vertex, render::{render_gui_rect, gui_pos_to_screen_pos, gui_size_to_screen_size}}};

use super::gui_alignment::GUIAlignment;

pub enum GUIElement {
	Rect {pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4]},
	Button {pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment},
}

impl GUIElement {
	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &Input) {
		match self {
			Self::Rect{pos, size, alignment, color} =>
				vertices.extend(render_gui_rect(*pos, *size, *alignment, *color, input)),
			Self::Button { pos, size, alignment } => {
				let mut color = [0, 255, 0, 255];
				let button_screen_pos = gui_pos_to_screen_pos(*pos, *alignment, input);
				let button_screen_size = gui_size_to_screen_size(*size, input);
				let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
				let mouse_pos = input.get_mouse_pos_as_gui_pos(*alignment);
				if mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] && mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1] {
					color = [0, 0, 255, 255];
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, input));
			}
		}
	}
}