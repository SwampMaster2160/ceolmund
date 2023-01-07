use super::load_world_data::LoadWorldData;

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
	LoadWorld { load_world_data: LoadWorldData/*, page: usize*/ },
}