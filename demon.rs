use allegro5::*;
use allegro_primitives::*;

//~ use std::cmp::{max, min};
use std::num::abs;

use world::{World, TILE_SIZE};
use camera::Camera;
use util::intersect_rect;

pub struct Demon
{
	pub x: i32,
	pub y: i32,
	pub vx: i32,
	pub vy: i32,
	pub dead: bool,
	pub moving_to_center: bool,
	pub active: bool,
	
	pub w: i32,
	pub h: i32,
}

impl Demon
{
	pub fn new(x: i32, y: i32) -> Demon
	{
		Demon
		{
			x: x - 12,
			y: y - 12,
			vx: 0,
			vy: 0,
			moving_to_center: false,
			w: 24,
			h: 24,
			dead: false,
			active: false,
		}
	}
	
	pub fn update(&mut self, world: &World, player_x: i32, player_y: i32, player_w: i32, player_h: i32) -> bool
	{
		if self.dead
		{
			return false;
		}
		
		if world.colliding(self.x, self.y, self.w, self.h)
		{
			self.dead = true;
			return false;
		}
		
		if intersect_rect(self.x, self.y, self.w, self.h, player_x, player_y, player_w, player_h)
		{
			return true;
		}
		
		let my_cx = self.x + self.w / 2;
		let my_cy = self.y + self.h / 2;
		
		if self.moving_to_center
		{
			let (cx, cy) = world.get_tile_center(self.x, self.y, self.w, self.h);
			
			self.vx = if my_cx > cx
			{
				-1
			}
			else
			{
				1
			};
			
			self.vy = if my_cy > cy
			{
				-1
			}
			else
			{
				1
			};
			
			if abs(cx - my_cx) < 4 && abs(cy - my_cy) < 4
			{
				self.moving_to_center = false;
			}
		}
		else
		{
			let p_cx = player_x + player_w / 2;
			let p_cy = player_h + player_h / 2;
			
			if abs(my_cx - p_cx) < 3 * TILE_SIZE / 2 &&
			   abs(my_cy - p_cy) < 3 * TILE_SIZE / 2
			{
				self.vx = if my_cx > p_cx
				{
					-1
				}
				else
				{
					1
				};
				
				self.vy = if my_cy > p_cy
				{
					-1
				}
				else
				{
					1
				};
			}
			else
			{
				match world.get_demon_policy(self.x + self.w / 2, self.y + self.h / 2)
				{
					Some(a) =>
					{
						let (sx, sy) = a.get_shift();
						self.vx = sx;
						self.vy = sy;
						self.active = true;
					},
					None => 
					{
						self.vx = 0;
						self.vy = 0;
					}
				}
			}
		}
		
		let (nx, ny) = world.checked_move(self.x, self.y, self.w, self.h, self.vx, self.vy, true);
		if nx == self.x && ny == self.y && (self.vx != 0 || self.vy != 0)
		{
			// Stuck, move to tile center and try again
			self.moving_to_center = true;
		}
		else
		{
			self.x = nx;
			self.y = ny;
		}
		
		false
	}

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, world: &World, camera: &Camera)
	{
		if self.dead
		{
			return;
		}
		if !self.active
		{
			return;
		}
		let x = (self.x - camera.x) as f32;
		let y = (self.y - camera.y) as f32;
		
		let l = world.get_light(self.x + self.w / 2, self.y + self.h / 2);
		
		prim.draw_filled_rectangle(x, y, x + self.w as f32, y + self.h as f32, core.map_rgb_f(l * 0.7, 0.0, 0.0));
	}
}

