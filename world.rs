use allegro5::*;
use allegro_primitives::*;

use std::cmp::{min, max};
use std::num::abs;
use camera::Camera;
use num::Integer;

pub static TILE_SIZE: i32 = 32;

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
	collision: TileCollision
}

impl Tile
{
	pub fn sky() -> Tile
	{
		Tile{ collision: Empty }
	}

	pub fn ground() -> Tile
	{
		Tile{ collision: Solid }
	}
	
	pub fn support() -> Tile
	{
		Tile{ collision: Support }
	}
}

pub struct World
{
	width: uint,
	height: uint,
	tiles: Vec<Tile>
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

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, camera: &Camera)
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
				let x = (tx as i32 * sz - camera.x) as f32;
				let y = (ty as i32 * sz - camera.y) as f32;
				let idx = ty * self.width + tx;
				if self.tiles.get(idx).collision == Solid
				{
					prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, core.map_rgb_f(1.0, 1.0, 1.0));
				}
				else if self.tiles.get(idx).collision == Support
				{
					prim.draw_filled_rectangle(x, y, x + sz as f32, y + sz as f32, core.map_rgb_f(1.0, 0.0, 1.0));
				}
			}
		}
	}
	
	pub fn get_tile<'l>(&'l self, tx: uint, ty: uint) -> &'l Tile
	{
		self.tiles.get(ty * self.width + tx)
	}

	pub fn colliding(&self, x: i32, y: i32) -> bool
	{
		let tx1 = x.div_floor(&TILE_SIZE);
		let ty1 = y.div_floor(&TILE_SIZE);
		let tx2 = (x + TILE_SIZE - 1).div_floor(&TILE_SIZE);
		let ty2 = (y + TILE_SIZE - 1).div_floor(&TILE_SIZE);
		
		//~ println!("tx1: {} ty1: {} tx2: {}, ty2: {}", tx1, ty1, tx2, ty2);

		tx1 < 0 || tx1 >= self.width as i32 ||
		tx2 < 0 || tx2 >= self.width as i32 ||
		ty1 < 0 || ty1 >= self.height as i32 ||
		ty2 < 0 || ty2 >= self.height as i32 ||
		self.get_tile(tx1 as uint, ty1 as uint).collision == Solid ||
		self.get_tile(tx1 as uint, ty2 as uint).collision == Solid ||
		self.get_tile(tx2 as uint, ty2 as uint).collision == Solid ||
		self.get_tile(tx2 as uint, ty2 as uint).collision == Solid
	}

	pub fn on_ground(&self, x: i32, y: i32) -> bool
	{
		self.colliding(x, y + 1)
	}
	
	pub fn in_support(&self, x: i32, y: i32) -> bool
	{
		let tx1 = x.div_floor(&TILE_SIZE);
		let ty1 = y.div_floor(&TILE_SIZE);
		let tx2 = (x + TILE_SIZE - 1).div_floor(&TILE_SIZE);
		let ty2 = (y + TILE_SIZE - 1).div_floor(&TILE_SIZE);
		
		if tx1 < 0 || tx1 >= self.width as i32 ||
		   tx2 < 0 || tx2 >= self.width as i32 ||
		   ty1 < 0 || ty1 >= self.height as i32 ||
		   ty2 < 0 || ty2 >= self.height as i32
		{
			false
		}
		else
		{
			self.get_tile(tx1 as uint, ty1 as uint).collision == Support ||
			self.get_tile(tx1 as uint, ty2 as uint).collision == Support ||
			self.get_tile(tx2 as uint, ty2 as uint).collision == Support ||
			self.get_tile(tx2 as uint, ty2 as uint).collision == Support
		}
	}
	
	pub fn checked_move(&self, start_x: i32, start_y: i32, vx: i32, vy: i32) -> (i32, i32)
	{
		// First check if we are currently in a support, we collide with it when falling
		let started_in_support = self.in_support(start_x, start_y);
		
		let mut x = start_x;
		let mut y = start_y;
		if vx != 0
		{
			let dx = if vx > 0 { 1 } else { -1 };
			for _ in range(0, abs(vx))
			{
				if self.colliding(x + dx, y)
				{
					break;
				}
				x += dx;
			}
		}

		if vy != 0
		{
			let dy = if vy > 0 { 1 } else { -1 };
			for _ in range(0, abs(vy))
			{
				if self.colliding(x, y + dy)
				{
					break;
				}
				// Landed on support while falling
				if dy > 0 && self.in_support(x, y + dy) && !started_in_support
				{
					break;
				}
				y += dy;
			}
		}
		(x, y)
	}
}
