all: debug

debug: test
	cargo build

release: test
	cargo build --release

test:
	cargo test -- --test-threads=1

otest:
	cargo test -- --test-threads=1 --show-output
