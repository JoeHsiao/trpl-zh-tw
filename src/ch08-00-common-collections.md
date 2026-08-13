# 常見集合

[ch08-00-common-collections.md](https://github.com/rust-lang/book/blob/2581c23b669eff30c26e036a13475ec5cf70c1b8/src/ch08-00-common-collections.md)

Rust 標準庫中包含一些非常有用的資料結構，它們被稱為 **集合**（*collections*）。大多數其他資料型別表示的是一個特定的值，而集合可以包含多個值。與內建的陣列和元組型別不同，這些集合指向的資料儲存在堆上，這意味著資料量不需要在編譯時已知，並且可以隨著程式執行而增長或縮小。每一種集合都有不同的能力和開銷，而根據當前場景選擇合適的集合，是一項你會隨著時間逐漸掌握的技能。本章將討論 Rust 程式中非常常用的三種集合：

- **向量**（*vector*）允許你把數量可變的值一個挨一個地存放起來。
- **字串**（*string*）是字元的集合。此前我們已經提到過 `String` 型別，不過本章會更深入地討論它。
- **雜湊對映**（*hash map*）允許你把某個值與特定的鍵關聯起來。它是更通用的資料結構 *map* 的一種具體實現。

要了解標準庫提供的其他集合型別，請參閱[文件][collections]。

我們將討論如何建立和更新 vector、字串和雜湊對映，以及它們各自的特別之處。

[collections]: https://doc.rust-lang.org/std/collections/index.html
