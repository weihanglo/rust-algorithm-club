# 佇列 Queue

![Queue - Wiki](https://upload.wikimedia.org/wikipedia/commons/thumb/5/52/Data_Queue.svg/450px-Data_Queue.svg.png)

佇列是一個具有*先進先出* FIFO 特性的資料結構。從 Wikipedia 上的圖為例，一個資料從左邊進入佇列並從右邊離開，最先進入佇列的資料會最先被取出。

佇列常見實作方式有：陣列 array、鏈結串列 linked list、堆疊 stack。為了使概念容易理解，我們選擇以類似陣列的 Vector 實作。

## 基本操作

- `enqueue`：將新資料加入佇列
- `dequeue`：將最先放入的資料移出佇列
- `peek`：在不將資料移出佇列的情況下取得最先放入的資料
- `size`：取得佇列大小

## 架構設計

```rust
pub struct Queue<T> {
    items: Vec<T>,
}
```

以 `items` 保存加入佇列的資料。再來我們將 `items[0]` 當作 rear，並將 `items[length-1]` 當作 front，所以並不需要定義 front 和 rear 在 struct 裡面。

### 定義佇列

```rust
pub fn new() -> Self {
    Self {
        items: Vec::new(),
    }
}
```

初始化具有 Vector 的佇列。

### 將新資料加入佇列

```rust
pub fn enqueue(&mut self, item: T) -> bool {
    self.items.push(item);
    true
}
```

由於 `enqueue` 會改變 `items`，因此需要佇列的 mutable reference。再來，我們沒有限制佇列大小，全由 Rust 的 vector 自行分配空間，將新資料放到 Vector 的最後端。

### 將最先放入的資料移出佇列

```rust
pub fn dequeue(&mut self) -> Option<T> {
    match self.items.is_empty() {
        false => Some(self.items.remove(0)),
        true => None,
    }
}
```

Vector 有可能是空的，在移出資料之前需要檢查，然後移出 index 為零的資料，也就是最先放入的資料。

### 取得佇列大小

```rust
pub fn size(&self) -> usize {
    self.items.len()
}
```

取得 `items` 的大小。

## 參考資料

- [Queue (abstract data type)](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>)
