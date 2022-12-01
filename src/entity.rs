pub mod entity_action_state;

use crate::direction::Direction4;

use self::entity_action_state::EntityActionState;

pub struct Entity {
	pub pos: [i64; 2],
	pub action_state: EntityActionState,
	pub facing: Direction4,
}

impl Entity {
	pub fn get_subtile_pos(&self) -> [i8; 2] {
		match self.action_state {
			EntityActionState::Idle => [0, 0],
			EntityActionState::Walking(walked_amount) => match self.facing {
				Direction4::North => [0, -(walked_amount as i8)],
				Direction4::East => [-(walked_amount as i8), 0],
				Direction4::South => [0, (walked_amount as i8)],
				Direction4::West => [(walked_amount as i8), 0],
			}
		}
	}
}