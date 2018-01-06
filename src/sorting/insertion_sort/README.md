# Insertion sort

[Insertion sort][wiki-insertion-sort] 是最簡單的排序法之一，比起 quicksort 等知名的排序法，對大資料的處理效能較不理想。其演算法是將欲排序元素直接插入正確位置，因而得名。

Insertion sort 基本特性如下：

- 實作簡單易理解。
- 資料量少時較高效，且比其他 $O(n^2)$ 的排序法高效（selection sort/bubble sort）。
- **自適應排序**：可根據當前資料排序情形加速排序，資料越接近排序完成，效率越高。
- **穩定排序**：相同鍵值的元素，排序後相對位置不改變。
- **原地排序**：不需額外花費儲存空間來排序。
- **即時演算法**：可處理逐步輸入的資料，不需等資料完全備妥。

## Algorithm

將序列分為未排序與部分排序兩個區域。

![](https://upload.wikimedia.org/wikipedia/commons/3/32/Insertionsort-before.png)

1. **取第一個元素**，將該元素視為已排序。
2. **取出下一元素**，該元素將插入序列的部分排序區域。
3. **尋找正確位置**：若部分排序元素比新元素大，則互換位置。並重複步驟 2 - 3，直到部分排序元素小於等於新元素。
4. **插入元素**：將新元素**插入**最後的位置。
5. 重複步驟 2 - 4，直到排序完成。

簡而言之，即是每次取一個元素，尋找並插入該元素在部分排序區域的排序位置，再逐步把序列單邊排序完成。

![](https://upload.wikimedia.org/wikipedia/commons/d/d9/Insertionsort-after.png)

Insertion sort 非常簡單，看動畫就能明瞭。
![](https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif)

## Performance

|              | Complexity       |
| :----------- | :--------------- |
| Worst        | $O(n^2)$         |
| Best         | $\Omega(n)$      |
| Average      | $\Theta(n^2)$    |
| Worst space  | $O(1)$ auxiliary |

最佳時間複雜度發生在資料已完成排序的狀況下，insertion sort 只需執行最外層的迴圈 $n$ 次。

最差時間複雜度發生在資料完全相反時，insertion sort 每取得一個新元素是，都需將資料插入序列最前面，，因此所需的操作如下（$c$ 為任意常數）：

$$ c \cdot 1 + c \cdot 2 + c \cdot 3 \cdots + c \cdot (n - 1) = \frac{c(n - 1 + 1)(n - 1)}{2}$$

最後等於

$$\frac{cn^2}{2} - \frac{cn}{2}$$

捨去低次項，得到時間複雜度為 $O(n^2)$。

## Variants

### Binary insertion sort (binsort)

在一般演算法討論中，通常以簡單的型別如 `i32` 來探討並實作。在真實世界中，做哪種操作，用哪種語言，都會影響到實際效能。例如 Python 的排序成本很高，要在 runtime 檢查物件是否實作 `__lt__` `__gt__` 等方法才能調用。排序法的實作就要特別注意減少比較操作的次數。

Binary insertion sort 的目的就是減少內層迴圈的比較次數。在內層迴圈開始之前，使用 [binary search][wiki-binary-search] 搜尋新元素應要插入哪個位置，最多僅需 $\log_2n$ 次比較。但 binary insertion sort 的複雜度依舊是 $O(n^2)$，因為除了比較之外，仍需置換（swap）、賦值（assign）等基礎操作。

## Reference

- [Wiki: Insertion sort][wiki-insertion-sort]
- [Wiki: Binary search algorithm][wiki-binary-search]
- [CPython: listsort note][cpython-listsort-note]

[wiki-insertion-sort]: https://en.wikipedia.org/wiki/Insertion_sort
[wiki-binary-search]: https://en.wikipedia.org/wiki/Binary_search
[cpython-listsort-note]: https://github.com/python/cpython/blob/15f44ab043b37c064d6891c7864205fed9fb0dd1/Objects/listsort.txt#L686-L703
