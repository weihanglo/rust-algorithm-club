set -ex

main() {
    source ~/.cargo/env || true
    cargo install cargo-update || true
    cargo install-update -a
    cargo install mdbook --vers "^0.2" | tee
}

main
