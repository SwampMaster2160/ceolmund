use crate::{io::{input::Input, game_key::GameKey}, render::{vertex::Vertex, render::{render_gui_rect, gui_pos_to_screen_pos, gui_size_to_screen_size, render_gui_string}, render_data::{self, RenderData}}, world::world::World};

use super::{gui_alignment::GUIAlignment, gui::GUI};

const BUTTON_GRAY_COLOR: [u8; 4] = [95, 95, 95, 255];
const BUTTON_HOVER_COLOR: [u8; 4] = [95, 195, 195, 255];

#[derive(Clone)]
pub enum GUIElement<'a> {
	Rect {pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4]},
	Button {text: &'a str, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment},
	Text {text: &'a str, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment}
}

impl GUIElement<'_> {
	pub fn is_mouse_over(&self, input: &Input, render_data: &RenderData) -> bool {
		match self {
			Self::Rect{pos, size, alignment, color} => false,
			Self::Button { pos, size, alignment, text } => {
				let mouse_pos = input.get_mouse_pos_as_gui_pos(*alignment);
				let button_screen_pos = gui_pos_to_screen_pos(*pos, *alignment, input);
				let button_screen_size = gui_size_to_screen_size(*size);
				let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
				mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
				mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
			}
			Self::Text { text: string, pos, alignment, text_alignment } => false,
		}
	}

	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &Input, render_data: &RenderData) {
		match self {
			Self::Rect{pos, size, alignment, color} =>
				vertices.extend(render_gui_rect(*pos, *size, *alignment, *color, input)),
			Self::Button { pos, size, alignment, text } => {
				let mut color = BUTTON_GRAY_COLOR;
				if self.is_mouse_over(input, render_data) {
					color = BUTTON_HOVER_COLOR;
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, input));

				let text_pos = [pos[0] + size[0] / 2, pos[1] + size[1] / 2 - 8];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, input, render_data, vertices);
			}
			Self::Text { text: string, pos, alignment, text_alignment } =>
				render_gui_string(string, *pos, *alignment, *text_alignment, input, render_data, vertices),
		}
	}

	pub fn tick_mut_self(&mut self, world: &mut Option<World>, input: &Input, render_data: &RenderData) {
		if input.get_game_key(GameKey::GUIInteract) {
			println!("Hi");
		}
		(||{});
	}

	pub fn tick_mut_gui(self, gui: &mut GUI, world: &mut Option<World>, input: &Input, render_data: &RenderData) {

	}
}