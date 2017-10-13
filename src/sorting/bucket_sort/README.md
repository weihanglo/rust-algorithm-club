# Bucket sort

[Bucket sort][wiki-bucket-sort]，是一個非[比較排序][wiki-comparison-sort]。原理是建立一些桶子，每個桶子對應一資料區間，在將待排序資料分配到不同的桶中，桶子內部各自排序。  
由於並非比較排序，使用 Bucket sort 需要事先知道資料的範圍與分佈，才能決定桶子對應的區間。

Bucket sort 基本特性如下：

- 又稱 **bin sort**。
- **穩定排序**：相同鍵值的元素，排序後相對位置不改變。
- **分配式排序**：不透過兩兩比較，而是分析鍵值分佈來排序。特定情況下可達線性執行時間。
- **預期**：資料為**均勻分佈**。

## Algorithm

假設要排序 $n$ 個元素的陣列，這些元素的值平均散落在某個**已知的預期範圍內**，例如 1 到 100。

1. **Create buckets**：建立 $k$ 個桶子（bucket）的陣列。每個桶子**對應預期範圍的某區間**，如第一個桶子放 1 到 10，第二個放 11 到 20。
2. **Scatter**：將每個元素依照該值放入對應的桶子中。
3. **Inner sort**：排序所有非空的桶子。
4. **Gather**：依序走訪所有桶子，將桶內的元素放回原本的陣列中。

## Explanation

以下用 ASCII diagram 視覺化解釋：

這裡有一些整數，落在 1 至 100 之間。我們有 $n = 10$ 的陣列要排序。

```
Original array

+-------------------------------------------------+
|  6 | 28 | 96 | 14 | 74 | 37 |  9 | 71 | 91 | 36 |
+-------------------------------------------------+
```

**1. Create buckets**：建立一定數量的桶子，這裡我們建立與原始陣列相同數量的桶子（10）。每個桶子對應 $n - 1 * 10$ 到 $n * 10$ 的區間。

```
Bucket array

+-------------------------------------------------+
|    |    |    |    |    |    |    |    |    |    |
+-------------------------------------------------+
  ^    ^
  |    |
  |    |
  |    holds values in range 11 to 20
  holds values in range 1 to 10
```

**2. Scatter**：將原始陣列中的元素，放入對應的桶中。

```
Bucket array

  6,9  14   28   37,36               74,71     96,91
  |    |    |    |                   |         |
+-v----v----v----v-------------------v---------v--+
|    |    |    |    |    |    |    |    |    |    |
+-------------------------------------------------+
```

**3. Inner sort**：排序所有非空桶子中的元素，桶內排序可用任意排序法，通常選用「insertion sort」。

```
Bucket array

  sort sort sort sort                sort      sort
  ---  --   --   -----               -----     -----
  6,9  14   28   36,37               71,74     91,96
  |    |    |    |                   |         |
+-v----v----v----v-------------------v---------v--+
|    |    |    |    |    |    |    |    |    |    |
+-------------------------------------------------+
```

**4. Gather**：排序完後，再將所有桶中元素依序放回原始的陣列。
```
Original array
+-------------------------------------------------+
|  6 |  9 | 14 | 28 | 36 | 37 | 71 | 74 | 91 | 96 |
+-------------------------------------------------+
```

## Performance

|              | Complexity               |
| :----------- | :----------------------- |
| Worst        | $O(n^2)$                 |
| Best         | $\Omega(n + k)$          |
| Average      | $\Theta(n + k)$          |
| Worst space  | $O(n + k)$ auxiliary |

> $k$ = 桶子的數量（number of buckets）
> $n$ = 資料筆數


### Worst case

Bucket sort 是一個分配式排序法，對資料分佈有既定的預期：「**所有元素平均分佈在每個 bucket 的區間內**」。可想而知，最差的狀況是所有元素都聚集（clustering）在同一個 bucket 中，整個 bucket sort 的會退化成單一一個 inner sort 的複雜度。而桶內排序通常選用 insertion sort（最差 $O(n^2)$），所以最差的時間複雜度為「$O(n^2)$」。

### Best case

最佳的狀況則是完全符合預期的平均分佈，一個蘿蔔一個坑，每個桶內排序的最佳時間複雜度為 $O(n / k)$，再乘上桶子總數 $k$，僅需 $O(k \cdot (n / k)) = O(n)$。計算結果看起來非常合理，但實際上最佳時間複雜度為 $O(n + k)$，為什麼呢？

無庸置疑，桶內排序最佳時間複雜度為 $O(n / k)$，但別忘了這是省略常數項過後式子，進行符號運算時，較精確的表達是 $c_0 O(n / k) + c_1$，對於實作層面的常數 $c_0$ 和 $c_1$ 則予以保留。

當我們乘上 $k$，試著算出總運算量時，

$$k \cdot (c_0(n / k) + c_1) $$

會得到：

$$ c_0n + c_1k $$

可以得知，整個計算與 $k$ 有關，所以需要耗時 $O(n + k)$。

撇開數學，我們從 pseudo code 來看。最佳情況下，將所有元素蒐集回陣列的步驟（Gather）如下：

```
for (each bucket b in all k buckets)
  for (each element x in b)
    append x to the array
```

最外層的迴圈依桶子數 $k$ 而定，至少需要執行 $k$ 次，複雜度為 $O(k)$。內層的迴圈則是每個桶內的元素都會執行，而我們的資料時均勻分布，因此執行時間與元素總數 $n$ 相關，為 $O(n)$。兩者加起來就是我們所說的 $O(n + k)$ 的最佳複雜度。

**那 $k$ 究竟會是多少，影響會比 $n$ 大嗎？**

端看桶子總數而定，若桶子總數很大，比元素個數 $n$ 大得多，則桶子總數對執行時間的影響恐較劇烈，就算大多數為空桶子，仍須挨家挨戶查看是否需要執行桶內排序。

### Space Complexity

Bucket sort 須額外建立 $k$ 個桶子，每個桶子需要配置長度為 $n$ 的 array，因此空間複雜度為 $O(n \cdot k)$。如果以 dynamic array 實作 bucket，並考慮平攤分析（Amortized analysis），則空間複雜度降至 $O(n + k)$，這也是大多數人接受的分析結果，畢竟不會有人無聊到預先配置 $n \cdot k$ 個 empty bucket。

## Implementation

## Reference

- [Wiki: Bucket sort][wiki-bucket-sort]
- [Wiki: Amortized analysis][wiki-amortized-analysis]
- [How is the complexity of bucket sort is O(n+k) if we implement buckets using linked lists?][stackoverflow-bucket-sort-analysis]
- [Bucket sort in Rust][bucket-sort-in-rust]


[wiki-bucket-sort]: https://en.wikipedia.org/wiki/Bucket_sort
[wiki-amortized-analysis]: https://en.wikipedia.org/wiki/Amortized_analysis
[wiki-comparison-sort]: https://en.wikipedia.org/wiki/Comparison_sort
[stackoverflow-bucket-sort-analysis]: https://stackoverflow.com/questions/7311415
[bucket-sort-in-rust]: https://codereview.stackexchange.com/questions/145113/bucket-sort-in-rust

