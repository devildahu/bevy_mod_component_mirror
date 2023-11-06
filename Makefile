check:
	cargo check

pre-hook:
	cargo fmt --all -- --check
	cargo clippy --no-default-features -- --deny clippy::all -D warnings
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
	cargo clippy --all-features -- --deny clippy::all -D warnings
	cargo test --all-features
