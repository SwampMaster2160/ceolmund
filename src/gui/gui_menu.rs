use crate::{render::{vertex::Vertex}, io::{io::IO, game_key::GameKey}, world::{world::World, entity::entity_type::{EntityType, EntityVariant}, difficulty::Difficulty, item::item::Item, tile::tile::Tile}};

use super::{gui_alignment::GUIAlignment, gui_element::GUIElement, gui::GUI, gui_menu_variant::GUIMenuVariant, load_world_data::WorldList, gui_rect::GUIRect};

const RECT_COLOR: [u8; 4] = [31, 31, 31, 255];
const RECT_BORDER_COLOR: [u8; 4] = [15, 15, 15, 255];
const GRAYOUT_COLOR: [u8; 4] = [63, 63, 63, 127];
const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

const SANDBOX_SPAWNABLE_ITEMS: [Item; 14] = [
	Item::SandboxDestroyWand,
	Item::Axe,
	Item::Shovel,
	Item::Tile(Tile::Grass),
	Item::Tile(Tile::Gravel),
	Item::Tile(Tile::Sand),
	Item::Tile(Tile::BlackSand),
	Item::Tile(Tile::Water),
	Item::Tile(Tile::Path),
	Item::Tile(Tile::Flowers),
	Item::Tile(Tile::FlowersRedYellow),
	Item::Tile(Tile::OakTree),
	Item::Tile(Tile::PineTree),
	Item::Tile(Tile::Rocks),
];

/// A GUI menu, these are stacked.
#[derive(Clone)]
pub struct GUIMenu {
	variant: GUIMenuVariant,
	pub elements: Vec<GUIElement>,
}

impl GUIMenu {
	/// Create the menu with all the elements that will to be stored between frames.
	pub fn new(variant: GUIMenuVariant) -> Self {
		Self {
			variant: variant.clone(),
			elements: match variant {
				GUIMenuVariant::CreateWorld => vec![
					GUIElement::RectContainer { rect: GUIRect::new(51, 28, 154, 200), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
						GUIElement::Text { text: "Create World".to_string(), pos: [77, -20], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
						GUIElement::Text { text: "Name:".to_string(), pos: [0, 0], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Left },
						GUIElement::TextEntry { text: "".to_string(), rect: GUIRect::new(0, 20, 150, 16), alignment: GUIAlignment::Center, is_selected: false, text_length_limit: 20 },
						GUIElement::Text { text: "Seed:".to_string(), pos: [0, 40], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Left },
						GUIElement::TextEntry { text: "".to_string(), rect: GUIRect::new(0, 60, 150, 16), alignment: GUIAlignment::Center, is_selected: false, text_length_limit: 10 },
						GUIElement::Text { text: "Difficulty:".to_string(), pos: [0, 80], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Left },
						GUIElement::MutuallyExclusiveButtonGroup { alignment: GUIAlignment::Center, selected_button: 1, buttons: vec![
							("Sandbox".to_string(), GUIRect::new(0, 100, 37, 16), true),
							("Easy".to_string(), GUIRect::new(38, 100, 37, 16), true),
							("Medium".to_string(), GUIRect::new(76, 100, 37, 16), true),
							("Hard".to_string(), GUIRect::new(114, 100, 36, 16), true),
						] },
						GUIElement::Button {
							rect: GUIRect::new(0, 160, 150, 16), alignment: GUIAlignment::Center, text: "Create World".to_string(), enabled: true,
							click_mut_gui: (|_, gui, world, io| {
								// Get content of GUI menu box.
								let menu_box = &gui.menus.last().unwrap().elements[0];
								let menu_box_content = match menu_box {
									GUIElement::RectContainer { inside_elements, .. } => inside_elements,
									_ => return,
								};
								// Get world name.
								let name_text = match menu_box_content[2].clone() {
									GUIElement::TextEntry{text, ..} => text,
									_ => return,
								};
								// Get world seed.
								let seed_text = match menu_box_content[4].clone() {
									GUIElement::TextEntry{text, ..} => text,
									_ => return,
								};
								let seed = match seed_text.parse::<u32>() {
									Ok(seed) => seed,
									Err(_) if *seed_text == "".to_string() => 420,
									Err(_) => {
										gui.menus.push(GUIMenu::new_error("Invalid seed.".to_string()));
										return
									},
								};
								// Get difficulty.
								let difficulty_buttons = menu_box_content[6].clone();
								let difficulty = match difficulty_buttons {
									GUIElement::MutuallyExclusiveButtonGroup { selected_button, .. } => match selected_button {
										0 => Difficulty::Sandbox,
										1 => Difficulty::Easy,
										2 => Difficulty::Medium,
										3 => Difficulty::Hard,
										_ => return,
									}
									_ => return,
								};
								// Create world.
								match World::new(seed, name_text.clone(), io, difficulty) {
									Some(valid_world) => *world = Some(valid_world),
									None => {
										gui.menus.push(GUIMenu::new_error("Unable to create world.".to_string()));
										return
									},
								};
								// Set menus to ingame HUD.
								gui.menus = vec![Self::new(GUIMenuVariant::IngameHUD)];
							}),
						},
						GUIElement::Button {
							rect: GUIRect::new(0, 180, 150, 16), alignment: GUIAlignment::Center, text: "Cancel".to_string(), enabled: true,
							click_mut_gui: (|_, gui, _, _| {
								gui.menus = vec![Self::new(GUIMenuVariant::Title)];
							}),
						},
					] },
				],
				GUIMenuVariant::Test => vec![
					GUIElement::ToggleButton { text: "Hi".to_string(), rect: GUIRect::new(53, 50, 150, 16), alignment: GUIAlignment::Center, enabled: true, state: true },
					GUIElement::MutuallyExclusiveButtonGroup { alignment: GUIAlignment::Center, selected_button: 0, buttons: vec![
						("Button 0".to_string(), GUIRect::new(0, 100, 100, 16), true),
						("Button 1".to_string(), GUIRect::new(104, 100, 100, 16), false),
						("Button 2".to_string(), GUIRect::new(208, 100, 100, 16), true),
					] },
					GUIElement::ScrollArea {
						rect: GUIRect::new(100, 120, 80, 130), alignment: GUIAlignment::Center, border_color: RECT_BORDER_COLOR, inside_color: RECT_COLOR,
						inside_height: 150, scroll: 0, inside_elements: vec![
							GUIElement::Button {
								rect: GUIRect::new(0, 0, 76, 16), alignment: GUIAlignment::Center, text: "S".to_string(), enabled: true,
								click_mut_gui: (|_, _, _, _| println!("S")),
							},
							GUIElement::SingleFunctionButtonGroup {
								alignment: GUIAlignment::Center, buttons: vec![
									("Button 0".to_string(), GUIRect::new(0, 40, 76, 16), true),
									("Button 1".to_string(), GUIRect::new(0, 60, 76, 16), false),
									("Button 2".to_string(), GUIRect::new(0, 80, 76, 16), true),
								],
								click_mut_gui: (|_, _, _, _, button_clicked_index| println!("Hi {button_clicked_index}.")),
							},
							GUIElement::Button {
								rect: GUIRect::new(0, 134, 76, 16), alignment: GUIAlignment::Center, text: "K".to_string(), enabled: true,
								click_mut_gui: (|_, _, _, _| println!("K")),
							},
						],
					},
					GUIElement::Rect { rect: GUIRect::new(10, 10, 10, 10), alignment: GUIAlignment::Left, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR },
					GUIElement::Button {
						rect: GUIRect::new(30, 20, 30, 15), alignment: GUIAlignment::Left, text: "Hi".to_string(), enabled: true,
						click_mut_gui: (|_, _, _, _| println!("Hi")),
					},
					GUIElement::Text { text: "Hello".to_string(), pos: [50, 40], alignment: GUIAlignment::Left, text_alignment: GUIAlignment::Left },
					GUIElement::SingleFunctionButtonGroup {
						alignment: GUIAlignment::Left, buttons: vec![
							("Button 0".to_string(), GUIRect::new(30, 130, 100, 16), true),
							("Button 1".to_string(), GUIRect::new(30, 160, 100, 16), false),
							("Button 2".to_string(), GUIRect::new(30, 190, 100, 16), true),
						],
						click_mut_gui: (|_, _, _, _, button_clicked_index| println!("Hi {button_clicked_index}.")),
					},
				],
				GUIMenuVariant::Title => vec![
					GUIElement::RectContainer {
						rect: GUIRect::new(51, 28, 154, 200), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
							GUIElement::Text { text: "Ceolmund".to_string(), pos: [77, -20], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
							GUIElement::Button {
								rect: GUIRect::new(0, 0, 150, 16), alignment: GUIAlignment::Center, text: "Create World".to_string(), enabled: true,
								click_mut_gui: (|_, gui, _, _| {
									gui.menus.pop();
									gui.menus.push(Self::new(GUIMenuVariant::CreateWorld));
								}),
							},
							GUIElement::Button {
								rect: GUIRect::new(0, 20, 150, 16), alignment: GUIAlignment::Center, text: "Load World".to_string(), enabled: true,
								click_mut_gui: (|_, gui, _, io| {
									gui.menus.pop();
									gui.menus.push(Self::new(GUIMenuVariant::LoadWorld{ world_list: WorldList::new(io) }));
								}),
							},
							GUIElement::Button {
								rect: GUIRect::new(0, 180, 150, 16), alignment: GUIAlignment::Center, text: "Exit Game".to_string(), enabled: true,
								click_mut_gui: (|_, gui, _, _| {
									gui.should_close_game = true;
								}),
							},
						],
					},
				],
				GUIMenuVariant::LoadWorld { world_list: load_world_data } => {
					// Create world buttons
					let world_count = load_world_data.worlds.len();
					let mut buttons = Vec::new();
					for world_index in 0..world_count {
						let world = load_world_data.worlds[world_index].clone();
						buttons.push((world.0, GUIRect::new(0, world_index as i16 * 20, 146, 16), true));
					}
					// Create the button group for the buttons with one function.
					let buttons = GUIElement::SingleFunctionButtonGroup {
						alignment: GUIAlignment::Center, buttons,
						click_mut_gui: (|_, gui, world, io, button_clicked_index| {
							let top_menu = &gui.menus.last().unwrap().variant;
							if let GUIMenuVariant::LoadWorld { world_list: load_world_data } = top_menu {
								let world_path = &load_world_data.worlds[button_clicked_index].1;
								if let Some(new_world) = World::load(world_path.clone(), io, false) {
									*world = Some(new_world);
									gui.menus = vec![GUIMenu::new(GUIMenuVariant::IngameHUD)];
								}
								else {
									gui.menus.push(GUIMenu::new_error("Unable to load world".to_string()));
								}
							}
						}),
					};
					// Vector with all elements.
					vec![
						GUIElement::RectContainer {
							rect: GUIRect::new(51, 28, 154, 200), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
								GUIElement::Text { text: "Load World".to_string(), pos: [77, -20], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
								GUIElement::ScrollArea {
									rect: GUIRect::new(0, 0, 150, 176), alignment: GUIAlignment::Center, border_color: RECT_BORDER_COLOR, inside_color: RECT_COLOR,
									inside_height: (world_count as u16).saturating_mul(20).saturating_sub(4), scroll: 0, inside_elements: vec![
										buttons,
									],
								},
								GUIElement::Button {
									rect: GUIRect::new(0, 180, 150, 16), alignment: GUIAlignment::Center, text: "Cancel".to_string(), enabled: true,
									click_mut_gui: (|_, gui, _, _| {
										gui.menus = vec![Self::new(GUIMenuVariant::Title)];
									}),
								},
							],
						},
					]
				}
				GUIMenuVariant::Paused => vec![
					GUIElement::RectContainer {
						rect: GUIRect::new(51, 28, 154, 200), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
							GUIElement::Text { text: "Game Paused".to_string(), pos: [77, -20], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
							GUIElement::Button {
								rect: GUIRect::new(0, 0, 150, 16), alignment: GUIAlignment::Center, text: "Resume".to_string(), enabled: true,
								click_mut_gui: (|_, gui, _, _| {gui.menus.pop();}),
							},
							GUIElement::Button {
								rect: GUIRect::new(0, 160, 150, 16), alignment: GUIAlignment::Center, text: "Exit to Title".to_string(), enabled: true,
								click_mut_gui: (|_, gui, world, _| {
									if let Some(world) = world {
										world.is_freeing = true;
									}
									gui.menus = Vec::new();
									gui.menus.push(Self::new(GUIMenuVariant::ExitingToTitle));
								}),
							},
							GUIElement::Button {
								rect: GUIRect::new(0, 180, 150, 16), alignment: GUIAlignment::Center, text: "Exit Game".to_string(), enabled: true,
								click_mut_gui: (|_, gui, world, _| {
									if let Some(world) = world {
										world.is_freeing = true;
									}
									gui.menus = Vec::new();
									gui.menus.push(Self::new(GUIMenuVariant::ExitingGame));
								}),
							},
						],
					},
				],
				GUIMenuVariant::ExitingGame | GUIMenuVariant::ExitingToTitle => vec![
					GUIElement::RectContainer {
						rect: GUIRect::new(51, 76, 154, 104), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
							GUIElement::Text { text: "Closing World...".to_string(), pos: [77, 44], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
						],
					},
				],
				GUIMenuVariant::Error => vec![
					GUIElement::Grayout { color: GRAYOUT_COLOR },
					GUIElement::RectContainer {
						rect: GUIRect::new(51, 88, 154, 80), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
							GUIElement::Text { text: "Error".to_string(), pos: [77, -20], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
							GUIElement::Button {
								rect: GUIRect::new(0, 60, 150, 16), alignment: GUIAlignment::Center, text: "OK".to_string(), enabled: true,
								click_mut_gui: (|_, gui, _, _| {
									gui.menus.pop();
								}),
							},
						],
					},
				],
				GUIMenuVariant::IngameHUD => Vec::new(),
				GUIMenuVariant::SpawnItems => {
					// Grid elements.
					let mut grid_elements = Vec::new();
					for (item_index, item) in SANDBOX_SPAWNABLE_ITEMS.iter().enumerate() {
						let mut cell_elements = Vec::new();
						let x = item_index as u16 % 10;
						let y = item_index as u16 / 10;
						// Add cell gray rect.
						let color = match (x % 2 == 0) ^ (y % 2 == 0)  {
							true => [63, 63, 63, 63],
							false => [31, 31, 31, 63],
						};
						cell_elements.push(GUIElement::Rect { rect: GUIRect::new(0, 0, 16, 16), alignment: GUIAlignment::Center, inside_color: NO_COLOR, border_color: color });
						// Item texture
						cell_elements.push(GUIElement::Texture { pos: [0, 0], alignment: GUIAlignment::Center, texture: item.get_texture() });

						grid_elements.push(GUIElement::ElementCollection { offset: [0, 0], inside_elements: cell_elements })
					}
					// All elements.
					vec![
						GUIElement::RectContainer {
							rect: GUIRect::new(46, 36, 164, 184), alignment: GUIAlignment::Center, inside_color: RECT_COLOR, border_color: RECT_BORDER_COLOR, inside_elements: vec![
								GUIElement::Text { text: "Spawn Items".to_string(), pos: [77, -20], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center },
								GUIElement::Grid { alignment: GUIAlignment::Center , cell_rect: GUIRect::new(0, 0, 16, 16), cell_counts: [10, 10], inside_elements: grid_elements, click_mut_gui: |_, _, world, _, item_clicked_on_index|{
									// Get player inventory object.
									let world = match world {
										Some(world) => world,
										_ => return,
									};
									let player = match &mut world.player {
										Some(player) => player,
										_ => return,
									};
									let inventory = match &mut player.entity_type {
										EntityType::Player { inventory, .. } => inventory,
										//_ => return,
									};
									// Get the item clicked on
									let item = match SANDBOX_SPAWNABLE_ITEMS.get(item_clicked_on_index) {
										Some(item) => item,
										None => return,
									};
									// Add the item to the inventory, ignore overflowing items.
									inventory.add_items((item.clone(), 1));
								} },
								GUIElement::Button {
									rect: GUIRect::new(0, 164, 160, 16), alignment: GUIAlignment::Center, text: "Resume".to_string(), enabled: true,
									click_mut_gui: (|_, gui, _, _| {gui.menus.pop();}),
								},
							],
						},
					]
				}
			},
		}
	}

	/// Get the elements that must be created each frame.
	pub fn get_frame_elements(&self, world: &Option<World>) -> Vec<GUIElement> {
		match &self.variant {
			GUIMenuVariant::IngameHUD => {
				let mut out = Vec::new();
				let world = world.as_ref().unwrap();
				let player = world.player.as_ref().unwrap();
				let (inventory, selected_item, is_swaping_items) = match &player.entity_type {
					EntityType::Player { inventory, selected_item, is_swaping_item, .. } => (inventory, selected_item, is_swaping_item),
				};
				// Item area
				let mut grid_elements = Vec::new();
				for (item_index, item_stack) in inventory.items.iter().enumerate() {
					let mut cell_elements = Vec::new();
					let x = item_index as u16 % 10;
					let y = item_index as u16 / 10;
					// Add cell gray rect.
					let color = match (x % 2 == 0) ^ (y % 2 == 0)  {
						true => [63, 63, 63, 63],
						false => [31, 31, 31, 63],
					};
					cell_elements.push(GUIElement::Rect { rect: GUIRect::new(0, 0, 16, 16), alignment: GUIAlignment::Left, inside_color: NO_COLOR, border_color: color });
					// Item texture
					let stack_size = item_stack.1;
					if stack_size > 0 {
						cell_elements.push(GUIElement::Texture { pos: [0, 0], alignment: GUIAlignment::Left, texture: item_stack.0.get_texture() });
					}
					// The text to show how many items are in a stack.
					if stack_size > 1 {
						cell_elements.push(GUIElement::Text { pos: [0, -4], alignment: GUIAlignment::Left, text: stack_size.to_string(), text_alignment: GUIAlignment::Left });
					}
					// Show selected item.
					if item_index == *selected_item as usize {
						let color = match *is_swaping_items {
							false => [63, 63, 63, 127],
							true => [31, 31, 31, 127],
						};
						cell_elements.push(GUIElement::Rect { rect: GUIRect::new(0, 0, 16, 16), alignment: GUIAlignment::Left, inside_color: NO_COLOR, border_color: color });
					}

					grid_elements.push(GUIElement::ElementCollection { offset: [0, 0], inside_elements: cell_elements })
				}
				out.push(GUIElement::Grid { alignment: GUIAlignment::Left , cell_rect: GUIRect::new(0, 0, 16, 16), cell_counts: [10, 5], inside_elements: grid_elements, click_mut_gui: |_, _, world, _, item_clicked_on_index|{
					let world = match world {
						Some(world) => world,
						_ => return,
					};
					let player = match &mut world.player {
						Some(player) => player,
						_ => return,
					};
					let (selected_item, is_swaping_item, inventory) = match &mut player.entity_type {
						EntityType::Player { selected_item, is_swaping_item, inventory, .. } => (selected_item, is_swaping_item, inventory),
						//_ => return,
					};
					if *selected_item == item_clicked_on_index as u8 {
						*is_swaping_item = !*is_swaping_item;
					}
					else {
						if *is_swaping_item {
							inventory.swap_items(*selected_item as usize, item_clicked_on_index);
						}
						*is_swaping_item = false;
						*selected_item = item_clicked_on_index as u8;
					}
				} });
				// Health bar
				if world.difficulty != Difficulty::Sandbox {
					out.push(GUIElement::ProgressBar {
						rect: GUIRect::new(256 - 202 - 2, 2, 202, 8), alignment: GUIAlignment::Right, inside_color: [255, 0, 0, 255], border_color: [0, 0, 0, 255],
						progress: player.health, max_progress: EntityVariant::Player.max_health(),
					});
				}
				out
			}
			_ => Vec::new(),
		}
	}

	/// Get all elements.
	pub fn get_elements(&self, world: &Option<World>) -> Vec<GUIElement> {
		let mut out = self.get_frame_elements(world);
		out.extend(self.elements.clone());
		out
	}

	/// Render all elements.
	pub fn render(&self, vertices: &mut Vec<Vertex>, io: &IO, world: &Option<World>) {
		for element in self.get_elements(world) {
			element.render(GUIRect::EVERYTHING, vertices, io, [0, 0]);
		}
	}

	/// Weather the game should pause when this menu is in the GUI stack.
	pub fn does_menu_pause_game(&self) -> bool {
		match self.variant {
			GUIMenuVariant::Test | GUIMenuVariant::Paused | GUIMenuVariant::ExitingGame | GUIMenuVariant::ExitingToTitle |
			GUIMenuVariant::Title | GUIMenuVariant::CreateWorld | GUIMenuVariant::Error | GUIMenuVariant::LoadWorld { .. } | GUIMenuVariant::SpawnItems => true,
			GUIMenuVariant::IngameHUD => false,
		}
	}

	/// What to do when Esc is pressed.
	pub fn menu_close_button_action(self, gui: &mut GUI, _world: &mut Option<World>, io: &mut IO) {
		match self.variant {
			GUIMenuVariant::Paused | GUIMenuVariant::Test | GUIMenuVariant::SpawnItems => {
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
				if io.get_game_key_starting_now(GameKey::OpenSpawnItemsMenu) {
					gui.menus.push(Self::new(GUIMenuVariant::SpawnItems))
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

	/// Create a error GUI menu from a string.
	pub fn new_error(error: String) -> Self {
		let mut out = Self::new(GUIMenuVariant::Error);
		out.elements.push(GUIElement::Text { text: error, pos: [127, 116], alignment: GUIAlignment::Center, text_alignment: GUIAlignment::Center });
		out
	}
}