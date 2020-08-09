# 貢獻指南

感謝您有興趣貢獻 Rust 演算法俱樂部。我們歡迎各種形式的協助。這裡列出幾種任務供你挑選。

- 增加新的演算法
- 修正已知的漏洞
- 改善文件的品質

接下來，將介紹幾個貢獻的注意事項。

## 開始貢獻之前

若您決定著手做些厲害的事，請先在[已知 issues 與 pull requests][issues] 搜尋，那裡可能已有回報相似的問題。

若沒有重複的問題，請發起一個「進行中（work-in-progress）」的 issue，告知其他人你正在做這項功能。你的時間很寶貴，必須防止重工發生。維護團隊也會追蹤這些 issue 以利管理俱樂部。

有些 meta issue 專門追蹤尚未完成的工作 🚧，可以去看看是否有感興趣的主題。

## 提交你的成果

在提交你的貢獻之前，確認成果滿足下列需求：

- 不要搞壞既有測試。發起 pull request 前執行 `cargo test`。新的演算法也需包含自身的單元測試。
- 每個對外介面都需要有文件。這個文件不需要完美無缺，但至少清楚說明它的目的與用法。
- 儘量維持文章間寫作風格與結構一致。例如：首段需包含簡扼的敘述、解釋效能時請愛用漸進符號。
- 程式碼撰寫風格應貼近 Rust 的慣例，例如：涉及所有權轉移請使用 `into`、替額外建構式命名請添加 `with` 前綴。目前為止，並不強制使用 [Clippy][rust-clippy] 與 [rustfmt][rust-fmt]。

[issues]: https://github.com/weihanglo/rust-algorithm-club/search?q=&type=Issues&utf8=%E2%9C%93
[rust-clippy]: https://github.com/rust-lang-nursery/rust-clippy
[rust-fmt]: https://github.com/rust-lang-nursery/rustfmt

### 歡迎加入 Rust 演算法俱樂部，願演算法與你同在！
