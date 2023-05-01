use std::ffi::CStr;
use std::path::PathBuf;

use std::fs::{read, remove_file, rename};

use crate::error::Error;

/// For reading a file structure that allows for a file version, a body consisting of an array of u8 values and an array of strings.
pub struct FileReader {
	pub data: Vec<u8>, // The content of the file.
	pub read_index: usize, // Where we have read to, advances each time we read data.
	strings_v0: Vec<u8>, // Legacy for reading version 0 files
}

/// Load a file reader from disk.
impl FileReader {
	pub fn read_from_file(path: &PathBuf) -> Result<(Self, bool), Error> { // Object and if it is a version 0 file.
		// Restore backup in case of file save error
		let mut backup_path = path.clone();
		backup_path.push(".bak");
		if backup_path.exists() {
			if path.exists() {
				remove_file(&path).map_err(|_| Error::CannotDeleteFile)?;
			}
			rename(&backup_path, &path).map_err(|_| Error::CannotRenameFile)?;
		}
		// Read from file
		let data = read(&path).map_err(|_| Error::CannotReadFile)?;
		// Special for files encoded in file version 0
		let val_0: [u8; 4] = data.get(0..4).ok_or(Error::OutOfBoundsFileRead)?.try_into().expect("[u8] of length 4 should be castable to [u8; 4].");
		let val_0 = u32::from_le_bytes(val_0);
		let (strings, is_version_0, body_start_ptr) = if val_0 == 0 {
			let string_area_ptr: [u8; 4] = data.get(4..8).ok_or(Error::OutOfBoundsFileRead)?.try_into().expect("[u8] of length 4 should be castable to [u8; 4].");
			let string_area_ptr = u32::from_le_bytes(string_area_ptr);
			let strings: Vec<u8> = (*data.get(string_area_ptr as usize..).ok_or(Error::OutOfBoundsFileRead)?).try_into().expect("[u8] should be castable to Vec<u8>.");
			(strings, true, 8)
		}
		else {
			(Vec::new(), false, 0)
		};
		// Read body
		let data: Vec<u8> = (*data.get(body_start_ptr..).ok_or(Error::OutOfBoundsFileRead)?).try_into().expect("[u8] should be castable to Vec<u8>.");
		// Return
		Ok((Self {
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

	pub fn read_u8(&mut self) -> Result<u8, Error> {
		let out = *self.data.get(self.read_index).ok_or(Error::OutOfBoundsFileRead)?;
		self.read_index += 1;
		Ok(out)
	}

	pub fn read_u16(&mut self) -> Result<u16, Error> {
		// Get the 2 bytes that make up the u16 value, they should be little endian.
		let u16_bytes = self.data.get(self.read_index..self.read_index + 2).ok_or(Error::OutOfBoundsFileRead)?.try_into().expect("[u8] of length 2 should be castable to [u8; 2].");
		// Convert to u16 value
		let out = u16::from_le_bytes(u16_bytes);
		// Increment the read index
		self.read_index += 2;

		Ok(out)
	}

	pub fn read_u32(&mut self) -> Result<u32, Error> {
		// Get the 4 bytes that make up the u32 value, they should be little endian.
		let u32_bytes = self.data.get(self.read_index..self.read_index + 4).ok_or(Error::OutOfBoundsFileRead)?.try_into().expect("[u8] of length 4 should be castable to [u8; 4].");
		// Convert to u32 value
		let out = u32::from_le_bytes(u32_bytes);
		// Increment the read index
		self.read_index += 4;

		Ok(out)
	}

	pub fn read_u64(&mut self) -> Result<u64, Error> {
		// Get the 8 bytes that make up the u64 value, they should be little endian.
		let u64_bytes = self.data.get(self.read_index..self.read_index + 8).ok_or(Error::OutOfBoundsFileRead)?.try_into().expect("[u8] of length 8 should be castable to [u8; 8].");
		// Convert to u64 value
		let out = u64::from_le_bytes(u64_bytes);
		// Increment the read index
		self.read_index += 8;

		Ok(out)
	}

	pub fn read_i64(&mut self) -> Result<i64, Error> {
		// Get the 8 bytes that make up the i64 value, they should be little endian.
		let i64_bytes = self.data.get(self.read_index..self.read_index + 8).ok_or(Error::OutOfBoundsFileRead)?.try_into().expect("[u8] of length 8 should be castable to [u8; 8].");
		// Convert to i64 value
		let out = i64::from_le_bytes(i64_bytes);
		// Increment the read index
		self.read_index += 8;

		Ok(out)
	}

	pub fn read_world_pos(&mut self) -> Result<[i64; 2], Error> {
		let x = self.read_i64()?;
		let y = self.read_i64()?;
		Ok([x, y])
	}

	pub fn read_string(&mut self) -> Result<String, Error> {
		// Get slice starting at the read index
		let string_start_onwards = self.data.get(self.read_index..).expect("read_index shold not be greater than the data length.");
		// Find null char and get the string length from it's position.
		let string_length = string_start_onwards.iter().position(|item| *item == 0).ok_or(Error::UnterminatedStringRead)?;
		// Get string from start untill null char
		let string_bytes = string_start_onwards.get(..string_length).expect("string_length is the length to the next null byte which should be in the file.").to_vec();
		let out = String::from_utf8(string_bytes).map_err(|_| Error::InvalidUTF8InString)?;
		// Increment the read index
		self.read_index += string_length + 1;

		Ok(out)
	}
}