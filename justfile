
default: out

out:
    cargo run -- -p -o out.vf test/nested.fd

debug:
    RUST_LOG=info,debug cargo run -- -p test/nested.fd

check:
    cargo check

devsetup:
    cp dev/hooks/* .git/hooks

format:
    cargo fmt --all

book:
    mdbook serve book --open
