use unicode_segmentation::UnicodeSegmentation;

use crate::{io::{io::IO, game_key::GameKey}, render::{vertex::Vertex, render::{render_gui_rect, gui_pos_to_screen_pos, gui_size_to_screen_size, render_gui_string, render_screen_grayout}, texture::Texture}, world::world::World};

use super::{gui_alignment::GUIAlignment, gui::GUI};

const BUTTON_GRAY_COLOR: [u8; 4] = [95, 95, 95, 255];
const BUTTON_HOVER_COLOR: [u8; 4] = [95, 195, 195, 255];
const BUTTON_DISABLED_COLOR: [u8; 4] = [15, 15, 15, 255];
const TEXT_ENTRY_GRAY_COLOR: [u8; 4] = [50, 50, 50, 255];
const TEXT_ENTRY_SELECT_COLOR: [u8; 4] = [0, 255, 255, 255];

/// A GUI element.
#[derive(Clone)]
pub enum GUIElement {
	Rect { pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4] },
	Button {
		text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, enabled: bool,
		tick_mut_gui: fn(GUIElement, gui: &mut GUI, world: &mut Option<World>, io: &IO) -> (),
	},
	Text { text: String, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment },
	TextEntry { text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, is_selected: bool, text_length_limit: usize },
	Grayout { color: [u8; 4] },
	Texture { pos: [u16; 2], alignment: GUIAlignment, texture: Texture },
}

impl GUIElement {
	/// Get weather the mouse is over an element.
	pub fn is_mouse_over(&self, io: &IO) -> bool {
		match self {
			Self::Button { pos, size, alignment, .. } => {
				let mouse_pos = io.get_mouse_pos_as_gui_pos();
				let button_screen_pos = gui_pos_to_screen_pos(*pos, *alignment, io);
				let button_screen_size = gui_size_to_screen_size(*size);
				let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
				mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
				mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
			}
			Self::TextEntry { pos, size, alignment, .. } => {
				let mouse_pos = io.get_mouse_pos_as_gui_pos();
				let button_screen_pos = gui_pos_to_screen_pos(*pos, *alignment, io);
				let button_screen_size = gui_size_to_screen_size(*size);
				let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
				mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
				mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
			}
			_ => false,
		}
	}

	/// Render the element
	pub fn render(&self, vertices: &mut Vec<Vertex>, io: &IO) {
		match self {
			Self::Rect{pos, size, alignment, color} =>
				vertices.extend(render_gui_rect(*pos, *size, *alignment, *color, io)),
			Self::Button { pos, size, alignment, text, enabled, .. } => {
				let mut color = BUTTON_GRAY_COLOR;
				if self.is_mouse_over(io) {
					color = BUTTON_HOVER_COLOR;
				}
				if !enabled {
					color = BUTTON_DISABLED_COLOR;
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, io));

				let text_pos = [pos[0] + size[0] / 2, pos[1] + size[1] / 2 - 8];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
			}
			Self::Text { text: string, pos, alignment, text_alignment } =>
				render_gui_string(string, *pos, *alignment, *text_alignment, io, vertices),
			Self::TextEntry { text, pos, size, alignment, is_selected, .. } => {
				let mut color = TEXT_ENTRY_GRAY_COLOR;
				if self.is_mouse_over(io) {
					color = BUTTON_HOVER_COLOR;
				}
				if *is_selected {
					color = TEXT_ENTRY_SELECT_COLOR;
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, io));

				let text_pos = [pos[0] + 1, pos[1] + size[1] / 2 - 8];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Left, io, vertices);
			}
			Self::Grayout { color } => vertices.extend(render_screen_grayout(*color, io)),
			Self::Texture { pos, alignment, texture } => {
				vertices.extend(texture.gui_render(*pos, *alignment, io));
			}
		}
	}

	/// Tick the element with a mutable refrence to self.
	pub fn tick_mut_self(&mut self, _world: &mut Option<World>, io: &IO) {
		let is_mouse_over = self.is_mouse_over(io);
		if io.get_game_key_starting_now(GameKey::GUIInteract) {
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
					for chr in io.key_chars.iter() {
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

	/// Tick a copy of the element with a mutable refrence to the gui.
	pub fn tick_mut_gui(self, gui: &mut GUI, world: &mut Option<World>, io: &IO) {
		if io.get_game_key_starting_now(GameKey::GUIInteract) && self.is_mouse_over(io) {
			match self {
				GUIElement::Button { tick_mut_gui, enabled, .. } if enabled => tick_mut_gui(self, gui, world, io),
				_ => {}
			}
		}
	}
}