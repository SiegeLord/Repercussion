use allegro5::*;
use allegro_primitives::*;

//~ use std::cmp::{max, min};

use world::World;
use camera::Camera;
use util::intersect_rect;

pub struct Gem
{
	pub x: i32,
	pub y: i32,
	pub vy: i32,
	pub dead: bool,
	
	pub w: i32,
	pub h: i32,
}

impl Gem
{
	pub fn new(x: i32, y: i32) -> Gem
	{
		Gem
		{
			x: x - 4,
			y: y - 4,
			vy: 0,
			w: 8,
			h: 8,
			dead: false,
		}
	}
	
	pub fn update(&mut self, world: &World, player_x: i32, player_y: i32, player_w: i32, player_h: i32) -> i32
	{
		if self.dead
		{
			return 0;
		}
		
		if world.colliding(self.x, self.y, self.w, self.h)
		{
			self.dead = true;
			return 0;
		}
		
		self.vy = if world.on_ground(self.x, self.y, self.w, self.h) && self.vy > 0 || world.on_support(self.x, self.y, self.w, self.h)
		{
			0
		}
		else
		{
			self.vy + 1
		};

		let (nx, ny) = world.checked_move(self.x, self.y, self.w, self.h, 0, self.vy, false);
		self.x = nx;
		self.y = ny;

		if intersect_rect(self.x, self.y, self.w, self.h, player_x, player_y, player_w, player_h)
		{
			self.dead = true;
			1
		}
		else
		{
			0
		}
	}

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, camera: &Camera)
	{
		if self.dead
		{
			return;
		}
		
		let x = (self.x - camera.x) as f32;
		let y = (self.y - camera.y) as f32;
		prim.draw_filled_rectangle(x, y, x + self.w as f32, y + self.h as f32, core.map_rgb_f(0.7, 0.0, 0.5));
	}
}
