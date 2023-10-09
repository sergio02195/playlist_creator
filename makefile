build-core:
	@cargo build --bin core --release

build-rest:
	@cargo build --bin rest --release

build-api:
	@cargo build --bin api --release

run-core:
	@cargo run --bin core

run-web:
	@cargo run --bin rest-web

run-api:
	@cargo run --bin api

lint:
	@cargo clippy

format-code:
	@cargo fix
	@cargo clippy --fix