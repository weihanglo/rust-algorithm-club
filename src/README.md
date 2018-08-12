# Rust Algorithm Club

[![Rust Edition](https://img.shields.io/badge/Rust_Edition-2018-green.svg)](https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html) 
[![Build Status](https://travis-ci.com/weihanglo/rust-algorithm-club.svg?token=jBygxQ3kLkkfxSeAJnP2&branch=master)](https://travis-ci.com/weihanglo/rust-algorithm-club)

本專案受 [Swift Algorithm Club][swift-algorithm-club] 啟發，專案中的演算法皆使用 [Rust 程式語言][rust]撰寫說明與實作！

[swift-algorithm-club]: https://github.com/raywenderlich/swift-algorithm-club
[rust]: https://www.rust-lang.org/

## 基礎概念

- [漸進符號 Asymptotic Notation](concepts/asymptotic-notation)

## 演算法

### 排序

簡單排序：

- [插入排序 Insertion sort](sorting/insertion_sort)
- [選擇排序 Selection sort](sorting/selection_sort)
- [氣泡排序 Bubble sort](sorting/bubble_sort)
- [希爾排序 Shellsort](sorting/shellsort)

高效排序：

- [堆積排序 Heapsort](sorting/heapsort)
- [快速排序 Quicksort](sorting/quicksort)
- [合併排序 Mergesort](sorting/mergesort)

混合排序（更高效）：

- 🚧 [內省排序 Introsort](sorting/introsort)
- 🚧 [自適應的合併排序 Timsort](sorting/timsort)
- 🚧 [模式消除快速排序 Pdqsort](sorting/pdqsort)

特殊排序：

- [計數排序 Counting sort](sorting/counting_sort)
- [桶排序 Bucket sort](sorting/bucket_sort)
- [基數排序 Radix sort](sorting/radix_sort)

## 資料結構

### 堆疊與佇列

- [🚧 堆疊 Stack](collections/stack_queue/stack.md)
- [🚧 佇列 Queue](collections/stack_queue/queue.md)
- [🚧 雙端佇列 Deque](collections/stack_queue/deque.md)

### 鏈結串列

- [鏈結串列概述](collections/linked_list/README.md)
- [單向鏈結串列 Singly linked list](collections/linked_list/singly.md)
- [🚧 雙向鏈結串列 Doubly linked ilst](collections/linked_list/doubly.md)
- [🚧 循環鏈結串列 Circular linked ilst](collections/linked_list/circular.md)

## 學習資源

有許多優秀的網站與學習資源，分享給大家學習演算法。

- [VisuAlgo](https://visualgo.net/) - 也許是最好的演算法視覺化專案。
- [Big-O Cheat Sheet](http://bigocheatsheet.com/) - 最全面的 Big O cheat sheet。
- [Rosetta Code](http://rosettacode.org) - 使用各種程式語言，解答上百種不同程式問題。
- [Competitive Programmer's Handbook](https://cses.fi/book.html) - 讓你更有競爭力。這書本身也很有競爭力。

## 維護者

- [@weihanglo](https://github.com/weihanglo)

## 授權條款

本專案分為兩部分授權：

- 程式碼與函式庫依據 [The MIT License (MIT)](LICENSE) 授權條款發佈。
- 文章與相關著作依據 [Creative Commons 4.0 (CC BY-NC-SA 4.0)](https://creativecommons.org/licenses/by-nc-sa/4.0/) 授權條款發佈。

Copyright © 2017 - 2018 Weihang Lo
