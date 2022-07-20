
default: run

run:
    RUST_LOG=info cargo run

out:
    cargo run 1> out.vf

check:
    cargo check

devsetup:
    cp dev/hooks/* .git/hooks

format:
    cargo fmt --all

book:
    mdbook serve book --open
