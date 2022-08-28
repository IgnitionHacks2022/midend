
default: server-debug

server-debug:
    cargo run --bin server -- 2 true

server:
    cargo run --bin server -- 0 false

gpio-debug:
    cargo run --bin gpio-debug

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
