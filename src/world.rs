use crate::{vertex::Vertex, texture::Texture, entity::{Entity, entity_action_state::EntityActionState}, direction::Direction4, world_pos_to_render_pos, input::Input, game_key::GameKey};

pub struct World {
	player: Entity,
}

impl World {
	pub fn new() -> Self {
		Self { 
			player: Entity {
				pos: [0, 0],
				action_state: EntityActionState::Idle,
    			facing: Direction4::South,
			}
		}
	}

	// Render the world getting a vector of tris and the center pos of the camera.
	pub fn render(&self) -> (Vec<Vertex>, [f32; 2]) {
		let mut vertices = Vec::new();
		for y in -32..32 {
			for x in -20..20 {
				let mut texture = Texture::Grass;
				if x == 0 || y == 0 {
					texture = Texture::Water;
				}
				vertices.extend(texture.to_tris([x, y], [0, 0]));
			}
		}
		let player = &self.player;
		(vertices, world_pos_to_render_pos(player.pos, player.get_subtile_pos()))
	}

	pub fn tick(&mut self, input: &Input) {
		if input.get_game_key(GameKey::WalkNorth) {
			self.player.pos[1] -= 1;
		}
		if input.get_game_key(GameKey::WalkEast) {
			self.player.pos[0] += 1;
		}
		if input.get_game_key(GameKey::WalkSouth) {
			self.player.pos[1] += 1;
		}
		if input.get_game_key(GameKey::WalkWest) {
			self.player.pos[0] -= 1;
		}
	}
}