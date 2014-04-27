use allegro5::*;

use std::cmp::{max, min};

use world::World;
use camera::Camera;
use gfx::Gfx;

#[deriving(Eq)]
pub enum EntityType
{
	Player,
	Demon
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
	pub dead: bool,
	pub face_left: bool,
	pub drill_direction: DrillDirection,
	
	pub max_vx: i32,
	pub w: i32,
	pub h: i32,
	pub entity_type: EntityType,
}

#[deriving(Eq)]
pub enum DrillDirection
{
	DrillUp,
	DrillDown,
	DrillLeft,
	DrillRight,
	DrillNone
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
			entity_type: Player,
			dead: false,
			face_left: false,
			drill_direction: DrillNone,
		}
	}
	
	pub fn make_demon(&mut self)
	{
		self.entity_type = Demon;
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
		
		if self.entity_type == Demon
		{
			return;
		}
		
		if self.want_right
		{
			self.ax = 1;
			self.face_left = false;
		}
		else if self.want_left
		{
			self.ax = -1;
			self.face_left = true;
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
		if self.entity_type == Demon
		{
			return;
		}

		if (world.on_ground(self.x, self.y, self.w, self.h) || world.on_support(self.x, self.y, self.w, self.h))
		   && !world.colliding(self.x, self.y - 1, self.w, self.h)
		{
			self.vy = -10;
			self.y -= 1;
		}
	}

	pub fn draw(&self, gfx: &Gfx, core: &Core, world: &World, camera: &Camera)
	{
		//~ if self.dead
		//~ {
			//~ return;
		//~ }
		let x = self.x - camera.x;
		let y = self.y - camera.y;
		
		let l = if self.dead
		{
			1.0
		}
		else
		{
			world.get_light(self.x + self.w / 2, self.y + self.h / 2)
		};
		
		let color = core.map_rgb_f(l, l, l);
		
		if self.dead
		{
			gfx.skeleton.draw_no_loop(core, x, y)
		}
		else
		{
			if self.entity_type == Demon
			{
				gfx.fun.draw(core, x, y)
			}
			else
			{			
				if self.drill_direction != DrillNone
				{
					match self.drill_direction
					{
						DrillLeft =>
						{
							gfx.drill_left.draw_tinted(core, x, y, color);
							gfx.drill_left_hi.draw(core, x, y );
						},
						DrillRight =>
						{
							gfx.drill_right.draw_tinted(core, x, y, color);
							gfx.drill_right_hi.draw(core, x, y );
						},
						DrillUp =>
						{
							gfx.drill_up.draw_tinted(core, x, y, color);
							gfx.drill_up_hi.draw(core, x, y );
						},
						DrillDown =>
						{
							gfx.drill_down.draw_tinted(core, x, y, color);
							gfx.drill_down_hi.draw(core, x, y );
						},
						_ => unreachable!()
					}
				}
				else
				{
					if self.face_left
					{
						if self.want_left || self.want_up || self.want_down
						{
							gfx.player_left.draw_tinted(core, x, y, color);
							gfx.player_left_hi.draw(core, x, y );
						}
						else
						{
							gfx.player_left.draw_frame(core, 0, x, y, color);
							gfx.player_left_hi.draw_frame(core, 0, x, y, core.map_rgb_f(1.0, 1.0, 1.0));
						}
					}
					else
					{
						if self.want_right || self.want_up || self.want_down
						{
							gfx.player_right.draw_tinted(core, x, y, color);
							gfx.player_right_hi.draw(core, x, y );
						}
						else
						{
							gfx.player_right.draw_frame(core, 0, x, y, color);
							gfx.player_right_hi.draw_frame(core, 0, x, y, core.map_rgb_f(1.0, 1.0, 1.0));
						}
					}
				}
			}
		}
	}
}
