use crate::world::item::crafting_recipes::CraftingRecipes;

use super::load_world_data::WorldList;

#[derive(Clone)]
pub enum GUIMenuVariant {
	Test,
	Paused,
	ExitingGame,
	ExitingToTitle,
	Title,
	IngameHUD,
	CreateWorld,
	Error,
	LoadWorld { world_list: WorldList },
	SpawnItems,
	Crafting(CraftingRecipes),
}