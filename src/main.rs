// Sawen's CLI Music Player
// A command-line app to play user-provided music

use uwutils as uwu;
use sclimp::library::Library;
use sclimp::turntable::Turntable;
use sclimp::Song;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;

fn main() {
	println!("-- Sawen's CLI Music Player --");
	println!();
	
	println!("Please enter the path to your Music folder:");
	let path = uwu::input_str();
	let lib = match Library::new(path) {
		Ok(lib) => lib,
		Err(e) => {
			eprintln!("Failed to open library: {e}");
			return;
		},
	};
	
	let mut tt = match Turntable::new() {
		Ok(tt) => tt,
		Err(e) => {
			eprintln!("Failed to create turntable: {e}");
			return;
		}
	};
	
	let song_count = lib.songs().iter().count();
	if song_count == 0 {
		println!("Successfully opened library, but there aren't any songs!");
		return;
	} else {
		println!("Successfully opened library with {song_count} songs");
	}
	
	let mut queue: Vec<&Song> = Vec::new();
	
	loop {
		println!();
		println!("What would you like to do?");
		uwu::choice::<for<'a> fn(&'a Library, &mut Turntable<'a>, &mut Vec<&'a Song>)>(&[
			("Add all songs to the queue", |l,_,q| q.extend(l.songs())),
			("Add songs/albums to the queue", |l,_,q| {
				println!();
				println!("Enter the songs/albums you wish to add (separate multiple with ';')");
				let input = uwu::input_str();
				for song in input.split(';').map(|s| s.trim()) {
					match l.get_node(song) {
						Some(n) => {
							let songs = n.songs();
							println!("Adding {song:?} ({} songs)", songs.len());
							q.extend(songs)
						},
						None => println!("Could not find {song:?}"),
					}
				}
			}),
			("Clear the queue", |_,_,q| q.clear()),
			("Shuffle the queue", |_,_,q| q.shuffle(&mut rand::rng())),
			("Display the queue", |_,_,q| q.iter().for_each(|s| println!("{}", s.name))),
			("Play all songs in the queue", |_,t,q| t.play_queue(q.clone())),
			("Play random song from queue", |_,t,q| t.play_song(q.choose(&mut rand::rng()).unwrap())),
			("Stop the music", |_,t,_| t.stop()),
			("Change the music volume", |_,t,_| {
				println!();
				match uwu::prompt::<u16>(&format!("Enter new volume (currently {}): ", t.get_vol())) {
					Ok(vol) => t.set_vol(vol),
					Err(e) => println!("Error: {e}"),
				}
			}),
			("Change the music speed", |_,t,_| {
				println!();
				match uwu::prompt::<u16>(&format!("Enter new speed (currently {}): ", t.get_speed())) {
					Ok(speed) => t.set_speed(speed),
					Err(e) => println!("Error: {e}"),
				}
			}),
			("Pause the music", |_,t,_| t.pause()),
			("Unpause the music", |_,t,_| t.unpause()),
			("Mute the music", |_,t,_| t.mute()),
			("Unmute the music", |_,t,_| t.unmute()),
			("List albums/songs in library", |l,_,_| l.child_names().iter().for_each(|n| println!("{n}"))),
			("List sub-albums/songs in album", |l,_,_| {
				println!();
				println!("Enter the albums you wish to see the contents of (separate multiple with ';')");
				let input = uwu::input_str();
				for album in input.split(';').map(|s| s.trim()) {
					match l.get_node(album) {
						Some(n) => {
							let names = n.child_names();
							if names.is_empty() {
								println!("{album:?} is empty");
							} else {
								println!("{album:?} contains the following:");
								names.iter().for_each(|name| println!("    {name}"));
							}
						},
						None => println!("Could not find {album:?}"),
					}
				}
			}),
			("Exit the program", |_,_,_| std::process::exit(0)),
		])(&lib, &mut tt, &mut queue);
	}
}
