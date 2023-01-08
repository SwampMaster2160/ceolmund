// Don't open a console window when the program starts
#![windows_subsystem = "windows"]

pub mod world;
pub mod render;
pub mod io;
pub mod gui;

use std::{io::Cursor, time::Instant};

use gui::gui::GUI;
use io::{io::IO, game_key::GameKey};
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Fullscreen}, dpi::LogicalSize, ContextBuilder, event::{Event, WindowEvent, VirtualKeyCode, ElementState, MouseScrollDelta}}, Display, Program, uniforms::{SamplerBehavior, MinifySamplerFilter, MagnifySamplerFilter, Sampler}, Blend, DrawParameters, Surface, VertexBuffer, index::{NoIndices, PrimitiveType}, texture::RawImage2d};
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

pub fn validate_filename(mut name: String) -> String {
	name = name.chars().map(|chr| match chr {
		'/' | '\\' | '<' | '>' | ':' | '\'' | '|' | '?' | '*' | '.' | '~' | '#' | '%' | '&' | '+' | '-' | '{' | '}' | '@' | '"' | '!' | '`' | '=' => '_',
		_ => chr,
	}).collect();
	match name.to_lowercase().as_str() {
		"con" | "prn" | "aux" | "nul" |
		"com1" | "com2" | "com3" | "com4" | "com5" | "com6" | "com7" | "com8" | "com9" |
		"lpt1" | "lpt2" | "lpt3" | "lpt4" | "lpt5" | "lpt6" | "lpt7" | "lpt8" | "lpt9" => name.push('_'),
		_ => {}
	}
	name
}

const NANOSECONDS_PER_TICK: u128 = 1_000_000_000 / 100;

fn main() {
	// Main objects
	let mut world = None;
	let mut gui = GUI::new();
	let mut io = IO::new();

	// Window
	let events_loop = EventLoop::new();
	let window_builder = WindowBuilder::new()
		.with_inner_size(LogicalSize::new(912u16, 512u16)).with_title("Ceolmund");
	let context_builder = ContextBuilder::new().with_vsync(true);
	let display = Display::new(window_builder, context_builder, &events_loop).unwrap();
	display.gl_window().window().set_fullscreen(Some(Fullscreen::Borderless(None)));
	io.set_window_size(&display.gl_window().window().inner_size());

	// Create OpenGL program
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

	// Game loop
	let mut last_frame_time = Instant::now();
	let mut time_overflow: u128 = 0;
	events_loop.run(move |ref event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent { event: window_event, .. } => match window_event {
				WindowEvent::CloseRequested => io.game_keys_keyboard[GameKey::CloseGame as usize] = true,
				WindowEvent::Resized(size) => io.set_window_size(size),
				WindowEvent::KeyboardInput { device_id: _, input: key_input, .. } => {
					if key_input.virtual_keycode == Some(VirtualKeyCode::F11) && key_input.state == ElementState::Released {
						display.gl_window().window().set_fullscreen(match display.gl_window().window().fullscreen() {
							Some(_) => None,
							None => Some(Fullscreen::Borderless(None)),
						})
					}
					else {
						io.key_press(key_input);
					}
				}
				WindowEvent::ReceivedCharacter(chr) => io.key_chars.push(*chr),
				WindowEvent::CursorMoved { device_id: _, position, .. } =>
					io.mouse_pos = [position.x as u32, position.y as u32],
				WindowEvent::MouseInput { device_id: _, state, button, .. } => io.mouse_press(*state, *button),
				WindowEvent::MouseWheel { delta, .. } => {
					match delta {
						MouseScrollDelta::LineDelta(_, y) => io.mouse_scroll = *y as i16,
						MouseScrollDelta::PixelDelta(amount) => io.mouse_scroll = amount.y as i16,
					}
				}
				_  => {}
			}
			// Draw
			Event::MainEventsCleared => {
				// Poll gamepad
				io.poll_gamepad();

				// GUI Tick
				gui.tick(&mut world, &mut io);
				
				// Get the amount of time for game ticks
				let now = Instant::now();
				let time_for_ticks = now.duration_since(last_frame_time).as_nanos() + time_overflow;
				last_frame_time = now;
				time_overflow = time_for_ticks % NANOSECONDS_PER_TICK;
				let player_visable_width = (io.aspect_ratio * 16.) as u64 + 2;
				// Preform the game ticks if the world is loaded
				if let Some(world) = &mut world {
					let ticks_to_execute = 5.min(time_for_ticks / NANOSECONDS_PER_TICK);
					for _ in 0..ticks_to_execute {
						if !gui.does_menu_pause_game() {
							world.tick(&io, player_visable_width, &mut gui);
						}
						world.tick_always(&io, player_visable_width, &mut gui);
						io.update_keys_pressed_last();
					}
				}
				// Set all keys pressed now as pressed last
				io.update_keys_pressed_last();
				io.key_chars.clear();

				// Get frame for drawing on
				let mut frame = display.draw();
				frame.clear_color(0., 0., 0., 0.);

				// Render world
				if let Some(world) = &mut world {
					let (vertices, camera_center) = world.render(player_visable_width);
					let indices = NoIndices(PrimitiveType::TrianglesList);
					
					let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
					let uniforms = glium::uniform! {
						matrix: [
							[1. / 8. / io.aspect_ratio, 0., 0., 0.],
							[0., -1. / 8., 0., 0.],
							[0., 0., 0., 0.],
							[-1. / 16. / io.aspect_ratio - (camera_center[0] / io.aspect_ratio / 8.), 1. / 16. + (camera_center[1] / 8.), 0., 1.0f32],
						],
						texture_sampler: Sampler(&texture, behavior),
					};
					frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();
				}

				// Render gui
				let vertices = gui.render(&io, &world);

				let indices = NoIndices(PrimitiveType::TrianglesList);
				let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
				let uniforms = glium::uniform! {
					matrix: [
						[1. / 128. / io.aspect_ratio, 0., 0., 0.],
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