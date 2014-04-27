use allegro5::*;

//~ use std::cmp::{max, min};

use world::{World, Cave, SupportType};
use camera::Camera;
use gfx::Gfx;

pub struct Torch
{
	pub x: i32,
	pub y: i32,
	pub dead: bool,
	
	pub w: i32,
	pub h: i32,
}

impl Torch
{
	pub fn new(x: i32, y: i32) -> Torch
	{
		Torch
		{
			x: x - 8,
			y: y - 8,
			w: 16,
			h: 16,
			dead: false,
		}
	}

	pub fn update(&mut self, world: &World)
	{
		if self.dead
		{
			return;
		}
		
		if world.colliding(self.x, self.y, self.w, self.h)
		{
			self.dead = true;
			return;
		}
	}

	pub fn draw(&self, gfx: &Gfx, core: &Core, camera: &Camera)
	{
		if self.dead
		{
			return;
		}
		
		let x = self.x - camera.x;
		let y = self.y - camera.y;
		gfx.torch.draw(core, x, y);
	}

	pub fn place_torch(world: &World, torches: &mut Vec<Torch>, player_x: i32, player_y: i32, player_w: i32, player_h: i32) -> bool
	{
		let (cx, cy) = world.get_tile_center(player_x, player_y, player_w, player_h);
		match world.get_tile_coords(cx, cy)
		{
			Some((tx, ty)) => if world.get_tile(tx, ty).tile_type != Cave && world.get_tile(tx, ty).tile_type != SupportType
			{
				return false;
			},
			_ => ()
		}
		
		for t in torches.iter()
		{
			let (cx2, cy2) = world.get_tile_center(t.x, t.y, t.w, t.h);
			if cx2 == cx && cy2 == cy
			{
				return false;
			}
		}
		torches.push(Torch::new(cx, cy));
		return true;
	}
}
