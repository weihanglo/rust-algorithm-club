# 二元搜尋 Binary Search

Binary search，又稱對數搜尋（logarithmic search），是一個在已排序的序列中，快速找出特定元素的搜尋演算法。二元搜尋的步驟就像玩猜數字，先猜一個數字，告訴你你的猜測比正確答案大或小，再繼續往對的方向猜，捨棄猜錯的另一半。這樣繼續進行，每猜一次，搜尋範圍就小一半，因此稱為「二元」搜尋。

二元搜尋有以下幾個特點：

- 概念簡單，搜尋高效，達到對數執行時間 $O(\log n)$。
- 不需額外實作資料結構或配置記憶體空間。
- 只能搜尋**已排序**的序列。

## 步驟

1. 從序列中間的元素開始，比較其與目標值
2. 若該元素為搜尋目標，則結束搜尋。
3. 若該元素較大或小，則將序列切一半，往較小或較大的一半搜尋。
4. 繼續從一半的序列中間的元素開始，重複步驟一到三，直到欲搜尋的序列為空。

## 說明

這裡有一個排好序的序列，共有 15 個元素，現在要找尋 9 是否在序列中。

```
                       *
[2, 3, 3, 6, 6, 7, 9, 13, 15, 19, 20, 22, 23, 24, 25]
```

首先，先找到中間的元素 15 / 2 ~= 8，第八個元素為 13，比 9 大，因此捨棄第八個元素之後的所有元素。

```
          *
[2, 3, 3, 6, 6, 7, 9, _, _, _, _, _, _, _, _]
```

接下來繼續對半搜尋，8 / 2 = 4，找尋第四個元素來比對，6 比 9 小，，因此捨棄第四個元素前的所有元素。

```
             *
[_, _, _, 6, 6, 7, 9, _, _, _, _, _, _, _, _]
```

對剩下的元素二元搜尋，4 / 2 = 2，並從第四個元素開始計算中點 4 + 2 = 6，取得第六個元素為 7，比 9 小，捨棄 7 之前的元素。

```
                   *
[_, _, _, _, _, 7, 9, _, _, _, _, _, _, _, _]
```

繼續切一半來搜尋，繼續找中間的元素 2 / 2 = 1，並從第六個元素計算索引位置 6 + 1 = 7，查看第七個元素是 9，終於找到了！

## 效能

|              | Complexity  |
| ------------ | ----------- |
| Worst        | $O(\log n)$ |
| Best         | $O(1)$      |
| Average      | $O(\log n)$ |
| Worst space  | $O(1)$      |

二元搜尋可以透過分治法（Divide and conquer）遞迴求解，而遞迴的終止條件是序列不能在切兩半。由此可知，二元搜尋的複雜度奠基在要切幾次，子序列長度才會等於 1。設 $n$ 為資料數目，$k$ 為要切幾次才會達成終止條件，可得：

$$ \frac{n}{2^k} = 1 $$

接下來同乘 $2^k$ 並取對數。
$$
\frac{n}{2^k} = 1 \\\\
\Rightarrow 2^k = n \\\\
$$

再將左式整理一下，得到 $k$。

$$
\log_2 2^k = log_2 n \\\\
\Rightarrow k \cdot \log_2 2 = log_2 n \\\\
\Rightarrow k = log_2 n
$$

於是，我們得到二元搜尋時間複雜度為 $O(k) = O(\log_2 n) = O(\log n)$。

寫這種式子也許不好理解，我們可以把搜尋過程和每個分支寫成樹狀圖，方便觀察。假設一個數列有七個元素 `[1, 2, 3, 4, 5, 6, 7]`，其二元搜尋所有可能路徑的樹狀圖如下：

```
          +---+
          | 4 |
          +---+
        /       \
     +---+      +---+
     | 2 |      | 6 |
     +---+      +---+
    /    \      /   \
+---+  +---+  +---+  +---+
| 1 |  | 3 |  | 5 |  | 7 |
+---+  +---+  +---+  +---+
```

樹中每一條路徑都代表任意搜尋會經過的步驟，總共有 7 種不同的搜尋路徑，最短路徑僅需要 $\lfloor{\log_2 n} = 3 \rfloor$ 個操作，也就是需要執行「樹高」次的操作。

## 實作

### 函式宣告

二元搜尋概念看似簡單，實際上誤區一堆，不易寫出完全正確的演算法。我們參考 [Rust slice binary_search][rust-slice-binary-search] 的實作。先來看看 function signature（函式的宣告）。

```
pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
```

二元搜尋函式宣告中，回傳值大概是最特別的部分。如果有找到目標元素，`Result` 會是 `Ok(目標索引位置)`，如果沒有找到則回傳 `Err(目標值若插入後，不會影響序列排序的位置)`。`Err` 回傳值提供了插入點，非常方便。

再來，`T` 泛型參數需是 [`PartialOrd`][rust-trait-partialord]，這是由於二元搜尋使用排序過後的元素，比起線性搜尋，仍需元素之間相互比較。

### 函式主體

市面上常見的實作通常以兩個變數 `l` 與 `r` 記錄搜尋範圍的上下界，而我們另闢蹊徑，記錄了

- `base`：搜尋範圍的下界，
- `size`：搜尋範圍的長度。

以下是完整實作：

```rust
pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    let mut size = arr.len();       // 1
    if size == 0 {
        return Err(0);
    }
    let mut base = 0_usize;

    while size > 1 {                // 2
        // mid: [base..size)
        let half = size / 2;        // 2.1
        let mid = base + half;
        if arr[mid] <= *target {    // 2.2
            base = mid
        }
        size -= half;               // 2.3
    }

    if arr[base] == *target {       // 3
        Ok(base)
    } else {
        Err(base + (arr[base] < *target) as usize)
    }
}
```

1. 第一部分先取得搜尋範圍 `size` 以及確定下界為 `0_usize`。這裡同時檢查若序列長度為零，直接回傳 `Err(0)`，告知呼叫端可直接在 index 0 新增元素。
2. 第二部分就是精髓了，將終止條件設在 `size <= 1`，以確保迴圈能夠正常結束。
    1. 先將搜尋範圍對半切，再與下界 `base` 相加，算出中點。
    2. 另中間元素與目標值比較，如果比較小，則移動下界至中點。
    3. 將 `size` 減半，縮小搜尋範圍。
3. 到了第三部分，`base` 已經是切到長度為一的序列了，若匹配目標值就直接回傳；若否，需要傳可供目標值插入的位置，將 bool 判斷是轉型成 `usize`，若 `arr[base]` 比目標值小，則目標值要加到其後 +1 位置，反之則加在其前 -1 位置。

[rust-slice-binary-search]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
[rust-trait-partialord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

## 常見誤區與解法

1. 只適用已排序序列： 這是使用二元搜尋的前提，千萬不能忽略這重要特性，否則後果絕對大錯特錯。

2. 處理重複元素：一般的實作通常是回傳任意符合目標值的索引位置，就算有重複的元素，仍然不可預期。若要回傳特定位置（leftmost 或 rightmost），則需特別處理。

3. 整數溢位：部分二元搜尋實作會 以兩個變數儲存搜尋範圍上下界的索引位置，而取中點時千萬不可直接將上下界相加再除二，否則很可能整數溢位（integer overflow）。  
    ```rust
    let mid = (end + start) / 2           // Wrong: integer overflow
    let mid = start + (end - start) / 2   // Correct
    ```

4. 終止條件錯誤：無論如何實作，請將終止條件設為「搜尋範圍為空」，也就是下界大於上界，而不要只比較上下界是否相等。其實搜尋範圍低於一定長度，即可使用線性搜尋替代，避免處理邊界值的麻煩，實務上也幾乎沒有太多效能損失。

## 變形與衍生

### Interpolation Search

Interpolation search 改良自二元搜尋，差別在於，二元搜尋選擇中間的元素作為二分點，而 interpolation search 人如其名，以內插法找尋二分點。在資料平均分佈時，比二元搜尋更高效。欲知後續，待下回[內插搜尋 Interpolation search](../interpolation_search) 分曉。

### Exponential Search

Exponential search 是一種特殊的二元搜尋，主要用在無限、無邊界的已排序序列，由於邊界未知長度就未知，無法以傳統二元搜尋找尋中點。Exponential 顧名思義就是不斷比較在 $2^0$，$2^1$ 直到 $2^n$ 的位置上資料是否比目標值大，若較大，再從該位置執行二元搜尋回頭找。詳情請看[指數搜尋 Exponential search](../exponential_search)。

### Binary Insertion Sort

Insertion sort 有一個步驟是在前面已經排完序的資料中，找到適合的地方插入待排序的元素，這部分可透過二元搜尋 加快在已排序資料搜尋的速度。詳情請參考 [Binary insertion sort](../../sorting/insertion_sort/#binary-insertion-sort)。

## 參考資料

- [Wiki: Binary search algorithm](https://en.wikipedia.org/wiki/Binary_search_algorithm)
- [知乎：二分查找有几种写法？它们的区别是什么？](https://www.zhihu.com/question/36132386)
