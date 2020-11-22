# 漢明距離 Hamming distance

漢明距離（Hamming distance）是指兩個相同長度的序列（sequence）在相同位置上，有多少個數值不同，對二進位序列來說就是「相異位元的數目」。漢明距離同時也是一種編輯距離，即是將一個字串轉換成另一個字串，需要經過多少次置換操作（substitute）。

漢明距離多應用於編碼理論中的錯誤更正（error-correcting），漢明碼（Hammming code）中計算距離的演算法即為漢明距離。

> 本次實作的程式碼置於
>
> - [`rust_algorithm_club::hamming_distance`][doc-hamming-dist]
> - [`rust_algorithm_club::hamming_distance_str`][doc-hamming-dist-naive]
>
> API 文件中。

[doc-hamming-dist]: /doc/rust_algorithm_club/fn.hamming_distance.html
[doc-hamming-dist-str]: /doc/rust_algorithm_club/fn.hamming_distance_str.html

## 位元版實作

計算相異位元的數目其實就是一堆位元運算，如下：

```rust
{{#include mod.rs:bit}}
```

1. 透過 XOR 操作，讓兩序列相異位元為 1，相同位元為 0。
2. 如果 XOR 操作不為零，表示還有相異位元，繼續計算。
3. 將 XOR 結果和 1 做 AND 運算，看最低有效位（least significant digit）是否為 1。
4. 將 XOR 做位元右移，捨棄最低有效位，並回到步驟二。

> 根據 [《The Rust Reference》][] 指出，Rust 的位元右移在
>
> - 無符號整數（unsigned）是邏輯右移（logical right shift），也就是直接在最高有效位補 0；
> - 有符號整數（signed）則是算術右移（arithmetic right shift），右移之後符號會被保留。

[《The Rust Reference》]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators

實際上，Rust 提供了一個原生的計算整數有多少個零的方法 [`{integer_type}::count_ones`][]，可以省去自己做位元運算的麻煩，實作如下，帥：

```rust
pub fn hamming_distance(source: u64, target: u64) -> u32 {
     (source ^ target).count_ones()
}
```

[`{integer_type}::count_ones`]: https://doc.rust-lang.org/stable/std/?search=count_ones

## 字串版實作

字串版的漢明距離就相對好懂了。

```rust
{{#include mod.rs:str}}
```

字串版同樣吃 `source` 和 `target` 兩個輸入。

1. 用 [`str::chars`][] 讓漢明距離可以比對 Unicode 字串，而非只有 ASCII，而 `str::chars` 是 `Iterator`，所以透過 `Iterator::next` 逐一比較每個字元。
2. 這裡 `if c1 != c2` 叫做 [match guard][]，是在模式匹配之外，額外條件式檢查，因此，只有 `source` 和 `target` 都有下一個字元而且兩字元不相等才會進入這個匹配分支。
3. 若有任何一個是 `None`，另外一個是 `Some`，標示輸入字串的長度不同，直接噴錯。
4. 如果都沒有下一個字元，直接結束迴圈。
5. 其他情況，例如兩個字元相同，就繼續疊代。

[`str::chars`]: http://doc.rust-lang.org/std/primitive.str.html#method.chars
[match guard]: https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards

## 效能

長度為 n 的序列，計算漢明距離的時間複雜度為 $O(n)$，空間複雜度為 $O(1)$。

## 參考資料

- [Wiki: Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
- [演算法筆記：Correction](https://web.ntnu.edu.tw/~algo/Correction.html)
