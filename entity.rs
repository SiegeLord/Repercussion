use allegro5::*;
use allegro_primitives::*;

use std::cmp::{max, min};

use world::World;
use camera::Camera;

pub enum EntityType
{
	Player
}

pub struct Entity
{
	pub x: i32,
	pub y: i32,
	pub vx: i32,
	pub vy: i32,
	pub ax: i32,
	pub want_right: bool,
	pub want_left: bool,
	pub want_up: bool,
	pub want_down: bool,
	
	pub max_vx: i32,
	pub w: i32,
	pub h: i32,
	pub entity_type: EntityType,
}

impl Entity
{
	pub fn player(x: i32, y: i32) -> Entity
	{
		Entity
		{
			x: x,
			y: y,
			vx: 0,
			vy: 0,
			ax: 0,
			w: 24,
			h: 24,
			max_vx: 4,
			want_right: false,
			want_left: false,
			want_up: false,
			want_down: false,
			entity_type: Player
		}
	}
	
	pub fn update(&mut self, world: &World)
	{
		if self.want_right
		{
			self.ax = 1;
		}
		else if self.want_left
		{
			self.ax = -1;
		}
		else
		{
			if self.vx > 0
			{
				self.ax = -1;
			}
			else if self.vx < 0
			{
				self.ax = 1;
			}
			else
			{
				self.ax = 0;
			}
		}
		
		self.vx += self.ax;
		self.vx = min(self.max_vx, max(self.vx, -self.max_vx));
		
		let mut descend = false;
		self.vy = 
		if world.in_support(self.x + self.vx, self.y, self.w, self.h)
		   || (self.want_down && world.in_support(self.x + self.vx, self.y + 1, self.w, self.h))
		   || (self.want_up && world.in_support(self.x + self.vx, self.y - 1, self.w, self.h))
		{
			if self.want_up
			{
				-4
			}
			else if self.want_down
			{
				descend = true;
				4
			}
			else
			{
				0
			}
		}
		else
		{
			if world.on_ground(self.x, self.y, self.w, self.h) && self.vy > 0 || world.on_support(self.x, self.y, self.w, self.h)
			{
				0
			}
			else
			{
				self.vy + 1
			}
		};
		
		let (nx, ny) = world.checked_move(self.x, self.y, self.w, self.h, self.vx, self.vy, descend);
		self.x = nx;
		self.y = ny;
	}
	
	pub fn jump(&mut self, world: &World)
	{
		if (world.on_ground(self.x, self.y, self.w, self.h) || world.on_support(self.x, self.y, self.w, self.h))
		   && !world.colliding(self.x, self.y - 1, self.w, self.h)
		{
			self.vy = -10;
			self.y -= 1;
		}
	}

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, camera: &Camera)
	{
		let x = (self.x - camera.x) as f32;
		let y = (self.y - camera.y) as f32;
		prim.draw_filled_rectangle(x, y, x + self.w as f32, y + self.h as f32, core.map_rgb_f(0.7, 0.0, 0.5));
	}
}
