use allegro5::*;
use allegro_font::*;

use std::cmp::{max, min};

use gfx::Gfx;

pub struct Message
{
	lines: Vec<~str>,
	progress: uint,
	total_len: uint,
	ready_to_hide: bool,
	char_timeout: i32,
	hide_timeout: i32,
	message_type: MessageType,
	duration: i32,
	max_width: f32,
}

#[deriving(Eq)]
enum MessageType
{
	RadioMessage,
	JohnMessage,
	CenteredMessage
}

impl Message
{
	pub fn new(message_type: MessageType, duration: i32, lines: &[~str]) -> Message
	{
		let mut max_width = 0.0f32;
		let mut total_len = 0;
		for line in lines.iter()
		{
			total_len += line.len();
			max_width = max_width.max(line.len() as f32 * 8.0);
		}
		Message
		{
			lines: Vec::from_slice(lines),
			progress: 0,
			ready_to_hide: false,
			char_timeout: 0,
			hide_timeout: 0,
			total_len: total_len,
			message_type: message_type,
			duration: duration,
			max_width: max_width
		}
	}
	
	pub fn intro() -> Message
	{
		Message::new(RadioMessage, 240,
		[~"The Philosopher's Stone is buried",
		 ~"deep beneath the earth. Dig it up",
		 ~"and return to the surface.",
		 ~"Press F1 for help."])
	}

	pub fn found() -> Message
	{
		Message::new(JohnMessage, 240,
		[~"This must be it... Can I escape",
		 ~"this hellish place?"])
	}

	pub fn surface() -> Message
	{
		Message::new(JohnMessage, 240,
		[~"The daylight! I'm out! No more",
		 ~"demons!"])
	}

	pub fn no_john() -> Message
	{
		Message::new(CenteredMessage, 240,
		[~"No John. You are the demons."])
	}

	pub fn eaten() -> Message
	{
		Message::new(CenteredMessage, 240,
		[~"Eaten by a demon!"])
	}

	pub fn crushed() -> Message
	{
		Message::new(CenteredMessage, 240,
		[~"Crushed by collapsing rock!"])
	}
	
	pub fn update(&mut self) -> bool
	{
		self.char_timeout = max(0, self.char_timeout - 1);
		self.hide_timeout = max(0, self.hide_timeout - 1);
		
		let old_done = self.progress > self.total_len;
		
		if self.char_timeout <= 0
		{
			self.progress += 1;
			self.char_timeout = 5;
		}
		
		let new_done = self.progress > self.total_len;
		
		if new_done && !old_done
		{
			self.hide_timeout = self.duration;
		}
		
		if new_done && self.hide_timeout == 0
		{
			true
		}
		else
		{
			false
		}
	}
	
	pub fn draw(&self, gfx: &Gfx, dw: i32, dh: i32, core: &Core, font: &Font)
	{
		let mut chars_left = self.progress;

		let (x, mut y) = if self.message_type == CenteredMessage
		{
			(dw / 2 - self.max_width as i32 / 2, dh / 2 + 25)
		}
		else
		{
			(dw / 2 - 105, dh - 70)
		};
		
		if self.message_type == RadioMessage
		{
			gfx.radio_message.draw(core, dw / 2 - 175, y - 10);
		}
		else if self.message_type == JohnMessage
		{
			gfx.john_message.draw(core, dw / 2 - 175, y - 10);
		}
		
		for line in self.lines.iter()
		{
			let chars_to_show = min(line.len(), chars_left);
			if chars_to_show == 0
			{
				return;
			}
			core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x as f32, y as f32, AlignLeft, line.slice_to(chars_to_show));
			
			y += 10;
			chars_left -= chars_to_show;
		}
	}

	pub fn draw_help(dw: i32, _dh: i32, core: &Core, font: &Font)
	{
		let help = 
		[
		    "Controls",
		    "",
		    "Arrows  - Move",
			"W/A/S/D - Dig",
			"Space   - Jump",
			"R       - Place support",
			"T       - Place torch",
		];
		
		let x = dw / 2 - 80;
		
		let mut y = 40;
		
		for &line in help.iter()
		{
			core.draw_text(font, core.map_rgb_f(1.0, 1.0, 1.0), x as f32, y as f32, AlignLeft, line);
			y += 10;
		}
	}
}
