// Don't open a console window when the program starts
//#![windows_subsystem = "windows"]

use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder, event::{Event, WindowEvent}}, Display, Program, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter}, Blend, DrawParameters, Surface};

fn main() {
	// Setup window
	let events_loop = EventLoop::new();
	let window_builder = WindowBuilder::new()
		.with_inner_size(LogicalSize::new(640u16, 480u16)).with_title("Ceolmund");
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

	//display.gl_window().buffer_age();

	// Vars
	//let mut cursor_pos = [0u16; 2];
	//let mut window_size = [640, 480];

	// Game loop
	events_loop.run(move |ref event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent { event: window_event, .. } => match window_event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				_  => println!("{:?}", event),
			}
			// Draw
			/*Event::MainEventsCleared => {
				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0.4, 0.4, 0.4, 0.);

				// Get tris
				/*let mut tris: Vec<vertex::Vertex> = Vec::new();
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
				frame.draw(&gui_vertex_buffer, &indices, &program, &gui_uniforms, &draw_parameters).unwrap();*/

				frame.finish().unwrap();
			}*/
			_ => println!("{:?}", event),
		}

		println!("{}", display.gl_window().buffer_age());

		// Get frame for drawing on
		if display.gl_window().buffer_age() != 0 {
			let mut frame = display.draw();
			frame.clear_color(0.8, 0.4, 0.4, 0.);

			frame.finish().unwrap();
			//display.gl_window().swap_buffers();
		}
		//println!("a");
	});
}