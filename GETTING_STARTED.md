# Getting Started

## Prerequisite

* rustup 1.16.0 or later
* Rust 1.31.0 or later
    * This repository need some features added in Rust 2018 Edition 

## Walkthrough

Install rustup:

```shell
$ curl https://sh.rustup.rs -sSf | sh
```

Install Rust 1.31.0 and set it default:

```shell
$ rustup default 1.31.0
```

Make sure we have Rust 1.31.0 installed:

```shell
$ rustc --version
rustc 1.31.0 (abe02cefd 2018-12-04)
```

Run tests:

```shell
$ cargo test
```
