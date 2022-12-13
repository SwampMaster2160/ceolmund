use crate::{render::{vertex::Vertex, render_data::RenderData}, io::input::Input, world::world::World};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement, gui::GUI};

const RECT_COLOR: [u8; 4] = [31, 31, 31, 255];

#[derive(Clone)]
pub enum GUIMenu {
	Test,
	Paused,
}

impl GUIMenu {
	pub fn get_const_elements(&self) -> Vec<GUIElement> {
		match self {
			Self::Test => vec![
				GUIElement::Rect { pos: [10, 10], size: [10, 10], alignment: GUIAlignment::Left, color: RECT_COLOR },
				GUIElement::Button {
					pos: [30, 20], size: [30, 15], alignment: GUIAlignment::Left, text: "Hi",
					tick_mut_self: (|_, _, _, _| println!("Hi")),
					tick_mut_gui: (|_, _, _, _, _| ()),
				},
				GUIElement::Text { text: "Hello", pos: [50, 40], alignment: GUIAlignment::Left, text_alignment: GUIAlignment::Left },
			],
			Self::Paused => vec![
				GUIElement::Rect { pos: [53, 30], size: [150, 196], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Button {
					pos: [53, 30], size: [150, 16], alignment: GUIAlignment::Center, text: "Resume",
					tick_mut_self: (|_, _, _, _| ()),
					tick_mut_gui: (|_, gui, _, _, _| {gui.menus.pop();}),
				},
				GUIElement::Text { text: "Game Paused", pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
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

	pub fn does_menu_pause_game(&self) -> bool {
		match self {
			Self::Test => false,
			Self::Paused => true,
		}
	}

	pub fn menu_close_button_action(self, gui: &mut GUI, world: &mut Option<World>, input: &mut Input, render_data: &RenderData) {
		match self {
			GUIMenu::Test => {}
			GUIMenu::Paused => {
				gui.menus.pop();
				input.update_keys_pressed_last();
			},
		}
	}
}