pub struct FormattedFileReader {
	version: u32,
	body: Vec<u8>,
	strings: Vec<u8>,
}

impl FormattedFileReader {
	pub fn read_from_file(version: u32) -> Self {
		Self {
			version,
			body: Vec::new(),
			strings: Vec::new(),
		}
	}
}