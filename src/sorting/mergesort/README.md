# Mergesort

Mergesort 是一個泛用且高效穩定的排序法，最佳與最差時間複雜都是 $O(n \log n)$。Mergesort 可謂著名「Divide and Conquer」手法的經典案例，先將序列分成更小的子序列（Divide），一個個排序後（Conquer），再合併已排序的子序列（Combine）。

- **高效穩定**：最佳、平均，與最差時間複雜度皆為 $O(n \log n)$。
- **穩定排序**：相同鍵值的元素，排序後相對位置不改變。
- **非原地排序**：除了資料本身，仍需額外花費儲存空間來排序。
- **分治演算法**：將主問題化作數個子問題，各個擊破。


## Algorithm

Mergesort 演算法分為以下步驟：

1. **Divide**：將含有 n 個元素的序列分割成含有 n / 2 個子序列。
2. **Conquer**：排序分割後的兩個子序列。
3. **Combine**：合併排序完成的兩子序列，成為一個排好序的序列。

其中，Conquer 步驟中的「排序」可以不斷遞迴 Mergesort 自身，因此需要停止遞迴的條件（base case），我們將條件設定為「子序列的長度小於 2」，因為長度為 1 的序列可視為已完成排序。

將 Mergesort 視覺化排序如下：

![mergsort](https://upload.wikimedia.org/wikipedia/commons/c/c5/Merge_sort_animation2.gif)

## Explanation

以 ASCII diagram 圖解 Mergesort。

先將原始序列分割成數個長度為一的子序列。

```
Split array into length 1 subarray.

    [8, 7, 1, 2, 4, 6, 5, 3]
                |
   [8, 7, 1, 2] | [4, 6, 5, 3]
                |
  [8, 7] [1, 2] | [4, 6] [5, 3]
                |
[8] [7] [1] [2] | [4] [6] [5] [3]
                V
              split
```

再將子序列依序合併成一個排好序的大序列。

```
Recursively merge subarray respecting the order.

              Merge
                |
[8] [7] [1] [2] | [4] [6] [5] [3]
                |
  [7, 8] [1, 2] | [4, 6] [3, 5]
                |
   [1, 2, 7, 8] | [3, 4, 5, 6]
                V
    [1, 2, 3, 4, 5, 6, 7, 8]
```

## Performance

|              | Complexity         |
| :----------- | :----------------- |
| Worst        | $O(n \log n)$      |
| Best         | $\Omega(n \log n)$ |
| Average      | $\Theta(n \log n)$ |
| Worst space  | $O(n)$ auxiliary   |

### Time Complexity

透過遞迴關係式，很容易計算 Mergesort 的時間複雜度。假設排序長度為 $n$ 的序列最多需要 $T(n)$ 時間。可以觀察到，如果序列只有一個元素，Mergesort 僅需要常數時間就可以完成排序，寫成 $T(n) = 1$。

如果 $n > 2$，Mergesort 會將序列分為 $\lceil \frac{n}{2} \rceil$ 部分，以及 $\lfloor \frac{n}{2} \rfloor$ 部分。我們可以將排序前者寫成 $T(\lceil \frac{n}{2} \rceil)$，而後者花費時間為 $ T(\lfloor \frac{n}{2} \rfloor)$。

最後，合併兩個子序列僅需 $n$ 個操作。可得下列遞迴關係式。  
（為了方便計算，把 floor 和 ceil 捨去）

$$
  T(n) =
  \begin{cases}
    1                   & \text{if } n = 1, \\
    2T(\frac{n}{2}) + n & \text{otherwise}.
  \end{cases}
$$

根據 [Master Theorem](master-theorem)，可得複雜度為 $O(n \log n)$。

[master-theorem]: https://en.wikipedia.org/wiki/Master_theorem_(analysis_of_algorithms)

### Space Complexity

Mergesort 的缺點之一就是在合併子序列時，需要額外的空間依序插入排序資料；若是遞迴版本的 Mergesort 還需額外加上遞迴花費的 call stack 空間，因此額外空間複雜度為 $O(n) + O(\log n) = O(n)$（以陣列實作）。

## Implementation

一般來說，Divide and Conquer 有兩種設計、解決問題的技巧：Top-down（自上而下）與 Buttom-up（自下而上）。前者是先對問題有整體的輪廓概念，再逐步針對細節一一處理；後者則是先準備每個問題需要的基礎步驟與元件，再將這些步驟結合，解決整體的問題。

Mergesort 的實作分為兩部分：

- `mergesort` 主程式：對外的接口，負責分割序列。對應 Divide 功能。
- `merge`：合併子序列，對應到 Conquer 與 Combine 功能。

先來看看如何分割序列。

### Top-down split

自上而下的解法會不斷以類似 binary search 的方式找中點，進而分割序列。

```rust
pub fn mergesort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {                 // 1
        return;
    }

    mergesort(&mut arr[..mid]);   // 2
    mergesort(&mut arr[mid..]);

    // Create an array to store intermediate result.
    let mut ret = arr.to_vec();   // 3

    // Merge the two piles.
    merge(&arr[..mid], &arr[mid..], &mut ret[..]);  // 4

    // Copy back the result back to original array.
    arr.copy_from_slice(&ret);    // 5
}
```

1. 設定遞迴的終止條件（base case），middle index 為 0 表示長度不大於 1。
2. 利用 Rust 的 [Range Operator][rust-ops-range]，可快速分割兩個 `slice`。
3. 建立一個 `Vec` 儲存排序結果。
4. 將兩個 `slice` 合併排序至 `ret` vector 中。
5. 將 `ret` 的結果複製到原始 `arr` 中，使回傳值保有相同起始位址。

[rust-ops-range]: https://doc.rust-lang.org/std/ops/struct.Range.html

### Buttom-up split

自下而上的解法則是預定好最小的子序列長度，直接使用 for 迴圈從頭開始逐一擊破。

```rust
pub fn mergesort_bottom_up(arr: &mut [i32]) {
    let mut width = 1;                                // 1
    // Create an array to store intermediate result.
    let mut ret = arr.to_vec();                       // 2
    let len = arr.len();

    while width < len {
        let mut i = 0;
        while i < len {
            // Check to avoid upper bound and middle index out of bound.
            let upper = ::std::cmp::min(i + 2 * width, len);  // 3
            let mid = ::std::cmp::min(i + width, len);

            merge(&arr[i..mid], &arr[mid..upper], &mut ret[i..upper]);

            // Copy the merged result back to original array.
            arr[i..upper].copy_from_slice(&ret[i..upper]);    // 4

            // Increase start index to merge next two subsequences.
            i += 2 * width;                           // 5
        }
        width *= 2;                                   // 6
    }
}
```

1. 設定最小的子序列長度，這個長度以下的子序列皆視為已排序。
2. 建立一個 `Vec` 儲存排序結果。
3. 取最小值，避免下標超出邊界，並且維持除了最後一組，其他子序列長度恆為 `width`。
4. 複製這部分排序結果 `ret` 到原始的 `arr` 中。
5. 繼續下兩個子序列的合併步驟。
6. 將下個迭代的子序列長度加倍，繼續合併。

### The merge part

無論是 Top-down 還是 Buttom-up 版本的解法，皆免不了 `merge` 這個共同步驟，將子序列合併為較大的序列。

```rust
fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
    let mut left = 0; // Head of left pile.             // 1
    let mut right = 0; // Head of right pile.
    let mut index = 0;

    // Compare element and insert back to result array.
    while left < arr1.len() && right < arr2.len() {     // 2
        if arr1[left] <= arr2[right] {                  // 3
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
    }

    // Copy the reset elements to returned array.
    // `memcpy` may be more performant than for-loop assignment.
    if left < arr1.len() {                              // 4
        ret[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        ret[index..].copy_from_slice(&arr2[right..]);
    }
}
```

1. 建立三個指標，分別給 `arr1`、`arr2` 與回傳陣列 `ret` 使用。
2. 這部分依序比較兩個子序列，排序較小者先進入回傳 `ret`。直到其中一序列所有元素都進入 `ret` 就停止。
3. 這邊判斷使用 `<=` 小於等於確保排序穩定（相同鍵值順序不換）。
4. 將剩餘未進入 `ret` 的元素，依序複製到 `ret` 中。

> `slice.copy_from_slice` 底層使用 C 的 `memcpy`，比起 for-loop 一個個賦值，直接複製整塊記憶體比較快了。

## Variants

### Timsort

在真實世界資料中，早有許多部分排序的分區（natural run），倘若跳過排序這些分區的步驟，就可減少許多不必要的操作，[Timsort](../timsort) 就是為了完全利用榨乾這些分區的混合排序法。

## Reference

- [Wiki: Merge sort](https://en.wikipedia.org/wiki/Merge_sort)
- [CMSC 351 Algorithms, Fall, 2011, University of Maryland.](www.cs.umd.edu/~meesh/)
- Sorting GIF was created By CobaltBlue [CC BY-SA 2.5](https://creativecommons.org/licenses/by-sa/2.5) via Wikimedia Commons.
