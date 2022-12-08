use crate::{io::input::Input, gui::gui_alignment::GUIAlignment};

use super::vertex::Vertex;

pub fn gui_pos_to_screen_pos(pos: [u16; 2], alignment: GUIAlignment, input: &Input) -> [f32; 2] {
	let offset = match alignment {
		GUIAlignment::Left => 0.,
		GUIAlignment::Center => (input.aspect_ratio - 1.) / 2.,
		GUIAlignment::Right => input.aspect_ratio - 1.,
	};
	[pos[0] as f32 + offset * 256., pos[1] as f32]
}

pub fn gui_size_to_screen_size(size: [u16; 2], input: &Input) -> [f32; 2] {
	[size[0] as f32, size[1] as f32]
}

pub fn render_gui_rect(pos: [u16; 2], size: [u16; 2], alignment: GUIAlignment, color: [u8; 4], input: &Input) -> [Vertex; 6] {
	let [start_x, start_y] = gui_pos_to_screen_pos(pos, alignment, input);
	let gui_size = gui_size_to_screen_size(size, input);
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