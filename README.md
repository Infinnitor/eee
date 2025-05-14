# EEE

A bash environment switcher

## Install

You will need cargo installed. Clone the repository.

```bash
cd eee
make install
```

# Usage

Navigate to ~/.config/eee/ and create a file called `env.sh`. Put the environment configuration inside this file (e.g. Prompt assignment, variables, function declarations)

```bash
# Example ~/.config/eee/env.sh
PS1="\w  prompt % "
echo "welcome to., the shell,,,"
```

Then run `eee --list` to see a list of environments

```
environments:
	- env
```

Run `eee env` to select your newly created env

```
eee: starting environment 'env'
welcome to., the shell,,,
~  prompt %
```

Use exit or Ctrl-D to exit the environment.

The eee environments also load your `~/.bashrc` automatically, if you do not want this behaviour, pass the `--raw` argument

```bash
eee env --raw
```
