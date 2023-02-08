build:
	cargo lambda build --release --bin api-1 --bin api-2

schema:
	cargo run --bin api-1-docs
	cargo run --bin api-2-docs