use crate::{render::{vertex::Vertex, render::{gui_size_to_screen_size, gui_pos_to_screen_pos}}, io::io::IO};

use super::gui_alignment::GUIAlignment;

#[derive(Clone, Copy)]
pub struct GUIRect {
	pub pos: [i16; 2],
	pub size: [u16; 2],
}

impl GUIRect {
	pub const EVERYTHING: Self = Self {
		pos: [i16::MIN, i16::MIN],
		size: [u16::MAX, u16::MAX],
	};

	pub const fn new(x: i16, y: i16, width: u16, height: u16) -> Self {
		Self {
			pos: [x, y],
			size: [width, height],
		}
	}

	pub fn new_from_start_end(start: [i16; 2], end: [i16; 2]) -> Self {
		Self {
			pos: start,
			size: [
				end[0].saturating_sub(start[0]).try_into().unwrap_or(0),
				end[1].saturating_sub(start[1]).try_into().unwrap_or(0),
			],
		}
	}

	pub const fn get_end(self) -> [i16; 2] {
		[
			self.pos[0].saturating_add_unsigned(self.size[0]),
			self.pos[1].saturating_add_unsigned(self.size[1]),
		]
	}

	pub const fn scrolled(self, scroll: [i16; 2]) -> Self {
		GUIRect {
			pos: [
				self.pos[0].saturating_add(scroll[0]),
				self.pos[1].saturating_add(scroll[1]),
			],
			size: self.size,
		}
	}

	pub fn overlap(self, other: Self) -> Self {
		let end = self.get_end();
		let other_end = other.get_end();
		Self::new_from_start_end(
			[self.pos[0].max(other.pos[0]), self.pos[1].max(other.pos[1])],
			[end[0].min(other_end[0]), end[1].min(other_end[1])],
		)
	}

	pub const fn without_outline(self) -> Self {
		Self {
			pos: [
				self.pos[0].saturating_add(1),
				self.pos[1].saturating_add(1),
			],
			size: [
				self.size[0].saturating_sub(2),
				self.size[1].saturating_sub(2),
			],
		}
	}

	pub fn render_shade(self, visable_area: Self, alignment: GUIAlignment, color: [u8; 4], io: &IO, vertices: &mut Vec<Vertex>) {
		let draw_rect = self.overlap(visable_area);

		let [start_x, start_y] = gui_pos_to_screen_pos(draw_rect.pos, alignment, io);
		let gui_size = gui_size_to_screen_size(draw_rect.size);
		let end_x = start_x + gui_size[0];
		let end_y = start_y + gui_size[1];
		let color = [color[0] as f32 / 255., color[1] as f32 / 255., color[2] as f32 / 255., color[3] as f32 / 255.];
		vertices.extend([
			Vertex { position: [start_x, start_y], texture_position: [0., 0.], color },
			Vertex { position: [end_x, start_y],   texture_position: [0., 0.], color },
			Vertex { position: [start_x, end_y],   texture_position: [0., 0.], color },
			Vertex { position: [end_x, start_y],   texture_position: [0., 0.], color },
			Vertex { position: [end_x, end_y],     texture_position: [0., 0.], color },
			Vertex { position: [start_x, end_y],   texture_position: [0., 0.], color },
		]);
	}

	pub fn render_shade_and_outline(self, visable_area: Self, alignment: GUIAlignment, color: [u8; 4], inside_color: [u8; 4], io: &IO, vertices: &mut Vec<Vertex>) {
		self.render_shade(visable_area, alignment, color, io, vertices);
		self.without_outline().render_shade(visable_area, alignment, inside_color, io, vertices);
	}
}