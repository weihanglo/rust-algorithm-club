set -ex

main() {
    # Build book
    mdbook --version # Show version info
    mdbook build

    # Build doc
    cargo doc --lib --no-deps
    mv "target/doc" "$BOOK_DIR/"
}

main
