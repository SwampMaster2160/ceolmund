use crate::{io::input::Input, gui::gui_alignment::GUIAlignment};

use super::{vertex::Vertex, texture::TEXTURE_SHEET_SIZE, render_data::RenderData};

const TEXTURE_SHEET_TEXT_START: [u32; 2] = [256, 0];

pub fn gui_pos_to_screen_pos(pos: [u16; 2], alignment: GUIAlignment, input: &Input) -> [f32; 2] {
	let offset = match alignment {
		GUIAlignment::Left => 0.,
		GUIAlignment::Center => (input.aspect_ratio - 1.) / 2.,
		GUIAlignment::Right => input.aspect_ratio - 1.,
	};
	[pos[0] as f32 + offset * 256., pos[1] as f32]
}

pub fn gui_size_to_screen_size(size: [u16; 2]) -> [f32; 2] {
	[size[0] as f32, size[1] as f32]
}

pub fn render_gui_rect(pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4], input: &Input) -> [Vertex; 6] {
	let [start_x, start_y] = gui_pos_to_screen_pos(pos, alignment, input);
	let gui_size = gui_size_to_screen_size(size);
	let end_x = start_x + gui_size[0];
	let end_y = start_y + gui_size[1];
	let render_color = [color[0] as f32 / 255., color[1] as f32 / 255., color[2] as f32 / 255., color[3] as f32 / 255.];
	[
		Vertex { position: [start_x, start_y], texture_position: [0., 0.], color: render_color },
		Vertex { position: [end_x, start_y],   texture_position: [0., 0.], color: render_color },
		Vertex { position: [start_x, end_y],   texture_position: [0., 0.], color: render_color },
		Vertex { position: [end_x, start_y],   texture_position: [0., 0.], color: render_color },
		Vertex { position: [end_x, end_y],     texture_position: [0., 0.], color: render_color },
		Vertex { position: [start_x, end_y],   texture_position: [0., 0.], color: render_color },
	]
}

pub fn render_gui_char(chr: char, pos: [u16; 2], alignment: GUIAlignment, input: &Input, render_data: &RenderData) -> ([Vertex; 6], u8) {
	let [start_x, start_y] = gui_pos_to_screen_pos(pos, alignment, input);
	let gui_size = gui_size_to_screen_size([8, 16]);
	let end_x = start_x + gui_size[0];
	let end_y = start_y + gui_size[1];

	let char_id: u32 = chr.into();
	let char_texture_x = TEXTURE_SHEET_TEXT_START[0] + char_id % 16 * 8 + char_id / 256 * 128;
	let char_texture_y = TEXTURE_SHEET_TEXT_START[0] + char_id / 16 * 16 % 256;

	let width = 8 as f32 / TEXTURE_SHEET_SIZE[0] as f32;
	let height = 16 as f32 / TEXTURE_SHEET_SIZE[1] as f32;

	let texture_x_start = char_texture_x as f32 / TEXTURE_SHEET_SIZE[0] as f32 + 0.0001;
	let texture_y_start = (char_texture_y as f32 / TEXTURE_SHEET_SIZE[1] as f32) - 0.0001;
	let texture_x_end = texture_x_start + width - 0.0002;
	let texture_y_end = texture_y_start + height + 0.0001;

	([
		Vertex { position: [start_x, start_y], texture_position: [texture_x_start, texture_y_start], color: [0., 0., 0., 0.] },
		Vertex { position: [end_x, start_y],   texture_position: [texture_x_end, texture_y_start],   color: [0., 0., 0., 0.] },
		Vertex { position: [start_x, end_y],   texture_position: [texture_x_start, texture_y_end],   color: [0., 0., 0., 0.] },
		Vertex { position: [end_x, start_y],   texture_position: [texture_x_end, texture_y_start],   color: [0., 0., 0., 0.] },
		Vertex { position: [end_x, end_y],     texture_position: [texture_x_end, texture_y_end],     color: [0., 0., 0., 0.] },
		Vertex { position: [start_x, end_y],   texture_position: [texture_x_start, texture_y_end],   color: [0., 0., 0., 0.] },
	], match render_data.widths.get(char_id as usize) {
		Some(width) => *width,
		None => 8,
	})
}

pub fn render_gui_string(string: &str, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment, input: &Input, render_data: &RenderData, vertices: &mut Vec<Vertex>) {
	let mut width = 0u32;
	for chr in string.chars() {
		let char_id: u32 = chr.into();
		width += (match render_data.widths.get(char_id as usize) {
			Some(width) => *width,
			None => 8,
		} + 1) as u32;
	}
	let offset = match text_alignment {
		GUIAlignment::Left => 0,
		GUIAlignment::Center => width / 2,
		GUIAlignment::Right => width,
	};
	let mut x = pos[0].saturating_sub(offset as u16);
	for chr in string.chars() {
		let (char_vertices, char_width) = render_gui_char(chr, [x, pos[1]], alignment, input, render_data);
		vertices.extend(char_vertices);
		x += char_width as u16 + 1;
	}
}