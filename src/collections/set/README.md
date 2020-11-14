# 集合 Set

集合是一種抽象資料型別（Abstract data type），概念衍生自數學的[集合論][wiki-set-theory]，和陣列類似，為不同元素所組成的資料結構，不同在於集合有些重要的特性：

- 無序性：集合內各元素無特定排序或排序不重要。
- 互異性：集合內每個元素且只能出現一次。

一般來說，集合的實作會盡量貼近集合論中的有限集合定義，本次實作同樣遵照數學定義。

> 本次實作的程式碼置於 [`rust_algorithm_club::collections::HashSet`][doc-hash-set] API 文件中。

[doc-hash-set]: /doc/rust_algorithm_club/collections/struct.HashSet.html

## 架構設計

### 以雜湊表為底層容器

集合能以許多不同的資料結構實現，通用的集合實作多會最佳化取得、增加、移除元素等運算，這讓我們想到了「[雜湊表](../hash_map)」，雜湊表正是能將集合運算最佳化的一種資料結構，我們將借用**雜湊表作為底層儲存容器**，進一步實作集合。

你可能好奇，集合明明只有元素，並沒有**鍵值對**，跟雜湊表有啥關係？讓我們回想雜湊表一個重要的特性：每一個鍵（key）只會出現一次，利用雜湊表這個特性，即可達成上述集合兩大特性。

既然選用雜湊表作為底層容器，集合的結構體就非常簡單了，我們可以將集合想像為在 HashMap 薄薄的一層封裝。

```rust
pub struct HashSet<T>
where
    T: Hash + Eq,
{
    hash_map: HashMap<T, ()>,
}
```

比較特別的是雜湊表的鍵值，這裡使用空的 tuple `()`，也就是以 [unit type][rust-unit] 作為雜湊表之值，只要將集合的元素作為雜湊表鍵，忽略對應的值，就是貨真價實的集合結構。

### 不佔空間的 Unit Type

這樣多儲存一個 `()` 會不會造成額外的儲存空間負擔？事實上，Unit type `()` 在 Rust 語言中是一種 [Zero Sized Types (ZSTs)][rust-zst]，在編譯時期 Rust 會將 ZST 當作一種型別來操作，但真正輸出的機器碼中，ZST 並不會佔用任何運算空間。`Set = HashMap<T, ()>` 完全體現了 Rust 零成本抽象的語言特性。

[rust-zst]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts
[rust-unit]: https://doc.rust-lang.org/std/primitive.unit.html

## 基本操作

身為一個容器，集合有以下幾個基本操作：

- `new`：初始化一個集合。
- `contains`：檢查集合內有無特定元素。
- `is_empty`：檢查集合內是否沒有任何元素。
- `insert`：新增一個元素。
- `remove`：移除特定元素。
- `len`：檢查集合內的元素數目。
- `iter`：產生一個迭代集合內所有元素的迭代器。

這些基本操作的實作上，只是對雜湊表的簡單封裝，詳細實作可以參考 [HashMap](../hash_map)。

```rust
impl<T> HashSet<T>
where
    T: Hash + Eq,
{
    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    pub fn insert(&mut self, value: T) -> bool { // 1
        self.hash_map.insert(value, ()).is_none()
    }

    pub fn contains<Q>(&self, value: &Q) -> bool // 2
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.get(value).is_some()
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool // 3
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.remove(value).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> { // 4
        self.hash_map.iter().map(|(k, _)| k)
    }
}
```

1. `insert` 將元素置於 key 上，value 則設為 `()`。
2. `contains` 利用 `HashMap::get` 回傳的 `Option<()>` 判斷是否已有該的元素。
3. `remove` 同樣直接呼叫 `HashMap::remove`，並透過回傳 `Option<()>` 判斷實際上是否有移除任何元素。
4. `iter` 則直接利用 `HashMap::iter` 並捨棄 value。

## 集合運算

電腦科學的集合型別衍生自[集合論][wiki-set-theory]，當然也得符合[集合代數（set algebra）][wiki-set-algebra]的特性，和算術的加減乘除，集合也有自己的二元運算，我們會實作以下幾個基本方法：

- `intersection`：交集，A ∩ B 定義為 A 與 B 共有的元素。
- `union`：聯集，A ∪ B 定義為 A 與 B 所有的元素。
- `symmetric_difference`：對稱差集，定義為 A △ B = (A ∪ B) - (A ∩ B)，就是只出現在 A 或 B，不會在兩集合內同時出現的元素。 
- `difference`：差集，A \ B 定義為 A 中所有未在 B 中出現的元素。


![venn digrams](https://i.imgur.com/jeLakB0.png)

此外，也會介紹許多方便的方法：

- `is_disjoint`：兩集合是否不交集。
- `is_subset`：包含於 ⊆，是否為子集。
- `is_superset`：包含 ⊇，是否為超集。

哇！好多方法要實作。那就事不宜遲！

[wiki-operator-overloading]: https://en.wikipedia.org/wiki/Operator_overloading
[wiki-set-algebra]: https://en.wikipedia.org/wiki/Algebra_of_sets

### 實作聯集

#### 第一次嘗試 - 建立新的聯集集合

要取得兩個集合的聯集，最直覺的想法可能是創造一個新的集合，再把 A 和 B 兩個集合的元素通通 `insert` 進去，就像這樣：

```rust
// 利用 Clone 實現的聯集。完整實作見 bit.ly/caab7fb
pub fn union(&self, other: &HashSet<T>) -> HashSet<T> {
    // 複製一整份 other 到新的 set
    let mut new_set: HashSet<T> = other.clone();

    // 把現有的 item 一一塞到新的 set，重複的 item 會直接覆寫掉
    self.hash_map.iter().for_each(|(k, _)| {
        new_set.insert(k.clone());
    });

    // 回傳 self 與 other 的聯集
    new_set
}
```

然而，上述做法有兩個缺點：

1. 建立了一個全新的 HashSet 實例，花了額外的時間與空間
2. 必須為 `HashSet` 和 item 的型別 `T` 額外加上 `Clone` trait 的限制

因此接下來我們嘗試利用 Rust 的 `iterator` 的特性實現更節省資源的版本！

> 更多詳情可參考實作過程的[討論串][pr15-comment-427587785]與[第一次嘗試的完整實作][pr15-commit]。

[pr15-comment-427587785]: https://github.com/weihanglo/rust-algorithm-club/pull/15#issuecomment-427587785
[pr15-commit]: https://github.com/weihanglo/rust-algorithm-club/pull/15/commits/caab7fbc5323e09eb1e30ea374eb21a59f955bad

#### 第二次嘗試 - Lazy Iterator

有了第一次嘗試的經驗，第二次決定讓 `union()` 回傳一個 lazy iterator，會迭代聯集的元素，必要才從迭代器收集所有元素，再建立新的集合，如此可以節省許多運算資源。

至於實作步驟，我們可以：

1. 先製造一個迭代器，包含所有 `other` 集合的元素，但過濾掉與 `self` 共有的元素。
2. 再將 `self` 的迭代器與步驟一產生的迭代器，利用 [`Iterator::chain`][rust-iterator-chain] 連起來。


這樣其實就是 `other \ self` 這個差集，加上 `self` 自身，剛好就是聯集。程式碼如下：


```rust
// 使用 impl trait 語法，避免宣告不同迭代器型別的麻煩。
pub fn union(&self, other: &HashSet<T>) -> impl Iterator<Item = &T> {
    // 實際上就是差集
    let other_but_not_self = other.iter().filter(|item| !self.contains(item));
    // 差集 + self 本身
    self.iter().chain(other_but_not_self)
}
```

#### Lifetime Elision

但很不幸地，Compiler error E0623（甚至查不到 E0623 是什麼）。

```bash
error[E0623]: lifetime mismatch
   --> src/collections/set/mod.rs:117:48
    |
117 |     pub fn union(&self, other: &HashSet<T>) -> impl Iterator<Item = &T> {
    |                                -----------     ^^^^^^^^^^^^^^^^^^^^^^^^
    |                                |               |
    |                                |               ...but data from `other` is returned here
    |                                this parameter and the return type are declared with different lifetimes...
```

是 `self` 與 `other` 的生命週期不同導致，當這兩個集合的迭代器被 chain 起來後回傳，編譯器無法確認 Iterator 的 Item 生命週期多長。你可能很好奇為什麼 `self` 與 `other` 生命週期不同，事實上，Rust 為了讓語法輕鬆一點，允許函數省略部分生命週期標註，這個行為稱作 [Lifetime Elision][nomicon-lifetime-elision]，會在各種不同的條件下加註生命週期，其中有一條是「**每個被省略的生命週期都會成為獨立的生命週期**」。因此，`union()` 加上被省略的生命週期，會長得像：

```rust
pub fn union<'a, 'b>(&'a self, other: &'b HashSet<T>) -> impl Iterator<Item = &'a ??? &'b ???T>;
```

於是乎，編譯器無法理解 Iterator<Item = &T> 的 `&T` 到底生命週期是 'a 還是 'b，就不給編譯。

解法也很簡單，合併 `self` 與 `other` 的生命週期到同一個，不論是語意上（兩個集合至少活得一樣長）還是編譯條件都說得通。

```rust
// 加上 'a ，讓 self、other 的生命週期至少在這個函數內一樣長
pub fn union<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
    let other_but_not_self = other.iter().filter(|item| !self.contains(item));
    self.iter().chain(other_but_not_self)
}
```

> 更多 Lifetime Elision 的資訊，可以參考 [Rust 黑魔法 Nomicon][nomicon-lifetime-elision] 與 [Rust TRPL 的解釋][trpl-lifetime-elision]，還有相關的 [RFC][rfc-lifetime-elision] 可以看。
> 

[rust-iterator-chain]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain
[nomicon-lifetime-elision]: https://doc.rust-lang.org/nomicon/lifetime-elision.html
[trpl-lifetime-elision]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
[rfc-lifetime-elision]: https://github.com/rust-lang/rfcs/blob/master/text/0141-lifetime-elision.md

#### `move` closure

解決了生命週期，編譯看看就知道誰沒穿褲子。[Compiler error E0373][rustc-err-e0373]，是野生的 borrow checker！
```bash
error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
   --> src/collections/set/mod.rs:118:53
    |
118 |         let other_but_not_self = other.iter().filter(|item| !self.contains(item));
    |                                                     ^^^^^^  ---- `self` is borrowed here
    |                                                     |
    |                                                     may outlive borrowed value `self`
    |
note: closure is returned here
   --> src/collections/set/mod.rs:119:9
    |
119 |         self.iter().chain(other_but_not_self)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
    |
118 |         let other_but_not_self = other.iter().filter(move |item| !self.contains(item));
```

讓我們嘗試理解，當我們回傳 iterator 時，整個過濾共同元素的 closure 會連帶一起回傳，若 closure 沒有使用 `move` 關鍵字修飾，編譯器會嘗試以傷害最小的方式去捕獲變數，在這裡會是 immutable borrow `&T` 捕獲。這裡的 `self` 實際型別是 `&'a mut HashSet<T>`，可以想像成 closure 捕獲了 `&(&'a mut HashSet<T>)`。

> Rust 編譯器遇到 closure 需要捕獲變數時，如果沒有用 `move` 修飾，會嘗試使用以下順序來捕獲： `&T` > `&mut T` > `T`。若用了 `move` 修飾，則會直接將所有權轉移進 clousure 中，也就是捕獲 `T`。

可惜的是，多出來的這層 borrow 並沒有相同的生命週期 `'a`，編譯器無法識別它會活到什麼時候，可能 `self` 的資源已被釋放，但 closure 還沒結束，有機會產生 [dangling pointer][wiki-dangling-pointer]，編譯器因此禁止這種危險操作。


編譯器也十分好心地提示，只要使用 `move` 關鍵字，將 `self` 的所有權轉移至 closure 中，就能避免生命週期不一致的問題。你可能有些疑惑，把 `self` move 進 closure 之後，`self` 不就會被 drop 釋放掉了？可以這樣理解，轉移進 closure 的型別是整個 `self`，也就是 `&'a mut HashSet<T>` 型別，解讀為編譯器將 `self` 的型別看作新的 owned type，所有權可合法轉移，但底層仍保留指向原始資料的 borrow，巧妙避開生命週期問題。

```rust
pub fn union<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
    // 用 move 修飾 closure
    // self（&'a HashSet<T>）被整個轉移進 closure
    let other_but_not_self = other.iter().filter(move |item| !self.contains(item));
    self.iter().chain(other_but_not_self)
}
```

[rustc-err-e0373]: https://doc.rust-lang.org/error-index.html#E0373
[wiki-dangling-pointer]: https://en.wikipedia.org/wiki/Dangling_pointer

### 實作交集、差集與對稱差集

呼，上面解決了最困難的生命週期和 borrow checker 問題，接下來的實作只要關注數學上的集合定義就能輕鬆解決了。

首先，交集的定義為「你有，而且我也有的」，簡單，迭代 `self` 並過濾出 `other` 也有的元素就好，秒殺！

```rust
pub fn intersection<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
    self.iter().filter(move |item| other.contains(item))
}
```

再來是差集，概念是就是「我有，但你沒有的」，一樣迭代 `self` 並過濾出 `other` 沒有的元素。

```rust
pub fn difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
    self.iter().filter(move |item| !other.contains(item))
}
```

剛剛實作 `union` 有用到差集，我們可以稍微改寫，讓 `union` 的程式碼更神清氣爽。

```rust
pub fn union<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
    // self ∪ (other \ self)
    self.iter().chain(other.difference(self))
}
```

最後，對稱差集可以透過組合上面的操作算出，可以是：「我有加上你有的，減去我們共同有的」，也可以是「我有但你沒有的，加上你有但我沒有的」，這裡我們選擇第二種實作。

```rust
pub fn symmetric_difference<'a>(&'a self, other: &'a HashSet<T>) -> impl Iterator<Item = &T> {
    // (self \ other) ∪ (other \ self)
    self.difference(other).chain(other.difference(self))
}
```

整個集合的基礎在這邊大功告成了！

### 子集、超集與不交集

經歷 Rust 編譯器的摧殘，來實作比較簡單的方法吧。集合運算常要比較兩個集合的關係，例如 A ⊆ B 代表 A 是 B 的子集，定義是 A 裡面的元素 B 都有。

我們先來實作 `is_subset` 檢查 `self` 是否為 `other` 的子集。

```rust
pub fn is_subset(&self, other: &HashSet<T>) -> bool {
    // 若 self 的元素比 other 多，就不可能是 self ⊆ other
    // 所以提前回傳 false
    if self.len() > other.len() {
        return false;
    }
    // 利用 all 確認 other 內包含 self 的所有元素
    self.iter().all(|item| other.contains(&item))
}
```

`is_superset` 檢查 `self` 是否為 `other` 的超集就更簡單了，只要把 `is_subset` 反過來使用就行了。

```rust
pub fn is_superset(&self, other: &HashSet<T>) -> bool {
    // self ⊇ other = other ⊆ self
    other.is_subset(self)
}
```

最後，我們常會檢查兩個集合是否沒有交集（disjoint），這個方法只要檢查交集 `intersection()` 迭代器的長度是否為 0 就可以了。

```rust
pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
    self.intersection(other).count() == 0
}
```

## 二元運算與運算子

上面的方法好冗，能不能和 [Python][python-set] 一樣，用簡單明瞭的語法操作集合的二元運算？當然行，Rust 的表達性很強，完全不輸 Python，透過[運算子多載（operator overloading）][wiki-operator-overloading]。

### 運算子多載

Rust 提供許多 trait 供使用者多載運算子，可以簡化集合的二元運算：

- `set | other`：bitwise or 運算子，效果同 `union` 聯集。
- `set & other`：bitwise and 效果同 `intersection` 交集。
- `set - other`：substraction 運算子，效果同 `difference` 差集。
- `set ^ other`：bitwise xor 運算子，效果同 `symmetric_difference` 對稱差集。

由於四個運算子實作的概念相同，這裡挑 `|` **bitwise or** 來解釋如何客製化運算子邏輯。

```rust
// 實作 BitOr 多載 `|` 運算子
impl<'a, 'b, T> BitOr<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone, // 實作 Clone 讓 Set 可透過 FromIterator 建立實例
{
    type Output = HashSet<T>;

    fn bitor(self, rhs: &'b HashSet<T>) -> Self::Output {
        // 利用 FromIterator 提供的 collect() 蒐集元素，產生新 Set
        self.union(&rhs).cloned().collect()
    }
}
```

1. 要多載 bit or 運算子，代表型別要實作 `BitOr` trait。
2. 由於運算子的 required method 通常會 consume `self` 的所有權，因此我們要在 impl 動手腳，改以 `&HashSet<T>` 作為 `BitOr` 實作的對象。
3. 為了簡化實作，交集聯集等運算子改為產生一個新的 Set 實例，也就是說，`T` 泛型型別需要實現 `Clone` trait，才能複製舊的值產生新的 Set。

> 多載運算子，可以參考 [Overloadable operators][rust-ops] 一頁的說明。

[rust-ops]: https://doc.rust-lang.org/beta/std/ops/index.html

### 比較運算子

除了交集、聯集等運算，我們還可以實作集合間的比較，作為檢查是否為子集或超集的運算子。

- `A <= B`：效果同 `is_subset`，A 是否為 B 的子集 A ⊆ B。
- `A < B`：A 是否為 B 的子集且不等於 B，等同於 A ⊂ B。
- `A >= B`：效果同 `is_superset`，A 是否為 B 的超集 A ⊇ B。
- `A > B`：A 是否為 B 的超集且不等於 B，等同於 A ⊃ B。

但眼尖的 Rustacean 肯定會發現，[`std::ops`][rust-ops] 裡面根本沒有 `lt`、`gt` 等比較運算子。Rust 的「比較」是透過實作幾個 Trait 後，自動推導生成的方法，這些 trait 放在 [`std::cmp`][rust-cmp] module 中，分別是[`Eq`][rust-eq]、[`ParitalEq`][rust-partialeq]、[`Ord`][rust-ord]，以及 [`ParitalOrd`][rust-partialord]。

在開始介紹如何實作比較前，先讓複習一下離散數學學到的二元關係：

若 ∼ 為一種二元關係，A 為任意集合。

- 自反性（Reflexive）：對所有 x ∈ A : x ∼ x。 
- 對稱性（Symmetric）：對所有 x, y ∈ A ，若 x ∼ y 則 y ∼ x。
- 傳遞性（Transitive）：對所有 x, y, z ∈ A ，若 x ∼ y 且 y ∼ z 則 x ∼ z。
- 反對稱（Antisymmetric）：對所有 x, y ∈ A，若 x ∼ y 且 x ≠ y 則 y ∼ x 不成立。

Rust 中的相等關係有其理論背景，`Eq` 就是數學上的 [Equivalence relation][wiki-total-eq]，須符合自反性、對稱性，及傳遞性；與之對應的是 `PartialEq`，[Partial equivalence][wiki-partial-eq] 具有對稱性和傳遞性，但並無自反性，有名的例子是 [IEEE754 的浮點數][ieee-754] 定義了 `NaN == NaN -> false`，浮點數因此不符合自反性定義。

回到集合，集合論中的集合相等（set equality）定義為：x = y ⇒ ∀z, (z ∈ x ⇔ z ∈ y)，也就所有屬於集合 x 的元素必屬於集合 y，反之亦然。因此，集合相等具有自反性、對稱性、傳遞性。實作 `==` 運算子，我們會

1. 比較集合 x, y 內元素數目（cardinality）是否一致，以及
2. 迭代集合 x，並檢查是否每個屬於 x 的元素都屬於 y。

```rust
impl<T> PartialEq for HashSet<T>
where
    T: Hash + Eq,
{
    fn eq(&self, other: &HashSet<T>) -> bool {
        // 1. 檢查 cardinality，不同長度就不可能相等
        if self.len() != other.len() {
            return false;
        }
        // 2. 利用 Iterator::all 確保每個 self 的元素都屬於 other。
        self.iter().all(|item| other.contains(&item))
    }
}

/// `Eq` 並沒有 required method，只要實作 `Partial::eq` 方法，就能直接推斷出 `Eq`。
impl<T> Eq for HashSet<T> where T: Hash + Eq {}
```

與相等關係相同，Rust 的排序關係同樣有理論依據，`Ord` 是數學上的 [Total order][wiki-total-ord]，符合反對稱性、傳遞性，以及 [connex relation][wiki-connex-relation]；而 `ParitalOrd` 則接近數學上的 partial order，Rust 的文件中描述該 trait 須符合反對稱性與傳遞性。

> Connex relation：在集合 X 下，所有 (x, y) pair 都會符合 x ∼ y 或 y ∼ x 的關係。在排序關係上，意指不是 x ≥ y 就是 y ≥ x。

要把集合的「包含於但不相等」關係 ⊂ 映射到排序關係 `x < y` 前，先來檢驗 ⊂ 有什麼特性。

- 具反對稱性：若 x ⊂ y 且 x ≠ y 則 y ⊄ x，換句話說，若 x ⊂ y 且 y ⊂ x 則 x = y。
- 具傳遞性：若 x ⊂ y 且 y ⊂ z 則 x ⊂ z。
- 不具 connex relation：若 x = {1,2}, y = {3,4}，則 x, y 無法以 ⊂ 表示兩者間的關係。

很明顯地，「包含於但不相等」符合 partial order 但不是 total order。我們選擇實作 `PartialOrd` trait，其有一個 required method `PartialOrd::partial_cmp`。

1. `partial_cmp` 回傳 `Option<Ordering>`，因為兩個集合可能無交集，導致無法相互比較。
2. 先檢查 `other` 是不是子集，再檢查是不是長度相同，得到兩個 `bool`。
3. 有了上述兩個 `bool`，就可以用 pattern matching 把所有情況列舉出來。
    - 是子集且同長度：相等 `=`。
    - 是子集但長度不同：包含於 ⊂（`<`）。
    - 不是子集但長度相同：不相交（disjoint）。
    - 不是子集且長度不同：先假設 self 是 other 的超集，再透過 [`Option::filter`][rust-option-filter] 過濾，是超集則回傳 `Some(Ordering::Greater)`，不是則回傳 `None`。

```rust
impl<T> PartialOrd for HashSet<T>
where
    T: Hash + Eq,
{
    fn partial_cmp(&self, other: &HashSet<T>) -> Option<Ordering> { // 1
        let is_subset = self.is_subset(other);                      // 2
        let same_size = self.len() == other.len();
        match (is_subset, same_size) {                              // 3
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => None,
            _ => Some(Ordering::Greater).filter(|_| self.is_superset(other)),
        }
    }
}
```

實作 `PartialEq`，`Eq` 與 `PartialOrd` 後，我們的集合型別終於能和 Python 的集合互別苗頭，有更高層次的表達性！

> 有人可能會認為，比較運算還要透過 `partial_cmp` 判斷 `Option` 多麻煩，事實上，C++ 20 也帶來了 `<=>` 運算子以及 [three way comparison][cpp-3way-cmp] 衍生的各種型別，partial order 或 parital equal 可說是更精確且必要的比較運算，也是一種趨勢。

[rust-cmp]: https://doc.rust-lang.org/std/cmp/index.html
[rust-eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[rust-partialeq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[rust-ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[rust-partialord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[ieee-754]: https://ieeexplore.ieee.org/document/4610935
[wiki-partial-eq]: https://en.wikipedia.org/wiki/Partial_equivalence_relation
[wiki-total-eq]: https://en.wikipedia.org/wiki/Equivalence_relation
[wiki-total-ord]: https://en.wikipedia.org/wiki/Total_order
[wiki-connex-relation]: https://en.wikipedia.org/wiki/Connex_relation
[rust-option-filter]: https://doc.rust-lang.org/std/option/enum.Option.html#method.filter
[cpp-3way-cmp]: https://en.cppreference.com/w/cpp/language/operator_comparison#Three-way_comparison

## 效能

以雜湊表為底層儲存容器的集合，各操作複雜度如下

| Operation    | Best case    | Worst case |
| ------------ | ------------ | ---------- |
| insert(v)    | $O(1)$~  | $O(n)$ |
| remove(v)    | $O(1)$~  | $O(n)$ |
| contains(v)  | $O(1)$   | $O(n)$ |
| union        | $O(n)$   | $O(n)$ |
| intersection | $O(n)$   | $O(n)$ |
| difference   | $O(n)$   | $O(n)$ |
| symmetric difference | $O(n)$   | $O(n)$ |

> $n$：資料筆數。  
> $v$：資料值。  
> **~**：平攤後的複雜度（amortized）。

操作的時間與空間複雜度，與其底層儲存容器的實作有關，本次集合實作只是對雜湊表的簡單封裝，詳細演算法複雜度可以參考 [HashMap](../hash_map#效能)。

## 參考資料

- [Rust Documentation: HashSet](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html)
- [Python 3: Set][python-set]
- [Wiki: Set theory][wiki-set-theory]
- Venn diagrams are screenshoot from Wikipedia via public domain.

[python-set]: https://docs.python.org/3/library/stdtypes.html#set
[wiki-set-theory]: https://en.wikipedia.org/wiki/Set_theory
