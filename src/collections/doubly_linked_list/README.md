# 雙向鏈結串列 Doubly linked list

```
           head                      tail
            |                         |
            v                         v
        +--------+   +--------+   +--------+
        |        |-->|        |-->|        |--> NULL
        | node 0 |   | node 1 |   | node 2 |
NULL <--|        |<--|        |<--|        |
        +--------+   +--------+   +--------+
```

## 術語

## 效能

## 操作

雙向鏈結串列的基本操作如下：

- `new`：初始化一個空串列。
- `push_front`：新增節點到開頭的位置。
- `push_back`：新增節點到最末端的位置。
- `pop_front`：將開頭第一個節點移除。
- `pop_back`：將最末端的節點移除。
- `insert`：在指定索引位置插入一個新節點，並將大於等於該索引位置的節點往後移動。
- `remove`：移除任意索引下的節點。
- `clear`：清除所有節點。
- `is_empty`：檢查串列是否沒有任何節點。
- `reverse`：反轉整個串列（head 變成 tail）。

## 變形

XOR linked list

## 參考資料

- [Wiki: Doubly linked list](https://en.wikipedia.org/wiki/Doubly_linked_list)

https://github.com/steveklabnik/indexlist
