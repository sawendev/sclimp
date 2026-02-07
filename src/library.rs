// Library struct

use crate::Song;
use std::collections::BTreeMap;
use std::path::Path;

fn o2r(o: &std::ffi::OsStr) -> String {
	o.to_string_lossy().to_string()
}

pub struct Library {
	nodes: BTreeMap<String, Node>,
}

impl Library {
	pub fn empty() -> Self {
		Self { nodes: BTreeMap::new() }
	}
	
	pub fn new(root_path: impl AsRef<Path>) -> std::io::Result<Self> {
		let mut nodes = BTreeMap::new();
		for entry in std::fs::read_dir(root_path.as_ref())? {
			let path = entry?.path();
			let name = o2r(path.file_name().unwrap());
			if let Some(node) = Node::from_path(&path)? {
				nodes.insert(name, node);
			}
		}
		Ok(Self { nodes })
	}
	
	pub fn get_node(&self, p: &str) -> Option<&Node> {
		match p.split_once('/') {
			Some((l, r)) => self.nodes.get(l.trim())?.get_node(r),
			None => self.nodes.get(p.trim()),
		}
	}
	
	pub fn songs(&self) -> Vec<&Song> {
		fn cum<'a>(n: &'a Node, v: &mut Vec<&'a Song>) { match n {
			Node::List(l) => l.values().for_each(|n| cum(n, v)),
			Node::Song(s) => v.push(s),
		}}
		let mut songs = Vec::new();
		self.nodes.values().for_each(|n| cum(n, &mut songs));
		songs
	}
	
	pub fn child_names(&self) -> Vec<&str> {
		self.nodes.keys().map(|k| k.as_str()).collect()
	}
}

pub enum Node {
	List(BTreeMap<String, Node>),
	Song(Song),
}

impl Node {
	pub fn get_node(&self, p: &str) -> Option<&Node> {
		match (self, p.split_once('/')) {
			(Self::List(k), Some((l, r))) => k.get(l.trim())?.get_node(r),
			(Self::List(k), None) => k.get(p.trim()),
			_ => None,
		}
	}
	
	pub fn songs(&self) -> Vec<&Song> {
		fn cum<'a>(n: &'a Node, v: &mut Vec<&'a Song>) { match n {
			Node::List(l) => l.values().for_each(|n| cum(n, v)),
			Node::Song(s) => v.push(s),
		}}
		let mut songs = Vec::new();
		cum(self, &mut songs);
		songs
	}
	
	pub fn child_names(&self) -> Vec<&str> { match self {
		Self::List(l) => {
			l.keys().map(|k| k.as_str()).collect()
		},
		Self::Song(_) => vec![]
	}}
	
	pub fn from_path(p: &Path) -> std::io::Result<Option<Self>> {
		if p.is_dir() {
			let mut children = BTreeMap::new();
			for entry in std::fs::read_dir(p)? {
				let path = entry?.path();
				let name = path.file_name().unwrap().to_string_lossy().to_string();
				if let Some(node) = Node::from_path(&path)? {
					children.insert(name, node);
				}
			}
			Ok(Some(Self::List(children)))
		} else {
			match p.extension().and_then(|s| s.to_str())
				.unwrap_or("").to_ascii_lowercase().as_str()
			{
				"mp3" | "wav" | "flac" | "ogg" => {
					let name = o2r(p.file_stem().unwrap());
					Ok(Some(Node::Song(Song {
						name,
						path: p.to_path_buf(),
					})))
				},
				_ => Ok(None),
			}
		}
	}
}
