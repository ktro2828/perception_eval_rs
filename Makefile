.PHONY: all
all: test

.PHONY: build
build:
	@cargo build --all

.PHONY: test
test:
	@cargo test --all

.PHONY: check
check:
	@cargo check --all

.PHONY: doctest
doctest:
	@cargo test --doc

.PHONY: format
format:
	@cargo fmt --all

.PHONY: format-check
format-check:
	@cargo fmt --all -- --check

.PHONY: lint
lint:
	@cargo clippy --all -- -D clippy::dbg-macro -D warnings

.PHONY: examples
examples:
	@cargo run --example dataset
	@cargo run --example detection