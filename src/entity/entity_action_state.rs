#[derive(Eq, PartialEq)]
pub enum EntityActionState {
	Idle,
	Walking(u8),
}