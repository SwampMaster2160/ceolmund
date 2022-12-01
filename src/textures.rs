use crate::{const_static_ptr, vertex::Vertex};

const TEXTURE_SHEET_SIZE: [u32; 2] = [256, 256];

const fn grid_texture(id: u8) -> [u16; 4] {
	[id as u16 % 16 * 16, id as u16 / 16 * 16, 16, 16]
}

#[derive(Copy, Clone)]
pub enum Texture {
	Grass,
	Water,
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
			Self::RedThing => const_static_ptr!([u16; 4], grid_texture(0xF)),
			Self::GreenThing => const_static_ptr!([u16; 4], grid_texture(0xF0)),
			Self::BlueThing => const_static_ptr!([u16; 4], grid_texture(0xFF)),
		}
	}

	pub fn to_tris(self, tile_pos: [i64; 2], subtile_pos: [i8; 2]) -> [Vertex; 6] {
		let texture_sheet_points = self.get_texture_sheet_points();

		let start_x = tile_pos[0] as f32 + subtile_pos[0] as f32 / 16.;
		let start_y = tile_pos[1] as f32 + subtile_pos[1] as f32 / 16.;
		let end_x = start_x + texture_sheet_points[2] as f32 / 16.;
		let end_y = start_y + texture_sheet_points[3] as f32 / 16.;

		let texture_x_start = texture_sheet_points[0] as f32 / TEXTURE_SHEET_SIZE[0] as f32;
		let texture_y_start = 1. - (texture_sheet_points[1] as f32 / TEXTURE_SHEET_SIZE[1] as f32);
		let texture_x_end = texture_x_start + texture_sheet_points[2] as f32 / TEXTURE_SHEET_SIZE[0] as f32;
		let texture_y_end = texture_y_start - texture_sheet_points[3] as f32 / TEXTURE_SHEET_SIZE[1] as f32;

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