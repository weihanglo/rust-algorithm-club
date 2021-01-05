# 佇列 Queue

![Queue - Wiki](https://upload.wikimedia.org/wikipedia/commons/thumb/5/52/Data_Queue.svg/450px-Data_Queue.svg.png)

佇列是一個具有*先進先出* FIFO 特性的資料結構。從 Wikipedia 上的圖為例，一個資料從左邊進入佇列並從右邊離開，最先進入佇列的資料會最先被取出。

佇列常見實作方式有：陣列 array、鏈結串列 linked list。為了使概念容易理解，我們選擇以類似陣列的 Vector 實作。

## 架構設計

```rust
pub struct Queue<T> {
    items: Vec<T>,
}
```

以 `items` 保存加入佇列的資料。大部分用陣列實作的佇列可能會有 `front` 和 `rear` 兩個欄位負責保存指向佇列開頭和尾端的索引，作為佇列新增刪除資料的依據，但是透過 Rust 的 [`std::vec::Vec`]（線形動態成長的陣列容器），我們可以直接取得佇列第一以及最後一筆資料，所以這邊實作忽略這兩個欄位。
。
[`std::vec::Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html

## 基本操作

- `enqueue`：將新資料加入佇列
- `dequeue`：將最先放入的資料移出佇列
- `peek`：在不將資料移出佇列的情況下取得最先放入的資料
- `size`：取得佇列大小

### 定義佇列

```rust
pub fn new() -> Self {
    Self {
        items: Vec::new(),
    }
}
```

初始化具有 `Vec` 的佇列。

### 將新資料加入佇列

```rust
pub fn enqueue(&mut self, item: T) {
    self.items.push(item);
}
```

由於 `enqueue` 會改變 `items`，因此需要佇列的 mutable reference。再來，我們沒有限制佇列大小，全由 Rust 的 `Vec` 自行分配空間，將新資料放到 `items` 的最後端。

### 將最先放入的資料移出佇列

```rust
pub fn dequeue(&mut self) -> Option<T> {
    match self.items.is_empty() {
        false => Some(self.items.remove(0)),
        true => None,
    }
}
```

`items` 有可能是空的，在移出資料之前需要檢查，然後移出 index 為零的資料，也就是最先放入的資料。

### 取得佇列大小

```rust
pub fn size(&self) -> usize {
    self.items.len()
}
```

取得 `items` 的大小。

## 效能

| Operation | Best Complexity | Worst Complexity |
| --- | --- | --- |
| enqueue (insert) | O(1) | O(1) |
| dequeue (delete) | O(n)\* | O(n)* |

\*：注意，一般來說 `dequeue` 會選用 `O(1)` 的實作方式，這裡直接呼叫 [`Vec::remove`][] 會導致整個 `Vec<T>` 的元素向前位移一個，是較耗費計算資源的 `O(n)` 操作。

我們可以選用其他方式實作，例如用額外指標紀錄當前 head 所在位置的[雙端佇列 Deque]()，或是使用[單向鏈結串列 Singly linked list][] 實作，都能達到 `O(1)` 的時間複雜度。

[`Vec::remove`]: http://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove
[雙端佇列 Deque]: ../deque/
[單向鏈結串列 Singly linked list]: ../singly_linked_list/

## 參考資料

- [Queue (abstract data type)](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>)
