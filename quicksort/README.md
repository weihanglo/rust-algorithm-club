# Quicksort

Quicksort 是一個 divide and conquer 的演算法，將

- Divide and conquer algorithm
- Comparison sort
- Not stable
- Additional space required

## Algorithm

1. **選擇 Pivot**：在序列中任意選擇一個元素，稱為 **Pivot**。
2. **分割序列**：將序列依序分為三堆「新序列」，_比 pivot 小_、_pivot_ 本身，以及_比 pivot 大_。
3. **遞迴**：分別將_比 pivot 小_，以及_比 pivot 大_ 兩「新序列」重複上述步驟，直到新序列的長度小於等於 1，無法繼續分割為止。

> _pivot_ 本身在「分割序列」步驟便已移至排序後的最終位置。

Quick sort 僅有「**選擇 Pivot**」與「**分割序列**」兩步驟，不同的實作會有非常不同的效能，也順理成章成為最佳化的探討方向之一。

## Explanation

這裡介紹幾個常見的分割序列方案（partition scheme）。

- Lomuto partition scheme
- Hoare partition scheme

### Lomuto partition scheme

Lomuto 是最簡單最容易理解的 partition scheme 之一，常被作為教材，我們就先從它下手吧。

給定一個序列，並選擇最後一個元素作為 pivot。

```
                                * -> pivot
[17, 20, 2, 1, 3, 21, 8, 3, 4, 9]
```

### Hoare partition scheme

## Performance

|              | Complexity                    |
| :----------- | :---------------------------- |
| Worst case   | `O(n^2)`                      |
| Best case    | `O(nlogn)` or `O(n)`          |
| Average case | `O(nlogn)`                    |
| Worst space  | `O(logn)` or `O(n)` auxiliary |
