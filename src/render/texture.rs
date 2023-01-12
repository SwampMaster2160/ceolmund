use std::mem::swap;

use crate::{const_static_ptr, render::vertex::Vertex, world::direction::Direction4, gui::gui_alignment::GUIAlignment, io::io::IO};

use super::{texture_type::TextureType, render::{world_pos_to_render_pos, gui_pos_to_screen_pos}};

/// Size of the texture sheet in pixels.
pub const TEXTURE_SHEET_SIZE: [u32; 2] = [640, 256];

const fn grid_texture(id: u8) -> [u16; 4] {
	[id as u16 % 16 * 16, id as u16 / 16 * 16, 16, 16]
}

/// A texture that can be drawn
#[derive(Copy, Clone)]
pub enum Texture {
	Grass,
	Water,
	Player,
	Sand,
	PineTree,
	OakTree,
	Flowers,
	FlowersRedYellow,
	Rocks,
	NoTexture,
	Gravel,
	Pit,
	BlackSand,
	Path,
	Hammer,
	Shovel,
	Axe,
	SandboxDestroyWand,
	DroppedItems,
	//GreenThing,
	//BlueThing,
}

impl Texture {
	/// Returns [start x and y, width and height]
	const fn get_texture_sheet_points(self) -> &'static [u16; 4] {
		match self {
			Self::Grass => const_static_ptr!([u16; 4], grid_texture(0)),
			Self::Water => const_static_ptr!([u16; 4], grid_texture(1)),
			Self::Player => const_static_ptr!([u16; 4], grid_texture(2)),
			Self::Sand => const_static_ptr!([u16; 4], grid_texture(0xB)),
			Self::PineTree => const_static_ptr!([u16; 4], grid_texture(0xC)),
			Self::OakTree => const_static_ptr!([u16; 4], grid_texture(0xD)),
			Self::Flowers => const_static_ptr!([u16; 4], grid_texture(0xE)),
			Self::FlowersRedYellow => const_static_ptr!([u16; 4], grid_texture(0xF)),
			Self::Rocks => const_static_ptr!([u16; 4], grid_texture(0x10)),
			Self::NoTexture => const_static_ptr!([u16; 4], grid_texture(0x11)),
			Self::Gravel => const_static_ptr!([u16; 4], grid_texture(0x12)),
			Self::Pit => const_static_ptr!([u16; 4], grid_texture(0x13)),
			Self::BlackSand => const_static_ptr!([u16; 4], grid_texture(0x14)),
			Self::Path => const_static_ptr!([u16; 4], grid_texture(0x15)),
			Self::Hammer => const_static_ptr!([u16; 4], grid_texture(0x16)),
			Self::Shovel => const_static_ptr!([u16; 4], grid_texture(0x17)),
			Self::Axe => const_static_ptr!([u16; 4], grid_texture(0x18)),
			Self::SandboxDestroyWand => const_static_ptr!([u16; 4], grid_texture(0x19)),
			Self::DroppedItems => const_static_ptr!([u16; 4], grid_texture(0x1A)),
			//Self::GreenThing => const_static_ptr!([u16; 4], grid_texture(0xF0)),
			//Self::BlueThing => const_static_ptr!([u16; 4], grid_texture(0xFF)),
		}
	}

	/// How should a texture be drawn, as is like the grass texture or with rotations like the player texture.
	const fn get_type(self) -> TextureType {
		match self {
			Self::Grass => TextureType::Basic,
			Self::Water => TextureType::Basic,
			Self::Player => TextureType::Entity,
			Self::Sand => TextureType::Basic,
			Self::PineTree => TextureType::Basic,
			Self::OakTree => TextureType::Basic,
			Self::Flowers => TextureType::Basic,
			Self::FlowersRedYellow => TextureType::Basic,
			Self::Rocks => TextureType::Basic,
			Self::NoTexture => TextureType::Basic,
			Self::Gravel => TextureType::Basic,
			Self::Pit => TextureType::Basic,
			Self::BlackSand => TextureType::Basic,
			Self::Path => TextureType::Basic,
			Self::Hammer => TextureType::Basic,
			Self::Shovel => TextureType::Basic,
			Self::Axe => TextureType::Basic,
			Self::SandboxDestroyWand => TextureType::Basic,
			Self::DroppedItems => TextureType::Basic,
			//Self::GreenThing => TextureType::Basic,
			//Self::BlueThing => TextureType::Basic,
		}
	}

	/// Render the basic texture getting it's tris.
	pub fn render_basic(self, tile_pos: [i64; 2], subtile_pos: [i8; 2]) -> [Vertex; 6] {
		self.render(tile_pos, subtile_pos, false, 0)
	}

	/// Render an entity texture getting it's tris.
	pub fn render_entity(self, tile_pos: [i64; 2], subtile_pos: [i8; 2], direction: Direction4, walk_alt_frame: u8) -> [Vertex; 6] {
		match self.get_type() {
			TextureType::Basic => self.render(tile_pos, subtile_pos, false, 0),
			TextureType::Entity => self.render(tile_pos, subtile_pos, direction == Direction4::West, walk_alt_frame + match direction {
				Direction4::South => 0,
				Direction4::North => 3,
				Direction4::East => 6,
				Direction4::West => 6,
			}),
		}
	}

	/// Render the texture
	fn render(self, tile_pos: [i64; 2], subtile_pos: [i8; 2], reverse: bool, index: u8) -> [Vertex; 6] {
		let texture_sheet_points = self.get_texture_sheet_points();

		let [start_x, start_y] = world_pos_to_render_pos(tile_pos, subtile_pos);
		let end_x = start_x + texture_sheet_points[2] as f32 / 16.;
		let end_y = start_y + texture_sheet_points[3] as f32 / 16.;

		let width = texture_sheet_points[2] as f32 / TEXTURE_SHEET_SIZE[0] as f32;
		let height = texture_sheet_points[3] as f32 / TEXTURE_SHEET_SIZE[1] as f32;

		let mut texture_x_start = texture_sheet_points[0] as f32 / TEXTURE_SHEET_SIZE[0] as f32 + width * index as f32 + 0.0001;
		let texture_y_start = 1. - (texture_sheet_points[1] as f32 / TEXTURE_SHEET_SIZE[1] as f32) - 0.0001;
		let mut texture_x_end = texture_x_start + width - 0.0002;
		let texture_y_end = texture_y_start - height + 0.0001;
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

	/// Render the texture to the gui
	pub fn gui_render(self, pos: [i16; 2], alignment: GUIAlignment, input: &IO) -> [Vertex; 6] {
	let texture_sheet_points = self.get_texture_sheet_points();

	let [start_x, start_y] = gui_pos_to_screen_pos(pos, alignment, input);
	let end_x = start_x + texture_sheet_points[2] as f32;
	let end_y = start_y + texture_sheet_points[3] as f32;

	let width = texture_sheet_points[2] as f32 / TEXTURE_SHEET_SIZE[0] as f32;
	let height = texture_sheet_points[3] as f32 / TEXTURE_SHEET_SIZE[1] as f32;

	let texture_x_start = texture_sheet_points[0] as f32 / TEXTURE_SHEET_SIZE[0] as f32 + 0.0001;
	let texture_y_start = 1. - (texture_sheet_points[1] as f32 / TEXTURE_SHEET_SIZE[1] as f32) - 0.0001;
	let texture_x_end = texture_x_start + width - 0.0002;
	let texture_y_end = texture_y_start - height + 0.0001;

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