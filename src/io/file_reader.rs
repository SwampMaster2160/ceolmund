use std::ffi::CStr;
use std::path::PathBuf;

use std::fs::{read, remove_file, rename};

/// For reading a file structure that allows for a file version, a body consisting of an array of u8 values and an array of strings.
pub struct FileReader {
	pub data: Vec<u8>, // The content of the file.
	pub read_index: usize, // Where we have read to, advances each time we read data.
	strings_v0: Vec<u8>, // Legacy for reading version 0 files
}

/// Load a file reader from disk.
impl FileReader {
	pub fn read_from_file(path: &PathBuf) -> Option<(Self, bool)> { // Object and if it is a version 0 file.
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
		// Special for files encoded in file version 0
		let val_0: [u8; 4] = data.get(0..4)?.try_into().ok()?;
		let val_0 = u32::from_le_bytes(val_0);
		let (strings, is_version_0, body_start_ptr) = if val_0 == 0 {
			let string_area_ptr: [u8; 4] = data.get(4..8)?.try_into().ok()?;
			let string_area_ptr = u32::from_le_bytes(string_area_ptr);
			let strings: Vec<u8> = (*data.get(string_area_ptr as usize..)?).try_into().ok()?;
			(strings, true, 8)
		}
		else {
			(Vec::new(), false, 0)
		};
		// Read body
		let data: Vec<u8> = (*data.get(body_start_ptr..)?).try_into().ok()?;
		// Return
		Some((Self {
			data,
			strings_v0: strings,
			read_index: 0,
		}, is_version_0))
	}

	/// Get a string at a index in the string area.
	pub fn get_string_v0(&self, start_index: u32) -> Option<String> {
		// Get slice starting at index
		let slice = self.strings_v0.get(start_index as usize..)?;
		// Find null char
		let end = slice.iter().position(|item| *item == 0)?;
		// Get C string from slice untill null char
		let cstr = CStr::from_bytes_with_nul(&slice[..=end]).ok()?;
		// Convert to String
		Some(cstr.to_str().ok()?.to_string())
	}

	pub fn get_string(&self, start_index: usize) -> Option<(String, usize)> {
		// Get slice starting at index
		let slice = self.data.get(start_index as usize..)?;
		// Find null char
		let end = slice.iter().position(|item| *item == 0)?;
		// Get C string from slice untill null char
		let cstr = CStr::from_bytes_with_nul(&slice[..=end]).ok()?;
		// Convert to String
		Some((cstr.to_str().ok()?.to_string(), end + 1))
	}

	pub fn read_u8(&mut self) -> Option<u8> {
		let out = *self.data.get(self.read_index)?;
		self.read_index += 1;
		Some(out)
	}

	pub fn read_u16(&mut self) -> Option<u16> {
		// Get the 2 bytes that make up the u16 value, they should be little endian.
		let u16_bytes = self.data.get(self.read_index..self.read_index + 2)?.try_into().ok()?;
		// Convert to u16 value
		let out = u16::from_le_bytes(u16_bytes);
		// Increment the read index
		self.read_index += 2;

		Some(out)
	}

	pub fn read_u32(&mut self) -> Option<u32> {
		// Get the 4 bytes that make up the u32 value, they should be little endian.
		let u32_bytes = self.data.get(self.read_index..self.read_index + 4)?.try_into().ok()?;
		// Convert to u32 value
		let out = u32::from_le_bytes(u32_bytes);
		// Increment the read index
		self.read_index += 4;

		Some(out)
	}

	pub fn read_u64(&mut self) -> Option<u64> {
		// Get the 8 bytes that make up the u64 value, they should be little endian.
		let u64_bytes = self.data.get(self.read_index..self.read_index + 8)?.try_into().ok()?;
		// Convert to u64 value
		let out = u64::from_le_bytes(u64_bytes);
		// Increment the read index
		self.read_index += 8;

		Some(out)
	}

	pub fn read_i64(&mut self) -> Option<i64> {
		// Get the 8 bytes that make up the i64 value, they should be little endian.
		let i64_bytes = self.data.get(self.read_index..self.read_index + 8)?.try_into().ok()?;
		// Convert to i64 value
		let out = i64::from_le_bytes(i64_bytes);
		// Increment the read index
		self.read_index += 8;

		Some(out)
	}

	pub fn read_world_pos(&mut self) -> Option<[i64; 2]> {
		let x = self.read_i64()?;
		let y = self.read_i64()?;
		Some([x, y])
	}

	pub fn read_string(&mut self) -> Option<String> {
		// Get slice starting at the read index
		let slice = self.data.get(self.read_index..)?;
		// Find null char and get the string length from it's position.
		let string_length = slice.iter().position(|item| *item == 0)? + 1;
		// Get C string from slice untill null char
		let cstr = CStr::from_bytes_with_nul(&slice[..string_length]).ok()?;
		// Convert to String
		let out = cstr.to_str().ok()?.to_string();
		// Increment the read index
		self.read_index += string_length;

		Some(out)
	}
}