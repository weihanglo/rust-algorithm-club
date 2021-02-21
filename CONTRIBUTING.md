# Contributing

Thank you for your interest in contributing to the Rust Algorithm Club. We appreciate all kinds of contributions. Here are some kinds of tasks you can take.

- Add new algorithms
- Fix existing bugs
- Polish documetation

Next, we'll introduce some tips to make contributions easily.

## Before Contribuing

If you decide to make a awesome work, please search [existing issues and pull requests][issues] first, as maybe there are some similar one already reported.

If there is no duplicate issue, please file a work-in-progress issue to notify others you are working on it. Your time are precious and must prevent from duplicate works. Maintainers would also track those issues in order to keep our club well organized.

There are also some meta issues tracking features under construction 🚧. Take a look if you are interested in them.

## Submit Your Contributions

Before submitting your contribution, make sure your works satisify the following requirements:

- Do not break existing tests. Run `cargo test` before sending pull requests. A new algorithm is also expected to contain its own unit tests.
- Every public interface must be documented. The documention needn't be perfect but at least explain its intent and usage clearly.
- Try to keep the writing style and structure consistent across posts. E.g. contains a brief description at first paragraph, explains performance with asymptotic notations.
- Coding style should conform to Rust conventions. Such as using `into` to refer to an ownership transfer or naming additional contructors prefixed by `with`. Currently, the use of [Clippy][rust-clippy] and [rustfmt][rust-fmt] are not required.

[issues]: https://github.com/weihanglo/rust-algorithm-club/search?q=&type=Issues&utf8=%E2%9C%93
[rust-clippy]: https://github.com/rust-lang-nursery/rust-clippy
[rust-fmt]: https://github.com/rust-lang-nursery/rustfmt

Welcome to join the Rust Algorithm Club and may algorithms be with you!
