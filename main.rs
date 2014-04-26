
#![feature(globs)]
#![feature(struct_variant)]
#![feature(phase)]

#[phase(syntax, link)]
extern crate allegro5;
extern crate allegro_image;
extern crate allegro_font;
extern crate allegro_ttf;
extern crate allegro_primitives;

use allegro5::*;
use allegro_image::*;
use allegro_font::*;
use allegro_ttf::*;
use allegro_primitives::*;

use world::World;
use camera::Camera;

mod camera;
mod world;

allegro_main!
{
	let mut core = Core::init().unwrap();
	ImageAddon::init(&core).expect("Failed to initialize the image addon");
	let font_addon = FontAddon::init(&core).expect("Failed to initialize the font addon");
	let _ttf_addon = TtfAddon::init(&font_addon).expect("Failed to initialize the ttf addon");
	let prim = PrimitivesAddon::init(&core).expect("Failed to initialize the primitives addon");
	
	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Gold gold gold gold.".to_c_str());

	core.install_keyboard();
	
	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());
	
	let font = font_addon.create_builtin_font().unwrap();
	let black = core.map_rgb_f(0.0, 0.0, 0.0);
	let white = core.map_rgb_f(1.0, 1.0, 1.0);
	
	let mut camera = Camera{ x: 0.0, y: 0.0, width: 800.0, height: 600.0 };
	let world = World::new(20, 20);
	
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);
			world.draw(&core, &prim, &camera);
			disp.flip();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose{..} =>
			{
				break 'exit;
			},
			KeyDown{keycode: k, ..} =>
			{
				match k
				{
					key::Escape => break 'exit,
					key::Left => camera.x -= 5.0,
					key::Right => camera.x += 5.0,
					key::Up => camera.y -= 5.0,
					key::Down => camera.y += 5.0,
					_ => ()
				}
			},
			TimerTick{..} =>
			{
				redraw = true;
			},
			_ => ()
		}
	}
}
