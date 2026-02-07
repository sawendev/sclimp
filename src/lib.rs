// Sawen's CLI Music Player Backend

pub mod library;
pub mod turntable;

use std::path::PathBuf;
pub struct Song {
	pub name: String,
	pub path: PathBuf,
}
