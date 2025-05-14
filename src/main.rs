#![allow(unused)]
mod args;
mod loader;
mod runner;

use std::path::PathBuf;

fn display_available_environments() {
	println!("environments:");
	for env in loader::find_envs_priority_sieve() {
		println!("\t- {}", env.key)
	}
}

fn main() {
	let mut program = args::argp();
	let help = program.render_help();
	let matches = program.get_matches();

	if matches.get_flag("list") {
		display_available_environments();
		return;
	}

	let optkey = matches.get_one::<String>("name");
	if let None = optkey {
		println!("eee: no environment provided!");
		display_available_environments();
		return;
	}
	let key = optkey.unwrap();

	let env = loader::get_env_from_key(&key);
	if let Some(e) = env {
		let key = e.key.clone();
		runner::start_environment(e, matches.get_flag("raw"));
		println!("eee: exited environment '{}'", key);
	} else {
		println!("eee: environment name '{}' was not found", key);
		display_available_environments();
	}
}
