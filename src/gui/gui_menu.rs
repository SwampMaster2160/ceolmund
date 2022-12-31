use crate::{render::{vertex::Vertex}, io::{io::IO, game_key::GameKey}, world::{world::World, entity::entity_type::EntityType}};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement, gui::GUI, gui_menu_variant::GUIMenuVariant, load_world_data::LoadWorldData};

const RECT_COLOR: [u8; 4] = [31, 31, 31, 255];
const RECT_BORDER_COLOR: [u8; 4] = [15, 15, 15, 255];
const GRAYOUT_COLOR: [u8; 4] = [63, 63, 63, 127];
const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

/// A GUI menu, these are stacked.
#[derive(Clone)]
pub struct GUIMenu {
	variant: GUIMenuVariant,
	pub extra_elements: Vec<GUIElement>,
}

impl GUIMenu {
	/// Get the elements that do not need to be stored from frame to frame.
	pub fn get_const_elements(&self, world: &Option<World>) -> Vec<GUIElement> {
		match &self.variant {
			GUIMenuVariant::Test => vec![
				GUIElement::Rect { pos: [10, 10], size: [10, 10], alignment: GUIAlignment::Left, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
				GUIElement::Button {
					pos: [30, 20], size: [30, 15], alignment: GUIAlignment::Left, text: "Hi".to_string(), enabled: true,
					tick_mut_gui: (|_, _, _, _| println!("Hi")),
				},
				GUIElement::Text { text: "Hello".to_string(), pos: [50, 40], alignment: GUIAlignment::Left, text_alignment: GUIAlignment::Left },
			],
			GUIMenuVariant::Paused => vec![
				GUIElement::Rect { pos: [51, 28], size: [154, 200], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
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
						gui.menus = Vec::new();
						gui.menus.push(Self::new(GUIMenuVariant::ExitingGame));
					}),
				},
				GUIElement::Button {
					pos: [53, 190], size: [150, 16], alignment: GUIAlignment::Center, text: "Exit to Title".to_string(), enabled: true,
					tick_mut_gui: (|_, gui, world, _| {
						if let Some(world) = world {
							world.is_freeing = true;
						}
						gui.menus = Vec::new();
						gui.menus.push(Self::new(GUIMenuVariant::ExitingToTitle));
					}),
				},
			],
			GUIMenuVariant::ExitingGame => vec![
				GUIElement::Rect { pos: [51, 76], size: [154, 104], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
				GUIElement::Text { text: "Closing World...".to_string(), pos: [127, 120], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
			GUIMenuVariant::ExitingToTitle => vec![
				GUIElement::Rect { pos: [51, 76], size: [154, 104], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
				GUIElement::Text { text: "Closing World...".to_string(), pos: [127, 120], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
			GUIMenuVariant::Title => vec![
				GUIElement::Rect { pos: [51, 28], size: [154, 200], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
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
			GUIMenuVariant::IngameHUD => {
				let mut out = Vec::new();
				let world = world.as_ref().unwrap();
				let player = world.player.as_ref().unwrap();
				let (inventory, selected_item) = match &player.entity_type {
					EntityType::Player { inventory, selected_item, .. } => (inventory, selected_item),
					//_ => return Vec::new(),
				};
				for (item_index, item_stack) in inventory.iter().enumerate() {
					let x = item_index as u16 % 10;
					let y = item_index as u16 / 10;
					let color = match (x % 2 == 0) ^ (y % 2 == 0)  {
						true => [63, 63, 63, 63],
						false => [31, 31, 31, 63],
					};
					out.push(GUIElement::Rect { pos: [x * 16, y * 16], size: [16, 16], alignment: GUIAlignment::Left, color: NO_COLOR, border_color: color });
					if item_stack.1 > 0 {
						out.push(GUIElement::Texture { pos: [x * 16, y * 16], alignment: GUIAlignment::Left, texture: item_stack.0.get_texture() });
					}
				}
				out.push(GUIElement::Rect { pos: [*selected_item as u16 % 10 * 16, *selected_item as u16 / 10 * 16], size: [16, 16], alignment: GUIAlignment::Left, color: NO_COLOR, border_color: [63, 63, 63, 127] });
				out
			}
			GUIMenuVariant::CreateWorld => vec![
				GUIElement::Rect { pos: [51, 28], size: [154, 200], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
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
				GUIElement::Rect { pos: [51, 88], size: [154, 80], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
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
					GUIElement::Rect { pos: [51, 28], size: [154, 200], alignment: GUIAlignment::Center, color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
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
						tick_mut_gui: (|element, gui, world, io| {
							if let GUIElement::Button { pos, .. } = element {
								let button_y = ((pos[1] - 30) / 20) as usize;
								let top_menu = &gui.menus.last().unwrap().variant;
								if let GUIMenuVariant::LoadWorld { load_world_data, page } = top_menu {
									let world_index = button_y + page * 8;
									let world_path = &load_world_data.worlds[world_index].1;
									if let Some(new_world) = World::load(world_path.clone(), io) {
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

	/// Get all elements.
	pub fn get_elements(&self, world: &Option<World>) -> Vec<GUIElement> {
		let mut out = self.get_const_elements(world);
		out.extend(self.extra_elements.clone());
		out
	}

	/// Render all elements.
	pub fn render(&self, vertices: &mut Vec<Vertex>, io: &IO, world: &Option<World>) {
		for element in self.get_elements(world) {
			element.render(vertices, io);
		}
	}

	/// Weather the game should pause when this menu is in the GUI stack.
	pub fn does_menu_pause_game(&self) -> bool {
		match self.variant {
			GUIMenuVariant::Test => true,
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

	/// What to do when Esc is pressed.
	pub fn menu_close_button_action(self, gui: &mut GUI, _world: &mut Option<World>, io: &mut IO) {
		match self.variant {
			GUIMenuVariant::Paused => {
				gui.menus.pop();
				io.update_keys_pressed_last();
			},
			GUIMenuVariant::Test => {
				gui.menus.pop();
				io.update_keys_pressed_last();
			},
			_ => {}
		}
	}

	/// Menu tick.
	pub fn tick(self, gui: &mut GUI, world: &mut Option<World>, io: &mut IO, request_game_close: bool) {
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
			GUIMenuVariant::IngameHUD => {
				if io.get_game_key_starting_now(GameKey::OpenTestMenu) {
					gui.menus.push(Self::new(GUIMenuVariant::Test))
				}
			}
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

	/// Create the menu setting all the elements that need to be stored between frames.
	pub fn new(variant: GUIMenuVariant) -> Self {
		Self {
			variant: variant.clone(),
			extra_elements: match variant {
				GUIMenuVariant::CreateWorld => vec![
					GUIElement::TextEntry { text: "".to_string(), pos: [53, 50], size: [150, 16], alignment: GUIAlignment::Center, is_selected: false, text_length_limit: 20 },
					GUIElement::TextEntry { text: "".to_string(), pos: [53, 90], size: [150, 16], alignment: GUIAlignment::Center, is_selected: false, text_length_limit: 10 },
				],
				GUIMenuVariant::Test => vec![
					GUIElement::ToggleButton { text: "Hi".to_string(), pos: [53, 50], size: [150, 16], alignment: GUIAlignment::Center, enabled: true, state: true },
					GUIElement::MutuallyExclusiveButtonGroup { alignment: GUIAlignment::Center, selected_button: 0, buttons: vec![
						("Button 0".to_string(), [0, 100], [100, 16]),
						("Button 1".to_string(), [104, 100], [100, 16]),
						("Button 2".to_string(), [208, 100], [100, 16]),
					] },
				],
				_ => Vec::new(),
			},
		}
	}

	/// Create a error GUI menu from a string.
	pub fn new_error(error: String) -> Self {
		Self {
			variant: GUIMenuVariant::Error,
			extra_elements: vec![
				GUIElement::Text { text: error, pos: [127, 116], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
			],
		}
	}

	/// Create a load world menu.
	pub fn new_load_world(page: usize, load_world_data: LoadWorldData) -> Self {
		Self {
			variant: GUIMenuVariant::LoadWorld { page, load_world_data },
			extra_elements: vec![],
		}
	}
}