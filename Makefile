.EXPORT_ALL_VARIABLES:

build:
	cargo build --all-targets --all-features --workspace

check-fmt:
	cargo  fmt --all -- --check

check-clippy:
	cargo  clippy --all-targets --all-features --workspace -- -D warnings

install-cargo-sort:
	cargo install cargo-sort@1.0.9

cargo-sort: install-cargo-sort
	cargo sort -c -w

install-cargo-machete:
	cargo install cargo-machete

cargo-machete: install-cargo-machete
	cargo machete

install-taplo-cli:
	cargo install taplo-cli@0.9.0

fix-toml: install-taplo-cli
	taplo fmt

check-toml: install-taplo-cli
	taplo check

check: check-fmt check-clippy cargo-sort check-toml cargo-machete

doc-test:
	cargo test --no-fail-fast --doc --all-features --workspace

unit-test: doc-test
	cargo test --no-fail-fast --lib --all-features --workspace

test: doc-test
	cargo test --no-fail-fast --all-targets --all-features --workspace