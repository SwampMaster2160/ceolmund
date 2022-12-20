use std::{path::PathBuf};

use crate::io::{io::IO, formatted_file_reader::FormattedFileReader};

#[derive(Clone)]
pub struct LoadWorldData {
	pub worlds: Vec<(String, PathBuf)>,
}

impl LoadWorldData {
	pub fn new(io: &IO) -> Self {
		let mut worlds = Vec::new();
		for item in io.worlds_path.read_dir().unwrap() {
			if let Ok(item) = item {
				let path = item.path();
				let mut overview_path = path.clone();
				overview_path.push("overview.wld");
				if let Some(overview) = FormattedFileReader::read_from_file(&overview_path) {
					if let Some(name_pos) = overview.body.get(0..4) {
						let name_pos: [u8; 4] = name_pos.try_into().unwrap();
						let name_pos = u32::from_le_bytes(name_pos);
						if let Some(name) = overview.get_string(name_pos) {
							worlds.push((name, path));
						}
					}
				}
			}
		}
		Self {
			worlds,
		}
	}
}