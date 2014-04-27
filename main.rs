
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
extern crate time;

use allegro5::*;
use allegro_image::*;
use allegro_font::*;
use allegro_ttf::*;
use allegro_primitives::*;

use world::{World, SURFACE_HEIGHT};
use camera::Camera;
use entity::Entity;
use gem::{Gem, Purple, Phil};
use demon::Demon;
use torch::Torch;
use message::Message;

mod camera;
mod world;
mod entity;
mod gem;
mod util;
mod demon;
mod torch;
mod message;

#[deriving(Eq, Clone)]
enum GameState
{
	Playing,
	Dead,
	Won,
	Ending,
}

allegro_main!
{
	let mut core = Core::init().unwrap();
	ImageAddon::init(&core).expect("Failed to initialize the image addon");
	let font_addon = FontAddon::init(&core).expect("Failed to initialize the font addon");
	let _ttf_addon = TtfAddon::init(&font_addon).expect("Failed to initialize the ttf addon");
	let prim = PrimitivesAddon::init(&core).expect("Failed to initialize the primitives addon");
	
	let dw = 800;
	let dh = 600;
	
	let disp = core.create_display(dw, dh).unwrap();
	disp.set_window_title(&"Repercussion".to_c_str());

	core.install_keyboard();
	
	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());
	
	let font = font_addon.create_builtin_font().unwrap();
	let black = core.map_rgb_f(0.0, 0.0, 0.0);
	let white = core.map_rgb_f(1.0, 1.0, 1.0);
	
	let mut world = World::new(30, 120);
	let mut camera = Camera::new(dw / 2, dh / 2, world.get_pixel_width(), world.get_pixel_height());
	let mut player = Entity::player(20, 20);
	
	let mut gems: Vec<Gem> = Vec::new();
	let mut demons: Vec<Demon> = Vec::new();
	let mut torches: Vec<Torch> = Vec::new();
	let mut phil = Gem::with_color(10, 10, Phil);
	let mut message = Some(Message::intro());
	let mut state = Playing;
	
	demons.push(Demon::new(128, 128));
	
	world.add_caves(
	|(x, y)|
	{
		demons.push(Demon::new(x, y))
	},
	|rare, (x, y)|
	{
		//~ println!("rare: {}, x: {}, y: {}", rare, x, y);
		gems.push(if rare
		{
			Gem::with_color(x, y, Purple)
		}
		else
		{
			Gem::new(x, y)
		});
	});
	
	let buffer = core.create_bitmap(dw / 2, dh / 2).unwrap();
	
	let mut mine_up = false;
	let mut mine_down = false;
	let mut mine_left = false;
	let mut mine_right = false;
	let mut place_support = false;
	let mut place_torch = false;
	let mut gem_count = 20i32;
	let mut eaten = false;
	
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.set_target_bitmap(&buffer);
			core.clear_to_color(black);
			
			if state != Ending && state != Dead
			{
				world.draw(&core, &prim, &font, &camera);
				
				for t in torches.iter()
				{
					t.draw(&core, &prim, &camera);
				}
			}
			
			player.draw(&core, &prim, &world, &camera);
			
			if state != Ending && state != Dead
			{
				for d in demons.iter()
				{
					d.draw(&core, &prim, &world, &camera);
				}
				
				for g in gems.iter()
				{
					g.draw(&core, &prim, &camera);
				}
				
				phil.draw(&core, &prim, &camera);
				
				core.draw_text(&font, white, 10.0, 10.0, AlignLeft, format!("Gems: {}", gem_count));
			}
			
			message.as_ref().map(|m|
			{
				m.draw(dw / 2, dh / 2, &core, &prim, &font);
			});
			
			core.set_target_bitmap(disp.get_backbuffer());
			core.draw_scaled_bitmap(&buffer, 0.0, 0.0, (dw / 2) as f32, (dh / 2) as f32, 0.0, 0.0, dw as f32, dh as f32, Flag::zero());
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
					key::R => place_support = true,
					key::T => place_torch = true,
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
					key::R => place_support = false,
					key::T => place_torch = false,
					_ => ()
				}
			},
			TimerTick{..} =>
			{
				let _start = time::precise_time_ns();
				
				if state == Playing
				{
					// Player
					player.update(&world);
					
					if player.dead
					{
						state = Dead;
						message = if eaten { Some(Message::eaten()) } else { Some(Message::crushed()) };
					}
					
					if message.is_none()
					{
						world.get_tile_coords(player.x + player.w / 2, player.y + player.h / 2).map(|(_, ty)|
						{
							if phil.dead && ty <= SURFACE_HEIGHT as uint && state == Playing
							{
								message = Some(Message::surface());
								state = Won;
							}
						});
					}
					
					// Gems
					for g in gems.mut_iter()
					{
						gem_count += g.update(&world, player.x, player.y, player.w, player.h);
					}
					gems.retain(|g| !g.dead);

					// Demons
					for d in demons.mut_iter()
					{
						eaten |= d.update(&world, player.x, player.y, player.w, player.h);
					}
					demons.retain(|d| !d.dead);
					
					if eaten
					{
						player.dead = true;
					}
					
					// Phil
					let phil_old_dead = phil.dead;
					phil.update(&world, player.x, player.y, player.w, player.h);
					if phil.dead && !phil_old_dead
					{
						torches.clear();
						world.need_new_light = true;
						message = Some(Message::found());
					}

					// Torches
					for t in torches.mut_iter()
					{
						t.update(&world);
					}
					let old_len = torches.len();
					torches.retain(|d| !d.dead);
					
					if torches.len() != old_len
					{
						world.need_new_light = true;
					}
					
					// World
					world.update(&mut camera, torches.as_slice(), player.x, player.y, player.w, player.h);
					// Camera
					camera.update(player.x, player.y);
				}
				
				// Messages
				let hide = message.as_mut().map(|m|
				{
					m.update()
				});
				hide.map(|hide|
					if hide
					{
						message = None;
						if state == Won
						{
							state = Ending;
							message = Some(Message::no_john());
						}
						else if state == Ending
						{
							player.make_demon();
						}
					}
				);
				//~ println!("{} {}", player.x, player.y);
				
				// Player actions
				if state == Playing && !player.dead &&
				   (world.on_ground(player.x, player.y, player.w, player.h) || world.on_support(player.x, player.y, player.w, player.h)) &&
				    player.vx == 0 && player.vy == 0
				{
					let spawn_gem = match (mine_left, mine_right, mine_up, mine_down)
					{
						(true, _, _, _) => world.mine(player.x, player.y, -1,  0),
						(_, true, _, _) => world.mine(player.x, player.y,  1,  0),
						(_, _, true, _) => world.mine(player.x, player.y,  0, -1),
						(_, _, _, true) => world.mine(player.x, player.y,  0,  1),
						_ => None
					};
					
					spawn_gem.map(|(x, y)|
					{
						gems.push(Gem::new(x, y));
					});
					
					if place_support && gem_count > 1
					{
						if world.place_support(player.x, player.y)
						{
							gem_count -= 2;
						}
					}

					if place_torch && gem_count > 0
					{
						if Torch::place_torch(&world, &mut torches, player.x, player.y, player.w, player.h)
						{
							gem_count -= 1;
							world.need_new_light = true;
						}
					}
					
				}
				
				let _end = time::precise_time_ns();
				
				//~ println!("Update duration (ms): {}", (end - start) as f64 / 1e6);
				
				redraw = true;
			},
			_ => ()
		}
	}
}
