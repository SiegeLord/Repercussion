use allegro5::*;
use allegro_font::*;

use std::cmp::{min, max};
use std::num::abs;
use camera::Camera;
use num::Integer;
use rand::{task_rng, Rng};
use std::f32::INFINITY;
use std::fmt;
use torch::Torch;
use sprite::Sprite;

pub static TILE_SIZE: i32 = 32;
pub static TILE_HEALTH: i32 = 32;
pub static MAX_ITERATIONS: i32 = 10;
pub static SURFACE_HEIGHT: i32 = 5;

#[deriving(Eq, Clone)]
#[repr(C)]
pub enum DemonAction
{
	MoveUp,
	MoveDown,
	MoveLeft,
	MoveRight,
}

struct DemonActionIterator
{
	action: DemonAction,
	count: u8
}

impl Iterator<DemonAction> for DemonActionIterator
{
	fn next(&mut self) -> Option<DemonAction>
	{
		let old = self.action;
		self.action = match old
		{
			MoveUp => MoveDown,
			MoveDown => MoveLeft,
			MoveLeft => MoveRight,
			MoveRight => MoveUp,
		};
		
		if self.count == 0
		{
			None
		}
		else
		{
			self.count -= 1;
			Some(old)
		}
	}
}

impl DemonAction
{
	pub fn get_shift(&self) -> (i32, i32)
	{
		match *self
		{
			MoveUp => (0, -1),
			MoveDown => (0, 1),
			MoveLeft => (-1, 0),
			MoveRight => (1, 0),
		}
	}

	pub fn iter() -> DemonActionIterator
	{
		DemonActionIterator{ action: MoveUp, count: 4 }
	}
}

impl fmt::Show for DemonAction
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		match *self
		{
			MoveUp => write!(buf.buf, "^"),
			MoveDown => write!(buf.buf, "v"),
			MoveLeft => write!(buf.buf, "<"),
			MoveRight => write!(buf.buf, ">"),
		}
	}
}

#[deriving(Eq, Clone)]
pub enum TileCollision
{
	Solid,
	Empty,
	Support
}

#[deriving(Eq, Clone)]
pub enum TileType
{
	Sky,
	Ground,
	CaveCeiling,
	Cave,
	SupportType,
	Bottom,
	Surface,
}

#[deriving(Clone)]
pub struct Tile
{
	collision: TileCollision,
	pub tile_type: TileType,
	health: i32,
	support: f32,
	fall_state: i32,
	has_gem: bool,
	demon_value: f32,
	demon_policy: DemonAction,
	light: f32,
}

impl Tile
{
	pub fn sky() -> Tile
	{
		Tile
		{
			collision: Empty,
			tile_type: Sky,
			health: TILE_HEALTH,
			support: 0.0,
			fall_state: 0,
			has_gem: false,
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 1.0,
		}
	}

	pub fn cave() -> Tile
	{
		Tile
		{
			collision: Empty,
			tile_type: Cave,
			health: TILE_HEALTH,
			support: 0.0,
			fall_state: 0,
			has_gem: false,
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 0.0,
		}
	}

	pub fn cave_ceil() -> Tile
	{
		Tile
		{
			collision: Solid,
			tile_type: CaveCeiling,
			health: TILE_HEALTH,
			support: 4.0,
			fall_state: 0,
			has_gem: false,
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 0.0,
		}
	}

	pub fn surface() -> Tile
	{
		Tile
		{
			collision: Solid,
			tile_type: Surface,
			health: TILE_HEALTH,
			support: 4.0,
			fall_state: 0,
			has_gem: false,
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 0.0,
		}
	}

	pub fn bottom() -> Tile
	{
		Tile
		{
			collision: Solid,
			tile_type: Bottom,
			health: TILE_HEALTH,
			support: 4.0,
			fall_state: 0,
			has_gem: false,
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 0.0,
		}
	}

	pub fn ground() -> Tile
	{
		Tile
		{
			collision: Solid,
			tile_type: Ground,
			health: TILE_HEALTH,
			support: 4.0,
			fall_state: 0,
			has_gem: task_rng().gen_weighted_bool(5),
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 0.0,
		}
	}
	
	pub fn support() -> Tile
	{
		Tile
		{
			collision: Support,
			health: TILE_HEALTH,
			tile_type: SupportType,
			support: 4.0,
			fall_state: 0,
			has_gem: false,
			demon_policy: MoveUp,
			demon_value: INFINITY,
			light: 0.0,
		}
	}
}

pub struct World
{
	width: uint,
	height: uint,
	tiles: Vec<Tile>,
	need_new_policy: bool,
	pub need_new_light: bool,
	old_player_tx: uint,
	old_player_ty: uint,
	policy_done: bool, // if false, then we have a policy that is not yet converged
	tile_sprite: Sprite
}

impl World
{
	pub fn new(core: &Core, width: uint, height: uint) -> World
	{
		assert!(width > 10);
		assert!(height > 10);
		
		let mut tiles = Vec::with_capacity(width * height);
		for row in range(0, height)
		{
			for _ in range(0, width)
			{
				tiles.push
				(
					if row == height - 1
					{
						Tile::bottom()
					}
					else if row == SURFACE_HEIGHT as uint
					{
						Tile::surface()
					}
					else if row < SURFACE_HEIGHT as uint
					{
						Tile::sky()
					}
					else
					{
						Tile::ground()
					}
				);
			}
		}
		
		World
		{
			width: width,
			height: height,
			tiles: tiles,
			need_new_policy: true,
			need_new_light: true,
			old_player_tx: 0,
			old_player_ty: 0,
			policy_done: true, // Has to be true, since we have no running policy yet
			tile_sprite: Sprite::new(core, "data/tiles.png", 32, 32)
		}
	}
	
	pub fn add_caves(&mut self, demon_callback: |(i32, i32)|, gem_callback: |bool, (i32, i32)|) -> (i32, i32)
	{
		let mut phil_loc = (0, 0);
		for _ in range(0, 30)
		{
			let cave_y = task_rng().gen_range(20, self.height - 6);
			let cave_width = task_rng().gen_range(2, 5u);
			let cave_x = task_rng().gen_range(0, self.width - cave_width);
			
			let mut num_demons = 0;
			let mut gem_spots = Vec::new();
			
			for x in range(cave_x, cave_x + cave_width)
			{
				let y1 = task_rng().gen_range(cave_y - 3, cave_y);
				let y2 = y1 + task_rng().gen_range(3, 5u);
				let mut add_gem = task_rng().gen_weighted_bool(2);
				
				for y in range(y1, y2)
				{
					if y == y1
					{
						*self.get_tile_mut(x, y) = Tile::cave_ceil();
					}
					else
					{					
						let center = (x as i32 * TILE_SIZE + TILE_SIZE / 2, y as i32 * TILE_SIZE + TILE_SIZE / 2);
						
						if center.val1() > phil_loc.val1()
						{
							phil_loc = center;
						}
						
						*self.get_tile_mut(x, y) = Tile::cave();
						
						if add_gem
						{
							gem_spots.push(center);
							add_gem = false;
						}
						
						if !task_rng().gen_weighted_bool(cave_y / 30)
						{
							num_demons += 1;
							demon_callback(center);
						}
					}
				}
			}
			
			for gem_spot in gem_spots.iter()
			{
				gem_callback(num_demons > 0, *gem_spot);
			}
			
			//~ println!("Cave: {} {}, demons: {}", cave_x, cave_y, num_demons);
		}
		//~ println!("Phil at: {}", phil_loc);
		phil_loc
	}
	
	pub fn get_pixel_width(&self) -> i32
	{
		self.width as i32 * TILE_SIZE
	}
	
	pub fn get_pixel_height(&self) -> i32
	{
		self.height as i32 * TILE_SIZE
	}

	pub fn draw(&self, core: &Core, font: &Font, camera: &Camera)
	{
		let sz = TILE_SIZE;
		let min_tx = min(max(camera.x / sz, 0) as uint, self.width);
		let min_ty = min(max(camera.y / sz, 0) as uint, self.height);
		let max_tx = min(min_tx + (camera.width / sz) as uint + 2, self.width);
		let max_ty = min(min_ty + (camera.height / sz) as uint + 2, self.height);
		
		for ty in range(min_ty, max_ty)
		{
			for tx in range(min_tx, max_tx)
			{
				let idx = ty * self.width + tx;
				let tile = self.tiles.get(idx);
				
				let x = tx as i32 * sz - camera.x;
				let y = ty as i32 * sz - camera.y + tile.fall_state;
				
				let l = (0.02 + 0.98 * tile.light).min(1.0);
				
				let color = core.map_rgb_f(l, l, l);
				
				let damage = (TILE_HEALTH - tile.health) * 3 / TILE_HEALTH;
				
				let frame = match tile.tile_type
				{
					Sky => 3,
					Surface => 4 + damage,
					CaveCeiling => 8 + damage,
					Bottom => 11,
					Ground => 0 + damage,
					Cave => 7,
					SupportType => 12 + damage,
				};
				
				self.tile_sprite.draw_frame(core, frame, x, y, color);
				
				//~ prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, color);
				
				//~ if tile.collision == Solid
				//~ {
					//~ let g = 0.5 * tile.health as f32 / TILE_HEALTH as f32;
					//~ prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, core.map_rgb_f(g, g, g));
					//~ core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x, y, AlignLeft, format!("{}", tile.support));
				//~ }
				//~ else if tile.collision == Support
				//~ {
					//~ prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, core.map_rgb_f(0.5, 1.0, 1.0));
					//~ core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x, y, AlignLeft, format!("{}", tile.support));
				//~ }
				
				if tile.collision != Solid
				{
					//~ core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x, y, AlignLeft, format!("{}", tile.demon_policy));
					//~ core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x, y + 8.0, AlignLeft, format!("{}", tile.demon_value));
				}
			}
		}
	}
	
	pub fn update(&mut self, camera: &mut Camera, torches: &[Torch], player_x: i32, player_y: i32, player_w: i32, player_h: i32)
	{
		for y in range(1, self.height - 1).rev()
		{
			for x in range(0, self.width)
			{
				// Deal with supports
				if self.get_tile(x, y).collision != Empty &&
				   self.get_tile(x, y).tile_type != CaveCeiling &&
				   self.get_tile(x, y).tile_type != Surface
				{
					let mut sup = 0.0f32;
					let mut num_supports = 0;
					if x > 0
					{
						let tile = self.get_tile(x - 1, y);
						sup = sup.max(tile.support * (tile.health as f32 / TILE_HEALTH as f32) - 1.0);
						if tile.support > 0.0
						{
							num_supports += 1;
						}
					}
					if x < self.width - 1
					{
						let tile = self.get_tile(x + 1, y);
						sup = sup.max(tile.support * (tile.health as f32 / TILE_HEALTH as f32) - 1.0);
						if tile.support > 0.0
						{
							num_supports += 1;
						}
					}
					{
						let tile = self.get_tile(x, y + 1);
						sup = sup.max(tile.support * (tile.health as f32 / TILE_HEALTH as f32));
						if tile.support > 0.0
						{
							num_supports += 1;
						}
					}

					if sup + num_supports as f32 * 0.4 <= 1.0 && self.get_tile(x, y).fall_state == 0
					{
						if self.get_tile(x, y + 1).collision == Empty
						{
							let mut tile = self.get_tile(x, y).clone();
							tile.fall_state = -TILE_SIZE;
							*self.get_tile_mut(x, y) = Tile::cave();
							*self.get_tile_mut(x, y + 1) = tile;
							self.need_new_policy = true;
							self.need_new_light = true;
						}
					}
					else
					{
						self.get_tile_mut(x, y).support = sup;
					}

					if self.get_tile(x, y).fall_state < 0
					{
						self.get_tile_mut(x, y).fall_state += 2;
						if self.get_tile(x, y).fall_state >= 0
						{
							self.get_tile_mut(x, y).fall_state = 0;
							camera.jolt(5.0);
						}
						else
						{
							camera.jolt(2.0);
						}
					}
				}
			}
		}
		
		// Deal with lights
		if self.need_new_light
		{
			// Darken under-surface
			for y in range(SURFACE_HEIGHT as uint, self.height)
			{
				for x in range(0, self.width)
				{
					self.get_tile_mut(x, y).light = 0.0;
				}
			}
			
			// Surface lights
			for x in range(0, self.width)
			{
				self.light_ray(x, SURFACE_HEIGHT as uint - 1, x, SURFACE_HEIGHT as uint + 5);
			}
			
			for torch in torches.iter()
			{
				self.get_tile_coords(torch.x, torch.y).map(|(tx, ty)|
				{
					let x1 = max(tx as i32 - 5, 0) as uint;
					let y1 = max(ty as i32 - 5, 0) as uint;
					let x2 = min(tx as i32 + 5, self.width as i32 - 1) as uint;
					let y2 = min(ty as i32 + 5, self.height as i32 - 1) as uint;
					
					for x in range(x1, x2 + 1)
					{
						self.light_ray(tx, ty, x, y1);
					}

					for x in range(x1, x2 + 1).rev()
					{
						self.light_ray(tx, ty, x, y2);
					}

					for y in range(y1 + 1, y2)
					{
						self.light_ray(tx, ty, x1, y);
					}

					for y in range(y1 + 1, y2)
					{
						self.light_ray(tx, ty, x2, y);
					}
				});
			}
			self.need_new_light = false;
		}
		
		// Compute the policy
		let mut player_tx = (player_x + player_w / 2).div_floor(&TILE_SIZE);
		let mut player_ty = (player_y + player_h / 2).div_floor(&TILE_SIZE);
		
		if player_tx < 0 || player_tx >= self.width as i32 || player_ty < 0 || player_ty >= self.height as i32
		{
			// Something's wrong
			player_tx = 0;
			player_ty = 0;
		}
		
		let player_tx = player_tx as uint;
		let player_ty = player_ty as uint;
		
		if self.policy_done && !self.need_new_policy && player_tx == self.old_player_tx && player_ty == self.old_player_ty
		{
			// Policy does not need changing
			return;
		}
		self.old_player_tx = player_tx;
		self.old_player_ty = player_ty;
		
		if self.policy_done
		{
			// First clear the values
			for y in range(0, self.height)
			{
				for x in range(0, self.width)
				{
					if self.get_tile(x, y).collision == Solid
					{
						continue;
					}
					self.get_tile_mut(x, y).demon_value = INFINITY;
				}
			}
			
			self.get_tile_mut(player_tx, player_ty).demon_value = 0.0;
		}
		
		let mut iterations = 0;
		let mut changed = true;
		while changed
		{
			iterations += 1;
			if iterations > MAX_ITERATIONS
			{
				self.policy_done = false;
				break;
			}
			changed = false;
			for y in range(0, self.height)
			{
				for x in range(0, self.width)
				{
					if self.get_tile(x, y).collision == Solid
					{
						continue;
					}
					
					if x == player_tx && y == player_ty && self.get_tile(x, y).demon_value != 0.0
					{
						self.get_tile_mut(x, y).demon_value = 0.0;
						changed = true;
						continue;
					}
					
					for a in DemonAction::iter()
					{
						let (dx, dy) = a.get_shift();
						
						let nx = (x as i32) + dx;
						let ny = (y as i32) + dy;
						
						if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32
						{
							let nx = nx as uint;
							let ny = ny as uint;
							if self.get_tile(nx, ny).collision != Solid
							{
								let new_value = self.get_tile(nx, ny).demon_value + 1.0;
								if new_value < self.get_tile(x, y).demon_value
								{
									let tile = self.get_tile_mut(x, y);
									tile.demon_policy = a;
									tile.demon_value = new_value;
									changed = true;
								}
							}
						}
					}
				}
			}
		}
		if changed == false
		{
			self.policy_done = true;
		}
		self.need_new_policy = false;
		//~ println!("iterations: {}", iterations);
	}
	
	pub fn get_tile<'l>(&'l self, tx: uint, ty: uint) -> &'l Tile
	{
		self.tiles.get(ty * self.width + tx)
	}
	
	pub fn get_tile_mut<'l>(&'l mut self, tx: uint, ty: uint) -> &'l mut Tile
	{
		self.tiles.get_mut(ty * self.width + tx)
	}
	
	pub fn check_tile_type(&self, x: i32, y: i32, w: i32, h: i32, coll: TileCollision) -> Option<bool>
	{
		let tx1 = x.div_floor(&TILE_SIZE);
		let ty1 = y.div_floor(&TILE_SIZE);
		let tx2 = (x + w - 1).div_floor(&TILE_SIZE);
		let ty2 = (y + h - 1).div_floor(&TILE_SIZE);

		if tx1 < 0 || tx1 >= self.width as i32 ||
		   tx2 < 0 || tx2 >= self.width as i32 ||
		   ty1 < 0 || ty1 >= self.height as i32 ||
		   ty2 < 0 || ty2 >= self.height as i32
		{
			None
		}
		else
		{
			Some(
				self.get_tile(tx1 as uint, ty1 as uint).collision == coll ||
				self.get_tile(tx1 as uint, ty2 as uint).collision == coll ||
				self.get_tile(tx2 as uint, ty1 as uint).collision == coll ||
				self.get_tile(tx2 as uint, ty2 as uint).collision == coll
			)
		}
	}

	pub fn get_tile_center(&self, x: i32, y: i32, w: i32, h: i32) -> (i32, i32)
	{
		((x + w / 2).div_floor(&TILE_SIZE) * TILE_SIZE + TILE_SIZE / 2,
		 (y + h / 2).div_floor(&TILE_SIZE) * TILE_SIZE + TILE_SIZE / 2)
	}

	pub fn light_ray(&mut self, stx: uint, sty: uint, dtx: uint, dty: uint)
	{
		let dx = dtx as i32 - stx as i32;
		let dy = dty as i32 - sty as i32;
		
		if dx == 0 && dy == 0
		{
			return;
		}
		
		if abs(dx) > abs(dy)
		{
			let dx1 = if dx > 0 { 1 } else { -1 };
			let mut x = stx as i32;
			for delta_x in range(0, abs(dx))
			{
				let delta_y = dy * delta_x / dx;				
				let y = sty as i32 + delta_y;
				let light = (1.0 / (0.1 + (delta_x as f32) * (delta_x as f32) + (delta_y as f32) * (delta_y as f32))).min(1.0);
				
				let tile = self.get_tile_mut(x as uint, y as uint);
				tile.light = tile.light.max(light);
				if tile.collision == Solid
				{
					break;
				}
				
				x += dx1;
			}
		}
		else
		{
			let dy1 = if dy > 0 { 1 } else { -1 };
			let mut y = sty as i32;
			for delta_y in range(0, abs(dy))
			{
				let delta_x = dx * delta_y / dy;				
				let x = stx as i32 + delta_x;
				let light = (1.0 / (0.1 + (delta_x as f32) * (delta_x as f32) + (delta_y as f32) * (delta_y as f32))).min(1.0);
				
				let tile = self.get_tile_mut(x as uint, y as uint);
				tile.light = tile.light.max(light);
				if tile.collision == Solid
				{
					break;
				}
				
				y += dy1;
			}
		}
	}
	
	pub fn get_tile_coords(&self, x: i32, y: i32) -> Option<(uint, uint)>
	{
		let tx = x.div_floor(&TILE_SIZE);
		let ty = y.div_floor(&TILE_SIZE);
		if tx < 0 || tx >= self.width as i32 ||
		   ty < 0 || ty >= self.height as i32
		{
			None
		}
		else
		{
			Some((tx as uint, ty as uint))
		}
	}

	pub fn get_demon_policy(&self, x: i32, y: i32) -> Option<DemonAction>
	{
		self.get_tile_coords(x, y).map_or(None, |(tx, ty)|
		{
			let tile = self.get_tile(tx, ty);
			if tile.demon_value < INFINITY
			{
				Some(tile.demon_policy)
			}
			else
			{
				None
			}
		})
	}

	pub fn colliding(&self, x: i32, y: i32, w: i32, h: i32) -> bool
	{
		match self.check_tile_type(x, y, w, h, Solid)
		{
			Some(ret) => ret,
			None => true
		}
	}

	pub fn on_ground(&self, x: i32, y: i32, w: i32, h: i32) -> bool
	{
		self.colliding(x, y + 1, w, h)
	}
	
	pub fn in_support(&self, x: i32, y: i32, w: i32, h: i32) -> bool
	{
		match self.check_tile_type(x, y, w, h, Support)
		{
			Some(ret) => ret,
			None => false
		}
	}
	
	pub fn on_support(&self, x: i32, y: i32, w: i32, h: i32) -> bool
	{
		self.in_support(x, y + 1, w, h)
	}
	
	pub fn checked_move(&self, start_x: i32, start_y: i32, w: i32, h: i32, vx: i32, vy: i32, descend: bool) -> (i32, i32)
	{
		// First check if we are currently in a support, we collide with it when falling
		let started_in_support = self.in_support(start_x, start_y, w, h);
		let started_on_ground = self.on_ground(start_x, start_y, w, h);
		
		let mut x = start_x;
		let mut y = start_y;
		
		if vy != 0
		{
			let dy = if vy > 0 { 1 } else { -1 };
			for _ in range(0, abs(vy))
			{
				if self.colliding(x, y + dy, w, h)
				{
					break;
				}
				// Landed on support while falling
				if !descend && dy > 0 && self.in_support(x, y + dy, w, h) && !started_in_support
				{
					break;
				}
				y += dy;
			}
		}
		
		let mut hack_fall = false;
		
		if vx != 0
		{
			let dx = if vx > 0 { 1 } else { -1 };
			for _ in range(0, abs(vx))
			{
				// Try falling... hack
				if !hack_fall && !self.on_ground(x, y, w, h) && started_on_ground && (descend || !self.on_support(x, y, w, h))
				{
					hack_fall = true;
					y += 1;
				}
				if self.colliding(x + dx, y, w, h)
				{
					break;
				}
				x += dx;
			}
		}
		
		if hack_fall
		{
			y -= 1;
		}

		(x, y)
	}
	
	pub fn get_light(&self, x: i32, y: i32) -> f32
	{
		self.get_tile_coords(x, y).map_or(1.0, |(tx, ty)|
		{
			self.get_tile(tx, ty).light
		})
	}

	pub fn place_support(&mut self, x: i32, y: i32) -> bool
	{
		let tx = (x + TILE_SIZE / 2).div_floor(&TILE_SIZE);
		let ty = (y + TILE_SIZE / 2).div_floor(&TILE_SIZE);
		
		if tx >= 0 && tx < self.width as i32 && ty >= 0 && ty < self.height as i32 && ty >= SURFACE_HEIGHT
		{
			let tile = self.get_tile_mut(tx as uint, ty as uint);
			if tile.collision == Empty
			{
				let old_value = tile.demon_value;
				let old_policy = tile.demon_policy;
				let old_light = tile.light;
				*tile = Tile::support();
				tile.demon_value = old_value;
				tile.demon_policy = old_policy;
				tile.light = old_light;
				true
			}
			else
			{
				false
			}
		}
		else
		{
			false
		}
	}

	pub fn mine(&mut self, x: i32, y: i32, dtx: i32, dty: i32) -> Option<(i32, i32)>
	{
		let tx = (x + TILE_SIZE / 2).div_floor(&TILE_SIZE) + dtx;
		let ty = (y + TILE_SIZE / 2).div_floor(&TILE_SIZE) + dty;
		
		let mut removed = false;
		
		let ret = if tx >= 0 && tx < self.width as i32 && ty >= 0 && ty < self.height as i32
		{
			let tile = self.get_tile_mut(tx as uint, ty as uint);
			
			if tile.tile_type == Ground ||
			   tile.tile_type == SupportType ||
			   tile.tile_type == Surface ||
			   tile.tile_type == CaveCeiling
			{
				tile.health -= 2;
				if tile.health <= 0
				{
					let ret = if tile.has_gem
					{
						Some((tx * TILE_SIZE + TILE_SIZE / 2, ty * TILE_SIZE + TILE_SIZE / 2))
					}
					else
					{
						None
					};
					*tile = Tile::cave();
					removed = true;
					ret
				}
				else
				{
					None
				}
			}
			else
			{
				None
			}
		}
		else
		{
			None
		};
		
		if removed
		{
			self.need_new_policy = true;
			self.need_new_light = true;
		}
		
		ret
	}
}
