fmt:
    cargo fmt --all

deps:
    cargo upgrade
    cargo update

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

release ver="": deps test
    rm -rf target

    git-cliff --tag v{{ver}} > CHANGELOG.md

    git add Cargo* CHANGELOG.md
    git commit -m "chore(release): prepare for v{{ver}}"
    git tag -a v{{ver}} -m "v{{ver}}"

    git push origin main --follow-tags

    CARGO_TARGET_DIR=target \
    cargo publish && \
    cargo build --bin nvrs --features=nvrs_cli --release
