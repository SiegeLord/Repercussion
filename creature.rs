use allegro5::*;
use allegro_primitives::*;

use std::cmp::{max, min};

use world::{World, TILE_SIZE};
use camera::Camera;

pub struct Creature
{
	pub x: i32,
	pub y: i32,
	pub vx: i32,
	pub vy: i32,
	pub ax: i32,
	pub max_vx: i32,
	pub want_right: bool,
	pub want_left: bool,
	pub want_up: bool,
	pub want_down: bool,
}

impl Creature
{
	pub fn player() -> Creature
	{
		Creature
		{
			x: 0,
			y: 0,
			vx: 0,
			vy: 0,
			ax: 0,
			max_vx: 4,
			want_right: false,
			want_left: false,
			want_up: false,
			want_down: false,
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
		if world.in_support(self.x + self.vx, self.y)
		   || (self.want_down && world.in_support(self.x + self.vx, self.y + 1))
		   || (self.want_up && world.in_support(self.x + self.vx, self.y - 1))
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
			if world.on_ground(self.x, self.y) && self.vy > 0 || world.in_support(self.x, self.y + 1)
			{
				0
			}
			else
			{
				self.vy + 1
			}
		};
		
		let (nx, ny) = world.checked_move(self.x, self.y, self.vx, self.vy, descend);
		self.x = nx;
		self.y = ny;
	}
	
	pub fn jump(&mut self, world: &World)
	{
		if world.on_ground(self.x, self.y) && !world.in_support(self.x, self.y)
		{
			self.vy = -10;
		}
	}

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, camera: &Camera)
	{
		let x = (self.x - camera.x) as f32;
		let y = (self.y - camera.y) as f32;
		prim.draw_filled_rectangle(x, y, x + TILE_SIZE as f32, y + TILE_SIZE as f32, core.map_rgb_f(0.7, 0.0, 0.5));
	}
}
