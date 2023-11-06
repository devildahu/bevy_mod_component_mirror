check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy --all-features -- -W clippy::all --deny warnings

lint: fmt clippy