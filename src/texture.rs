use std::mem::swap;

use crate::{const_static_ptr, vertex::Vertex, world_pos_to_render_pos, texture_type::TextureType, direction::Direction4};

const TEXTURE_SHEET_SIZE: [u32; 2] = [256, 256];

const fn grid_texture(id: u8) -> [u16; 4] {
	[id as u16 % 16 * 16, id as u16 / 16 * 16, 16, 16]
}

#[derive(Copy, Clone)]
pub enum Texture {
	Grass,
	Water,
	Player,
	RedThing,
	GreenThing,
	BlueThing,
}

impl Texture {
	/// Returns [start x and y, width and height]
	const fn get_texture_sheet_points(self) -> &'static [u16; 4] {
		match self {
			Self::Grass => const_static_ptr!([u16; 4], grid_texture(0)),
			Self::Water => const_static_ptr!([u16; 4], grid_texture(1)),
			Self::Player => const_static_ptr!([u16; 4], grid_texture(2)),
			Self::RedThing => const_static_ptr!([u16; 4], grid_texture(0xF)),
			Self::GreenThing => const_static_ptr!([u16; 4], grid_texture(0xF0)),
			Self::BlueThing => const_static_ptr!([u16; 4], grid_texture(0xFF)),
		}
	}

	const fn get_type(self) -> TextureType {
		match self {
			Self::Grass => TextureType::Basic,
			Self::Water => TextureType::Basic,
			Self::Player => TextureType::Entity,
			Self::RedThing => TextureType::Basic,
			Self::GreenThing => TextureType::Basic,
			Self::BlueThing => TextureType::Basic,
		}
	}

	pub fn render(self, tile_pos: [i64; 2], subtile_pos: [i8; 2]) -> [Vertex; 6] {
		self.render_basic(tile_pos, subtile_pos, false, 0)
	}

	pub fn render_entity(self, tile_pos: [i64; 2], subtile_pos: [i8; 2], direction: Direction4, walk_alt_frame: u8) -> [Vertex; 6] {
		match self.get_type() {
			TextureType::Basic => self.render_basic(tile_pos, subtile_pos, false, 0),
			TextureType::Entity => self.render_basic(tile_pos, subtile_pos, direction == Direction4::West, walk_alt_frame + match direction {
				Direction4::South => 0,
				Direction4::North => 3,
				Direction4::East => 6,
				Direction4::West => 6,
			}),
		}
	}

	pub fn render_basic(self, tile_pos: [i64; 2], subtile_pos: [i8; 2], reverse: bool, index: u8) -> [Vertex; 6] {
		let texture_sheet_points = self.get_texture_sheet_points();

		let [start_x, start_y] = world_pos_to_render_pos(tile_pos, subtile_pos);
		let end_x = start_x + texture_sheet_points[2] as f32 / 16.;
		let end_y = start_y + texture_sheet_points[3] as f32 / 16.;

		let width = texture_sheet_points[2] as f32 / TEXTURE_SHEET_SIZE[0] as f32;
		let height = texture_sheet_points[3] as f32 / TEXTURE_SHEET_SIZE[1] as f32;

		let mut texture_x_start = texture_sheet_points[0] as f32 / TEXTURE_SHEET_SIZE[0] as f32 + width * index as f32;
		let texture_y_start = 1. - (texture_sheet_points[1] as f32 / TEXTURE_SHEET_SIZE[1] as f32);
		let mut texture_x_end = texture_x_start + width;
		let texture_y_end = texture_y_start - height;
		if reverse {
			swap(&mut texture_x_start, &mut texture_x_end);
		}

		[
			Vertex { position: [start_x, start_y], texture_position: [texture_x_start, texture_y_start], color: [0., 0., 0., 0.] },
			Vertex { position: [end_x, start_y],   texture_position: [texture_x_end, texture_y_start],   color: [0., 0., 0., 0.] },
			Vertex { position: [start_x, end_y],   texture_position: [texture_x_start, texture_y_end],   color: [0., 0., 0., 0.] },
			Vertex { position: [end_x, start_y],   texture_position: [texture_x_end, texture_y_start],   color: [0., 0., 0., 0.] },
			Vertex { position: [end_x, end_y],     texture_position: [texture_x_end, texture_y_end],     color: [0., 0., 0., 0.] },
			Vertex { position: [start_x, end_y],   texture_position: [texture_x_start, texture_y_end],   color: [0., 0., 0., 0.] },
		]
	}
}