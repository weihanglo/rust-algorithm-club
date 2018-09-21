# 希爾排序 Shellsort

眾所周知，[Insertion sort](../insertion_sort) 用在幾乎完成排序的序列上非常高效，換句話說，當元素置換不需移動太遠時，效率很高。反之，如果有元素錯位非常遙遠，效能就會大打折扣。Shellsort 以一個 gap sequence 將資料依指定的間隔（gap）分組進行 insertion sort，使得較遠的元素能夠快速歸位，下一次的排序就會因前次排序結果愈來愈接近完成而加速。

Shellsort 最後一個 gap 必定是 1，也就是排序會退化成 insertion sort，此時大部分元素皆排序完成，insertion sort 會非常高效。

Shellsort 特性如下：

- **自適應排序**：可根據當前資料排序情形加速排序，資料越接近排序完成，效率越高。
- **不穩定排序**：排序後，相同鍵值的元素相對位置可能改變。
- **原地排序**：不需額外花費儲存空間來排序。
- 可視為一般化（Generalizaion）的 [insertion sort](../insertion_sort)。

## 步驟

Shellsort 分為兩個步驟：

1. 決定一組 gap sequence。
2. 迭代 gap sequence 進行分組排序，每次執行有間隔的 insertion sort。也就是每個元素與其相鄰 gap 的元素比較與置換。

> 最後一次排序（gap = 1）會退化為 insertion sort，完成整個排序。

### Gap Sequneces

Shellsort 的效率取決於 gap sequence 的選擇，這邊舉幾個常見的 gap sequence：

|              | Sequence                        |
| ------------ | ------------------------------- |
| Marcin Ciura | 1, 4, 10, 23, 57, 132, 301, 701 |
| $2^{k} - 1 $  | 1, 3, 7, 15, 31, 63,...         |
| $\lfloor {\frac {N}{2^k}} \rfloor $ | $\lfloor {\frac {N}{2}} \rfloor $, $\lfloor {\frac {N}{4}} \rfloor $, ..., 1|

感受一下 gap sequence 為 23, 10, 4, 1 的 shellsort 吧。

![](https://upload.wikimedia.org/wikipedia/commons/d/d8/Sorting_shellsort_anim.gif)

## 說明

Shellsort 其實就是進行好幾次不同 gap 的 insertion sort，以下用 ASCII diagram 解釋。

假定這裡有一個序列需要遞增排序。

```
[5, 3, 8, 7, 4, 9, 6, 2]
```

我們選擇最簡單的 $\lfloor {\frac {N}{2^k}} \rfloor $ gap sequence 來排序。我們以**星號**標示出每次 insertion sort 對應排序

首先算出第一個 gap 為 $8 / 2^1 = 4 $。開始 insertion sort。

```
 *           *
[5, 3, 8, 7, 4, 9, 6, 2]

-> (sort subsequence [5, 4])

    *           *
[4, 3, 8, 7, 5, 9, 6, 2]

-> (skip)
       *           *
[4, 3, 8, 7, 5, 9, 6, 2]

-> (sort subsequence [8, 6])
          *           *
[4, 3, 6, 7, 5, 9, 8, 2]

-> (sort subsequence [7, 2])

[4, 3, 8, 2, 5, 9, 6, 7]
```

再來算出第二個 gap 為 $8 / 2^2 = 2 $。開始 insertion sort。

```
 *     *
[4, 3, 8, 2, 5, 9, 6, 7]

-> (skip)
    *     *
[4, 3, 8, 2, 5, 9, 6, 7]

-> (sort subsequence [3, 2])
 *     *     *
[4, 2, 8, 3, 5, 9, 6, 7]

-> (sort subsequence [4, 8, 5])
    *     *     *
[4, 2, 5, 3, 8, 9, 6, 7]

-> (skip)
 *     *     *     *
[4, 2, 5, 3, 8, 9, 6, 7]

-> (sort subsequence [4, 5, 8, 6])
    *     *     *     *
[4, 2, 5, 3, 6, 9, 8, 7]

-> (sort subsequence [2, 3, 9, 7])
[4, 2, 5, 3, 6, 7, 8, 9]
```

再來進行第三次排序。gap 為 $8 / 2^3 = 1 $，shellsort 退化至 insertion sort，但前一次結果已經很接近排序完成，insertion sort 可以幾乎在 one pass 完成排序。

> Insertion sort 的 ASCII diagram 我們就不展示了，請參考 [Insertion sort](../insertion_sort)。

## 效能

|              | Complexity                                            |
| ------------ | ----------------------------------------------------- |
| Worst        | $O(n^2) $ ~  $O(n \log^2 n) $ (Depends on gap sequence) |
| Best         | $O(n \log n) $                                    |
| Average      | Depends on gap sequence                               |
| Worst space  | $O(1) $ auxiliary                                 |

Shellsort 的複雜度不容易計算，取決於 gap sequence 怎麼安排，太少 gap 會讓速度太接近 insertion sort，太多 gap 則會有過多額外開銷。目前已知的 gap sequence 中，最差時間複雜度可以達到 $O(n \log^2 n) $，有著不錯的表現。有興趣可以參考[這篇文章][best-sequence]。

## 實作

我們這裡以 [Marcin 的 Paper][marcin-sequence-paper] 中提到的經驗式為例，首先，先建立一個 gap sequence 的常數。

```rust
/// Marcin Ciura's gap sequence.
pub const MARCIN_GAPS: [usize; 8] = [701, 301, 132, 57, 23, 10, 4, 1];
```

再來就是主程式的部分，總共會有三個迴圈，

- 最外層是迭代 gap sequence，
- 中間層是迭代整個資料序列，
- 內層就是每個元素的插入排序動作。

```rust
/// Shellsort
pub fn shellsort(arr: &mut [i32]) {
    let len = arr.len();
    for gap in MARCIN_GAPS.iter() {                     // 1
        let mut i = *gap;                               // 4
        while i < len {                                 // 2
            let mut j = i;
            while j >= *gap && arr[j - gap] > arr[j] {  // 3
                arr.swap(j - *gap, j);
                j -= *gap;
            }
            i += 1;
        }
    }
}
```

1. 最外層的迴圈，利用 `iter()` trait 產生迭代器，迭代 gap sequence。
2. 中間層迴圈，控制 `i` 是否超出資料序列，以迭代整合資料序列。
3. 最內層迴圈，執行插入動作，將每個元素置換到正確位置。
4. 由於 `gap` 的型別是 `&usize`，需透過 `*gap` dereference 得到 `usize` 型別。

## 參考資料

- [Wiki: Shellsort](https://en.wikipedia.org/wiki/Shellsort)
- [Best Increments for the Average Case of Shellsort, M. Ciura, 2001][marcin-sequence-paper]
- [Shellsort and Sorting Networks (Outstanding Dissertations in the Computer Sciences)][best-sequence]

[best-sequence]: http://www.dtic.mil/get-tr-doc/pdf?AD=AD0740110
[marcin-sequence-paper]: http://sun.aei.polsl.pl/~mciura/publikacje/shellsort.pdf

