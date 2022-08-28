
default: server-debug

server-debug:
    RUST_LOG=info,debug cargo run --bin server -- 0 true

server:
    RUST_LOG=info cargo run --bin server -- 0 false

gpio-debug:
    RUST_LOG=info cargo run --bin gpio-debug

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
