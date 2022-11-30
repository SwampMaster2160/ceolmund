// Don't open a console window when the program starts
//#![windows_subsystem = "windows"]

mod game;
mod vertex;

use std::io::Cursor;

use game::Game;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Fullscreen}, dpi::LogicalSize, ContextBuilder, event::{Event, WindowEvent, VirtualKeyCode, ElementState}}, Display, Program, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter, Sampler}, Blend, DrawParameters, Surface, VertexBuffer, index::{NoIndices, PrimitiveType}, texture::RawImage2d};
use image::ImageFormat;
use vertex::Vertex;

fn main() {
	// Game

	let mut game = Game::new();

	// Window
	let events_loop = EventLoop::new();
	let window_builder = WindowBuilder::new()
		.with_inner_size(LogicalSize::new(640u16, 480u16)).with_title("Ceolmund");
	let context_builder = ContextBuilder::new().with_vsync(true);
	let display = Display::new(window_builder, context_builder, &events_loop).unwrap();
	let window_size = display.gl_window().window().inner_size();
	let mut window_size = [window_size.width, window_size.height];

	// Create program
	let vertex_shader = include_str!("shaders/vertex_shader.glsl");
	let fragment_shader = include_str!("shaders/fragment_shader.glsl");
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

	// Create texture
	let image = image::load(Cursor::new(&include_bytes!("textures.png")),
						ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	// Game loop
	events_loop.run(move |ref event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent { event: window_event, .. } => match window_event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				WindowEvent::Resized(size) => window_size = [size.width, size.height],
				WindowEvent::KeyboardInput { device_id: _, input, .. } => {
					if input.virtual_keycode == Some(VirtualKeyCode::F11) && input.state == ElementState::Released {
						display.gl_window().window().set_fullscreen(match display.gl_window().window().fullscreen() {
							Some(_) => None,
							None => Some(Fullscreen::Borderless(None)),
						})
					}
				}
				_  => {}
			}
			// Draw
			Event::MainEventsCleared => {
				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0., 0., 0., 0.);

				// Get tris
				let mut vertices = vec![
					Vertex { position: [0., 0.], texture_position: [0., 1.], color: [0., 0., 0., 0.] },
					Vertex { position: [1., 0.], texture_position: [1. / 16., 1.], color: [0., 0., 0., 0.] },
					Vertex { position: [0., 1.], texture_position: [0., 15. / 16.], color: [0., 0., 0., 0.] },
					Vertex { position: [1., 0.], texture_position: [1. / 16., 1.], color: [0., 0., 0., 0.] },
					Vertex { position: [1., 1.], texture_position: [1. / 16., 15. / 16.], color: [0., 0., 0., 0.] },
					Vertex { position: [0., 1.], texture_position: [0., 15. / 16.], color: [0., 0., 0., 0.] },
				];
				let mut tris: Vec<Vertex> = Vec::new();
				tris.append(&mut vertices);

				let indices = NoIndices(PrimitiveType::TrianglesList);

				// Draw tris
				let vertex_buffer = VertexBuffer::new(&display, &tris).unwrap();
				let aspect_ratio = window_size[0] as f32 / window_size[1] as f32;
				let offset_x = 0.;
				let offset_y = 0.;
				let uniforms = glium::uniform! {
					matrix: [
						[1. / 8. / aspect_ratio, 0., 0., 0.],
						[0., -1. / 8., 0., 0.],
						[0., 0., 0., 0.],
						[-1. / 16. / aspect_ratio - (offset_x / aspect_ratio / 8.), 1. / 16. + (offset_y / 8.), 0., 1.0f32],
					],
					texture_sampler: Sampler(&texture, behavior),
				};
				frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();

				frame.finish().unwrap();
			}
			_ => {}
		}
	});
}