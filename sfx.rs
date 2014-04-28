use allegro_audio::*;

pub struct Sfx
{
	walk_sound: Sample,
	pub walk_instance: SampleInstance,
	drill_sound: Sample,
	pub drill_instance: SampleInstance,
	collapse_sound: Sample,
	pub collapse_instance: SampleInstance,
	typing_sound: Sample,
	pub typing_instance: SampleInstance,
	sink: Sink,
	
	gem_sound: Sample,
	gem_instance: Option<SampleInstance>,
	dead_sound: Sample,
	dead_instance: Option<SampleInstance>,
	phil_sound: Sample,
	phil_instance: Option<SampleInstance>,
	place_sound: Sample,
	place_instance: Option<SampleInstance>,
	invalid_sound: Sample,
	invalid_instance: Option<SampleInstance>,
	fun_sound: Sample,
	fun_instance: Option<SampleInstance>,
	end_sound: Sample,
	end_instance: Option<SampleInstance>,
}

pub fn load_sample(audio: &AudioAddon, filename: &str) -> Sample
{
	audio.load_sample(filename).expect(format!("Failed to load {}", filename))
}

impl Sfx
{
	pub fn new(audio: &AudioAddon) -> Sfx
	{
		let mut sink = audio.create_sink().expect("Failed to create audio sink");
		
		let walk_sound = load_sample(audio, "data/walk.ogg");
		let mut walk_instance = audio.create_sample_instance().unwrap();
		walk_instance.set_sample(&walk_sound);
		walk_instance.set_playing(false);
		walk_instance.attach(&mut sink);
		
		let drill_sound = load_sample(audio, "data/drill.ogg");
		let mut drill_instance = audio.create_sample_instance().unwrap();
		drill_instance.set_sample(&drill_sound);
		drill_instance.set_playing(false);
		drill_instance.attach(&mut sink);
		
		let collapse_sound = load_sample(audio, "data/collapse.ogg");
		let mut collapse_instance = audio.create_sample_instance().unwrap();
		collapse_instance.set_sample(&collapse_sound);
		collapse_instance.set_playing(false);
		collapse_instance.attach(&mut sink);
		
		let typing_sound = load_sample(audio, "data/typing.ogg");
		let mut typing_instance = audio.create_sample_instance().unwrap();
		typing_instance.set_sample(&typing_sound);
		typing_instance.set_playing(false);
		typing_instance.attach(&mut sink);
		
		Sfx
		{
			sink: sink,
			walk_sound: walk_sound,
			walk_instance: walk_instance,
			drill_sound: drill_sound,
			drill_instance: drill_instance,
			collapse_sound: collapse_sound,
			collapse_instance: collapse_instance,
			typing_sound: typing_sound,
			typing_instance: typing_instance,
			gem_sound: load_sample(audio, "data/gem.ogg"),
			gem_instance: None,
			phil_sound: load_sample(audio, "data/phil.ogg"),
			phil_instance: None,
			dead_sound: load_sample(audio, "data/dead.ogg"),
			dead_instance: None,
			place_sound: load_sample(audio, "data/place.ogg"),
			place_instance: None,
			invalid_sound: load_sample(audio, "data/invalid.ogg"),
			invalid_instance: None,
			fun_sound: load_sample(audio, "data/fun.ogg"),
			fun_instance: None,
			end_sound: load_sample(audio, "data/end.ogg"),
			end_instance: None,
		}
	}
	
	pub fn play_gem(&mut self)
	{
		self.gem_instance = self.sink.play_sample(&self.gem_sound, 1.0, None, 1.0, PlaymodeOnce);
	}
	
	pub fn play_dead(&mut self)
	{
		self.dead_instance = self.sink.play_sample(&self.dead_sound, 1.0, None, 1.0, PlaymodeOnce);
	}
	
	pub fn play_phil(&mut self)
	{
		self.phil_instance = self.sink.play_sample(&self.phil_sound, 1.0, None, 1.0, PlaymodeOnce);
	}
	
	pub fn play_place(&mut self)
	{
		self.place_instance = self.sink.play_sample(&self.place_sound, 1.0, None, 1.0, PlaymodeOnce);
	}

	pub fn play_invalid(&mut self)
	{
		self.invalid_instance = self.sink.play_sample(&self.invalid_sound, 1.0, None, 1.0, PlaymodeOnce);
	}
	
	pub fn play_fun(&mut self)
	{
		self.fun_instance = self.sink.play_sample(&self.fun_sound, 1.0, None, 1.0, PlaymodeOnce);
	}

	pub fn play_end(&mut self)
	{
		self.end_instance = self.sink.play_sample(&self.end_sound, 1.0, None, 1.0, PlaymodeOnce);
	}
}
