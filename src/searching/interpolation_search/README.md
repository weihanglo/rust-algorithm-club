# 內插搜尋 Interpolation Search

Interpolation search 改良自[二元搜尋][../binary_search]，差別在於二分點的選擇方法，二元搜尋選擇中間的元素作為二分點，而內插搜尋則名副其實，以內插法找尋二分點。

內插搜尋的特色如下：

- 資料需要是可計算內插的數值資料。
- 對資料分佈敏感，資料均勻分佈時，效能比二元搜尋佳。
- 資料分佈不均勻時，最差複雜度高達 $O(n)$。

## 效能

|              | Complexity  |
| ------------ | ----------- |
| Worst        | $O(\log i)$ |
| Best         | $O(1)$      |
| Average      | $O(\log i)$ |
| Worst space  | $O(1)$      |

## 參考資料

- [Wiki: Interpolation search](https://en.wikipedia.org/wiki/Interpolation_search)
- [Infinite Loop: Interpolation Search](http://program-lover.blogspot.com/2008/12/interpolation-search.html)


