
default: out

out:
    cargo run -- -p -o out.vf

debug:
    RUST_LOG=info,debug cargo run -- -p

check:
    cargo check

devsetup:
    cp dev/hooks/* .git/hooks

format:
    cargo fmt --all

book:
    mdbook serve book --open
