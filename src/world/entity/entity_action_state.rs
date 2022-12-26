#[derive(Eq, PartialEq, Clone)]
pub enum EntityActionState {
	Idle,
	Walking(u8),
}