
#![feature(globs)]
#![feature(struct_variant)]
#![feature(phase)]

#[phase(syntax, link)]
extern crate allegro5;
extern crate allegro_image;
extern crate allegro_font;
extern crate allegro_ttf;
extern crate allegro_primitives;
extern crate num;
extern crate rand;

use allegro5::*;
use allegro_image::*;
use allegro_font::*;
use allegro_ttf::*;
use allegro_primitives::*;

use world::World;
use camera::Camera;
use creature::Creature;

mod camera;
mod world;
mod creature;

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
	//~ let white = core.map_rgb_f(1.0, 1.0, 1.0);
	
	let mut world = World::new(30, 30);
	let mut camera = Camera::new(800, 600, world.get_pixel_width(), world.get_pixel_height());
	let mut player = Creature::player();
	
	let mut mine_up = false;
	let mut mine_down = false;
	let mut mine_left = false;
	let mut mine_right = false;
	let mut place_support = false;
	
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);
			world.draw(&core, &prim, &font, &camera);
			player.draw(&core, &prim, &camera);
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
					key::Left => player.want_left = true,
					key::Right => player.want_right = true,
					key::Up => player.want_up = true,
					key::Down => player.want_down = true,
					key::A => mine_left = true,
					key::D => mine_right = true,
					key::W => mine_up = true,
					key::S => mine_down = true,
					key::F => place_support = true,
					key::Space => player.jump(&world),
					_ => ()
				}
			},
			KeyUp{keycode: k, ..} => 
			{
				match k
				{
					key::Left => player.want_left = false,
					key::Right => player.want_right = false,
					key::Up => player.want_up = false,
					key::Down => player.want_down = false,
					key::A => mine_left = false,
					key::D => mine_right = false,
					key::W => mine_up = false,
					key::S => mine_down = false,
					key::F => place_support = false,
					_ => ()
				}
			},
			TimerTick{..} =>
			{
				player.update(&world);
				world.update(&mut camera);
				camera.update(player.x, player.y);
				//~ println!("{} {}", player.x, player.y);
				
				if world.on_ground(player.x, player.y, player.w, player.h) || world.on_support(player.x, player.y, player.w, player.h) && player.vx == 0 && player.vy == 0
				{
					match (mine_left, mine_right, mine_up, mine_down)
					{
						(true, _, _, _) => world.mine(player.x, player.y, -1,  0),
						(_, true, _, _) => world.mine(player.x, player.y,  1,  0),
						(_, _, true, _) => world.mine(player.x, player.y,  0, -1),
						(_, _, _, true) => world.mine(player.x, player.y,  0,  1),
						_ => ()
					}
					
					if place_support
					{
						world.place_support(player.x, player.y);
					}
				}
				
				redraw = true;
			},
			_ => ()
		}
	}
}
