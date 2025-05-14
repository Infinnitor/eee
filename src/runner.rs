use crate::loader::Eeenv;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use subprocess::Exec;

pub fn parse_shebang(path: PathBuf) -> Option<PathBuf> {
	let fp = File::open(path).ok()?;
	let mut reader = BufReader::new(fp);
	let mut buf = String::from("");

	reader.read_line(&mut buf);
	if buf.starts_with("#!") {
		Some(PathBuf::from(&buf[2..].trim()))
	} else {
		None
	}
}

pub fn start_environment(env: Eeenv, raw: bool) {
	// Start bash shell
	let bash_command_exec = format!(
		"/bin/bash --init-file <(cat {} {})",
		if raw {""} else {"~/.bashrc"},
		env.path.display()
	);

	println!("eee: starting environment '{}'", env.key);

	let cmdline = Exec::cmd("/bin/bash")
		.arg("-c")
		.arg(bash_command_exec)
		.join();
}
