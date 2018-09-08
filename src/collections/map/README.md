# 關聯容器 Associative Container

關聯容器是一種抽象資料型別，儲存鍵與值的配對關係（key-value pair）的集合，並透過鍵存取元素。關聯容器有許多別名，例如字典（dictionary）、關聯陣列（associative array）、映射（map）、表（table）等。在大多數程式語言函式庫中，關聯容器通常是最基本的容器型別之一，如 Python 的 `dict`，JavaScript 的 `Map`，以及 Rust 的 `HashMap`。

方便起見，本文以「**映射表**」統稱這類集合型別。

## 特性

一般來說，映射表有以下特性：

- **鍵值對為單向關係**：可透過鍵取得其唯一值；但無法確保一值僅對應唯一的鍵。
- **鍵值唯一性**：同個映射表內，同個鍵不重複，只會出現一次。
- **元素組合性**：映射表內每個元素都是「鍵值對」，鍵或值無法單獨存在。
- **操作開銷小**：合理實作下，基本操作開銷相對較小，不高於線性時間。

> 註：多重映射表為一對多的例外。

映射表會有以下幾種基本操作

- **新增 Addition**：配對鍵值關聯，又稱為綁定 binding。
- **修改 Modification**：修改任意鍵之下的值。
- **移除 Removal**：透過任意鍵移除該鍵值對，又稱 unbinding。
- **查找 Lookup**：透過任意鍵搜尋該鍵值對。

不難看出，基本操作都是透過鍵取得值。事實上，合理實作的映射表，只要透過鍵來操作，就能有良好效能，甚至上述操作能達到 \\(O(1)\\) 複雜度。

## 適用場景

雖然映射表依實作不同，效能有所權衡。但其最大優勢仍是可「高效地透過鍵尋找值」，只要有映射關係的資料，都非常適合使用映射表。例如，快取暫存機制需透過特定鍵快速查找暫存值。此外，現代常用的 JSON、TOML 等資料交換格式，都是「鍵——值對」的形式，非常適合使用映射表處理。

總的來說，只要資料有對應綁定關係，例如學號與學生，就可以考慮使用映射表處理。

## 種類

以下簡單介紹常見的映射表，詳情請點擊各連結。

### 雜湊表 Hash Map

[雜湊表](hash_map.md)是以雜湊函數實作的映射表。透過[雜湊函數][hash-function]將任意資料轉換為固定長度的雜湊值，並將此鍵與一筆資料綁定，再映射到內部資料結構的某位置。理論上，只要雜湊函數品質過得去，雜湊表的基本操作都能在常數時間完成。

[hash-function]: https://en.wikipedia.org/wiki/Hash_function

### 有序映射表 Ordered Map

[有序映射表](ordered_map.md)係一種有特定排序方式的映射表。常見兩種排序方式，其一是依照插入映射表的先後順序；其二則是依照鍵的大小。不同排序的底層資料結構各異，操作複雜度也不盡相同，如依鍵大小排序的映射表通常使用搜索樹實作，因此「新增」操作的複雜度為較差的 \\(O(\log n)\\)。

### 多重映射表 Multimap

[多重映射表](multimap.md)允許鍵值對重複，一個鍵可對應多個值（一對多）。類似於映射表內放入陣列，但能以較方便輕鬆的接口操作或迭代整張映射表。

### 集合 Set

[集合](set.md)實際上並無鍵值「關聯」，可將其想像成普通的映射表。只關心鍵而值不重要。集合借用了數學[集合論（set theory）][set-theory]中有限集合的概念，常應用於需要操作交集、聯集、差集等集合運算場景。

[set-theory]: https://en.wikipedia.org/wiki/Set_theory

## 參考資料

- [Wiki: Associative array](https://en.wikipedia.org/wiki/Associative_array)
- [Wiki: Associative continers](https://en.wikipedia.org/wiki/Associative_containers)
- [cpprefernce.com: std::map](https://en.cppreference.com/w/cpp/container/map)
- [Rust documentation: std::colledtion](https://doc.rust-lang.org/stable/std/collections/)
