// Turntable struct

use crate::Song;

pub const DEFAULT_VOLUME: u16 = 50;
pub const DEFAULT_SPEED: u16 = 100;

pub struct Turntable<'a> {
	#[allow(unused)]
	stream: rodio::OutputStream,
	sink: rodio::Sink,
	queue: Vec<&'a Song>,
	idx: usize,
	vol: u16,
	speed: u16,
}

impl<'a> Turntable<'a> {
	pub fn new() -> Result<Self, rodio::StreamError> {
		let stream = rodio::OutputStreamBuilder::open_default_stream()?;
		let sink = rodio::Sink::connect_new(&stream.mixer());
		let queue = Vec::new();
		let (idx, vol, speed) = (0, DEFAULT_VOLUME, DEFAULT_SPEED);
		sink.set_volume(vol as f32 / 100.0);
		sink.set_speed(speed as f32 / 100.0);
		Ok(Self {
			stream, sink, queue,
			idx, vol, speed,
		})
	}
	
	pub fn play_song(&mut self, song: &'a Song) {
		self.play_queue(vec![song])
	}
	
	pub fn play_queue(&mut self, queue: Vec<&'a Song>) {
		self.stop();
		self.queue = queue;
		for song in self.queue.iter() {
			// This is a terrible way of doing this
			// But I am too much of a lazy bitch to do it properly
			let f = std::fs::File::open(&song.path).unwrap();
			self.sink.append(rodio::Decoder::try_from(f).unwrap());
		}
	}
	
	pub fn get_vol(&self) -> u16 { self.vol }
	pub fn get_speed(&self) -> u16 { self.speed }
	
	pub fn set_vol(&mut self, vol: u16) {
		if vol > 199 { println!("SORRY, I CAN'T HEAR YOU OVER THIS BANGER MUSIC!") }
		self.vol = vol;
		self.sink.set_volume(vol as f32 / 100.0);
	}
	
	pub fn set_speed(&mut self, speed: u16) {
		if speed > 100 { println!("It seems you like nightcore :)") }
		self.speed = speed;
		self.sink.set_speed(speed as f32 / 100.0);
	}
	
	pub fn pause(&self) { self.sink.pause() }
	pub fn unpause(&self) { self.sink.play() }
	pub fn mute(&self) { self.sink.set_volume(0.0) }
	pub fn unmute(&self) { self.sink.set_volume(self.vol as f32 / 100.0) }
	
	pub fn stop(&mut self) {
		self.sink.stop();
		self.queue.clear();
		self.idx = 0;
	}
}
