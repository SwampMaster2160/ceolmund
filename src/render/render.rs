use crate::{io::input::Input, gui::gui_alignment::GUIAlignment};

pub fn gui_pos_to_screen_pos(pos: [u16; 2], alignment: GUIAlignment, input: &Input) -> [f32; 2] {
	let offset = match alignment {
		GUIAlignment::Left => 0.,
		GUIAlignment::Center => (input.aspect_ratio - 1.) / 2.,
		GUIAlignment::Right => input.aspect_ratio - 1.,
	};
	[pos[0] as f32 + offset * 256., pos[1] as f32]
}