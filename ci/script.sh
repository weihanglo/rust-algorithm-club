set -ex

main() {
    cargo clippy
    cargo build
}

main
