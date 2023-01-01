use std::{path::PathBuf, fs::{File, remove_file}, io::Write};

pub struct FormattedFileWriter {
	pub body: Vec<u8>, // The main vec for storing data
	strings: Vec<u8>, // Where we store strings, usually we will have some pointer to a string in the body vec.
}

/// For writing a file structure that allows for a file version, a body consisting of an array of u8 values and an array of strings.
impl FormattedFileWriter {
	pub fn new() -> Self {
		Self {
			body: Vec::new(),
			strings: Vec::new(),
		}
	}

	/// Write the data in the file writer to a new file at path, overwriting it if it exists.
	pub fn write(&self, path: &PathBuf) -> Option<()> {
		// Move old file
		let mut backup_path = path.clone();
		backup_path.push(".bak");
		if backup_path.exists() {
			remove_file(&backup_path).ok()?;
		}
		if path.exists() {
			remove_file(&path).ok()?;
			//rename(&path, &backup_path).ok()?;
		}
		// Create file
		let mut file = File::create(path).ok()?;
		// Write string area pointer
		let string_area_ptr: u32 = (4 + self.body.len()).try_into().ok()?;
		let string_area_ptr = string_area_ptr.to_le_bytes();
		file.write(&string_area_ptr).ok()?;
		// Write body and strings
		file.write(self.body.as_slice()).ok()?;
		file.write(self.strings.as_slice()).ok()?;
		// Delete old file
		if backup_path.exists() {
			remove_file(backup_path).ok()?;
		}
		Some(())
	}

	/// Write the data in the file writer to a u8 vector, allowing it to be written to a file later.
	pub fn write_to_vec(&self) -> Option<Vec<u8>> {
		let mut out = Vec::new();
		// Write a pointer that will point to where the string area is.
		let string_area_ptr: u32 = (4 + self.body.len()).try_into().ok()?;
		let string_area_ptr = string_area_ptr.to_le_bytes();
		out.extend(&string_area_ptr);
		// Write body and strings
		out.extend(self.body.as_slice());
		out.extend(self.strings.as_slice());

		Some(out)
	}

	/// Adds a string into the string area of the file writer, returning a pointer to the location in the string area.
	pub fn push_string(&mut self, string: &String) -> Option<u32> {
		// Convert string to bytes
		let string_bytes = string.as_bytes();
		// If the string is already in the string area then return the already existing one.
		match self.strings.windows(string_bytes.len())
			.enumerate().find(|window| window.1 == string_bytes)
			.map(|window| window.0)
		{
			Some(pos) => return Some(pos as u32),
			None => {}
		}
		// If not then add the string
		let pos = self.strings.len().try_into().ok()?;
		self.strings.extend(string_bytes);
		self.strings.push(0);
		Some(pos)
	}
}