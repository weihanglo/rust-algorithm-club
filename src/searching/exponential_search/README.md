# 指數搜尋 Exponential Search

指數搜尋，又稱為 galloping search，是一種特殊的[二元搜尋](../binary_search)，主要用在搜尋無限、無邊界的已排序序列。由於邊界未知長度就未知，無法以傳統二元搜尋來找中點。而 Exponential 顧名思義就是從底數為 2，指數為 0 的索引（$2^0$ ）開始，不斷比較在 $2^1$、$2^2$ 直到 $2^k$ 位置上的值，若比目標值大，則停止指數成長，直接從該位置執行二元搜尋，回頭尋找目標值。

指數搜尋的特點如下：

- 可以搜尋邊界未知的已排序序列。
- 縮小搜尋範圍，可比 naïve 的二元搜尋效率高些。
- 若目標值實際位置很靠近序列前端，效率會非常棒。

## 步驟

指數搜尋的步驟只有非常簡單的兩步驟：

1. 依照目標值大小，劃出搜尋範圍。
2. 在上述範圍內執行二元搜尋。

而劃出搜尋範圍這部分也很直觀：

1. 選定一個底數 $k$，通常為 2。
2. 比較 $k^i$ 索引下的值是否比目標值大，$i$ 從零開始。
3. 若較小，指數加一 $k^{i + 1}$ 後繼續重複步驟二比較。
4. 若較大，停止比較，得搜尋範圍為 $k^{i - 1}$ 到 $k^i$。

## 效能

|              | Complexity  |
| ------------ | ----------- |
| Worst        | $O(\log i)$ |
| Best         | $O(1)$      |
| Average      | $O(\log i)$ |
| Worst space  | $O(1)$      |

> $i$：目標值在序列中實際的索引位置。

指數搜尋的複雜度分為兩部分分析：

### 劃定搜尋範圍

設 $i$ 為目標值在序列中實際的索引位置，則搜尋上界，指數增加的操作需執行 $\lceil \log(i) \rceil$ 次，例如匹配目標值的搜尋結果位於序列第 9 個，則指數需增加 $\lceil \log(9) \rceil = 4$ 次，上界才會超過目標值。我們設這部分的複雜度為 $O(log i)$。

### 執行二元搜尋

第二部分就是二元搜尋，複雜度為 $O(log n)$，$n$ 為搜尋範圍的長度。根據第一部分，可以得知範圍長度為 $2^{\log i} - 2^{\log{i - 1}} = 2^{log{i - 1}}$ 個元素，帶入二元搜尋的複雜度，計算出第二部分的複雜度為 $log (2^{\log{i - 1}}) = \log{(i)} - 1 = O(\log i)$。


最後，將兩部分的複雜度合起來，就是指數搜尋的時間複雜度了。

$$O(\log i) + O(\log i) = 2 O(\log i) = O(\log i)$$

## 實作

本次實作有邊界的指數搜尋，主要分為三個部分：

1. 處理空序列的狀況。
2. 利用指數，決定搜尋範圍。
3. 執行二元搜尋，並將輸出結果映射回原始序列。

話不多說，直接看程式碼。

```rust
use crate::searching::binary_search;

pub fn exponential_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    // 1. Handle empty scenario.
    let size = arr.len();
    if size == 0 {
        return Err(0);
    }

    // 2. Determine searching boundaries.
    let mut hi = 1_usize; // Upper bound.
    while hi < size && arr[hi] < *target {
        hi <<= 1;
    }
    let lo = hi >> 1; // Lower bound.

    // 3. Do binary search.
    binary_search(&arr[lo..size.min(hi + 1)], target)
        .map(|index| lo + index)
        .map_err(|index| lo + index)
}
```

1. 和二元搜尋同，遇到空序列就返回 `Err(0)` 告知呼叫端可新增資料在位置 0。
2. 決定搜尋上下界，只要 上界不超過序列長度，且 `arr[hi]` 小於目標值，就讓上界指數成長。這裡用位元左移運算子（bitwise left shift）實作乘以 2。  
    找到上界後，再將上界除以 2（位元右移），就是下界了。
3. 確定範圍後，利用上下界切序列的 sub slice 作為引數，傳遞給二元搜尋。要注意的是，為了避免 sub slice 超出邊界，上界需在 `size` 與 `hi + 1` 之間找最小值。  
    由於回傳結果的位置是以 sub slice 起始，需加上位移量（下界 `lo`）才會對應原始 slice 的位置。

## 參考資料

[Wiki: Exponential search](https://en.wikipedia.org/wiki/Exponential_search)
