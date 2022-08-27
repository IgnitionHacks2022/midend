
default: server

server:
    cargo run --bin server

gpio-debug:
    cargo run --bin gpio-debug

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
