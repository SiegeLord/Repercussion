use allegro5::*;
use allegro_primitives::*;

use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use rand::task_rng;

//~ use std::cmp::{max, min};

use world::World;
use camera::Camera;
use util::intersect_rect;

#[deriving(Eq, Clone)]
pub enum GemColor
{
	Red,
	Green,
	Blue,
	Yellow,
	Purple
}

impl GemColor
{
	fn get_value(&self) -> i32
	{
		match *self
		{
			Red => 1,
			Green => 2,
			Blue => 5,
			Yellow => 10,
			Purple => 25
		}
	}

	fn get_color(&self, core: &Core) -> Color
	{
		match *self
		{
			Red => core.map_rgb_f(1.0, 0.0, 0.0),
			Green => core.map_rgb_f(0.0, 1.0, 0.0),
			Blue => core.map_rgb_f(0.0, 0.0, 1.0),
			Yellow => core.map_rgb_f(1.0, 1.0, 0.0),
			Purple => core.map_rgb_f(0.7, 0.0, 0.7)
		}
	}
}

pub struct Gem
{
	pub x: i32,
	pub y: i32,
	pub vy: i32,
	pub dead: bool,
	color: GemColor,
	
	pub w: i32,
	pub h: i32,
}

impl Gem
{
	pub fn new(x: i32, y: i32) -> Gem
	{
		let wc = WeightedChoice::new(
		vec![Weighted { weight: 100, item: Red },
		     Weighted { weight: 50,  item: Green },
		     Weighted { weight: 25,  item: Blue },
		     Weighted { weight: 5,   item: Yellow },
		     Weighted { weight: 1,   item: Purple }]
		);
		
		
		Gem
		{
			x: x - 4,
			y: y - 4,
			vy: 0,
			w: 8,
			h: 8,
			dead: false,
			color: wc.ind_sample(&mut task_rng())
		}
	}

	pub fn with_color(x: i32, y: i32, color: GemColor) -> Gem
	{
		Gem
		{
			x: x - 4,
			y: y - 4,
			vy: 0,
			w: 8,
			h: 8,
			dead: false,
			color: color,
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

		let (nx, ny) = world.checked_move(self.x, self.y, self.w, self.h, 0, self.vy, true);
		self.x = nx;
		self.y = ny;

		if intersect_rect(self.x, self.y, self.w, self.h, player_x, player_y, player_w, player_h)
		{
			self.dead = true;
			self.color.get_value()
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
		prim.draw_filled_rectangle(x, y, x + self.w as f32, y + self.h as f32, self.color.get_color(core));
	}
}
