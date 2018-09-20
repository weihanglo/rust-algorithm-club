set -ex

main() {
    cargo build
    cargo test
}

main
