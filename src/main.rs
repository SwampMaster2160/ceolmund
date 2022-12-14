// Don't open a console window when the program starts
//#![windows_subsystem = "windows"]

pub mod world;
pub mod render;
pub mod io;
pub mod gui;

use std::{io::Cursor, time::Instant};

use gui::gui::GUI;
use io::input::Input;
use render::render_data::RenderData;
use tokio::runtime::Runtime;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Fullscreen}, dpi::LogicalSize, ContextBuilder, event::{Event, WindowEvent, VirtualKeyCode, ElementState}}, Display, Program, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter, Sampler}, Blend, DrawParameters, Surface, VertexBuffer, index::{NoIndices, PrimitiveType}, texture::RawImage2d};
use image::ImageFormat;

#[macro_export]
macro_rules! const_static_ptr {
	( $t:ty, $x:expr ) => {
		{
			const OUT: $t = $x;
			&OUT
		}
	};
}

pub fn world_pos_to_render_pos(pos: [i64; 2], offset: [i8; 2]) -> [f32; 2] {
	[pos[0] as f32 + offset[0] as f32 / 16., pos[1] as f32 + offset[1] as f32 / 16.]
}

const NANOSECONDS_PER_TICK: u128 = 1_000_000_000 / 100;

fn main() {
	// Async runtime
	let async_runtime = Runtime::new().unwrap();

	// Game
	let mut world = None;
	//let mut guis = vec![GUIMenu::Test];
	let mut gui = GUI::new();
	let mut input = Input::new();
	let render_data = RenderData::new();

	// Window
	let events_loop = EventLoop::new();
	let window_builder = WindowBuilder::new()
		.with_inner_size(LogicalSize::new(912u16, 512u16)).with_title("Ceolmund");
	let context_builder = ContextBuilder::new().with_vsync(true);
	let display = Display::new(window_builder, context_builder, &events_loop).unwrap();
	display.gl_window().window().set_fullscreen(Some(Fullscreen::Borderless(None)));
	let window_size = display.gl_window().window().inner_size();
	let mut window_size = [window_size.width, window_size.height];

	// Create program
	let vertex_shader = include_str!("asset/shader/vertex_shader.glsl");
	let fragment_shader = include_str!("asset/shader/fragment_shader.glsl");
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
	let image = image::load(Cursor::new(&include_bytes!("asset/texture.png")),
						ImageFormat::Png).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

	// Vars
	let mut last_frame_time = Instant::now();
	let mut time_overflow: u128 = 0;

	// Game loop
	events_loop.run(move |ref event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent { event: window_event, .. } => match window_event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				WindowEvent::Resized(size) => window_size = [size.width, size.height],
				WindowEvent::KeyboardInput { device_id: _, input: key_input, .. } => {
					if key_input.virtual_keycode == Some(VirtualKeyCode::F11) && key_input.state == ElementState::Released {
						display.gl_window().window().set_fullscreen(match display.gl_window().window().fullscreen() {
							Some(_) => None,
							None => Some(Fullscreen::Borderless(None)),
						})
					}
					else {
						input.key_press(key_input);
					}
				}
				WindowEvent::CursorMoved { device_id: _, position, .. } =>
					input.mouse_pos = [position.x as u32, position.y as u32],
				WindowEvent::MouseInput { device_id: _, state, button, .. } => input.mouse_press(*state, *button),
				_  => {}
			}
			// Draw
			Event::MainEventsCleared => {
				// Poll gamepad
				input.aspect_ratio = window_size[0] as f32 / window_size[1] as f32;
				input.window_size = window_size;
				input.poll_gamepad();

				// GUI Tick
				gui.tick(&mut world, &mut input, &render_data);
				
				// World ticks
				let now = Instant::now();
				let time_for_ticks = now.duration_since(last_frame_time).as_nanos() + time_overflow;
				last_frame_time = now;
				time_overflow = time_for_ticks % NANOSECONDS_PER_TICK;
				if let Some(world) = &mut world {
					let ticks_to_execute = 5.min(time_for_ticks / NANOSECONDS_PER_TICK);
					for _ in 0..ticks_to_execute {
						if !gui.does_menu_pause_game() {
							world.tick(&input, &async_runtime, ((window_size[0] as f32 / window_size[1] as f32) * 16.) as u64 + 2, &mut gui);
						}
						world.tick_always(&input, &async_runtime, ((window_size[0] as f32 / window_size[1] as f32) * 16.) as u64 + 2, &mut gui);
					}
				}
				input.update_keys_pressed_last();

				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0., 0., 0., 0.);

				// Render world
				if let Some(world) = &mut world {
					let (vertices, camera_center) = world.render(((window_size[0] as f32 / window_size[1] as f32) * 16.) as u64 + 2);
					let indices = NoIndices(PrimitiveType::TrianglesList);
					
					let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
					let aspect_ratio = window_size[0] as f32 / window_size[1] as f32;
					let uniforms = glium::uniform! {
						matrix: [
							[1. / 8. / aspect_ratio, 0., 0., 0.],
							[0., -1. / 8., 0., 0.],
							[0., 0., 0., 0.],
							[-1. / 16. / aspect_ratio - (camera_center[0] / aspect_ratio / 8.), 1. / 16. + (camera_center[1] / 8.), 0., 1.0f32],
						],
						texture_sampler: Sampler(&texture, behavior),
					};
					frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();
				}

				// Render gui
				let vertices = gui.render(&input, &render_data);

				let indices = NoIndices(PrimitiveType::TrianglesList);
				let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
				let aspect_ratio = window_size[0] as f32 / window_size[1] as f32;
				let uniforms = glium::uniform! {
					matrix: [
						[1. / 128. / aspect_ratio, 0., 0., 0.],
						[0., -1. / 128., 0., 0.],
						[0., 0., 0., 0.],
						[-1., 1., 0., 1.0f32],
					],
					texture_sampler: Sampler(&texture, behavior),
				};
				frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();

				frame.finish().unwrap();
			}
			_ => {}
		}
		if gui.should_close_game {
			*control_flow = ControlFlow::Exit;
		}
	});
}