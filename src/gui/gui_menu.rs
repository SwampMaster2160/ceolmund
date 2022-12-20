use crate::{render::vertex::Vertex, io::io::IO, world::world::World};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement, gui::GUI, gui_menu_variant::GUIMenuVariant, load_world_data::LoadWorldData};

const RECT_COLOR: [u8; 4] = [31, 31, 31, 255];
const GRAYOUT_COLOR: [u8; 4] = [63, 63, 63, 127];

#[derive(Clone)]
pub struct GUIMenu {
	variant: GUIMenuVariant,
	pub extra_elements: Vec<GUIElement>,
}

impl GUIMenu {
	pub fn get_const_elements(&self) -> Vec<GUIElement> {
		match &self.variant {
			GUIMenuVariant::Test => vec![
				GUIElement::Rect { pos: [10, 10], size: [10, 10], alignment: GUIAlignment::Left, color: RECT_COLOR },
				GUIElement::Button {
					pos: [30, 20], size: [30, 15], alignment: GUIAlignment::Left, text: "Hi".to_string(), enabled: true,
					tick_mut_gui: (|_, _, _, _| println!("Hi")),
				},
				GUIElement::Text { text: "Hello".to_string(), pos: [50, 40], alignment: GUIAlignment::Left, text_alignment: GUIAlignment::Left },
			],
			GUIMenuVariant::Paused => vec![
				GUIElement::Rect { pos: [53, 30], size: [150, 196], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Game Paused".to_string(), pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
				GUIElement::Button {
					pos: [53, 30], size: [150, 16], alignment: GUIAlignment::Center, text: "Resume".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, _, _| {gui.menus.pop();}),
				},
				GUIElement::Button {
					pos: [53, 210], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit Game".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, world, _| {
						if let Some(world) = world {
							world.is_freeing = true;
						}
						gui.menus.pop();
						gui.menus.push(Self::new(GUIMenuVariant::ExitingGame));
					}),
				},
				GUIElement::Button {
					pos: [53, 190], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit to Title".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, world, _| {
						if let Some(world) = world {
							world.is_freeing = true;
						}
						gui.menus.pop();
						gui.menus.push(Self::new(GUIMenuVariant::ExitingToTitle));
					}),
				},
			],
			GUIMenuVariant::ExitingGame => vec![
				GUIElement::Rect { pos: [53, 78], size: [150, 100], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Closing World...".to_string(), pos: [127, 120], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
			GUIMenuVariant::ExitingToTitle => vec![
				GUIElement::Rect { pos: [53, 78], size: [150, 100], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Closing World...".to_string(), pos: [127, 120], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
			GUIMenuVariant::Title => vec![
				GUIElement::Rect { pos: [53, 30], size: [150, 196], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Ceolmund".to_string(), pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
				GUIElement::Button {
					pos: [53, 210], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit Game".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, _, _| {
						gui.should_close_game = true;
					}),
				},
				GUIElement::Button {
					pos: [53, 30], size: [150, 16], alignment: GUIAlignment::Center, text: "Create World".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, _, _| {
						gui.menus.pop();
						gui.menus.push(Self::new(GUIMenuVariant::CreateWorld));
					}),
				},
				GUIElement::Button {
					pos: [53, 50], size: [150, 16], alignment: GUIAlignment::Center, text: "Load World".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, _, io| {
						gui.menus.pop();
						gui.menus.push(Self::new_load_world(0, LoadWorldData::new(io)));
					}),
				},
			],
			GUIMenuVariant::IngameHUD => vec![],
			GUIMenuVariant::CreateWorld => vec![
				GUIElement::Rect { pos: [53, 30], size: [150, 196], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Create World".to_string(), pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
				GUIElement::Text { text: "Name:".to_string(), pos: [53, 30], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Left },
				GUIElement::Text { text: "Seed:".to_string(), pos: [53, 70], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Left },
				GUIElement::Button {
					pos: [53, 190], size: [150, 16], alignment: GUIAlignment::Center, text: "Create World".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, world, io| {
						if let GUIElement::TextEntry{text: name_text, ..} = &gui.menus.last().unwrap().extra_elements[0] {
							if let GUIElement::TextEntry{text: seed_text, ..} = &gui.menus.last().unwrap().extra_elements[1] {
								let seed = seed_text.parse::<u32>();
								let seed = match seed {
									Ok(seed) => seed,
									Err(_) if *seed_text == "".to_string() => 420,
									Err(_) => {
										gui.menus.push(GUIMenu::new_error("Invalid seed.".to_string()));
										return
									},
								};
								match World::new(seed, name_text.clone(), io) {
									Some(valid_world) => *world = Some(valid_world),
									None => {
										gui.menus.push(GUIMenu::new_error("Unable to create world.".to_string()));
										return
									},
								};
								gui.menus = vec![Self::new(GUIMenuVariant::IngameHUD)];
							}
						}
					}),
				},
				GUIElement::Button {
					pos: [53, 210], size: [150, 16], alignment: GUIAlignment::Center, text: "Cancel".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, _, _| {
						gui.menus = vec![Self::new(GUIMenuVariant::Title)];
					}),
				},
			],
			GUIMenuVariant::Error => vec![
				GUIElement::Grayout { color: GRAYOUT_COLOR },
				GUIElement::Rect { pos: [53, 90], size: [150, 76], alignment: GUIAlignment::Center, color: RECT_COLOR },
				GUIElement::Text { text: "Error".to_string(), pos: [127, 74], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
				GUIElement::Button {
					pos: [53, 150], size: [150, 16], alignment: GUIAlignment::Center, text: "OK".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, _, _| {
						gui.menus.pop();
					}),
				},
			],
			GUIMenuVariant::LoadWorld { page, load_world_data } => {
					let mut out = vec![
					GUIElement::Rect { pos: [53, 30], size: [150, 196], alignment: GUIAlignment::Center, color: RECT_COLOR },
					GUIElement::Text { text: "Load World".to_string(), pos: [127, 14], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
					GUIElement::Button {
						pos: [53, 210], size: [150, 16], alignment: GUIAlignment::Center, text: "Cancel".to_string(), enabled: true,
						tick_mut_gui: (|_, gui, _, _| {
							gui.menus = vec![Self::new(GUIMenuVariant::Title)];
						}),
					},
					GUIElement::Button {
						pos: [53, 190], size: [73, 16], alignment: GUIAlignment::Center, text: "<-".to_string(), enabled: *page > 0,
						tick_mut_gui: (|_, gui, _, _| {
							let GUIMenu { variant, .. } = gui.menus.pop().unwrap();
							if let GUIMenuVariant::LoadWorld { load_world_data, page } = variant {
								gui.menus.push(GUIMenu::new_load_world(page - 1, load_world_data));
							}
						}),
					},
					GUIElement::Button {
						pos: [130, 190], size: [73, 16], alignment: GUIAlignment::Center, text: "->".to_string(), enabled: (page + 1) * 8 < load_world_data.worlds.len(),
						tick_mut_gui: (|_, gui, _, _| {
							let GUIMenu { variant, .. } = gui.menus.pop().unwrap();
							if let GUIMenuVariant::LoadWorld { load_world_data, page } = variant {
								gui.menus.push(GUIMenu::new_load_world(page + 1, load_world_data));
							}
						}),
					},
				];

				let world_count = load_world_data.worlds.len();
				let start = page * 8;
				let end = (start + 8).min(world_count);
				let button_count = end - start;

				for x in 0..button_count {
					let world_index = x + start;
					let world = load_world_data.worlds[world_index].clone();
					out.push(GUIElement::Button {
						pos: [53, 30 + x as u16 * 20], size: [150, 16], alignment: GUIAlignment::Center, text: world.0, enabled: true,
						tick_mut_gui: (|element, gui, world, _| {
							if let GUIElement::Button { pos, .. } = element {
								let button_y = ((pos[1] - 30) / 20) as usize;
								let top_menu = &gui.menus.last().unwrap().variant;
								if let GUIMenuVariant::LoadWorld { load_world_data, page } = top_menu {
									let world_index = button_y + page * 8;
									let world_path = &load_world_data.worlds[world_index].1;
									if let Some(new_world) = World::load(world_path.clone()) {
										*world = Some(new_world);
										gui.menus = vec![GUIMenu::new(GUIMenuVariant::IngameHUD)];
									}
									else {
										gui.menus.push(GUIMenu::new_error("Unable to load world".to_string()));
									}
								}
							}
						}),
					},)
				}
				out
			}
		}
	}

	pub fn get_elements(&self) -> Vec<GUIElement> {
		let mut out = self.get_const_elements();
		out.extend(self.extra_elements.clone());
		out
	}

	pub fn render(&self, vertices: &mut Vec<Vertex>, io: &IO) {
		for element in self.get_elements() {
			element.render(vertices, io);
		}
	}

	pub fn does_menu_pause_game(&self) -> bool {
		match self.variant {
			GUIMenuVariant::Test => false,
			GUIMenuVariant::Paused => true,
			GUIMenuVariant::ExitingGame => true,
			GUIMenuVariant::ExitingToTitle => true,
			GUIMenuVariant::Title => true,
			GUIMenuVariant::IngameHUD => false,
			GUIMenuVariant::CreateWorld => true,
			GUIMenuVariant::Error => true,
			GUIMenuVariant::LoadWorld { .. } => true,
		}
	}

	pub fn menu_close_button_action(self, gui: &mut GUI, _world: &mut Option<World>, io: &mut IO) {
		match self.variant {
			GUIMenuVariant::Paused => {
				gui.menus.pop();
				io.update_keys_pressed_last();
			},
			_ => {}
		}
	}

	pub fn tick(self, gui: &mut GUI, world: &mut Option<World>, _io: &mut IO, request_game_close: bool) {
		if request_game_close {
			if world.is_some() {
				if let Some(world) = world {
					world.is_freeing = true;
				}
				gui.menus = Vec::new();
				gui.menus.push(Self::new(GUIMenuVariant::ExitingGame));
			}
			else if matches!(self.variant, GUIMenuVariant::ExitingGame) || matches!(self.variant, GUIMenuVariant::ExitingToTitle) {

			}
			else {
				gui.should_close_game = true;
			}
		}
		match self.variant {
			GUIMenuVariant::ExitingGame => {
				if let Some(world) = world {
					if world.is_freed {
						gui.should_close_game = true;
					}
				}
			}
			GUIMenuVariant::ExitingToTitle => {
				let mut set_world_to_none = false;
				if let Some(world) = world {
					if world.is_freed {
						gui.menus.pop();
						gui.menus.push(Self::new(GUIMenuVariant::Title));
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

	pub fn new(variant: GUIMenuVariant) -> Self {
		Self {
			variant: variant.clone(),
			extra_elements: match variant {
				GUIMenuVariant::CreateWorld => vec![
					GUIElement::TextEntry { text: "".to_string(), pos: [53, 50], size: [150, 16], alignment: GUIAlignment::Center, is_selected: false, text_length_limit: 20 },
					GUIElement::TextEntry { text: "".to_string(), pos: [53, 90], size: [150, 16], alignment: GUIAlignment::Center, is_selected: false, text_length_limit: 10 },
				],
				_ => Vec::new(),
			},
		}
	}

	pub fn new_error(error: String) -> Self {
		Self {
			variant: GUIMenuVariant::Error,
			extra_elements: vec![
				GUIElement::Text { text: error, pos: [127, 116], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
		}
	}

	pub fn new_load_world(page: usize, load_world_data: LoadWorldData) -> Self {
		Self {
			variant: GUIMenuVariant::LoadWorld { page, load_world_data },
			extra_elements: vec![],
		}
	}
}