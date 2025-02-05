fmt:
    cargo fmt --all

run-cli:
    cargo run --bin nvrs --features=nvrs_cli

run-tui:
    cargo run --bin nvrs_tui --features=nvrs_tui

check:
    cargo fmt --all --check
    cargo clippy --all-features -- -Dwarnings
    cargo clippy --all-features --release -- -Dwarnings

test: check
    cargo test --all-features --no-fail-fast

release:
    CARGO_TARGET_DIR=target \
    cargo build --bin nvrs --features=nvrs_cli --release
