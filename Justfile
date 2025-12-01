template_dir := `mktemp -d`

list:
    @just --list

build:
    cargo build --release

check:
    cargo +nightly fmt --all -- --check --unstable-features
    cargo +nightly clippy --workspace --all-targets --all-features -- -D warnings
    cargo +nightly check --workspace --locked

fix:
    cargo +nightly clippy --all-targets --all-features --fix --allow-dirty --allow-staged
    cargo +nightly fix --allow-dirty --allow-staged
    cargo +nightly fmt --all -- --unstable-features

test:
    cargo test --all
