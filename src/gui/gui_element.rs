use unicode_segmentation::UnicodeSegmentation;

use crate::{io::{io::IO, game_key::GameKey}, render::{vertex::Vertex, render::{render_gui_rect, gui_pos_to_screen_pos_unsigned, gui_size_to_screen_size, render_gui_string, render_screen_grayout}, texture::Texture}, world::world::World};

use super::{gui_alignment::GUIAlignment, gui::GUI};

const BUTTON_GRAY_COLOR: [u8; 4] = [95, 95, 95, 255];
const BUTTON_HOVER_COLOR: [u8; 4] = [95, 195, 195, 255];
const BUTTON_DISABLED_COLOR: [u8; 4] = [7, 7, 7, 255];
const BUTTON_BORDER: [u8; 4] = [15, 15, 15, 255];
const BUTTON_ON_COLOR: [u8; 4] = [0, 95, 0, 255];
const BUTTON_OFF_COLOR: [u8; 4] = [95, 0, 0, 255];
const BUTTON_ON_HOVER_COLOR: [u8; 4] = [0, 255, 0, 255];
const BUTTON_OFF_HOVER_COLOR: [u8; 4] = [255, 0, 0, 255];
const TEXT_ENTRY_GRAY_COLOR: [u8; 4] = [50, 50, 50, 255];
const TEXT_ENTRY_SELECT_COLOR: [u8; 4] = [0, 255, 255, 255];
const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

/// A GUI element.
#[derive(Clone)]
pub enum GUIElement {
	Rect { pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4], border_color: [u8; 4] },
	ProgressBar { pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4], border_color: [u8; 4], progress: u32, max_progress: u32 },
	Button {
		text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, enabled: bool,
		tick_mut_gui: fn(GUIElement, gui: &mut GUI, world: &mut Option<World>, io: &IO) -> (),
	},
	ToggleButton {
		text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, enabled: bool, state: bool,
	},
	MutuallyExclusiveButtonGroup {
		buttons: Vec<(String, [u16; 2], [u16; 2], bool)>, // text, pos, size, enabled
		alignment: GUIAlignment, selected_button: usize,
	},
	Text { text: String, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment },
	TextEntry { text: String, pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, is_selected: bool, text_length_limit: usize },
	Grayout { color: [u8; 4] },
	Texture { pos: [u16; 2], alignment: GUIAlignment, texture: Texture },
}

pub fn is_mouse_over_area(pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, io: &IO) -> bool {
	let mouse_pos = io.get_mouse_pos_as_gui_pos();
	let button_screen_pos = gui_pos_to_screen_pos_unsigned(pos, alignment, io);
	let button_screen_size = gui_size_to_screen_size(size);
	let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
	mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
	mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
}

impl GUIElement {
	/// Get weather the mouse is over an element.
	pub fn is_mouse_over(&self, io: &IO) -> bool {
		match self {
			Self::Button { pos, size, alignment, .. } => is_mouse_over_area(*pos, *size, *alignment, io),
			Self::ToggleButton { pos, size, alignment, .. } => is_mouse_over_area(*pos, *size, *alignment, io),
			Self::TextEntry { pos, size, alignment, .. } => is_mouse_over_area(*pos, *size, *alignment, io),
			_ => false,
		}
	}

	/// Render the element
	pub fn render(&self, vertices: &mut Vec<Vertex>, io: &IO) {
		match self {
			Self::Rect{pos, size, alignment, color, border_color} =>
				vertices.extend(render_gui_rect(*pos, *size, *alignment, *color, *border_color, io)),
			Self::ProgressBar{pos, size, alignment, color, border_color, progress, max_progress} => {
				let progress_width = (((size[0] - 2) as u64) * *progress as u64 / *max_progress as u64) as u16;
				vertices.extend(render_gui_rect(*pos, *size, *alignment, NO_COLOR, *border_color, io));
				vertices.extend(render_gui_rect([pos[0] + 1, pos[1] + 1], [progress_width, size[1] - 2], *alignment, NO_COLOR, *color, io));
			}
			Self::Button { pos, size, alignment, text, enabled, .. } => {
				let mut color = BUTTON_GRAY_COLOR;
				if self.is_mouse_over(io) {
					color = BUTTON_HOVER_COLOR;
				}
				if !enabled {
					color = BUTTON_DISABLED_COLOR;
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, BUTTON_BORDER, io));

				let text_pos = [pos[0] + size[0] / 2, pos[1] + size[1] / 2 - 8];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
			}
			Self::ToggleButton { pos, size, alignment, text, enabled, state, .. } => {
				let mut color = match (*state, self.is_mouse_over(io)) {
					(false, false) => BUTTON_OFF_COLOR,
					(false, true) => BUTTON_OFF_HOVER_COLOR,
					(true, false) => BUTTON_ON_COLOR,
					(true, true) => BUTTON_ON_HOVER_COLOR,
				};
				if !enabled {
					color = BUTTON_DISABLED_COLOR;
				}
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, BUTTON_BORDER, io));

				let text_pos = [pos[0] + size[0] / 2, pos[1] + size[1] / 2 - 8];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
			}
			Self::MutuallyExclusiveButtonGroup { buttons, alignment, selected_button } => {
				for (button_index, button) in buttons.iter().enumerate() {
					let is_selected = button_index == *selected_button;
					let pos = button.1;
					let size = button.2;
					let is_mouse_over = is_mouse_over_area(pos, size, *alignment, io);
					let text = button.0.as_str();
					let is_enabled = button.3;
					let mut color = match (is_selected, is_mouse_over) {
						(false, false) => BUTTON_OFF_COLOR,
						(false, true) => BUTTON_OFF_HOVER_COLOR,
						(true, false) => BUTTON_ON_COLOR,
						(true, true) => BUTTON_ON_HOVER_COLOR,
					};
					if !is_enabled {
						color = BUTTON_DISABLED_COLOR;
					}
					vertices.extend(render_gui_rect(pos, size, *alignment, color, BUTTON_BORDER, io));
					let text_pos = [pos[0] + size[0] / 2, pos[1] + size[1] / 2 - 8];
					render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
				}
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
				vertices.extend(render_gui_rect(*pos, *size, *alignment, color, BUTTON_BORDER, io));

				let text_pos = [pos[0] + 2, pos[1] + size[1] / 2 - 8];
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
				GUIElement::ToggleButton { state, .. } => {
					if is_mouse_over {
						*state = !*state;
					}
				},
				GUIElement::MutuallyExclusiveButtonGroup { buttons, alignment, selected_button } => {
					for (button_index, button) in buttons.iter().enumerate() {
						let pos = button.1;
						let size = button.2;
						let is_enabled = button.3;
						let is_mouse_over = is_mouse_over_area(pos, size, *alignment, io);
						if is_mouse_over && is_enabled {
							*selected_button = button_index;
						}
					}
				}
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