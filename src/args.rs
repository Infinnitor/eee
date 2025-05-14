use clap::{arg, command, Command};

pub fn argp() -> Command {
	command!()
		.arg(arg!([name] "The name of the env to load"))
		.arg(arg!(-r --raw "Do not load ~/.bashrc").required(false))
		.arg(arg!(-p --path "Treat the name as a path").required(false))
		.arg(arg!(-l --list "List all envs and paths").required(false))
}
