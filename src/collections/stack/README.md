# 堆疊 Stack

![Stack - Wikipedia](https://upload.wikimedia.org/wikipedia/commons/b/b4/Lifo_stack.png)

堆疊是一個具有*後進先出* LIFO 特性的資料結構。以從 Wikipedia 借來的上圖為例，在第五張圖的狀況下，如果要取得 2，就必須先把 3、4、5 都退出堆疊。

堆疊的底部與頂部都是抽象的概念，頂部是資料被加入、移除、較為繁忙的那一端，底部即另一端。

堆疊的空間可能是有限的，亦即也有可能實現空間無限的堆疊。有鑑於有限空間的堆疊較為常見，我們選擇實作空間有限的堆疊。

堆疊 stack 有兩種實作方式：陣列 array 與鏈結串列 linked list，在此選擇以類似陣列的 Vector 實現。

## 架構設計

```rust
pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}
```

`maxsize` 用於模擬堆疊空間有限的特性；`items` 負責保存加入堆疊的資料。

在此刻意將 `maxsize`、`items` 定義為 private member，避免外部直接存取。

## 基本操作

* `with_capacity`：定義一個空間有限的堆疊。
* `push`：將新資料加入資料結構。
* `pop`：將最新加入的資料移出資料結構。
* `size`：（選用）取得堆疊的大小。
* `peek`：（選用）在不將資料退出堆疊的情況下偷看最後加入堆疊的資料。

### 定義一個空間有限的堆疊

```rust
{{#include mod.rs:with_capacity}}
```

初始化一個帶有預先分配空間 Vector 的堆疊。

⚠ 注意，即使預先分配了有限的空間，Rust 的 vector 在空間已滿的情況下會重新分配。假設一開始為 vector 分配了 10 單位的空間，在將第 11 筆資料插入 vector 前，vector 在記憶體的空間將被重新分配，以容納這第 11 筆資料。為了模擬堆疊空間有限的特性，我們會在 `push` 的操作動點手腳。

### 將新資料加入資料結構

```rust
{{#include mod.rs:push}}
```

由於 `push` 操作會改變 `items`，因此需要堆疊的 mutable reference。由於 Rust 的 vector 有重新分配的特性，在將資料正式加入堆疊之前，必須先檢查堆疊初始化時設定的空間是否已經被塞滿了。如果結果為是，則拒絕將資料加入堆疊。

### 將最新加入的資料移出資料結構

```rust
{{#include mod.rs:pop}}
```

堆疊有可能是空的，在此以 `Option` 表現這個情況。如果針對一個空堆疊進行 `pop` 操作，將會得到 `None`。

### 取得堆疊的大小

```rust
{{#include mod.rs:size}}
```

一個空堆疊的大小是 0，加入一筆資料後是 1⋯⋯以此類推。注意容量 capcity 與大小 size 是兩個不同的概念。容量是這個堆疊最多可以塞下多少資料，大小則是這個堆疊已經被塞入了多少資料。由於 `push` 的檢查機制，堆疊的大小永遠不會超過 `maxsize`。

### 在不將資料退出堆疊的情況下偷看最後加入堆疊的資料

```rust
{{#include mod.rs:peek}}
```

與 `pop` 操作類似，但不會對堆疊造成任何影響。如果偷看的是一個空堆疊，會得到 `None`。

## 效能

| Operation | Best Complexity | Worst Complexity |
| --- | --- | --- |
| push (insert) | O(1) | O(1) |
| pop (delete) | O(1) | O(1) |

無論堆疊大小如何變化，`push` 與 `pop` 的效能都不會被影響。

## 參考資料

* [Stack (abstract data type)
](https://en.wikipedia.org/wiki/Stack_\(abstract_data_type\))
* [Big-O Algorithm Complexity Cheat Sheet](http://bigocheatsheet.com/)
