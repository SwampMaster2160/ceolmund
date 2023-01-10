use std::{path::PathBuf, fs::{File, remove_file}, io::Write};

pub struct FileWriter {
	pub data: Vec<u8>, // The main vec for storing data
}

/// For writing a file structure that allows for a file version, a body consisting of an array of u8 values and an array of strings.
impl FileWriter {
	pub fn new() -> Self {
		Self {
			data: Vec::new(),
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
		// Write body
		file.write(self.data.as_slice()).ok()?;
		// Delete old file
		if backup_path.exists() {
			remove_file(backup_path).ok()?;
		}
		Some(())
	}

	/// Write the data in the file writer to a u8 vector, allowing it to be written to a file later.
	pub fn write_to_vec(&self) -> Option<Vec<u8>> {
		let mut out = Vec::new();
		// Write body
		out.extend(self.data.as_slice());

		Some(out)
	}

	pub fn push_string(&mut self, string: &String) {
		// Convert string to bytes
		let mut string_bytes = string.as_bytes().to_vec();
		string_bytes.push(0);
		self.data.extend(string_bytes);
	}

	pub fn push_str(&mut self, string: &str) {
		// Convert string to bytes
		let mut string_bytes = string.as_bytes().to_vec();
		string_bytes.push(0);
		self.data.extend(string_bytes);
	}

	pub fn push_u8(&mut self, to_push: u8) {
		self.data.push(to_push);
	}

	pub fn push_u16(&mut self, to_push: u16) {
		self.data.extend(to_push.to_le_bytes());
	}

	pub fn push_u32(&mut self, to_push: u32) {
		self.data.extend(to_push.to_le_bytes());
	}

	pub fn push_u64(&mut self, to_push: u64) {
		self.data.extend(to_push.to_le_bytes());
	}

	pub fn push_i64(&mut self, to_push: i64) {
		self.data.extend(to_push.to_le_bytes());
	}

	pub fn push_world_pos(&mut self, to_push: [i64; 2]) {
		self.push_i64(to_push[0]);
		self.push_i64(to_push[1]);
	}
}