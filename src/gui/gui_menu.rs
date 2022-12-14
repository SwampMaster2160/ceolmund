use crate::{render::{vertex::Vertex, render_data::RenderData}, io::input::Input, world::world::World};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement, gui::GUI};

const RECT_COLOR: [u8; 4] = [31, 31, 31, 255];

#[derive(Clone)]
pub enum GUIMenu {
	Test,
	Paused,
	ExitingGame,
	ExitingToTitle,
	Title,
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
				GUIElement::Text { text: "Game Paused", pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
				GUIElement::Button {
					pos: [53, 30], size: [150, 16], alignment: GUIAlignment::Center, text: "Resume",
					tick_mut_self: (|_, _, _, _| ()),
					tick_mut_gui: (|_, gui, _, _, _| {gui.menus.pop();}),
				},
				GUIElement::Button {
					pos: [53, 210], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit Game",
					tick_mut_self: (|_, _, _, _| ()),
					tick_mut_gui: (|_, gui, world, _, _| {
						if let Some(world) = world {
							world.is_freeing = true;
						}
						gui.menus.pop();
						gui.menus.push(GUIMenu::ExitingGame);
					}),
				},
				GUIElement::Button {
					pos: [53, 190], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit to Title",
					tick_mut_self: (|_, _, _, _| ()),
					tick_mut_gui: (|_, gui, world, _, _| {
						if let Some(world) = world {
							world.is_freeing = true;
						}
						gui.menus.pop();
						gui.menus.push(GUIMenu::ExitingToTitle);
					}),
				},
			],
			Self::ExitingGame => vec![
				GUIElement::Rect { pos: [53, 78], size: [150, 100], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Closing World...", pos: [127, 120], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
			Self::ExitingToTitle => vec![
				GUIElement::Rect { pos: [53, 78], size: [150, 100], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Closing World...", pos: [127, 120], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
			Self::Title => vec![
				GUIElement::Rect { pos: [53, 30], size: [150, 196], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Ceolmund", pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
				GUIElement::Button {
					pos: [53, 210], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit Game",
					tick_mut_self: (|_, _, _, _| ()),
					tick_mut_gui: (|_, gui, _, _, _| {
						gui.should_close_game = true;
					}),
				},
				GUIElement::Button {
					pos: [53, 30], size: [150, 16], alignment: GUIAlignment::Center, text: "Create World",
					tick_mut_self: (|_, _, _, _| ()),
					tick_mut_gui: (|_, gui, world, _, _| {
						*world = Some(World::new());
						gui.menus = vec![];
					}),
				},
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
			Self::ExitingGame => true,
			Self::ExitingToTitle => true,
			Self::Title => true,
		}
	}

	pub fn menu_close_button_action(self, gui: &mut GUI, _world: &mut Option<World>, input: &mut Input, _render_data: &RenderData) {
		match self {
			GUIMenu::Paused => {
				gui.menus.pop();
				input.update_keys_pressed_last();
			},
			_ => {}
		}
	}

	pub fn tick(self, gui: &mut GUI, world: &mut Option<World>, input: &mut Input, _render_data: &RenderData) {
		match self {
			GUIMenu::ExitingGame => {
				if let Some(world) = world {
					if world.is_freed {
						gui.should_close_game = true;
					}
				}
			}
			GUIMenu::ExitingToTitle => {
				let mut set_world_to_none = false;
				if let Some(world) = world {
					if world.is_freed {
						gui.menus.pop();
						gui.menus.push(GUIMenu::Title);
						set_world_to_none = true;
					}
				}
				if set_world_to_none {
					*world = None;
				}
			}
			_ => {}
		}
	}
}