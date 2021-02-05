# 快速排序 Quicksort

Quicksort 是一個非常熱門且應用廣泛的排序法，相對簡單的實作就可達到 $O(n \log n) $ 的平均時間複雜度。雖然最差時間複雜度與 [bubble sort](../bubble_sort) 同為 $O(n^2) $，但這種情形非常少見。簡單的最佳化實作下，Quicksort 僅需 $O(\log n) $ 的額外儲存空間，比它的競爭對手 [mergesort](../mergesort) 來得節省。非常適合運用在真實世界中的排序法。

Quicksort 基本特性如下：

- 實作簡單，速度快。
- **不穩定排序**：排序後，相同鍵值的元素相對位置可能改變。
- **非原地排序**：除了資料本身，仍需額外花費儲存空間來排序。
- **分治演算法**：將主問題化作數個子問題，各個擊破。

## 步驟

Quicksort 是一個分治演算法（divide-and-conquer），不斷遞迴下列三個步驟：

1. **選擇 Pivot**：在序列中任意選擇一個元素，稱為 **Pivot**。
2. **分割序列**：將序列重新排序，分為兩部分，比 pivot 小 的元素置換到 pivot 之前，比 pivot 大的元素置換到 pivot 之後，而 pivot 本身會座落在它最終的正確位置。
3. **遞迴**：分別將「比 pivot 小」及「比 pivot 大」兩部分重複上述步驟，直到新序列的長度小於等於 1，無法繼續分割為止，此時排序完成。
 
### Lomuto partition scheme

為了達成上述條件，Quicksort 有許多不同的分割序列實作方案（partition scheme），其中以 Lomuto partition 最易理解，常被做為教材。

1. 以序列最後一個元素當做 pivot。
2. 利用兩個指標 `i` `j`，其中 `j` 從頭疊代整個序列
    - 若有序列第 j 個元素小於 pivot，則與第 i 個元素置換。
    - 第 i 個元素已落在小於 pivot 的範圍，將 i 指標往後移一個，處理下個元素。
3. 疊代完成後，小於 pivot 的元素全都置換至序列前端，此時將 pivot 與第 i 個元素置換，pivot 會剛好在最終正確位置上（符合不等式）。

ASCII 畫出來的分割圖如下：

```
[ values <= pivot | values > pivot | not checked yet | pivot ]
  low           i   i+1        j-1   j        high-1   high
```

- `arr[low...i]` 包含所有小於等於 pivot 的元素。
- `arr[i+1...j-1]` 包含所有大於 pivot 的元素。
- `arr[j...high-1]` 包含所有尚未疊代的元素。
- `arr[high]` pivot 本身。

## 說明

以 Lomuto partition scheme 為例，使用 ASCII diagram 解釋。

給定一個序列，並選擇最後一個元素作為 pivot，`i` `j` 指標則在第一個元素位置。

```
                      * -> pivot
[17, 20, 2, 1, 3, 21, 8]
 i
 j
```

第 `j` 個元素 17 大於 pivot 8，不置換。

```
17 > 8, no swap
                       * -> pivot
[17| 20, 2, 1, 3, 21, 8]
 i
 j
```

第 `j` 個元素 20 大於 pivot 8，不置換。

```
20 > 8, no swap
                      * -> pivot
[17, 20| 2, 1, 3, 21, 8]
 i
     j
```

第 `j` 個元素 2 小於 pivot 8，置換 `i` `j`。`i` 往後一個位置。

```
2 <= 8,
swap i, j
                      * -> pivot
[2, 20, 17| 1, 3, 21, 8]
 i->i
        j
```

第 `j` 個元素 1 小於 pivot 8，置換 `i` `j`。`i` 往後一個位置。

```
1 <= 8
swap i, j
                      * -> pivot
[2, 1, 17, 20| 3, 21, 8]
    i->i
            j
```

第 `j` 個元素 3 小於 pivot 8，置換 `i` `j`。`i` 往後一個位置。

```
3 <= 8
swap i, j
                      * -> pivot
[2, 1, 3, 20, 17| 21, 8]
       i->i
               j
```

第 `j` 個元素 21 大於 pivot 8，不置換。

```
21 > 8, no swap
                      * -> pivot
[2, 1, 3, 20, 17, 21| 8]
           i
                   j
```

最後，將 pivot 與第 `i` 個元素置換，此時 pivot 已在最終位置上，前面的元素皆小於等於 8，其後的元素皆大於 8。

```
swap pivot, i
          i    <->   * -> pivot
[2, 1, 3, 8, 17, 21, 20]
```

這樣就完成一次的 partition 了！

之後再遞迴分割 subarray 即可完成 Quicksort。

```
[2, 1, 3, 8, 17, 21, 20]
 #     #     *       *
 |     |     |       |
 -------     ---------
 quicksort    quicksort
```

## 效能

|              | Complexity         |
| ------------ | ------------------ |
| Worst        | $O(n^2) $      |
| Best         | $O(n \log n) $ |
| Average      | $O(n \log n) $ |
| Worst space  | $O(\log n) $ or $O(n) $ auxiliary |

### Time complexity

Quicksort 僅有「**選擇 Pivot**」與「**分割序列**」兩步驟，不同的實作的效能各異，也影響 Quicksort 的時間複雜度。

#### 最差情況

最差的分割序列狀況發生在挑選的 pivot 總是最大或最小值（或在 Lomuto partition 下，所有元素值都一樣）。由於 Lomuto 總是選擇最後一個元素作為 pivot，這種情形好發於已排序或接近排序完成的資料上。

而當每次的 partition 都是最不平衡的分割序列，就會產生最差時間複雜度的狀況。遞迴在序列長度等於 1 時停止，因此整個排序法的 call stack 需要 $n - 1 $ 的嵌套遞迴呼叫（nested call）；而第 $i $ 次分割會執行 $n - i $ 次基本操作（ $O(n) $），所以總共需執行

$$\sum_{i = 0}^n (n - i) = n^2 - \frac{n(n + 1)}{2}$$

次基本操作，最差時間複雜度為 $O(n^2) $。

#### 最佳情況

既然最差情況發生在 pivot 總選到最大或最小值，反之，最佳情況則發生在每次 pivot 都可以順利選到序列的中位數（median），如此一來，每次遞迴分割的序列長度都會減半（ $n / 2 $），call stack 的嵌套遞迴總共需要 $2 \log_2{n} $ 次，序列的長度就會減至 1，而每次分割同樣有 $O(n) $ 的複雜度，因此最佳情況為：

$$O(n \cdot 2 \log_2{n}) = O(n \log n)$$

### Space complexity

Quicksort 的空間複雜度取決於實作細節，由於**分割序列**步驟需 $O(1) $ 的空間複雜度，因此僅需分析遞迴式會在 call stack 產生多少 stack frame 即可。

[前面提及](#最差情況)，最 naïve 的 Lomuto partition 最糟糕的情形下，會產生 $n - 1 $ 個嵌套遞迴，也就是需額外使用 $O(n) $ 的空間儲存 call stack frame，但只要 compiler 有支援[尾端呼叫][tail-call]最佳化（tail-call optimization，TCO），Quicksort 很容易最佳化至 $O(\log n) $。

[tail-call]: https://en.wikipedia.org/wiki/Tail_call

## 實作

Quicksort 實作主要分為兩部分：遞迴，以及分割序列（partition）。

### Recursion

遞迴函式本身實作非常簡單，分別將小於 pivot 與大於 pivot 兩部分遞迴呼叫自身即可。

```rust
/// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {                               // 1
        let pivot = partition(arr, lo, hi);     // 2
        quicksort_helper(arr, lo, pivot - 1);   // 3
        quicksort_helper(arr, pivot + 1, hi);   // 4
    }
}
```

1. 利用 `lo` 與 `hi` 兩個指標決定每次的遞迴範圍，並在 `lo` 大於 `hi` 時停止遞迴，避免重複分割序列。
2. 分割序列步驟，回傳該序列範圍內 pivot 的 index。
3. 遞迴小於 pivot 的部分。
4. 遞迴大於 pivot 的部分。

> 這邊比較特別的是，`lo` 和 `hi` 兩個指標的型別為 `isize`，因為當 pivot 可能為 0，在第三步驟 - 1 時會產生型別錯誤，故為之。有任何更好的寫法歡迎提供！

由於外部不需知道排序法實作細節，我們將函式命名為 `quicksort_helper` ，對外再多封裝一層主函式 `quicksort_lomuto`，實作如下：

```rust
pub fn quicksort_lomuto(arr: &mut [i32]) {
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr, 0, hi);
}
```

### Partitioning

一般來說，分割序列的實作有下列兩個步驟：

- 選擇 pivot
- 遍歷序列置換元素

我們以 Lomuto scheme 實作 partition。

```rust
fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    // -- Determine the pivot --
    // In Lomuto parition scheme,
    // the latest element is always chosen as the pivot.
    let pivot = arr[hi as usize];               // 1
    let mut i = lo;

    // -- Swap elements --
    for j in lo..hi {                           // 2
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;                             // 3
        }
    }
    // Swap pivot to the middle of two piles.
    arr.swap(i as usize, hi as usize);          // 4
    i // Return the final index of the pivot
}
```

1. Lomuto scheme 選擇 pivot 的方式很直接，就是選擇最後一個元素。
2. 利用 `i`、`j` 兩個指標疊代指定的序列範圍，若第 j 個值小於 pivot 時，則於第 i 個元素置換。
3. `i` 指標加一，繼續處理下個元素。
4. 最後置換第 i 個元素於 pivot，此時 pivot 已落在最終正確的位置。

## 最佳化與變形

Quicksort 有數個方向可以探討最佳化：

- [降低額外空間複雜度](#降低額外空間複雜度)
- [選擇 Pivot 的方法](#選擇-pivot-的方法)
- [對付重複的元素](#對付重複的元素)
- [選擇不同的分割方案](選擇不同的分割方案)

### 降低額外空間複雜度

前述提到最佳情形下（每次 pivot 都選到中位數），僅需 $\log n $ 個嵌套遞迴，額外空間複雜度僅需 $O(\log n) $。
倘若編譯器有實作 **尾端呼叫最佳化**，Quicksort 可以達到 $O(\log n) $ 對數級別的額外空間使用。

實作尾端呼叫最佳化的思路很簡單，「**先遞迴較少元素的部分，再利用 tall-call 遞迴另一部分**」，如此以來，較多元素的遞迴則會直接被編譯器展開，消去遞迴時需要的 call stack 空間。剩下較少元素的部分，則與最佳情形相同，最多僅需 $\log n $ 次嵌套遞迴。

簡單實作如下：

```rust
fn quicksort_helper_optimized(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {                      // 1
          quicksort_helper_optimized(arr, lo, pivot - 1);
          quicksort_helper_optimized(arr, pivot + 1, hi); // 2
        } else {
          quicksort_helper_optimized(arr, pivot + 1, hi);
          quicksort_helper_optimized(arr, lo, pivot - 1); // 3
        }
    }
}
```

1. 說穿了就只有這個判斷式，決定哪部分該先遞迴而已。
2. 這是一個尾端呼叫，會展開。
3. 這也是一個尾端呼叫。

實際上，截至 2018.2，[Rust Core Team 決定暫緩 TCO 的實作][rust-rfc-1888]，目前 Rust 並沒有支援 TCO。但我們還是可以手動實作 TCO，減少 call stack。

[rust-rfc-1888]: https://github.com/rust-lang/rfcs/pull/1888

我們先把原始的 lomuto partition 實作改成手動 TCO 版本。利用 `while` loop，將 `lo` 替換成下一個遞迴的引數，減少部分的 call stack。

```diff
- fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
+ fn quicksort_helper_manual_tco(arr: &mut [i32], mut lo: isize, mut hi: isize) {
-     if lo <= hi {
+     while lo < hi {
          let pivot = partition(arr, lo, hi);
-         quicksort_helper(arr, lo, pivot - 1);
-         quicksort_helper(arr, pivot + 1, hi);
+         quicksort_helper_manual_tco(arr, lo, pivot - 1);
+         lo = pivot + 1;
      }
  }
```

再來，選擇性遞迴較小的部分。Iterative 版本的尾端呼叫消除（tail-call eliminate）就做完了！

```rust
fn quicksort_helper_manual_tco(arr: &mut [i32], mut lo: isize, mut hi: isize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {
            quicksort_helper_manual_tco(arr, lo, pivot - 1);
            lo = pivot + 1;
        } else {
            quicksort_helper_manual_tco(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
    }
}
```

### 選擇 Pivot 的方法

選擇 pivot 的方法大致上有以下幾種：

- 總是選擇最後一個元素。
- 總是選擇第一個元素。
- 選擇特定位置（如中位數）的元素。
- 隨機選擇任意元素。

選擇第一個或最後一個元素會印序列已經接近排序完成或相反排序，造成 $O(n^2) $ 最壞的時間複雜度。隨機或選擇特定位置的方法較能避免這種情況，但實作上較困難。

除了選擇 pivot 的方法，近幾年來多 pivot（multi-pivot）Quicksort 也愈趨流行，可以減少 20% 的元素置換。相關的討論與證明可以參考[這篇文章](https://cs.stanford.edu/~rishig/courses/ref/l11a.pdf)。

### 對付重複的元素

若輸入序列有許多重複的元素，使用原本 Lomuto scheme 實作的 Quicksort 仍然會比較置換等於 pivot 的所有元素。3-way partition scheme 就是將序列多分出「等於 pivot」部分，減少重複置換相等元素的排序法。

```
[ values < pivot | values == pivot | value > pivot ]
```

通常是使用著名的 [Dutch national flag algorithm][dnf] 來解決這個問題。實作上和 Lomuto 非常類似。

```rust
fn partition(arr: &mut [i32], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = arr[hi as usize];
    let mut i = lo;         // smaller
    let mut j = lo;         // equal
    let mut k = hi;         // large

    while j <= k {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
            j += 1;
        } else if arr[j as usize] > pivot {
            arr.swap(k as usize, j as usize);
            k -= 1;
        } else {
            // No swap when identicial.
            j += 1;
        }
    }

    // Return smaller and larger pointer to avoid iterate duplicate elements.
    (i, k)
}
```

[dnf]: https://en.wikipedia.org/wiki/Dutch_national_flag_problem

### 選擇不同的分割方案

不同的分割方案有著不同的應用場景，如上述的 3-way scheme 就適合重複元素多的序列。這裡再多介紹另一個常見的分割實作方案 Hoare partition，是 Quicksort 發明這 Hoare 自己提出的分割法，Rust 實作演算法如下：

```rust
fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[lo];
    let mut i = lo;
    let mut j = hi;

    loop {
        // Find element >= pivot from leftmost element.
        while arr[i] < pivot {                            // 1
            i += 1;
        }
        // Find element <= pivot from rightmost element.
        while arr[j] > pivot {                            // 2
            j -= 1;
        }
        if i >= j {
            return j;
        }
        // Two elements are misplaced, swap them.
        arr.swap(i, j);                                   // 3
        i += 1;
        j -= 1;
    }
}
```

1. 從最左邊開始找比 pivot 大或相等的元素。
2. 從最右邊開始找比 pivot 小或相等的元素。
3. 若找到這兩個元素，置換之，以符合小於 pivot 在前，大於 pivot 在後的分割準則。

## 參考資料

- [Wiki: Quicksort](https://en.wikipedia.org/wiki/Quicksort)
- [Algorithms, 4th Edition by R. Sedgewick and K. Wayne](https://algs4.cs.princeton.edu/23quicksort/)
- [GeeksForGeeks: QuickSort](https://www.geeksforgeeks.org/quick-sort/)
- [Swift Algorithm Club: Quicksort](https://github.com/raywenderlich/swift-algorithm-club/tree/master/Quicksort)
