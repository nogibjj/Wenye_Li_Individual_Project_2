check:
	cargo check

build:
	cargo build --release

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

all: check build format lint test