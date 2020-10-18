<p align="center">
  <img src="src/logo.svg" alt="logo">
<p>

# Rust Algorithm Club

> ### 🚧 🚧 This repo is under construction. Most materials are written in Chinese. [Check it out here](https://rust-algo.club) if you are able to read Chinese.

Welcome to the Rust Algorithm Club! This repository was originally inspired by [Swift Algorithm Club][swift-algorithm-club]. All algorithms here would be explained and implemented in [Rust programming language][rust]!
You can find out more on the [Rust Algorithm Club][main-site] main site. Just pick up some algorithms you are interested in and start learning. If you are brave enough, we recommend you the [auto-generated API documentation][generated-doc]. Go and fight with the source code.

This project along with its source code are on [GitHub][source-code] and we are looking forward to your contributions.

[![Rust Edition](https://img.shields.io/badge/Rust_Edition-2018-green.svg)][edition-guide]
[![Build Status](https://github.com/weihanglo/rust-algorithm-club/workflows/CI/badge.svg)][ci-status]
[![Documentation](https://img.shields.io/badge/doc-available-blue.svg)][generated-doc]

[swift-algorithm-club]: https://github.com/raywenderlich/swift-algorithm-club
[rust]: https://www.rust-lang.org/
[source-code]: https://github.com/weihanglo/rust-algorithm-club
[main-site]: https://rust-algo.club
[ci-status]: https://github.com/weihanglo/rust-algorithm-club/actions?query=workflow%3ACI
[generated-doc]: https://rust-algo.club/doc/rust_algorithm_club/
[edition-guide]: https://rust-lang.github.io/edition-guide/rust-2018

## General Concepts

- [Asymptotic Notation](src/concepts/asymptotic-notation)

## Algorithms

### Searching

- [Linear search](src/searching/linear_search)
- [Binary search](src/searching/binary_search)
- [Interpolation search](src/searching/interpolation_search)
- [Exponential search](src/searching/exponential_search)

### Sorting

Simple sorts:

- [Insertion sort](src/sorting/insertion_sort)
- [Selection sort](src/sorting/selection_sort)
- [Bubble sort](src/sorting/bubble_sort)
- [Shellsort](src/sorting/shellsort)

Efficient sorts:

- [Heapsort](src/sorting/heapsort)
- [Quicksort](src/sorting/quicksort)
- [Mergesort](src/sorting/mergesort)

Hybrid sorts (more efficient):

- 🚧 [Introsort](src/sorting/introsort)
- 🚧 [Timsort](src/sorting/timsort)
- 🚧 [Pdqsort](src/sorting/pdqsort)

Special-purpose sorts:

- [Counting sort](src/sorting/counting_sort)
- [Bucket sort](src/sorting/bucket_sort)
- [Radix sort](src/sorting/radix_sort)

### Searching

## Data Structures

### Stack and Queue

- [Stack](src/collections/stack)
- [🚧 Queue](src/collections/queue)
- [🚧 Deque](src/collections/deque)

### Linked List

[Introduction to linked list](src/collections/linked_list)

- [Singly linked list](src/collections/singly_linked_list)
- [🚧 Doubly linked list](src/collections/doubly_linked_list)
- [🚧 Circular linked list](src/collections/circular_linked_list)

### Associative Container

[Introduction to associative container](src/collections/associative-container)

- [Hash map](src/collections/hash_map)
- [🚧 Ordered map](src/collections/ordered_map)
- [🚧 Multimap](src/collections/multimap)
- [Set](src/collections/set)
- [Bloom filter](src/collections/bloom_filter)

## Learning Resources

For learning more, you may check out following online resources:

- [VisuAlgo](https://visualgo.net/) - Probably the best algorithms visualization website.
- [Big-O Cheat Sheet](http://bigocheatsheet.com/) - Comprehensive Big-O notation cheat sheet.
- [Rosetta Code](http://rosettacode.org) - Hundred of solutions of tasks in almost every programming languages.
- [Competitive Programmer's Handbook](https://cses.fi/book.html) - Make you more competitive. The book itself is also competitive.

## Contributing

All contributions are welcome, including typo fix! Please read the [contrubuting](CONTRIBUTING.md) guideline first before starting your work.

## Contributors

- [@weihanglo](https://github.com/weihanglo)
- [@choznerol](https://github.com/choznerol)
- [@henry40408](https://github.com/henry40408)

## License

This project is released under different licenses based on type of the content.

- Source code is licensed under [The MIT License (MIT)](LICENSE).
- Articles and creative works are licensed under [Creative Commons 4.0 (CC BY-NC-SA 4.0)](https://creativecommons.org/licenses/by-nc-sa/4.0/).

Copyright © 2017 - 2020 Weihang Lo
