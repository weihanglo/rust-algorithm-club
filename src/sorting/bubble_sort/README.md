# 氣泡排序 Bubble sort

Bubble sort 是最簡單的排序法之一，由於排序時每個元素會如同泡泡般，一個一個浮出序列頂部，因而得名。由於其簡單好理解，名稱又有趣，常作為第一個學習的入門排序法。不過其效率不彰，甚至不如同為 quardratic time 的 insertion sort。Bubble sort 的原理很平凡，就是相鄰兩兩元素互相比較，如果大小順序錯了，就置換位置。再往下一個 pair 比較。

Bubble sort 的特性如下：

- 又稱為 **sinking sort**。
- **穩定排序**：相同鍵值的元素，排序後相對位置不改變。
- **原地排序**：不需額外花費儲存空間來排序。

## 步驟

1. 比較兩個相鄰元素，若首個元素比次個元素大，置換兩者的位置。
2. 依序對相鄰元素執行步驟一，直到抵達序列頂端，此時頂端元素排序完成。
3. 重複步驟 1 - 2 的整個序列迭代，直到任何一次迭代沒有執行元素置換。

![](https://upload.wikimedia.org/wikipedia/commons/c/c8/Bubble-sort-example-300px.gif)
_Swfung8 - CC BY-SA 3.0_

## 說明

給定一組序列 `[5, 3, 8, 7, 2]`，以 bubble sort 遞增排序。以 ASCII diagram 表示：

**第一次迭代**

```bash
 *  *               *  *
[5, 3, 8, 7, 4] -> [3, 5, 8, 7, 4] # 置換 3 與 5

    *  *               *  *
[3, 5, 8, 7, 4] -> [3, 5, 8, 7, 4] # 不需置換

       *  *               *  *
[3, 5, 8, 7, 4] -> [3, 5, 7, 8, 4] # 置換 7 與 8

          *  *               *  *
[3, 5, 7, 8, 4] -> [3, 5, 7, 4, 8] # 置換 4 與 8，8 已排好序
```

**第二次迭代**

```bash
 *  *               *  *
[3, 5, 7, 4, 8] -> [3, 5, 7, 4, 8] # 不需置換

    *  *               *  *
[3, 5, 7, 4, 8] -> [3, 5, 7, 4, 8] # 不需置換

       *  *               *  *
[3, 5, 7, 4, 8] -> [3, 5, 4, 7, 8] # 置換 4 與 7

          *  *               *  *
[3, 5, 4, 7, 8] -> [3, 5, 4, 7, 8] # 不需置換
```
> naïve bubble sort 會跑完整個序列，即是已排序完成。

**第三次迭代**

```bash
 *  *               *  *
[3, 5, 4, 7, 8] -> [3, 5, 4, 7, 8] # 不需置換

    *  *               *  *
[3, 5, 4, 7, 8] -> [3, 4, 5, 7, 8] # 置換 4 與 5

       *  *               *  *
[3, 5, 4, 7, 8] -> [3, 4, 5, 7, 8] # 不需置換

          *  *               *  *
[3, 5, 4, 7, 8] -> [3, 4, 5, 7, 8] # 不需置換
```

**第四次迭代**

```bash
 *  *               *  *
[3, 4, 5, 7, 8] -> [3, 4, 5, 7, 8] # 不需置換

    *  *               *  *
[3, 4, 5, 7, 8] -> [3, 4, 5, 7, 8] # 不需置換

       *  *               *  *
[3, 4, 5, 7, 8] -> [3, 4, 5, 7, 8] # 不需置換

          *  *               *  *
[3, 4, 5, 7, 8] -> [3, 4, 5, 7, 8] # 不需置換
```

很簡單的排序法！

## 效能

|              | Complexity    |
| ------------ | ------------- |
| Worst        | \\(O(n^2) \\) |
| Best         | \\(O(n) \\)   |
| Average      | \\(O(n^2) \\) |
| Worst space  | \\(O(1) \\) auxiliary |

### Time complexity

Bubble sort 總共需要 \\(n - 1 \\) 次迭代，每次迭代至少需要執行 \\(n - 1 - i \\) 置換（ \\)i \\) 為第幾次迭代），總共需要迭代

$$\sum_{i=0}^{n-1} (n - i - 1) = n^2 - \sum_{i=0}^{n-1}i - n = n^2 - \frac{n(n - 1)}{2} - n = \frac{n^2}{2} - \frac{n}{2}$$

次，因此，時間複雜度為 \\(O(n^2) \\)。

Bubble sort 在已排序完成的序列上，只需要迭代序列一次，發現完全沒有置換任何元素，即停止排序，可達到最佳時間複雜度。

## 實作

Bubble sort 簡單實作如下：

```rust
pub fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;                 // 1
    while swapped {
        swapped = false;
        for i in 1..arr.len() {             // 2
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true              // 3
            }
        }
    }
}
```

1. 建立一個旗標，標誌該次迭代是否有元素置換。
2. 內層迴圈依序比較兩兩相鄰元素。
3. 若有任何置換動作，將旗標標誌為「已置換（`true`）」。

倘若記錄已排好序的元素位置，雖然複雜度仍是 \\(O(n^2) \\)，但如此以來，每次迭代都可少一次元素比較，對比較操作成本高的語言或實作來說，仍不失為最佳化的方法。程式碼如下：

```rust
pub fn bubble_sort_optimized(arr: &mut [i32]) {
    let mut new_len: usize;
    let mut len = arr.len();            // 1
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;            // 2
            }
        }
        if new_len == 0 {               // 3
            break;
        }
        len = new_len;                  // 4
    }
}
```

1. 將當前的序列長度記錄到 `len`。
2. 內層迴圈負責比較、置換，以及記錄未排序部分的序列長度到 `new_len`。
3. 若未排序部分 `new_len` 為零，代表排序完成。
4. 外層迴圈將新長度值 `new_len` 賦予 `len`，下一次迭代就可少做一次比較。

## 參考資料

- [Wiki: Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort)
- Sorting GIF was created by Swfung8 (Own work) [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0) via Wikimedia Commons.
