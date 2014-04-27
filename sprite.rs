use allegro5::*;
use allegro5::ffi::al_get_time;

use std::cmp::min;

pub struct Sprite
{
	base_bitmap: Bitmap,
	width: i32,
	height: i32,
	x_tiles: i32,
	y_tiles: i32,
	pub offset: f64
}

impl Sprite
{
	pub fn new(core: &Core, filename: &str, width: i32, height: i32) -> Sprite
	{
		let bmp = core.load_bitmap(filename).expect(format!("Could not load {}", filename));
		let x_tiles = bmp.get_width() / width;
		let y_tiles = bmp.get_height() / height;
		
		Sprite
		{
			base_bitmap: bmp,
			width: width,
			height: height,
			x_tiles: x_tiles,
			y_tiles: y_tiles,
			offset: 0.0
		}
	}

	pub fn reset(&mut self, _: &Core)
	{
		self.offset = 
		unsafe
		{
			al_get_time() as f64
		};
	}
		
	
	pub fn draw(&self, core: &Core, x: i32, y: i32)
	{
		self.draw_tinted(core, x, y, core.map_rgb_f(1.0, 1.0, 1.0));
	}

	pub fn draw_no_loop(&self, core: &Core, x: i32, y: i32)
	{
		let frame = min(unsafe
		{
			(al_get_time() as f64 - self.offset) * 4.0
		} as i32, self.x_tiles * self.y_tiles - 1);
		
		self.draw_frame(core, frame, x, y, core.map_rgb_f(1.0, 1.0, 1.0));
	}

	pub fn draw_tinted(&self, core: &Core, x: i32, y: i32, color: Color)
	{
		let frame = unsafe
		{
			(al_get_time() as f64 - self.offset) * 8.0
		} as i32 % (self.x_tiles * self.y_tiles);
		
		self.draw_frame(core, frame, x, y, color);
	}

	pub fn draw_frame(&self, core: &Core, frame: i32, x: i32, y: i32, color: Color)
	{
		let tile_x = (frame % self.x_tiles) as f32;
		let tile_y = (frame / self.x_tiles) as f32;

		let w = self.width as f32;
		let h = self.height as f32;
		
		core.draw_tinted_bitmap_region(&self.base_bitmap, color, tile_x * w, tile_y * h, w, h, x as f32, y as f32, Flag::zero());
	}
}
