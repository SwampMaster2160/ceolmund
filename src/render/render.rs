use crate::{io::io::IO, gui::{gui_alignment::GUIAlignment, gui_rect::GUIRect}};

use super::{vertex::Vertex, texture::TEXTURE_SHEET_SIZE};

const TEXTURE_SHEET_TEXT_START: [u32; 2] = [256, 0];

/// Converts a GUI pos and alignment to a y 0-256 screen pos.
pub fn gui_pos_to_screen_pos_unsigned(pos: [u16; 2], alignment: GUIAlignment, input: &IO) -> [f32; 2] {
	let offset = match alignment {
		GUIAlignment::Left => 0.,
		GUIAlignment::Center => (input.aspect_ratio - 1.) / 2.,
		GUIAlignment::Right => input.aspect_ratio - 1.,
	};
	[pos[0] as f32 + offset * 256., pos[1] as f32]
}

/// Converts a GUI pos and alignment to a y 0-256 screen pos.
pub fn gui_pos_to_screen_pos(pos: [i16; 2], alignment: GUIAlignment, input: &IO) -> [f32; 2] {
	let offset = match alignment {
		GUIAlignment::Left => 0.,
		GUIAlignment::Center => (input.aspect_ratio - 1.) / 2.,
		GUIAlignment::Right => input.aspect_ratio - 1.,
	};
	[pos[0] as f32 + offset * 256., pos[1] as f32]
}

/// Converts a world pos and alignment to a y 0-256 screen pos.
pub fn world_pos_to_render_pos(pos: [i64; 2], offset: [i8; 2]) -> [f32; 2] {
	[pos[0] as f32 + offset[0] as f32 / 16., pos[1] as f32 + offset[1] as f32 / 16.]
}

/// Converts a GUI size and alignment to a y 0-256 screen size.
pub fn gui_size_to_screen_size(size: [u16; 2]) -> [f32; 2] {
	[size[0] as f32, size[1] as f32]
}

/// Render a rectangle at the GUI pos and alignment.
pub fn render_gui_rect(pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4], border_color: [u8; 4], input: &IO) -> [Vertex; 12] {
	let [start_x, start_y] = gui_pos_to_screen_pos_unsigned(pos, alignment, input);
	let gui_size = gui_size_to_screen_size(size);
	let end_x = start_x + gui_size[0];
	let end_y = start_y + gui_size[1];
	let color = [color[0] as f32 / 255., color[1] as f32 / 255., color[2] as f32 / 255., color[3] as f32 / 255.];
	let border_color = [border_color[0] as f32 / 255., border_color[1] as f32 / 255., border_color[2] as f32 / 255., border_color[3] as f32 / 255.];
	[
		Vertex { position: [start_x, start_y], texture_position: [0., 0.], color: border_color },
		Vertex { position: [end_x, start_y],   texture_position: [0., 0.], color: border_color },
		Vertex { position: [start_x, end_y],   texture_position: [0., 0.], color: border_color },
		Vertex { position: [end_x, start_y],   texture_position: [0., 0.], color: border_color },
		Vertex { position: [end_x, end_y],     texture_position: [0., 0.], color: border_color },
		Vertex { position: [start_x, end_y],   texture_position: [0., 0.], color: border_color },

		Vertex { position: [start_x + 1., start_y + 1.], texture_position: [0., 0.], color: color },
		Vertex { position: [end_x - 1., start_y + 1.],   texture_position: [0., 0.], color: color },
		Vertex { position: [start_x + 1., end_y - 1.],   texture_position: [0., 0.], color: color },
		Vertex { position: [end_x - 1., start_y + 1.],   texture_position: [0., 0.], color: color },
		Vertex { position: [end_x - 1., end_y - 1.],     texture_position: [0., 0.], color: color },
		Vertex { position: [start_x + 1., end_y - 1.],   texture_position: [0., 0.], color: color },
	]
}

/// Tint the entire screen a color.
pub fn render_screen_grayout(color: [u8; 4], io: &IO) -> [Vertex; 6] {
	let [start_x, start_y] = gui_pos_to_screen_pos_unsigned([0, 0], GUIAlignment::Left, io);
	let [end_x, end_y] = gui_pos_to_screen_pos_unsigned([256, 256], GUIAlignment::Right, io);
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

/// Render a char at pos and alignment getting the tris and the GUI pixel width of the char
pub fn render_gui_char_u16(chr: char, pos: [u16; 2], alignment: GUIAlignment, io: &IO) -> ([Vertex; 6], u8) {
	let [start_x, start_y] = gui_pos_to_screen_pos_unsigned(pos, alignment, io);
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
	], match io.char_widths.get(char_id as usize) {
		Some(width) => *width,
		None => 8,
	})
}

/// Render a char at pos and alignment getting the tris and the GUI pixel width of the char
pub fn render_gui_char(chr: char, pos: [i16; 2], alignment: GUIAlignment, io: &IO, visable_area: GUIRect) -> ([Vertex; 6], u8) {
	let visable_area_end = visable_area.get_end();
	let visable_start = [
		pos[0].clamp(visable_area.pos[0], visable_area_end[0]),
		pos[1].clamp(visable_area.pos[1], visable_area_end[1]),
	];
	let visable_end = [
		pos[0].saturating_add(8).clamp(visable_area.pos[0], visable_area_end[0]),
		pos[1].saturating_add(16).clamp(visable_area.pos[1], visable_area_end[1]),
	];
	let start_offset = [
		visable_start[0].saturating_sub(pos[0]) as i32,
		visable_start[1].saturating_sub(pos[1]) as i32,
	];
	let end_offset = [
		visable_end[0].saturating_sub(pos[0].saturating_add(8)) as i32,
		visable_end[1].saturating_sub(pos[1].saturating_add(16)) as i32,
	];

	let [start_x, start_y] = gui_pos_to_screen_pos(visable_start, alignment, io);
	let [end_x, end_y] = gui_pos_to_screen_pos(visable_end, alignment, io);

	let char_id: u32 = chr.into();
	let char_texture_x_start = TEXTURE_SHEET_TEXT_START[0] + char_id % 16 * 8 + char_id / 256 * 128;
	let char_texture_y_start = TEXTURE_SHEET_TEXT_START[0] + char_id / 16 * 16 % 256;
	let char_texture_x_end = char_texture_x_start + 8;
	let char_texture_y_end = char_texture_y_start + 16;

	let texture_x_start = (char_texture_x_start.saturating_add_signed(start_offset[0]) as f32 / TEXTURE_SHEET_SIZE[0] as f32) + 0.0001;
	let texture_y_start = (char_texture_y_start.saturating_add_signed(start_offset[1]) as f32 / TEXTURE_SHEET_SIZE[1] as f32) - 0.0001;
	let texture_x_end = (char_texture_x_end.saturating_add_signed(end_offset[0]) as f32 / TEXTURE_SHEET_SIZE[0] as f32) - 0.0002;
	let texture_y_end = (char_texture_y_end.saturating_add_signed(end_offset[1]) as f32 / TEXTURE_SHEET_SIZE[1] as f32) + 0.0001;

	([
		Vertex { position: [start_x, start_y], texture_position: [texture_x_start, texture_y_start], color: [0., 0., 0., 0.] },
		Vertex { position: [end_x, start_y],   texture_position: [texture_x_end, texture_y_start],   color: [0., 0., 0., 0.] },
		Vertex { position: [start_x, end_y],   texture_position: [texture_x_start, texture_y_end],   color: [0., 0., 0., 0.] },
		Vertex { position: [end_x, start_y],   texture_position: [texture_x_end, texture_y_start],   color: [0., 0., 0., 0.] },
		Vertex { position: [end_x, end_y],     texture_position: [texture_x_end, texture_y_end],     color: [0., 0., 0., 0.] },
		Vertex { position: [start_x, end_y],   texture_position: [texture_x_start, texture_y_end],   color: [0., 0., 0., 0.] },
	], match io.char_widths.get(char_id as usize) {
		Some(width) => *width,
		None => 8,
	})
}

/// Render a string at a GUI pos and alignment.
pub fn render_gui_string_u16(string: &str, pos: [u16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment, io: &IO, vertices: &mut Vec<Vertex>) {
	let mut width = 0u32;
	for chr in string.chars() {
		let char_id: u32 = chr.into();
		width += (match io.char_widths.get(char_id as usize) {
			Some(width) => *width,
			None => 8,
		} + 1) as u32;
	}
	width = width.saturating_sub(1);
	let offset = match text_alignment {
		GUIAlignment::Left => 0,
		GUIAlignment::Center => width / 2,
		GUIAlignment::Right => width,
	};
	let mut x = pos[0].saturating_sub(offset as u16);
	for chr in string.chars() {
		let (char_vertices, char_width) = render_gui_char_u16(chr, [x, pos[1]], alignment, io);
		vertices.extend(char_vertices);
		x += char_width as u16 + 1;
	}
}

/// Render a string at a GUI pos and alignment.
pub fn render_gui_string(string: &str, pos: [i16; 2], alignment: GUIAlignment, text_alignment: GUIAlignment, io: &IO, vertices: &mut Vec<Vertex>, visable_area: GUIRect) {
	let mut width = 0u32;
	for chr in string.chars() {
		let char_id: u32 = chr.into();
		width += (match io.char_widths.get(char_id as usize) {
			Some(width) => *width,
			None => 8,
		} + 1) as u32;
	}
	width = width.saturating_sub(1);
	let offset = match text_alignment {
		GUIAlignment::Left => 0,
		GUIAlignment::Center => width / 2,
		GUIAlignment::Right => width,
	};
	let mut x = pos[0].saturating_sub_unsigned(offset as u16);
	for chr in string.chars() {
		let (char_vertices, char_width) = render_gui_char(chr, [x, pos[1]], alignment, io, visable_area);
		vertices.extend(char_vertices);
		x += char_width as i16 + 1;
	}
}