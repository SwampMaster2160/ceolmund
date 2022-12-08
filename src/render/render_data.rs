pub struct RenderData {
	pub widths: Vec<u8>,
}

impl RenderData {
	pub fn new() -> Self {
		// Get char widths
		let mut out = Vec::new();
		out.extend(include_bytes!("../asset/render_width/0.cwt"));
		out.extend(include_bytes!("../asset/render_width/1.cwt"));
		out.extend(include_bytes!("../asset/render_width/2.cwt"));
		Self {
			widths: out,
		}
	}
}