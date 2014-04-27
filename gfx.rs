use allegro5::*;
use sprite::Sprite;

pub struct Gfx
{
	pub player_left: Sprite,
	pub player_right: Sprite,
	pub drill_left: Sprite,
	pub drill_right: Sprite,
	pub drill_up: Sprite,
	pub drill_down: Sprite,

	pub player_left_hi: Sprite,
	pub player_right_hi: Sprite,
	pub drill_left_hi: Sprite,
	pub drill_right_hi: Sprite,
	pub drill_up_hi: Sprite,
	pub drill_down_hi: Sprite,
	
	pub gem: Sprite,
	pub gem_hi: Sprite,
	pub fun: Sprite,
	pub fun_hi: Sprite,
	pub torch: Sprite,
	pub ui_gem: Sprite,
	pub skeleton: Sprite,
	pub radio_message: Sprite,
	pub john_message: Sprite,
}

impl Gfx
{
	pub fn new(core: &Core) -> Gfx
	{
		Gfx
		{
			player_left: Sprite::new(core, "data/player_left.png", 24, 24),
			player_right: Sprite::new(core, "data/player_right.png", 24, 24),
			drill_left: Sprite::new(core, "data/drill_left.png", 24, 24),
			drill_right: Sprite::new(core, "data/drill_right.png", 24, 24),
			
			drill_up: Sprite::new(core, "data/drill_up.png", 24, 24),
			drill_down: Sprite::new(core, "data/drill_down.png", 24, 24),
			
			player_left_hi: Sprite::new(core, "data/player_left_hi.png", 24, 24),
			player_right_hi: Sprite::new(core, "data/player_right_hi.png", 24, 24),
			drill_left_hi: Sprite::new(core, "data/drill_left_hi.png", 24, 24),
			drill_right_hi: Sprite::new(core, "data/drill_right_hi.png", 24, 24),
			
			drill_up_hi: Sprite::new(core, "data/drill_up_hi.png", 24, 24),
			drill_down_hi: Sprite::new(core, "data/drill_down_hi.png", 24, 24),
			
			gem: Sprite::new(core, "data/gem.png", 16, 16),
			gem_hi: Sprite::new(core, "data/gem_hi.png", 16, 16),
			fun: Sprite::new(core, "data/fun.png", 24, 24),
			fun_hi: Sprite::new(core, "data/fun_hi.png", 24, 24),
			torch: Sprite::new(core, "data/torch.png", 16, 16),
			ui_gem: Sprite::new(core, "data/ui_gem.png", 32, 32),
			skeleton: Sprite::new(core, "data/skeleton.png", 24, 24),
			
			radio_message: Sprite::new(core, "data/radio_message.png", 350, 60),
			john_message: Sprite::new(core, "data/john_message.png", 350, 60),
		}
	}
}
