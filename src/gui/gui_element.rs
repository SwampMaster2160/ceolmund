use unicode_segmentation::UnicodeSegmentation;

use crate::{io::{io::IO, game_key::GameKey}, render::{vertex::Vertex, render::{render_gui_rect, gui_pos_to_screen_pos, gui_size_to_screen_size, render_gui_string, render_screen_grayout}, render_data::{RenderData}}, world::world::World};

use super::{gui_alignment::GUIAlignment, gui::GUI};

const BUTTON_GRAY_COLOR: [u8; 4] = [95, 95, 95, 255];
const BUTTON_HOVER_COLOR: [u8; 4] = [95, 195, 195, 255];
const TEXT_ENTRY_GRAY_COLOR: [u8; 4] = [50, 50, 50, 255];
const TEXT_ENTRY_SELECT_COLOR: [u8; 4] = [0, 255, 255, 255];

#[derive(Clone)]
pub enum GUIElement {
	Rect {pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4]},
	Button {
		text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment,
		tick_mut_gui: fn(GUIElement, gui: &mut GUI, world: &mut Option<World>, input: &IO, render_data: &RenderData) -> (),
	},
	Text {text: String, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment},
	TextEntry {text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, is_selected: bool, text_length_limit: usize},
	Grayout {color: [u8; 4]},
}

impl GUIElement {
	pub fn is_mouse_over(&self, input: &IO, _render_data: &RenderData) -> bool {
		match self {
			Self::Button { pos, size, alignment, .. } => {
				let mouse_pos = input.get_mouse_pos_as_gui_pos();
				let button_screen_pos = gui_pos_to_screen_pos(*pos, *alignment, input);
				let button_screen_size = gui_size_to_screen_size(*size);
				let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
				mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
				mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
			}
			Self::TextEntry { pos, size, alignment, .. } => {
				let mouse_pos = input.get_mouse_pos_as_gui_pos();
				let button_screen_pos = gui_pos_to_screen_pos(*pos, *alignment, input);
				let button_screen_size = gui_size_to_screen_size(*size);
				let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
				mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
				mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
			}
			_ => false,
		}
	}

	pub fn render(&self, vertices: &mut Vec<Vertex>, input: &IO, render_data: &RenderData) {
		match self {
			Self::Rect{pos, size, alignment, color} =>
				vertices.extend(render_gui_rect(*pos, *size, *alignment, *color, input)),
			Self::Button { pos, size, alignment, text, .. } => {
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
			Self::TextEntry { text, pos, size, alignment, is_selected, .. } => {
				let mut color = TEXT_ENTRY_GRAY_COLOR;
				if self.is_mouse_over(input, render_data) {
					color = BUTTON_HOVER_COLOR;
				}
				if *is_selected {
					color = TEXT_ENTRY_SELECT_COLOR;
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, input));

				let text_pos = [pos[0] + 1, pos[1] + size[1] / 2 - 8];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Left, input, render_data, vertices);
			}
			Self::Grayout { color } => vertices.extend(render_screen_grayout(*color, input)),
		}
	}

	pub fn tick_mut_self(&mut self, _world: &mut Option<World>, input: &IO, render_data: &RenderData) {
		let is_mouse_over = self.is_mouse_over(input, render_data);
		if input.get_game_key_starting_now(GameKey::GUIInteract) {
			match self {
				GUIElement::TextEntry { is_selected, .. } => {
					*is_selected = is_mouse_over;
				},
				_ => {}
			}
		}
		match self {
			GUIElement::TextEntry { is_selected, text, text_length_limit, .. } => {
				if *is_selected {
					for chr in input.key_chars.iter() {
						if !chr.is_control() && text.graphemes(true).count() < *text_length_limit {
							text.push(*chr);
						}
						if *chr == '\x08' {
							text.pop();
						}
					}
				}
			},
			_ => {}
		}
	}

	pub fn tick_mut_gui(self, gui: &mut GUI, world: &mut Option<World>, input: &IO, render_data: &RenderData) {
		if input.get_game_key_starting_now(GameKey::GUIInteract) && self.is_mouse_over(input, render_data) {
			match self {
				GUIElement::Button { tick_mut_gui, .. } => tick_mut_gui(self, gui, world, input, render_data),
				_ => {}
			}
		}
	}
}