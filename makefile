build-core:
	@cargo build --bin core --release

run-core:
	@cargo run --bin core

build-rest:
	@cargo build --bin rest --release

run-rest:
	@cargo run --bin rest

lint:
	@cargo clippy

format-code:
	@cargo fix
	@cargo clippy --fix