# Selection sort

Selection sort 是最易實作的入門排序法之一，會將資料分為 sorted pile 與 unsorted pile，每次從 unsorted pile 尋找最大／最小值，加入 sorted pile 中。

Selection sort 的特性如下：

- 最簡單的排序法之一。
- 對小資料序列排序效率較高。
- **不穩定排序**：排序後，相同鍵值的元素相對位置可能改變。
- **原地排序**：不需額外花費儲存空間來排序。

## Algorithm

1. 將資料分為 sorted pile 與 unsorted pile。
2. 從 unsorted pile 尋找最小值。
3. 置換該最小值元素與 unsorted pile 第一個元素。
4. 重複步驟 2 - 3，直到排序完成。

> 注意，這個 naïve 的 selection sort 實作為**不穩定排序**。

![](https://upload.wikimedia.org/wikipedia/commons/9/94/Selection-Sort-Animation.gif)

## Explanation

為什麼 naïve 的 selection sort 會是不穩定排序？

假定有一個序列要遞增排序，其中有重複的 `2` 元素，我們將其標上 `2a`、`2b` 以利辨識。

```
[2a, 3, 4, 2b, 1]
```

開始迭代找出最小值並指環。

```bash
 *             *
[1, 3, 4, 2b, 2a] # 1. 置換 2a, 1

     *     *
[1, 2b, 4, 3, 2a] # 2. 置換 3, 2b

        *       *
[1, 2b, 2a, 3, 4] # 3. 置換 4, 2a
```

有沒有發現，`2a` 與 `2b` 的相對順序顛倒了呢？

首先，回想一下穩定排序的定義：**相同鍵值的元素，排序後相對位置不改變。**

問題出在 naïve selection sort 是以置換的方式排序每次迭代的最小值。若我們將置換（swap）改為插入（insert），那麼 selection sort 就會是穩定排序，但相對地，需要位移剩餘未排序的元素，除非使用 linked list 或其他提供 $O(1)$ insertion 的資料結構，不然就會多出額外 $O(n^2)$ 的寫入成本。


## Performance

|              | Complexity       |
| :----------- | :--------------- |
| Worst        | $O(n^2)$         |
| Best         | $\Omega(n^2)$    |
| Average      | $\Theta(n^2)$    |
| Worst space  | $O(1)$ auxiliary |

對於接近排序完成的序列，selector sort 並無法有自適應的方式加快排序迭代。第一個元素要做 $n - 1$ 次比較，第二個 $n - 2$ 次，總比較次數如下：

$$ (n -1) + (n-2) + \cdots + 1 = \sum_{i=1}^{n-1} i = \frac{n(n - 1)}{2}$$

因此無論序列是否排序完成，selection sort 仍需執行 $n^2$ 次比較，時間複雜度為 $O(n^2)$。

## Implementation

## Variants

### Heapsort

[Heapsort][heapsort] 是一個高效的排序法，使用 selection sort 融合 [heap][wiki-heap] 這種半排序的資料結構，讓時間複雜度進化至 $O(n \log n)$。更多詳情可以參考[這篇介紹][heapsort]。

## Reference

- [Wiki: Selection sort][wiki-selection-sort]
- [Why Selection sort can be stable or unstable][why-selection-sort-can-be-stable-or-unstable]

[wiki-selection-sort]: https://en.wikipedia.org/wiki/Selection_sort
[why-selection-sort-can-be-stable-or-unstable]: https://stackoverflow.com/questions/20761396/
[wiki-heap]: https://en.wikipedia.org/wiki/Heap_(data_structure)
[heapsort]: ../heapsort/README.md
