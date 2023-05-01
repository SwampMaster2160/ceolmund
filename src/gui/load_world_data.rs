use std::path::PathBuf;

use crate::{io::io::IO, world::world::World};

/// A struct containing the name and path of all the worlds in the users world folder.
#[derive(Clone)]
pub struct WorldList {
	pub worlds: Vec<(String, PathBuf)>,
}

impl WorldList {
	/// Get a vector of all valid world name and filepath pairs.
	pub fn new(io: &IO) -> Self {
		let mut out = Vec::new();
		// For each valid item in the worlds path.
		for item in io.worlds_path.read_dir().unwrap() {
			if let Ok(item) = item {
				// Get the path
				let filepath = item.path();
				// Add it to the list if it is a valid world.
				if let Ok(basic_world) = World::load(filepath.clone(), io, true) {
					out.push((basic_world.name, filepath));
				}
			}
		}
		
		Self {
			worlds: out,
		}
	}
}