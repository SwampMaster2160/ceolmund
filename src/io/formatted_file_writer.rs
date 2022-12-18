use std::{path::PathBuf, fs::{File, remove_file, rename}, io::Write};

pub struct FormattedFileWriter {
	version: u32,
	body: Vec<u8>,
	strings: Vec<u8>,
}

impl FormattedFileWriter {
	pub fn new(version: u32) -> Self {
		Self {
			version,
			body: Vec::new(),
			strings: Vec::new(),
		}
	}

	pub fn write(&self, path: PathBuf) -> Option<()> {
		// Move old file
		let mut backup_path = path.clone();
		backup_path.push(".bak");
		if backup_path.exists() {
			remove_file(&backup_path).ok()?;
		}
		if path.exists() {
			rename(&path, &backup_path).ok()?;
		}
		// Create file
		let mut file = File::create(path).ok()?;
		// Write version
		let version = self.version.to_le_bytes();
		file.write(&version).ok()?;
		// Write string area pointer
		let string_area_ptr: u32 = (version.len() * 2 + self.body.len()).try_into().ok()?;
		let string_area_ptr = string_area_ptr.to_le_bytes();
		file.write(&string_area_ptr).ok()?;
		// Write body and strings
		file.write(self.body.as_slice()).ok()?;
		file.write(self.strings.as_slice()).ok()?;
		// Delete old file
		remove_file(backup_path).ok()?;
		Some(())
	}
}