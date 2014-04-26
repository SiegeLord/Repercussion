use allegro5::*;
use allegro_primitives::*;
use allegro_font::*;

use std::cmp::{min, max};
use std::num::abs;
use camera::Camera;
use num::Integer;

pub static TILE_SIZE: i32 = 32;
pub static TILE_HEALTH: i32 = 32;

#[deriving(Eq, Clone)]
enum TileCollision
{
	Solid,
	Empty,
	Support
}

#[deriving(Clone)]
pub struct Tile
{
	collision: TileCollision,
	health: i32,
	support: f32,
	fall_state: i32,
}

impl Tile
{
	pub fn sky() -> Tile
	{
		Tile{ collision: Empty, health: TILE_HEALTH, support: 0.0, fall_state: 0 }
	}

	pub fn cave() -> Tile
	{
		Tile{ collision: Empty, health: TILE_HEALTH, support: 0.0, fall_state: 0 }
	}

	pub fn ground() -> Tile
	{
		Tile{ collision: Solid, health: TILE_HEALTH, support: 4.0, fall_state: 0 }
	}
	
	pub fn support() -> Tile
	{
		Tile{ collision: Support, health: TILE_HEALTH, support: 4.0, fall_state: 0 }
	}
}

pub struct World
{
	width: uint,
	height: uint,
	tiles: Vec<Tile>,
}

impl World
{
	pub fn new(width: uint, height: uint) -> World
	{
		let mut tiles = Vec::with_capacity(width * height);
		for row in range(0, height)
		{
			for col in range(0, width)
			{
				tiles.push
				(
					if col == 7
					{
						Tile::support()
					}
					else if row < 10
					{
						Tile::sky()
					}
					else if row + col > 15
					{
						Tile::ground()
					}
					else
					{
						Tile::sky()
					}
				);
			}
		}
		
		World
		{
			width: width,
			height: height,
			tiles: tiles,
		}
	}
	
	pub fn get_pixel_width(&self) -> i32
	{
		self.width as i32 * TILE_SIZE
	}
	
	pub fn get_pixel_height(&self) -> i32
	{
		self.height as i32 * TILE_SIZE
	}

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, font: &Font, camera: &Camera)
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
				
				let x = (tx as i32 * sz - camera.x) as f32;
				let y = (ty as i32 * sz - camera.y + tile.fall_state) as f32;
				if tile.collision == Solid
				{
					let g = 0.5 * tile.health as f32 / TILE_HEALTH as f32;
					prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, core.map_rgb_f(g, g, g));
					core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x, y, AlignLeft, format!("{}", tile.support));
				}
				else if tile.collision == Support
				{
					prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, core.map_rgb_f(0.5, 1.0, 1.0));
					core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x, y, AlignLeft, format!("{}", tile.support));
				}
			}
		}
	}
	
	pub fn update(&mut self, camera: &mut Camera)
	{
		for y in range(1, self.height - 1).rev()
		{
			for x in range(0, self.width)
			{
				if self.get_tile(x, y).collision != Empty
				{
					let mut sup = 0.0f32;
					if x > 0
					{
						let tile = self.get_tile(x - 1, y);
						sup = sup.max(tile.support * (tile.health as f32 / TILE_HEALTH as f32) - 1.0);
					}
					if x < self.width - 1
					{
						let tile = self.get_tile(x + 1, y);
						sup = sup.max(tile.support * (tile.health as f32 / TILE_HEALTH as f32) - 1.0);
					}
					{
						let tile = self.get_tile(x, y + 1);
						sup = sup.max(tile.support * (tile.health as f32 / TILE_HEALTH as f32));
					}
					
					if sup <= 1.0
					{
						if self.get_tile(x, y + 1).collision == Empty
						{
							let mut tile = self.get_tile(x, y).clone();
							tile.fall_state = -TILE_SIZE;
							*self.get_tile_mut(x, y) = Tile::cave();
							*self.get_tile_mut(x, y + 1) = tile;
						}
					}
					else
					{
						self.get_tile_mut(x, y).support = sup;
					}

					if self.get_tile(x, y).fall_state < 0
					{
						self.get_tile_mut(x, y).fall_state += 1;
						if self.get_tile(x, y).fall_state == 0
						{
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

	pub fn place_support(&mut self, x: i32, y: i32) -> bool
	{
		let tx = (x + TILE_SIZE / 2).div_floor(&TILE_SIZE);
		let ty = (y + TILE_SIZE / 2).div_floor(&TILE_SIZE);
		
		if tx >= 0 && tx < self.width as i32 && ty >= 0 && ty < self.height as i32
		{
			let tile = self.get_tile_mut(tx as uint, ty as uint);
			if tile.collision == Empty
			{
				*tile = Tile::support();
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

	pub fn mine(&mut self, x: i32, y: i32, dtx: i32, dty: i32)
	{
		let tx = (x + TILE_SIZE / 2).div_floor(&TILE_SIZE) + dtx;
		let ty = (y + TILE_SIZE / 2).div_floor(&TILE_SIZE) + dty;
		
		if tx >= 0 && tx < self.width as i32 && ty >= 0 && ty < self.height as i32
		{
			let tile = self.get_tile_mut(tx as uint, ty as uint);
			tile.health -= 1;
			if tile.health <= 0
			{
				*tile = Tile::sky();
			}
		}
	}
}
