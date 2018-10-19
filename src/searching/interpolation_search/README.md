# 內插搜尋 Interpolation Search

內插搜尋 Interpolation search 為[二元搜尋][binary-search]的變種，差別在於二分點的選擇方法，二元搜尋選擇中間的元素作為二分點，而內插搜尋則名副其實，以內插法找尋二分點。內插法有許多種類，本次搜尋演算法選擇使用常見的[線性內插（linear interpolation）][wiki-lerp]實作。

內插搜尋的特色如下：

- 資料需要是可計算[內插（interpolation）][wiki-interp]的數值資料。
- 對資料分佈敏感，資料均勻分佈時，效能勝過二元搜尋。
- 資料分佈不均勻時，最差複雜度高達 $O(n)$。

[wiki-interp]: https://en.wikipedia.org/wiki/Interpolation
[wiki-lerp]: https://en.wikipedia.org/wiki/Linear_interpolation
[binary-search]: ../binary_search

## 步驟

1. 確認資料已經排好序。
2. 利用第一個元素 a 與最後的元素 b，以及搜尋上下界 hi 與 lo 位置，作為兩個端點。
3. 利用上述兩點 (lo, a) 與 (hi, b)，對搜尋目標計算內插，得到可能的位置。
    1. 若該位置上元素較小，則令其為新搜尋下界 a'，重複步驟二到三，繼續求內插。
    2. 若該位置上元素較大，則令其為新搜尋上界 b'，重複步驟二到三，繼續求內插。
    3. 若相等，則完成搜尋。
4. 搜尋停止在 a'、b' 兩元素搜尋位置重疊，以及目標值比下界 a' 小或比上界 b' 大。

## 說明

迅速說明線性內插法。線性內插法是中學必修的數學概念，給定兩點 $(x_0,y_0)$ 與 $(x_1,y_1)$，欲求在 $[x_0,x_1]$ 區間內直線上 $x'$ 點的 y 值，可以透過斜率公式求解：

$$
\frac{y - y_0}{x' - x_0} = \frac{y_1 - y_0}{x_1 - x_0 }
$$

接下來就是小學解方程式的事兒了。

<div style="text-align: center">
<img style="width: 400px" src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/aa/Linear_interpolation_visualisation.svg/512px-Linear_interpolation_visualisation.svg.png" />
<p><i>Cmglee - CC BY-SA 3.0</i></p>
</div>

回到正題，以下用文字解釋內插搜尋。

這裡有一個已排序有 14 個元素的序列，我們需要從中找出 **27**。

```
[1, 9, 10, 15, 17, 17, 18, 23, 27, 28, 29, 30, 31, 34]
```

我們將序列索引當作 x 軸，元素值作為 y 軸。可得已知兩點為 $(0, 1)$ 及 $(13, 34)$。

首先，透過斜率公式，計算出在 $y = 27$ 時，$x'$，也就是 27 在序列中可能的位置為 

$$x' = \lfloor 27 / (34 - 1) \cdot (13 - 0) \rfloor = 10$$

查看 `arr[10]` 為 29，比搜尋目標 27 來得大。將 29 當作我們新的上界，搜尋範變成第 [0, 9] 個元素（29 不需列入搜尋），繼續計算內插

$$x' = \lfloor 27 / (28 - 1) \cdot (9 - 0) \rfloor = 9$$

查看 `arr[9]` 為 28，比搜尋目標 27 來得大。將 28 當作我們新的上界，搜尋範變成第 [0, 8] 個元素（28 不需列入搜尋），繼續計算內插

$$x' = \lfloor 27 / (27 - 1) \cdot (8 - 0) \rfloor = 8$$

查看 `arr[8]` 為 27，恰恰是搜尋目標 27，搜尋到此結束。

## 效能

|              | Complexity |
| ------------ | ---------- |
| Worst        | $O(n)$     |
| Best         | $O(1)$     |
| Average      | $O(n)$     |
| Average      | $O(\log \log n)$ on uniform distributed data |
| Worst space  | $O(1)$      |

> $n$：資料筆數

線性內差搜尋的最差時間複雜度為 $O(n)$，也就是每次內差的結果都落在邊界旁，搜尋範圍只縮小一個元素。這種情況容易發生在資料依排序呈指數或對數等非線性函數。例如 $y = 2^x$。

線性內插搜尋對資料的期望是均勻機率分佈（uniform probability distribution）。想求平均時間複雜度 $O(\log \log n)$ ，須先透過機率密度函數，計算條件機率，一步步縮小範圍，求得平均誤差，最後求得期望值。這部分計算較為複雜，有興趣的朋友可以參考閱讀資料「[Perl, Y., Itai, A., & Avni, H. (1978). Interpolation search—a log log N search.][perl-interp-paper]」。

![](https://upload.wikimedia.org/wikipedia/commons/thumb/9/96/Uniform_Distribution_PDF_SVG.svg/320px-Uniform_Distribution_PDF_SVG.svg.png)

_PDF of uniform distribution by IkamusumeFan - CC BY-SA 3.0_

[wiki-uniform-dist]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)

## 實作

內插搜尋的實作共分為幾部分：

1. 處理空序列狀況。
2. 建立迴圈迭代共用的變數。
3. 計算線性插值的主要迴圈。
4. 將內插值映射到結果的 `Result`。

首先是函式宣告。

```rust
pub fn interpolation_search(
    arr: &[i32],
    target: &i32,
) -> Result<usize, usize>
```

映入眼簾的是 `i32`，而非泛型參數，為什麼呢？是因為內插搜尋為了計算線性內插，資料僅限定在「數值資料」，而 Rust 並沒有特別一類 **Numeric** 的型別，自己透過 trait 實作又異常繁瑣，因此先以 `i32` 代替。而回傳值的部分，與指數搜尋／二元搜尋一模一樣，回傳的 `Result`

- 若為 `Ok`，其值代表目標值在序列內的索引位置；
- 若為 `Err`，則是可以將目標值插入序列內又不會破壞排序的位置。

> 延續數值型別的話題，Rust 社群提供 [num](https://github.com/rust-num/num) crate，定義了各種數值型別與 trait，大整數、複數、虛數、有理數都囊括其中，非常有趣。

再來就是第一第二部分，處理空序列與建立共用變數，非常直觀。


```rust
    if arr.is_empty() {
        return Err(0)
    }

    let mut hi = arr.len() - 1;
    let mut lo = 0_usize;

    let mut interpolant = 0_usize;
```

- `hi`、`lo` 兩個變數劃定的搜尋範圍上下界。
- `interpolant` 儲存線性插值，代表每次迭代的搜尋位置。

接下來就是主要的迴圈，負責迭代計算內插值。分為三個部分，直接看程式碼先。

```rust
    loop {
        let lo_val = arr[lo];
        let hi_val = arr[hi];

        // 1.
        if hi <= lo || *target < lo_val || *target > hi_val {
            break
        }

        // 2. The linear interpolation part
        let offset = (*target - lo_val) * (hi - lo) as i32 / (hi_val - lo_val);
        interpolant = lo + offset as usize;

        let mid_val = arr[interpolant];

        // 3.
        if mid_val > *target {
            hi = interpolant - 1;
        } else if mid_val < *target {
            lo = interpolant + 1;
        } else {
            break
        }
    }
```

1. 迴圈的三個終止條件，分別為：
    - `hi`、`lo` 兩個變數劃定的搜尋範圍重疊，長度為零。
    - 搜尋目標值比上界還大。
    - 搜尋目標值比下界還小。
2. 線性內插的計算方程式，要注意我們是寫 Rust 不是 JavaScript，`i32` 與 `usize` 不能混用，要手動轉型。
3. 比較插值與目標值。相等則跳出迴圈；若目標大於小於插值，則縮小搜尋範圍。注意，範圍需手動加減一，排除上下界，以免無限迴圈產生。

最後一部分則是決定線性插值所得的索引位置是否為目標值，並將該值映射到 `Result` 上。

```rust
    if *target > arr[hi] {
        Err(hi + 1)
    } else if *target < arr[lo] {
        Err(lo)
    } else {
        Ok(interpolant)
    }
```

完整的程式碼如下。

```rust
pub fn interpolation_search(
    arr: &[i32],
    target: &i32,
) -> Result<usize, usize> {
    // 1. Handle empty sequence.
    if arr.is_empty() {
        return Err(0)
    }

    // 2. Setup variable storing iteration informaion.
    let mut hi = arr.len() - 1;
    let mut lo = 0_usize;

    let mut interpolant = 0_usize;

    // 3. Main loop to calculate the interpolant.
    loop {
        let lo_val = arr[lo];
        let hi_val = arr[hi];

        // 3.1. Three condition to exit the loop
        if hi <= lo || *target < lo_val || *target > hi_val {
            break
        }

        // 3.2. The linear interpolation part
        let offset = (*target - lo_val) * (hi - lo) as i32 / (hi_val - lo_val);
        interpolant = lo + offset as usize;

        let mid_val = arr[interpolant];

        // 3.3. Comparison between the interpolant and targert value.
        if mid_val > *target {
            hi = interpolant - 1;
        } else if mid_val < *target {
            lo = interpolant + 1;
        } else {
            break
        }
    }

    // 4. Determine whether the returning interpolant are equal to target value.
    if *target > arr[hi] {
        Err(hi + 1)
    } else if *target < arr[lo] {
        Err(lo)
    } else {
        Ok(interpolant)
    }
}
```

> 如同[二元搜尋][binary-search]與[指數搜尋
](../exponential_search)，未特別處理重複元素的內插搜尋，並無法預期會選擇哪一個元素。

## 變形與衍生

### Interpolation Search Tree

Interpolation search tree（IST），姑且稱它「內插搜尋樹」，是一個將內插搜尋結合樹的資料結構。如上述提及，內插搜尋達到 $O(\log \log n)$ 的搜尋時間，但僅適用於均勻機率分佈的資料。而 IST 利用動態內插搜尋，讓 1）內插搜尋樹的搜尋可以使用在更多元的**規律機率分佈**的資料中，且 2）可以達到以下的執行效能：

- $O(n)$ 空間複雜度。
- 預期有 $O(\log \log n)$ 的平攤增減節點操作時間，最差有 $(O \log n)$。
- 在規律分佈的資料中，預期搜尋時間為 $O(\log \log n)$，最差時間複雜度則為 $O((\log n)^2)$
- 線性時間的循序存取，而取得前後節點或最小值都是常數時間。

更多詳細證明可以閱讀參考資料「[Andersson, A. (1996, October). Faster deterministic sorting and searching in linear space][andersson-paper]」。

## 參考資料

- [Wiki: Interpolation search](https://en.wikipedia.org/wiki/Interpolation_search)
- [Perl, Y., Itai, A., & Avni, H. (1978). Interpolation search—a log log N search. Communications of the ACM, 21(7), 550-553.][perl-interp-paper]
- [Andersson, A. (1996, October). Faster deterministic sorting and searching in linear space. In Foundations of Computer Science, 1996. Proceedings., 37th Annual Symposium on (pp. 135-141). IEEE.][andersson-paper]
- Linear interpolation visualisation SVG By Cmglee [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0), via Wikimedia Commons.
- Probability density function of uniform distribution SVG By IkamusumeFan [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0), via Wikimedia Commons.

[andersson-paper]: https://people.mpi-inf.mpg.de/~mehlhorn/ftp/DynamicInterpolationSearch.pdf
[perl-interp-paper]: http://www.cs.technion.ac.il/~itai/publications/Algorithms/p550-perl.pdf
