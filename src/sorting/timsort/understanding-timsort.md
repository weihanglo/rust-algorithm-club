# 三個問題

1. Can we make merges faster?
2. Can we perform fewer merges?
3. Are there cases where we're actually better off doing something different and not using mergesort?

第三題在小陣列可以使用 insertion sort


## Can we perform fewer merges?

檢查是否已經排好序來減少 merge -> 會有過多額外開銷

至少這個 idea 不錯：Use the already sorted subarrays as the basis of partition


## 自適應合併排序

1. 找到排好序的 natural run 放到 stack 中
    - 若 natural run 是 reversed 的，則原地 reverse。
2. 若 stack 上有兩個 runs 則合併（minrun）


## Minruns

小於minrun 則執行 binary insertion sort

maintain 數量在等於或稍微小於 2^n

## gallop

合併時利用 exponential search 找到需要合併的位置，減少比較。

> Galloping noticeably improves performance on partially sorted inputs, but worsens it on random ones.


$O(\log(n))$

**為什麼不能直接用 binary search？？？？？**

目標值靠近序列前端，expo search 比較幾次就會找到，不需要做 n 次 binary search

run1 = [1, 3, 5, 7, 9]
run2 = [2, 4, 6, 8, 10]

## 合併 run




## Rust stable sort modified from timsort

- remove gallops

## Python timsort
