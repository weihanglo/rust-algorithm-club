# Rust Algorithm Club

> ### 🚧 🚧 This repo is under construction. Most materials are written in Chinese. [Check it out here](https://rust-algo.club) if you are able to read Chinese.

[![Rust Edition](https://img.shields.io/badge/Rust_Edition-2018-green.svg)](https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html) 
[![Build Status](https://travis-ci.com/weihanglo/rust-algorithm-club.svg?token=jBygxQ3kLkkfxSeAJnP2&branch=master)](https://travis-ci.com/weihanglo/rust-algorithm-club)

This repository was originally inspired by [Swift Algorithm Club][swift-algorithm-club]. All algorithms here would be explained and implemented in [Rust programming language][rust]! Source code is on [GitHub][source-code] and we are looking forward to your contributions.

[swift-algorithm-club]: https://github.com/raywenderlich/swift-algorithm-club
[rust]: https://www.rust-lang.org/
[source-code]: https://github.com/weihanglo/rust-algorithm-club

## General Concepts

- [Asymptotic Notation](src/concepts/asymptotic-notation)

## Algorithms

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

## Data Structures

### Stack and Queue

- [🚧 Stack](src/collections/stack_queue/stack.md)
- [🚧 Queue](src/collections/stack_queue/queue.md)
- [🚧 Deque](src/collections/stack_queue/deque.md)

### Linked List

[Introduction of linked list](src/collections/linked_list)

- [Singly linked list](src/collections/linked_list/singly.md)
- [🚧 Doubly linked list](src/collections/linked_list/doubly.md)
- [🚧 Circular linked list](src/collections/linked_list/circular.md)

### Associative Container

[Introduction of associative container](src/collections/map)

- [Hash map](src/collections/map/hash_map.md)
- [🚧 Ordered map](src/collections/map/ordered_map.md)
- [🚧 Multimap](src/collections/map/multimap.md)
- [🚧 Set](src/collections/map/set.md)

## Learning Resources

For learning more, you may check out following online resources:

- [VisuAlgo](https://visualgo.net/) - Probably the best algorithms visualization website.
- [Big-O Cheat Sheet](http://bigocheatsheet.com/) - Comprehensive Big-O notation cheat sheet.
- [Rosetta Code](http://rosettacode.org) - Hundred of solutions of tasks in almost every programming languages.
- [Competitive Programmer's Handbook](https://cses.fi/book.html) - Make you more competitive. The book itself is also competitive.

## Maintainers

- [@weihanglo](https://github.com/weihanglo)

## License

This project is released under different licenses based on type of the content.

- Source code is licensed under [The MIT License (MIT)](LICENSE).
- Articles and creative works are licensed under [Creative Commons 4.0 (CC BY-NC-SA 4.0)](https://creativecommons.org/licenses/by-nc-sa/4.0/).

Copyright © 2017 - 2018 Weihang Lo
