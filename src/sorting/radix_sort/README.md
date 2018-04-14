# Radix sort

[Radix sort][wiki-radix-sort] 是一個特殊的[整數排序法][wiki-integer-sorting]，透過 sorting subroutine 依序比較整數的每個位數（digit／radix）來排序。通常 subroutine 選用 Bucket sort 或 Counting sort。

Radix sort 與 [Counting sort](../counting_sort)、[Bucket sort](../bucket_sort) 都是能夠突破比較排序法 $O(n \log n)$ 限制的排序法。比起後兩者僅能比較單一 key 來排序，Radix sort 可以比較多個 key，並降低 subroutine 每個 key 的範圍。

例如：欲排序一群範圍在 0 - 999 的整數，若以 Counting sort 排序，則需建立一個「1000 元素的陣列」來計算每個整數的出現次數；若使用以 10 為基數的 Radix sort，則 key 的整數範圍僅 0 - 9，這種小範圍整數非常適合 Counting sort 作為 subroutine，也節省了配置 `int arr[1000]` 的 count array 的時空間。

Radix sort 基本特性如下：

- **整數排序法**：以整數作為排序的鍵值。
- **分配式排序法**：不透過兩兩比較，而是分析鍵值分佈來排序。特定情況下可達線性執行時間。
- **穩定性**：採用 LSD 的 Radix sort 屬穩定排序法（Stable sort）；透過優化，採用 MSD 也可以是穩定排序法。

## Algorithm

Radix sort 是比較整數的每個位數來排序，依照位數排序的先後不同，可分為兩種：

- Least significant digit (LSD)：從小位數排到大。
- Most significant digit (MSD)：從大位數排到小。

簡單的 LSD Radix sort 步驟如下：

1. **LSD of each key**：取得每個資料鍵值的最小位數（LSD）。
2. **Sorting subroutine**：依據該位數大小排序資料。
3. **Repeating**：取得下一個有效位數，並重複步驟二，至最大位數（MSD）為止。


而 MSD Radix sort 的步驟相似，但取得資料鍵值的方向相反。

1. **MSD of each key**：取得每個資料鍵值的最大位數（MSD）。
2. **Sorting subroutine**：依據該位數大小排序資料。
3. **Repeating**：取得下一個有效位數，並重複步驟二，至最小位數（LSD）為止。

> 由於 MSD Radix sort 先排序最大位數，會出現 **8 > 126** 的結果，這種順序通常稱為 [Lexicographical order][lexicographical-order]，有如字典一般，越前面的字母排序權重越重，也因此，基本版的 MSD Radix sort 並非穩定排序法。

## Explanation

我們選用 LSD Radix sort 示範，並且為了增加可讀性，將基數設為 10。需注意在現實場景中，有時使用 bytes 作為基數可能更適合。

有下列數列需要排序。

```
[170, 45, 75, 90, 802, 2, 24, 66]
```

> Radix sort 根據每個位數的排序 subroutine，通常選用 counting sort 或 bucket sort 作為 subroutine，因此，開始排序前，需建立供其使用的 buckets（或 count array）。這裡屬其他排序法的範疇，有興趣可看 counting sort 或 bucket sort。

首先，從最小位數開始排序。
注意，同樣鍵值的資料，相對位置不會改變（stable sort）。

```
   0   5   5   0    2  2   4   6
   _   _   _   _    _  _   _   _
[170, 45, 75, 90, 802, 2, 24, 66]

sort by rightmost digit -->

   0   0    2  2   4   5   5   6
   _   _    _  _   _   _   _   _
[170, 90, 802, 2, 24, 45, 75, 66]
```

再來，對下一個位數排序資料。位數不足的資料，予以補 0。

```
  7   9    0   0  2   4   7   6
  _   _    _      _   _   _   _
[170, 90, 802, 2, 24, 45, 75, 66]

sort by next digit -->

  0   0  2   4   6    7   7   9
  _      _   _   _    _   _   _
[802, 2, 24, 45, 66, 170, 75, 90]
```

最終，對最後一個位數進行排序。大功告成！

```
 8    0  0   0   0   1    0   0
 _                   _
[802, 2, 24, 45, 66, 170, 75, 90]

sort by leftmost digit -->

 0  0   0   0   0   0   1    8
                        _    _
[2, 24, 45, 66, 75, 90, 170, 802]
```

## Performance

|              | Complexity           |
| :----------- | :------------------- |
| Worst        | $O(dn)$              |
| Best         | $\Omega(dn)$         |
| Average      | $\Theta(dn)$         |
| Worst space  | $O(d + n)$ auxiliary |

> $n$：資料筆數。  
> $d$：number of digit，資料中最多有幾個位數。  
> $k$：基數，就是一個位數最多有幾種可能的值。

### Time complexity

欲分析 Radix sort 的時間複雜度，我們可以逐一擊破，先從 subroutine 開始分析。

Radix sort 的 subroutine 通常採用 Counting sort 或 Bucket sort，因此每個 subroutine 的複雜度為 $O(n + k)$，$k$ 為 key 的範圍，以 10 為基數，就是 0 - 9 之間 $k = 10$。

再來我們分析整個主程式，Radix sort 每個位數各需排序一次，若最多位數的資料有 $d$ 位數，時間複雜度需乘上 $d$，為 $O(d (n + k))$，那這個 $k$ 是否可以捨去呢？

分析 Counting sort 或 Bucket sort 時，範圍 $k$ 會隨輸入資料而變化，若 $k$ 過大，對複雜度的影響甚至會超過 $n$，因此分析複雜度時無法將 $k$ 捨去。而在 Radix sort，$k$ 通常為一個已知的常數，例如以 bytes 為基數 $k = 8$，$k$ 可以捨去。最後可得 Radix sort 的時間複雜度為 $O(d \cdot n)$。

### Space complexity

Radix sort 的空間複雜度同樣取決於 subroutine，Counting sort 與 Bucket sort 的空間複雜度皆為 $O(n \cdot k)$。Radix 的 $k$ 是常數，予以捨去。在乘上 $d$ 個位數，最差的空間複雜度為 $O(d \cdot n)$。

## Reference

- [Wiki: Radix sort][wiki-radix-sort]
- [Princeton Universioty DSA Course: Radix sort][dsa-radix-sort]

[wiki-radix-sort]: https://en.wikipedia.org/wiki/Radix_sort

[wiki-integer-sorting]: https://en.wikipedia.org/wiki/Integer_sorting
[wiki-lexicographical-order]: https://en.wikipedia.org/wiki/Lexicographical_order

[dsa-radix-sort]: https://www.cs.princeton.edu/~rs/AlgsDS07/18RadixSort.pdf
