// Don't open a console window when the program starts
//#![windows_subsystem = "windows"]

use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder}, Display, Program, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter}, Blend, DrawParameters};

fn main() {
	// Setup window
	let events_loop = EventLoop::new();
	let window_builder = WindowBuilder::new()
		.with_inner_size(LogicalSize::new(100u16, 100u16)).with_title("Ceolmund");
	let context_builder = ContextBuilder::new().with_vsync(true);
	let display = Display::new(window_builder, context_builder, &events_loop).unwrap();

	// Create texture
	/*let image = image::load(Cursor::new(&include_bytes!("textures.png")),
						image::ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();*/

	// Create program
	let vertex_shader = include_str!("vertex_shader.glsl");
	let fragment_shader = include_str!("fragment_shader.glsl");
	let program = Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

	// Behavior
	let behavior = SamplerBehavior {
		minify_filter: MinifySamplerFilter::Nearest,
		magnify_filter: MagnifySamplerFilter::Nearest,
		..Default::default()
	};
	let draw_parameters = DrawParameters {
		blend: Blend::alpha_blending(),
		..DrawParameters::default()
	};

	// Vars
	let mut cursor_pos = [0u16; 2];
	let mut window_size = [640, 480];

	// Game loop
	events_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			_ => println!("{:?}", event),
			/*glutin::event::Event::WindowEvent { event, .. } => match event {
				// On exit button press
				event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
				// On cursor move
				event::WindowEvent::CursorMoved { device_id: _, position, .. } => cursor_pos = [
						(position.x as f32 / window_scale) as u16, (position.y as f32 / window_scale) as u16
					],
				// Window resize
				event::WindowEvent::Resized(size) => window_size = [size.width as u16, size.height as u16],
				event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => window_scale = scale_factor as f32,
				// Mouse click
				event::WindowEvent::MouseInput { device_id: _, state, button, .. } => {
					if cursor_pos[1] > header_size && !game_over && state == event::ElementState::Released {
						game_over = bord.click(
							[cursor_pos[0], (cursor_pos[1] as f32) as u16 - header_size],
							button,
						);
					}
					else if cursor_pos[0] < 32 && cursor_pos[1] < 32 {
						bord = bord::Bord::new([10, 12]);
						game_over = false;
					}
				}
				_ => {}
			},

			// Draw
			glutin::event::Event::MainEventsCleared => {
				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0.4, 0.4, 0.4, 0.);

				// Get tris
				let mut tris: Vec<vertex::Vertex> = Vec::new();
				tris.extend(bord.draw(header_size));

				tris.extend(texture::Texture::Cell.generate_tris([0, 0]));
				tris.extend(texture::Texture::Reset.generate_tris([0, 0]));

				let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

				// Draw tris
				let gui_vertex_buffer = glium::vertex::VertexBuffer::new(&display, &tris).unwrap();
				let gui_uniforms = glium::uniform! {
					matrix: [
						[1. / window_size[0] as f32 * 2. * window_scale, 0., 0., 0.],
						[0., -1. / window_size[1] as f32 * 2. * window_scale, 0., 0.],
						[0., 0., 0., 0.],
						[-1., 1., 0., 1.0f32],
					],
					texture_sampler: uniforms::Sampler(&texture, behavior),
				};
				frame.draw(&gui_vertex_buffer, &indices, &program, &gui_uniforms, &draw_parameters).unwrap();

				frame.finish().unwrap();
			}
			_ => {}*/
		}
	});
}