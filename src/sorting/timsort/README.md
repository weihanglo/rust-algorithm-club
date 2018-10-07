# 自適應合併排序 Timsort

***********
## Timsort 與 adaptive mergesort 的差異
***********

Timsort 融合了 [Merge sort](../mergesort) 的效能與 [Insertion sort](../insertion_sort) 對簡潔，是一個高度最佳化的混合排序法。Tim Peter 在 2002 將 Timsort 實作在 Python 語言中。Timsort 是設計來處理真實世界資料的排序法，能更有效率地處理部分排序（partially sorted）的資料。

Timsort 將真實世界的資料中連續已排好序的元素，稱之為 **natural runs**。原理是先搜尋這些排好序的元素，並將之組合為分區（run），再依據各分區的特性，使用不同的演算法排序，將分區合併。

Timsort 的特性如下：

- **自適應排序**：可根據當前資料排序情形加速排序，資料越接近排序完成，效率越高。
- **混合式排序**：結合 Merge sort 與 Insertion sort。
- **穩定排序**：相同鍵值的元素，排序後相對位置不改變。
- **額外空間**：計算排序時，需要額外的儲存空間。
- **併發性**：Timsort 將待排序的資料分區，每一分區可獨立併發排序。

## 步驟

需要一個額外的暫時陣列來儲存 $N / 2$ 個指標。

對於已排序的資料，可以防止退化到 O(n lg n)

1. Runs

**Search for runs.**

在資料中搜尋 natural runs，一個 natural run 至少需有連續兩個元素為遞增／非遞減（non-descending，各元素皆大於等於前者）

$$ a0 \leq a1 \leq a2 \leq ...$$

或嚴格遞減（strictly descending，各元素皆小於前者），

$$ a0 > a1 > a2 > ...$$

由於主要的操作會反轉每個 run，因此嚴格的遞減定義可確保排序法的穩定性。

**Minimum size (minrun)**

每一個 run 可能有不同的長度，Timsort 針對不同長度有對應的排序策略，當 run 的長度小於 minrun 時，timsort 會退化成使用穩定的 binary insertion sort 排序；反之使用 merge sort。

當 runs 的長度相近，merging 最有效率，所以要使 runs 長度一致。 <-- maintain merge balance

Runs 最後會被 push to stack

**Computing minrun**

如果 $N < 64$，那麼 minrun 會設置為 $N$，也就是說，整個 array 都會以 binary insertion sort 排序。


2. **Merging**

Concurrently merges runs with merge sort.

除了 timsort 優化的部分，其他 runs 都是兩兩合併，以確保穩定性與合併平衡性（merge balance）。

非連續的 runs 不能合併。否則會違反 stability。

欲達成合併平衡性，Timsort 會維持三個 run 在 stack 上，並維持以下的不變條件（invariants）：

（$A$、$B$、$C$ 為尚未合併的 run 的序列長度）

- i. $|A| > |B| + |C|$
- ii. $|B| > |C|$

如果違反其中的 invariant，$B$ 將會與 $A$ 或 $C$ 中較小者合併，並再次確認 invariants，直到 invariants 成立，下一個 run 就此形成。舉例來說：

若 stack 上的三個 run 長度如下

```
A:30 B:20 C:10
```

則 B 與 C 合併，並留在 stack 上：

```
A:30 BC:30
```

又或者如下面的例子：

```
A:500 B:400 C:1000
```

則 A 與 B 合併，並留在 stack 上：

```
AB:900 C:1000
```

**Individual merges**

**Galloping mode**

## 說明

## 效能

**Exponential search** -> gallop

**Timsort 可以降低比較的次數，因為 Python 的比較很貴！！**

|              | Complexity       |
| :----------- | :--------------- |
| Worst        | $O(n \log n)$    |
| Best         | $O(n)$           |
| Average      | $O(n \log n)$    |
| Worst space  | $O(n)$ auxiliary |

## 實作

## 後記

開放原始碼 Rust 社群討論串非常值得閱讀！

## 參考資料

- [Wiki: Timsort](https://en.wikipedia.org/wiki/Timsort)
- [Rust sort implementation](https://github.com/rust-lang/rust/pull/38192)
- [Python sort implementation](https://github.com/python/cpython/blob/3.8/Objects/listsort.txt)
- [David R. MacIver - Understanding timsort, Part 1: Adaptive Mergesort](https://www.drmaciver.com/2010/01/understanding-timsort-1adaptive-mergesort/)
- [Taipei.py - Timsort](https://youtu.be/uVWGZyekGos)
