_default:
    just --list

test:
    cargo test

check:
    cargo check

check-clip:
    cargo check |& clip
