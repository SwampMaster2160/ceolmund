use crate::const_static_ptr;

const fn grid_texture(id: u8) -> [u16; 4] {
	[id as u16 % 16 * 16, id as u16 / 16 * 16, 16, 16]
}

enum Texture {
	Grass,
}

impl Texture {
	/// Returns [start x and y, width and height]
	const fn get_texture_sheet_points(self) -> &'static [u16; 4] {
		match self {
			Self::Grass => const_static_ptr!([u16; 4], grid_texture(0)),
		}
	}
}