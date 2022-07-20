
default: run

run:
    cargo run

out:
    cargo run 1> out.vf

devsetup:
    cp dev/hooks/* .git/hooks

format:
    cargo fmt --all
