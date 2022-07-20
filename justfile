
default: out

out:
    cargo run 1> out.vf

debug:
    RUST_LOG=info,debug cargo run

check:
    cargo check

devsetup:
    cp dev/hooks/* .git/hooks

format:
    cargo fmt --all

book:
    mdbook serve book --open
