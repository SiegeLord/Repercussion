use allegro5::*;
use allegro_primitives::*;

use std::cmp::{min, max};
use camera::Camera;

pub static TILE_SIZE: uint = 32;

#[deriving(Clone)]
pub struct Tile
{
	solid: bool
}

impl Tile
{
	pub fn sky() -> Tile
	{
		Tile{ solid: false }
	}

	pub fn ground() -> Tile
	{
		Tile{ solid: true }
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
					if row < 10
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
		}
	}

	pub fn draw(&self, core: &Core, prim: &PrimitivesAddon, camera: &Camera)
	{
		let sz = TILE_SIZE as f32;
		let min_col = min((camera.x / sz).max(0.0) as uint, self.width);
		let min_row = min((camera.y / sz).max(0.0) as uint, self.height);
		let max_col = min(min_col + (camera.width / sz) as uint + 2, self.width);
		let max_row = min(min_row + (camera.height / sz) as uint + 2, self.height);
		
		for row in range(min_row, max_row)
		{
			for col in range(min_col, max_col)
			{
				let x = col as f32 * sz - camera.x;
				let y = row as f32 * sz - camera.y;
				let idx = row * self.width + col;
				if self.tiles.get(idx).solid
				{
					prim.draw_filled_rectangle(x, y, x + sz, y + sz, core.map_rgb_f(1.0, 1.0, 1.0));
				}
			}
		}
	}
}
