# 線性搜尋 Linear Search

線性搜尋，又稱為循序搜尋（sequential search），是一個在序列中找尋目標的方法。正如字面上的意義，線性搜尋會按照順序迭代序列，挨家挨戶比對每個元素與目標值是否相等，若相等則停止迭代，並回傳搜尋所得結果。

線性搜尋乍看之下，是最簡單實作也最 naïve 的實作，效能應該不怎麼好。事實上，在資料量不多時（少於 100 個元素），線性搜尋的效能也不會太差，因為其他搜尋演算法可能需要建立特殊資料結構，就會導致時空間初始開銷暴增，複雜度的常數項成本變大。

## 效能

|              | Complexity |
| ------------ | ---------- |
| Worst        | $O(n)$     |
| Best         | $O(1)$     |
| Average      | $O(n)$     |
| Worst space  | $O(1)$     |

若序列中總共有 $n$ 個元素，則線性搜尋最差的狀況為元素不在序列中，就是全部元素都比較一次，共比較 $n - 1$ 次，最差複雜度為  $O(n)$。

## 實作

線性搜尋就是用一個 for-loop 解決。要注意的是，`T` 泛型參數至少要實作 `PartialEq` 才能比較。程式碼中使用了迭代器的 [enumerate][rust-iterator-enumerate]，建立一個新迭代器，每次迭代產生迭代次數與對應的值。

```rust
pub fn linear_search<T>(arr: &[T], target: &T) -> Option<usize>
    where T: PartialEq
{
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}
```

事實上，若利用 Rust 內建的 [`iterator.position`][rust-iterator-position]，程式碼也許會更簡潔。

```rust
pub fn linear_search<T>(arr: &[T], obj: &T) -> Option<usize>
    where T: PartialEq
{
    arr.iter().position(|x| x == obj)
}
```

[rust-iterator-enumerate]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
[rust-iterator-position]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position

## 參考資料

[Wiki: Linear search](https://en.wikipedia.org/wiki/Linear_search)
