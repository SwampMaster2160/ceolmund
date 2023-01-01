use std::ffi::CStr;
use std::path::PathBuf;

use std::fs::{read, remove_file, rename};

/// For reading a file structure that allows for a file version, a body consisting of an array of u8 values and an array of strings.
pub struct FormattedFileReader {
	//pub version: u32,
	pub body: Vec<u8>,
	strings: Vec<u8>,
}

/// Load a file reader from disk.
impl FormattedFileReader {
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
		let (string_area_ptr, is_version_0, body_start_ptr) = if val_0 == 0 {
			let string_area_ptr: [u8; 4] = data.get(4..8)?.try_into().ok()?;
			let string_area_ptr = u32::from_le_bytes(string_area_ptr);
			(string_area_ptr, true, 8)
		}
		else {
			(val_0, false, 4)
		};
		// Read body
		let body: Vec<u8> = (*data.get(body_start_ptr..string_area_ptr as usize)?).try_into().ok()?;
		// Read strings
		let strings: Vec<u8> = (*data.get(string_area_ptr as usize..)?).try_into().ok()?;
		// Return
		Some((Self {
			body,
			strings,
		}, is_version_0))
	}

	/// Get a string at a index in the string area.
	pub fn get_string(&self, start_index: u32) -> Option<String> {
		// Get slice starting at index
		let slice = self.strings.get(start_index as usize..)?;
		// Find null char
		let end = slice.iter().position(|item| *item == 0)?;
		// Get C string from slice untill null char
		let cstr = CStr::from_bytes_with_nul(&slice[..=end]).ok()?;
		// Convert to String
		Some(cstr.to_str().ok()?.to_string())
	}
}