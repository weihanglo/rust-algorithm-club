# 基數排序 Radix sort

如果你對 [Counting sort](../counting_sort) 與 [Bucket sort](../bucket_sort) 有認識，應該知道這兩個排序都能突破比較排序法複雜度 $O(n \log n) $ 限制的特殊排序法。[Radix sort][wiki-radix-sort] 同樣是個特殊的[整數排序法][wiki-integer-sorting]，效能同樣可達突破限制。差別在於，前兩者僅依據一個鍵值排序，而 Radix sort 則是依據多個鍵值排序。

舉例來說，欲排序一群範圍在 0 - 999 的整數，若以 Counting sort 排序，則需建立一個「1000 元素的陣列」來計算每個整數的出現次數；若使用以 10 為基數的 Radix sort，則僅需以個位數、十位數、百位數作為鍵值分別排序三次。通常 Radix sort 的排序副程式（Sorting subroutine）會選用 Counting sort 或 Bucket sort，而以 10 為基數的鍵值範圍僅 0 - 9，這種小範圍整數非常適合 Counting sort 作為排序副程式，節省了配置 `int arr[1000]` 的 count array 的時空間。

Radix sort 基本特性如下：

- **整數排序法**：以整數作為排序的鍵值。
- **分配式排序法**：不透過兩兩比較，而是分析鍵值分佈來排序。特定情況下可達線性執行時間。
- **穩定性**：採用 LSD 的 Radix sort 屬穩定排序法（Stable sort）；透過優化，採用 MSD 也可以是穩定排序法。

[wiki-integer-sorting]: https://en.wikipedia.org/wiki/Integer_sorting

## 步驟

常見的 Radix sort 依據整數的每個位數來排序，依照位數排序的先後順序，可分為兩種：

- **Least significant digit (LSD)**：從最低有效鍵值開始排序（最小位數排到大）。
- **Most significant digit (MSD)**：從最高有效鍵值開始排序（最大位數排到小）。

簡單的 LSD Radix sort 步驟如下：

1. **LSD of each key**：取得每個資料鍵值的最小位數（LSD）。
2. **Sorting subroutine**：依據該位數大小排序資料。
3. **Repeating**：取得下一個有效位數，並重複步驟二，至最大位數（MSD）為止。


而 MSD Radix sort 的步驟相似，但取得資料鍵值的方向相反。

1. **MSD of each key**：取得每個資料鍵值的最大位數（MSD）。
2. **Sorting subroutine**：依據該位數大小排序資料。
3. **Repeating**：取得下一個有效位數，並重複步驟二，至最小位數（LSD）為止。

> 由於 MSD Radix sort 先排序最大位數，會出現 **8 > 126** 的結果，這種順序通常稱為 [Lexicographical order][wiki-lexicographical-order]，有如字典一般，越前面的字母排序權重越重，也因此，基本版的 MSD Radix sort 並非穩定排序法。

[wiki-lexicographical-order]: https://en.wikipedia.org/wiki/Lexicographical_order

## 說明

我們選用 LSD Radix sort 示範，並且為了增加可讀性，將基數設為 10。需注意在現實場景中，有時使用 bytes 作為基數可能更適合。

待排序的數列如下。

```
[170, 45, 75, 90, 802, 2, 24, 66]
```

> Radix sort 的排序副程式，通常選用 counting sort 或 bucket sort，因此，開始排序前，需建立供其使用的 buckets（或 count array）。這屬於其他排序法的範疇，有興趣可看 [Counting sort](../counting_sort) 或 [Bucket sort](../bucket_sort)。

首先，從最小位數開始排序。
注意，同樣鍵值的資料，相對位置不會改變（穩定排序）。

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

## 效能

|              | Complexity   |
| ------------ | ------------ |
| Worst        | $O(dn) $ |
| Best         | $O(dn) $ |
| Average      | $O(dn) $ |
| Worst space  | $O(d + n) $ auxiliary |

> $n $：資料筆數。  
> $d $：number of digit，資料中最多有幾個位數（或鍵值）。  
> $k $：基數，就是一個位數最多有幾種可能的值。

### Time complexity

欲分析 Radix sort 的時間複雜度，我們可以逐一擊破，先從排序副程式開始分析。

Radix sort 的 subroutine 通常採用 Counting sort 或 Bucket sort，因此每個 subroutine 的複雜度為 $O(n + k) $， $k $ 為 key 的範圍，以 10 為基數，就是 0 - 9 之間 $k = 10 $。

再來，我們分析整個主程式，Radix sort 每個位數各需排序一次，若最多位數的資料有 $d $ 位數，時間複雜度需乘上 $d $，為 $O(d (n + k)) $，那這個 $k $ 是否可以捨去呢？

分析 Counting sort 或 Bucket sort 時，範圍 $k $ 會隨輸入資料而變化，若 $k $ 過大，對複雜度的影響甚至會超過 $n $，因此分析複雜度時無法將 $k $ 捨去。而在 Radix sort， $k $ 通常為一個已知的常數，例如以 bytes 為基數 $k = 8 $， $k $ 可以捨去。最後可得 Radix sort 的時間複雜度為 $O(d \cdot n) $。

### Space complexity

Radix sort 的空間複雜度同樣取決於排序副程式，Counting sort 與 Bucket sort 的空間複雜度皆為 $O(n \cdot k) $。Radix sort 的 $k $ 是常數，予以捨去。再乘上 $d $ 個位數，最差的空間複雜度為 $O(d \cdot n) $。

## 實作

這裡示範實作以 10 為基數，用來排序非負整數的 Radix sort。

首先，我們的排序副程式使用 Counting sort。

```rust
// 0. Include counting sort.
use ::sorting::counting_sort;
```

再來，就是 Radix sort 本體了。為了凸顯 Radix sort 的概念，簡化了函式參數數量，除去泛型宣告，並將基數選擇寫死在函式裡。

```rust
pub fn radix_sort(arr: &mut [i32]) {
    let radix = 10;             // 1
    let mut digit = 1;          // 2
    let max_value = arr         // 3
      .iter()
      .max()
      .unwrap_or(&0)
      .clone();
    while digit <= max_value {  // 4
        counting_sort(arr, 0, 9, |t| (t / digit % radix) as usize); // 5
        digit *= radix;         // 6
    }
}
```

1. 設定基數為 10。
2. 設定一個旗標，記錄當前在排序哪一位數，1 表示從最小位數（個位數）開始。
3. 先找到輸入資料的最大值，作為之後副程式迴圈結束的條件。尋找最大值的複雜度為 $O(n)$，因此不影響 Radix Sort 的複雜度。如果 `arr` 為空序列，則最大值設為 0，在第四步驟就會自動結束排序。
4. 判斷當前排序的位數是否大於最大值，例如當前排序百分位，`digit` 為 `100`，而最大值 `x` 為 26，則不需再排序百分位。
5. 使用 Counting sort 作為排序副程式，只需要有 0 - 9 十個桶子。而 `key` 參數則取出當前欲比較的位數。
6. 位數乘上基數，移至下一個位數繼續比較。

> 小提醒：這是簡單又容易理解的實作，相對有許多額外的運算開銷（例如尋找最大值）。實務上，會在對資料有些了解才採用 Radix sort，因此實作並不會這麼 naive。

## 參考資料

- [Wiki: Radix sort][wiki-radix-sort]
- [Princeton University DSA Course: Radix sort](https://www.cs.princeton.edu/~rs/AlgsDS07/18RadixSort.pdf)
- [ByVoid: 三種線性排序算法 計數排序、桶排序與基數排序](https://www.byvoid.com/zht/blog/sort-radix)

[wiki-radix-sort]: https://en.wikipedia.org/wiki/Radix_sort
