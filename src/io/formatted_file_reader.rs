use std::path::PathBuf;

use std::fs::{read, remove_file, rename};

pub struct FormattedFileReader {
	pub version: u32,
	pub body: Vec<u8>,
	pub strings: Vec<u8>,
}

impl FormattedFileReader {
	pub fn read_from_file(path: PathBuf) -> Option<Self> {
		// Restore backup in case of file save error
		let mut backup_path = path.clone();
		backup_path.push(".bak");
		if backup_path.exists() {
			if path.exists() {
				remove_file(&path).ok()?;
			}
			rename(&backup_path, &path).ok()?;
		}
		// Read from file
		let data = read(&path).ok()?;
		// get version
		let version: [u8; 4] = data.get(0..4)?.try_into().ok()?;
		let version = u32::from_le_bytes(version);
		// get string area ptr
		let string_area_ptr: [u8; 4] = data.get(4..8)?.try_into().ok()?;
		let string_area_ptr = u32::from_le_bytes(string_area_ptr);
		// Read body
		let body: Vec<u8> = data.get(8..string_area_ptr as usize)?.iter().map(|byte| *byte).collect();
		// Read strings
		let strings: Vec<u8> = data.get(string_area_ptr as usize..)?.iter().map(|byte| *byte).collect();
		// Return
		Some(Self {
			version,
			body,
			strings,
		})
	}
}