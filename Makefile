check:
	cargo run

build:
	cargo build

release:
	cargo build --release

install:
	cargo build --release
	sudo cp ./target/release/eee /usr/local/bin/
