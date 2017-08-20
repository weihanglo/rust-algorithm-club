# Counting sort

- 非原地演算法（Out-of-place algorithm）
- 非比較式排序法（Non-comparative sort）
- 穩定排序法（Stable sort）
- 整數排序法（Integer sort）：以整數作為排序的鍵值
- 預期：輸入的資料為已知範圍內的的正整數（例如 **1 - k**）
- 當輸入資料量 **n** 與已知範圍上下界之差值 **d** 相近時，執行時間接近線型（**O(n)**）

## Algorithm

1. **Count occurrence**：計每個 key 的出現頻率
2. **Prefix sum as start index**：計算前綴和（Prefix sum），作為該元素的 start index。
3. **Copy output**：利用步驟二的前綴和，遍歷輸入資料，取得元素排序後的索引。

## Explanation

這裡有資料需要透過正整數的 key 來排序。key 的範圍在 0 - 9 之間，格式為 `(key, value)`。

```
Input: (1, A) (5, B) (8, C) (2, D) (2, E) (9, F)
```

**壹**：首先，先計算每個 key 的出現頻率。

```
Key  : 0 1 2 3 4 5 6 7 8 9
Count: 0 1 2 0 0 1 0 0 1 1
```

**貳**：再計算 prefix sum，也就是將當前 index 前累計的 key 數量加總。例如 **key 5** 的 prefix sum **1 + 2 = 3**。這個 prefix sum 等同於每筆資料排序後的位置（index）。例如排序後，**8** 位於陣列第四位。

```
Key  : 0 1 2 3 4 5 6 7 8 9
Count: 0 0 1 3 3 3 4 4 4 5
```

**參**：透過 key 與 prefix sum 的映射關係，找到原始資料對應的位置。

實作上，每筆資料找到對應的 start index（prefix sum） 後，要將**該 index 之值 +1**，使得重複的元素可取得正確的 index offset（對唯一的 key 沒有影響）。

```
(1, A)
--> prefix sum 為 0，寫入 array[0]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) |        |        |        |        |        |
+--------+--------+--------+--------+--------+--------+

(5, B)
--> prefix sum 為 3，寫入 array[3]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) |        |        | (5, B) |        |        |
+--------+--------+--------+--------+--------+--------+

(8, C)
--> prefix sum 為 4，寫入 array[4]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) |        |        | (5, B) | (8, C) |        |
+--------+--------+--------+--------+--------+--------+

(2, D)
--> prefix sum 為 2，寫入 array[4]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) | (2, D) |        | (5, B) | (8, C) |        |
+--------+--------+--------+--------+--------+--------+

(2, E)
--> prefix sum 為 3（前一步驟 + 1），寫入 array[3]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) | (2, D) | (2, E) | (5, B) | (8, C) |        |
+--------+--------+--------+--------+--------+--------+

(9, F)
--> prefix sum 為 5，寫入 array[5]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) | (2, D) | (2, E) | (5, B) | (8, C) | (9, F) |
+--------+--------+--------+--------+--------+--------+
```

這樣就完成排序了。此外，觀察**（2, D）**與**（2， E）**排序前後的位置，會發現 counting sort 的的確確是個穩定排序，很棒。

## Performance

|              | Complexity           |
| :----------- | :------------------- |
| Worst case   | $O(n + k)$           |
| Best case    | $\Omega(n + k)$      |
| Average case | $\Theta(n + k)$      |
| Worst space  | $O(n + k)$ auxiliary |

## Implementation

## Optimization

Reduce space usage

## Reference

https://en.wikipedia.org/wiki/Integer_sorting

http://research.ijcaonline.org/icthc2015/number1/icthc28250.pdf
