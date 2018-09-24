<p align="center">
  <img src="logo.svg" alt="logo">
<p>

# Rust Algorithm Club

歡迎來到 Rust 演算法俱樂部！本專案受 [Swift Algorithm Club][swift-algorithm-club] 啟發，專案中的演算法皆使用 [Rust 程式語言][rust]撰寫說明與實作！您可以在 [Rust Algorithm Club][main-site] 一站，依您的意願，挑選有興趣的演算法知識學習；若您夠大膽，推薦您閱讀[自動生成的 API 文件][generated-doc]，直接單挑程式原始碼。

本專案原始碼放在 [GitHub][source-code] 上，非常期待您的貢獻。

[![Rust Edition](https://img.shields.io/badge/Rust_Edition-2018-green.svg)][edition-guide]
[![Build Status](https://travis-ci.com/weihanglo/rust-algorithm-club.svg?token=jBygxQ3kLkkfxSeAJnP2&branch=master)][ci-status]
[![Documentation](https://img.shields.io/badge/doc-available-blue.svg)][generated-doc]

[swift-algorithm-club]: https://github.com/raywenderlich/swift-algorithm-club
[rust]: https://www.rust-lang.org/
[source-code]: https://github.com/weihanglo/rust-algorithm-club
[main-site]: https://rust-algo.club
[ci-status]: https://travis-ci.com/weihanglo/rust-algorithm-club
[generated-doc]: https://rust-algo.club/doc/rust_algorithm_club/
[edition-guide]: https://rust-lang-nursery.github.io/edition-guide/rust-2018

## 基礎概念

- [漸進符號 Asymptotic Notation](concepts/asymptotic-notation)

## 演算法

### 搜尋

- [線性搜尋 Linear search](searching/linear_search)
- [二元搜尋 Binary search](searching/binary_search)
- [🚧 內插搜尋 Interpolation search](searching/interpolation_search)
- [指數搜尋 Exponential search](searching/exponential_search)

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

- [🚧 堆疊 Stack](collections/stack)
- [🚧 佇列 Queue](collections/queue)
- [🚧 雙端佇列 Deque](collections/deque)

### 鏈結串列

[鏈結串列概述](collections/linked_list)

- [單向鏈結串列 Singly linked list](collections/singly_linked_list)
- [🚧 雙向鏈結串列 Doubly linked list](collections/doubly_linked_list)
- [🚧 循環鏈結串列 Circular linked list](collections/circular_linked_list)

### 關聯容器

[關聯容器概述](collections/associative-container)

- [雜湊表 Hash map](collections/hash_map)
- [🚧 有序映射表 Ordered map](collections/ordered_map)
- [🚧 多重映射表 Multimap](collections/multimap)
- [🚧 集合 Set](collections/set)

## 學習資源

有許多優秀的網站與學習資源，分享給大家學習演算法。

- [VisuAlgo](https://visualgo.net/) - 也許是最好的演算法視覺化專案。
- [Big-O Cheat Sheet](http://bigocheatsheet.com/) - 最全面的 Big O cheat sheet。
- [Rosetta Code](http://rosettacode.org) - 使用各種程式語言，解答上百種不同程式問題。
- [Competitive Programmer's Handbook](https://cses.fi/book.html) - 讓你更有競爭力。這書本身也很有競爭力。

## 如何貢獻

歡迎各式各樣的貢獻，修正錯字也行！開始動手之前，請先閱讀[貢獻指南](CONTRIBUTING.md)。

## 維護者

- [@weihanglo](https://github.com/weihanglo)

## 授權條款

本專案分為兩部分授權：

- 程式碼與函式庫依據 [The MIT License (MIT)](https://github.com/weihanglo/rust-algorithm-club/blob/master/LICENSE) 授權條款發佈。
- 文章與相關著作依據 [Creative Commons 4.0 (CC BY-NC-SA 4.0)](https://creativecommons.org/licenses/by-nc-sa/4.0/) 授權條款發佈。

Copyright © 2017 - 2018 Weihang Lo
