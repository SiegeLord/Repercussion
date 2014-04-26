use rand::{task_rng, Rng};
use std::cmp::{min, max};

pub struct Camera
{
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub shake_amp: f32,
	pub world_width: i32,
	pub world_height: i32,
}

impl Camera
{
	pub fn new(width: i32, height: i32, world_width: i32, world_height: i32) -> Camera
	{
		Camera
		{
			x: 0,
			y: 0,
			width: width,
			height: height,
			shake_amp: 0.0,
			world_width: world_width,
			world_height: world_height,
		}
	}

	pub fn update(&mut self, player_x: i32, player_y: i32)
	{
		let (jolt_x, jolt_y) = if self.shake_amp > 0.0
		{
			self.shake_amp *= 0.95;
			(task_rng().gen_range(-self.shake_amp, self.shake_amp) as i32,
			 task_rng().gen_range(-self.shake_amp, self.shake_amp) as i32)
		}
		else
		{
			(0, 0)
		};
		
		let new_x = player_x - self.width / 2;
		let new_y = player_y - self.height / 2;
		
		let max_dev_x = self.width / 6;
		let max_dev_y = self.height / 6;
		
		if new_x - self.x > max_dev_x
		{
			self.x = new_x - max_dev_x;
		}
		else if new_x - self.x < -max_dev_x
		{
			self.x = new_x + max_dev_x;
		}

		if new_y - self.y > max_dev_y
		{
			self.y = new_y - max_dev_y;
		}
		else if new_y - self.y < -max_dev_y
		{
			self.y = new_y + max_dev_y;
		}
		
		self.x = min(max(self.x, 0), self.world_width - self.width);
		self.y = min(max(self.y, 0), self.world_height - self.height);

		self.x += jolt_x;
		self.y += jolt_y;
	}
	
	pub fn jolt(&mut self, amount: f32)
	{
		self.shake_amp = self.shake_amp.max(amount);
	}
}
