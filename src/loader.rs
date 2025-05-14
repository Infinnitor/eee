use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

pub const FOLDERS: [&'static str; 3] = [&"/usr/share/eee/", &".eee/", &"~/.config/eee/"];

#[derive(Debug)]
pub struct Eeenv {
	pub path: PathBuf,
	pub key: String,
	pub priority: usize,
}

impl Eeenv {
	pub fn build_from_dir(entry: DirEntry) -> Option<Self> {
		Some(Eeenv {
			path: entry.path(),
			key: entry.path().file_stem()?.to_str()?.into(),
			priority: FOLDERS
				.iter()
				.map(expanduser::expanduser)
				.filter_map(Result::ok)
				.map(|folder| entry.path().starts_with(folder))
				.enumerate()
				.filter(|(_, fmatch)| *fmatch)
				.map(|(idx, _)| idx)
				.next()?,
		})
	}
}

pub fn find_envs() -> Vec<Eeenv> {
	FOLDERS
		.iter()
		.map(expanduser::expanduser)
		.filter_map(Result::ok)
		.map(fs::read_dir)
		.filter_map(Result::ok)
		.flatten()
		.filter_map(Result::ok)
		.filter_map(Eeenv::build_from_dir)
		.collect()
}

pub fn find_envs_priority_sieve() -> Vec<Eeenv> {
	let envs = find_envs();

	let mut name_reg: Vec<String> = Vec::new();
	envs.into_iter()
		.filter(|env| {
			if name_reg.contains(&env.key) {
				false
			} else {
				name_reg.push(env.key.clone());
				true
			}
		})
		.collect()
}

pub fn get_env_from_key(key: &str) -> Option<Eeenv> {
	let mut dirs = find_envs();
	dirs.sort_by(|a, b| b.priority.cmp(&a.priority));

	for env in dirs {
		if env.key == key {
			return Some(env);
		}
	}
	None
}

pub fn get_path_from_key(key: &str) -> Option<PathBuf> {
	Some(get_env_from_key(key)?.path)
}
