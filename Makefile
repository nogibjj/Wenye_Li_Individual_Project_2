check:
	cargo check

build:
	cargo build --release

run:
	cargo run

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

all: check build run format lint test