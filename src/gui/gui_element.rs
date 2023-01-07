use unicode_segmentation::UnicodeSegmentation;

use crate::{io::{io::IO, game_key::GameKey}, render::{vertex::Vertex, render::{gui_size_to_screen_size, render_gui_string_u16, render_screen_grayout, gui_pos_to_screen_pos, render_gui_string}, texture::Texture}, world::world::World};

use super::{gui_alignment::GUIAlignment, gui::GUI, gui_rect::GUIRect};

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

/// A GUI element.
#[derive(Clone)]
pub enum GUIElement {
	Rect { rect: GUIRect, alignment: GUIAlignment, inside_color: [u8; 4], border_color: [u8; 4] },
	ProgressBar { rect: GUIRect, alignment: GUIAlignment, inside_color: [u8; 4], border_color: [u8; 4], progress: u32, max_progress: u32 },
	Button {
		text: String, rect: GUIRect, alignment: GUIAlignment, enabled: bool,
		click_mut_gui: fn(GUIElement, gui: &mut GUI, world: &mut Option<World>, io: &IO) -> (),
	},
	ToggleButton {
		text: String, rect: GUIRect, alignment: GUIAlignment, enabled: bool, state: bool,
	},
	MutuallyExclusiveButtonGroup {
		buttons: Vec<(String, GUIRect, bool)>, // text, rect, enabled
		alignment: GUIAlignment, selected_button: usize,
	},
	SingleFunctionButtonGroup {
		buttons: Vec<(String, GUIRect, bool)>, // text, rect, enabled
		alignment: GUIAlignment,
		click_mut_gui: fn(GUIElement, gui: &mut GUI, world: &mut Option<World>, io: &IO, button_clicked_index: usize) -> (),
	},
	Text { text: String, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment },
	TextEntry { text: String, rect: GUIRect, alignment: GUIAlignment, is_selected: bool, text_length_limit: usize },
	Grayout { color: [u8; 4] },
	Texture { pos: [u16; 2], alignment: GUIAlignment, texture: Texture },
	ScrollArea { rect: GUIRect, alignment: GUIAlignment, inside_color: [u8; 4], border_color: [u8; 4], inside_height: u16, inside_elements: Vec<GUIElement>, scroll: u16 },
}

pub fn is_mouse_over_rect(rect: GUIRect, alignment: GUIAlignment, io: &IO) -> bool {
	let mouse_pos = io.get_mouse_pos_as_gui_pos();
	let button_screen_pos = gui_pos_to_screen_pos(rect.pos, alignment, io);
	let button_screen_size = gui_size_to_screen_size(rect.size);
	let button_screen_end = [button_screen_pos[0] + button_screen_size[0], button_screen_pos[1] + button_screen_size[1]];
	mouse_pos[0] >= button_screen_pos[0] && mouse_pos[1] >= button_screen_pos[1] &&
	mouse_pos[0] <= button_screen_end[0] && mouse_pos[1] <= button_screen_end[1]
}

impl GUIElement {
	/// Get weather the mouse is over an element.
	pub fn is_mouse_over(&self, io: &IO, scroll: [i16; 2]) -> bool {
		match self {
			Self::Button { rect, alignment, .. } => {
				is_mouse_over_rect(rect.scrolled(scroll), *alignment, io)
			},
			Self::ToggleButton { rect, alignment, .. } => is_mouse_over_rect(*rect, *alignment, io),
			Self::TextEntry { rect, alignment, .. } => is_mouse_over_rect(*rect, *alignment, io),
			Self::ScrollArea { rect, alignment, .. } => is_mouse_over_rect(*rect, *alignment, io),
			_ => false,
		}
	}

	/// Render the element
	pub fn render(&self, visable_area: GUIRect, vertices: &mut Vec<Vertex>, io: &IO, scroll: [i16; 2]) {
		match self {
			Self::Rect{rect, alignment, inside_color, border_color} =>
				rect.render_shade_and_outline(visable_area, *alignment, *border_color, *inside_color, io, vertices),
			Self::ScrollArea { rect, alignment, inside_color, border_color, inside_elements, scroll: scroll_area_scroll, .. } => {
				rect.render_shade_and_outline(visable_area, *alignment, *border_color, *inside_color, io, vertices);
				for element in inside_elements {
					let scroll = [
						scroll[0].saturating_add(rect.pos[0]).saturating_add(2),
						scroll[1].saturating_add(rect.pos[1]).saturating_add(2).saturating_sub_unsigned(*scroll_area_scroll),
					];
					element.render(visable_area, vertices, io, scroll);
				}
			}
			Self::ProgressBar{rect, alignment, inside_color: color, border_color, progress, max_progress} => {
				let progress_width = (((rect.size[0].saturating_sub(2)) as u64) * *progress as u64 / *max_progress as u64) as u16;
				rect.render_shade(visable_area, *alignment, *border_color, io, vertices);
				let progress_inside_rect = GUIRect {
					pos: [rect.pos[0] + 1, rect.pos[1] + 1],
					size: [progress_width, rect.size[1].saturating_sub(2)],
				};
				progress_inside_rect.render_shade(visable_area, *alignment, *color, io, vertices);
			}
			Self::Button { rect, alignment, text, enabled, .. } => {
				let mut inside_color = BUTTON_GRAY_COLOR;
				if self.is_mouse_over(io, scroll) {
					inside_color = BUTTON_HOVER_COLOR;
				}
				if !enabled {
					inside_color = BUTTON_DISABLED_COLOR;
				}
				let rect = rect.scrolled(scroll);
				rect.render_shade_and_outline(visable_area, *alignment, BUTTON_BORDER, inside_color, io, vertices);

				let text_pos = [rect.pos[0].saturating_add_unsigned(rect.size[0] / 2), rect.pos[1].saturating_add_unsigned(rect.size[1] / 2).saturating_sub(8)];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
			}
			Self::ToggleButton { rect, alignment, text, enabled, state, .. } => {
				let mut inside_color = match (*state, self.is_mouse_over(io, scroll)) {
					(false, false) => BUTTON_OFF_COLOR,
					(false, true) => BUTTON_OFF_HOVER_COLOR,
					(true, false) => BUTTON_ON_COLOR,
					(true, true) => BUTTON_ON_HOVER_COLOR,
				};
				if !enabled {
					inside_color = BUTTON_DISABLED_COLOR;
				}
				rect.render_shade_and_outline(visable_area, *alignment, BUTTON_BORDER, inside_color, io, vertices);

				let text_pos = [rect.pos[0].saturating_add_unsigned(rect.size[0] / 2), rect.pos[1].saturating_add_unsigned(rect.size[1] / 2).saturating_sub(8)];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
			}
			Self::MutuallyExclusiveButtonGroup { buttons, alignment, selected_button } => {
				for (button_index, button) in buttons.iter().enumerate() {
					let is_selected = button_index == *selected_button;
					let rect = button.1;
					let is_mouse_over = is_mouse_over_rect(rect, *alignment, io);
					let text = button.0.as_str();
					let is_enabled = button.2;
					let mut inside_color = match (is_selected, is_mouse_over) {
						(false, false) => BUTTON_OFF_COLOR,
						(false, true) => BUTTON_OFF_HOVER_COLOR,
						(true, false) => BUTTON_ON_COLOR,
						(true, true) => BUTTON_ON_HOVER_COLOR,
					};
					if !is_enabled {
						inside_color = BUTTON_DISABLED_COLOR;
					}
					rect.render_shade_and_outline(visable_area, *alignment, BUTTON_BORDER, inside_color, io, vertices);
					let text_pos = [rect.pos[0].saturating_add_unsigned(rect.size[0] / 2), rect.pos[1].saturating_add_unsigned(rect.size[1] / 2).saturating_sub(8)];
					render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
				}
			}
			Self::SingleFunctionButtonGroup { buttons, alignment, .. } => {
				for button in buttons {
					let rect = button.1;
					let is_mouse_over = is_mouse_over_rect(rect, *alignment, io);
					let text = button.0.as_str();
					let is_enabled = button.2;
					let mut inside_color = BUTTON_GRAY_COLOR;
					if is_mouse_over {
						inside_color = BUTTON_HOVER_COLOR;
					}
					if !is_enabled {
						inside_color = BUTTON_DISABLED_COLOR;
					}
					rect.render_shade_and_outline(visable_area, *alignment, BUTTON_BORDER, inside_color, io, vertices);
					let text_pos = [rect.pos[0].saturating_add_unsigned(rect.size[0] / 2), rect.pos[1].saturating_add_unsigned(rect.size[1] / 2).saturating_sub(8)];
					render_gui_string(text, text_pos, *alignment, GUIAlignment::Center, io, vertices);
				}
			}
			Self::Text { text: string, pos, alignment, text_alignment } =>
				render_gui_string_u16(string, *pos, *alignment, *text_alignment, io, vertices),
			Self::TextEntry { text, rect, alignment, is_selected, .. } => {
				let mut inside_color = TEXT_ENTRY_GRAY_COLOR;
				if self.is_mouse_over(io, scroll) {
					inside_color = BUTTON_HOVER_COLOR;
				}
				if *is_selected {
					inside_color = TEXT_ENTRY_SELECT_COLOR;
				}
				rect.render_shade_and_outline(visable_area, *alignment, BUTTON_BORDER, inside_color, io, vertices);

				let text_pos = [rect.pos[0].saturating_add(2), rect.pos[1].saturating_add_unsigned(rect.size[1] / 2).saturating_sub(8)];
				render_gui_string(text, text_pos, *alignment, GUIAlignment::Left, io, vertices);
			}
			Self::Grayout { color } => vertices.extend(render_screen_grayout(*color, io)),
			Self::Texture { pos, alignment, texture } => {
				vertices.extend(texture.gui_render(*pos, *alignment, io));
			}
		}
	}

	/// Tick the element with a mutable refrence to self.
	pub fn tick_mut_self(&mut self, _world: &mut Option<World>, io: &IO, scroll: [i16; 2]) {
		let is_mouse_over = self.is_mouse_over(io, scroll);
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
						let rect = button.1;
						let is_enabled = button.2;
						let is_mouse_over = is_mouse_over_rect(rect, *alignment, io);
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
			GUIElement::ScrollArea { scroll, rect, inside_height, .. } => {
				if is_mouse_over {
					let max_scroll = inside_height.saturating_sub(rect.size[1]).saturating_add(4);
					*scroll = scroll.saturating_add_signed(io.mouse_scroll.saturating_neg().saturating_mul(8)).min(max_scroll);
				}
			}
			_ => {}
		}
	}

	/// Tick a copy of the element with a mutable refrence to the gui.
	pub fn tick_mut_gui(self, gui: &mut GUI, world: &mut Option<World>, io: &IO) {
		if io.get_game_key_starting_now(GameKey::GUIInteract) {
			self.click_mut_gui(gui, world, io, [0, 0]);
		}
	}

	/// Tick a copy of the element with a mutable refrence to the gui.
	pub fn click_mut_gui(self, gui: &mut GUI, world: &mut Option<World>, io: &IO, scroll: [i16; 2]) {
		if io.get_game_key_starting_now(GameKey::GUIInteract) {
			match self {
				GUIElement::Button { click_mut_gui, enabled, .. } => {
					if enabled && self.is_mouse_over(io, scroll) {
						click_mut_gui(self, gui, world, io);
					}
				}
				GUIElement::SingleFunctionButtonGroup { ref buttons, alignment, click_mut_gui, .. } => {
					for (button_index, button) in buttons.iter().enumerate() {
						let is_enabled = button.2;
						let rect = button.1;
						if is_enabled && is_mouse_over_rect(rect, alignment, io) {
							click_mut_gui(self.clone(), gui, world, io, button_index);
						}
					}
				}
				GUIElement::ScrollArea { rect, alignment, inside_elements, scroll: scroll_area_scroll, .. } => {
					if is_mouse_over_rect(rect, alignment, io) {
						for element in inside_elements {
							let scroll = [
								scroll[0].saturating_add(rect.pos[0]).saturating_add(2),
								scroll[1].saturating_add(rect.pos[1]).saturating_add(2).saturating_sub_unsigned(scroll_area_scroll),
							];
							element.click_mut_gui(gui, world, io, scroll);
						}
					}
				}
				_ => {}
			}
		}
	}
}