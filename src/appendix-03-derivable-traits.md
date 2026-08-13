## 附錄 C：可派生的 trait

[appendix-03-derivable-traits.md](https://github.com/rust-lang/book/blob/042803ac0fc63e2cb62a4e3b7aaedec018c583f9/src/appendix-03-derivable-traits.md)

在本書的各個部分中，我們討論了可應用於結構體和列舉定義的 `derive` 屬性。`derive` 屬性會在使用 `derive` 語法標記的型別上生成對應 trait 的預設實現的程式碼。

在本附錄中提供了標準庫中所有可以使用 `derive` 的 trait 的參考。這些部分涉及到：

- 該 trait 將會派生什麼樣的運算子和方法
- 由 `derive` 提供什麼樣的 trait 實現
- 實現該 trait 對型別意味著什麼
- 在何種條件下允許或不允許實現該 trait
- 需要 trait 操作的例子

如果你希望不同於 `derive` 屬性所提供的行為，請查閱[標準庫文件](https://doc.rust-lang.org/std/index.html) 中每個 trait 的細節以瞭解如何手動實現它們。

這裡列出的 trait 是僅有的在標準庫中定義且能透過 `derive` 在型別上實現。標準庫中定義的其它 trait 不能透過 `derive` 在型別上實現。這些 trait 不存在有意義的預設行為，所以由你負責以合理的方式實現它們。

一個無法被派生的 trait 的例子是為終端使用者處理格式化的 `Display` 。你應該時常考慮使用合適的方法來為終端使用者顯示一個型別。終端使用者應該看到型別的什麼部分？他們會找出相關部分嗎？對他們來說最相關的資料格式是什麼樣的？Rust 編譯器沒有這樣的洞察力，因此無法為你提供合適的預設行為。

本附錄所提供的可派生 trait 列表並不全面：庫也可以為它們自己的 trait 實現 `derive`，因此可以與 `derive` 搭配使用的 trait 列表實際上是開放的。實現 `derive` 需要用到過程宏，這在第二十章的 [“自定義 `derive` 宏”][custom-derive-macros] 一節中有介紹。

### 用於程式設計師輸出的 `Debug`

`Debug` trait 用於開啟格式化字串中的除錯格式，其透過在 `{}` 佔位符中增加 `:?` 表明。

`Debug` trait 允許以除錯目的來列印一個型別的例項，所以使用該型別的程式設計師可以在程式執行的特定時間點觀察其例項。

例如，在使用 `assert_eq!` 宏時，`Debug` trait 是必需的。如果等式斷言失敗，這個宏就把給定例項的值作為引數打印出來，如此程式設計師可以看到兩個例項為什麼不相等。

### 等值比較的 `PartialEq` 和 `Eq`

`PartialEq` trait 可以比較某個型別的例項以檢查是否相等，並開啟了 `==` 和 `!=` 運算子的功能。

派生的 `PartialEq` 實現了 `eq` 方法。當 `PartialEq` 在結構體上派生時，只有**所有**的欄位都相等時兩個例項才相等，同時只要有任何欄位不相等則兩個例項就不相等。當在列舉上派生時，每一個變體都和其自身相等，且和其它變體都不相等。

例如，當使用 `assert_eq!` 宏時，需要比較一個型別的兩個例項是否相等，則 `PartialEq` trait 是必須的。

`Eq` trait 沒有方法。其作用是表明每一個被標記型別的值等於其自身。`Eq` trait 只能應用於那些實現了 `PartialEq` 的型別，但並非所有實現了 `PartialEq` 的型別都可以實現 `Eq`。浮點型別就是一個例子：浮點數的實現表明兩個非數字（`NaN`，not-a-number）值是互不相等的。

例如，對於一個 `HashMap<K, V>` 中的鍵（key）來說，`Eq` 是必須的，這樣 `HashMap<K, V>` 就可以知道兩個鍵是否相等了。

### 排序比較的 `PartialOrd` 和 `Ord`

`PartialOrd` trait 可以基於排序的目的而比較一個型別的例項。實現了 `PartialOrd` 的型別可以使用 `<`、 `>`、`<=` 和 `>=` 運算子。但只能在同時實現了 `PartialEq` 的型別上使用 `PartialOrd`。

派生 `PartialOrd` 實現了 `partial_cmp` 方法，其返回一個 `Option<Ordering>`，但當給定值無法產生順序時將返回 `None`。儘管大多數型別的值都可以比較，但一個無法產生順序的例子是：浮點型別的非數字值（not-a-number，`NaN`）。對任何浮點數與 `NaN` 呼叫 `partial_cmp` 都會返回 `None`。

當在結構體上派生時，`PartialOrd` 按照結構體定義中欄位出現的順序，依次比較每個欄位的值，以此來比較兩個例項。當在列舉上派生時，認為在列舉定義中宣告較早的列舉變體小於其後的變體。

例如，對於來自於 `rand` crate 中的 `gen_range` 方法來說，當在一個範圍表示式指定的範圍內生成一個隨機值時，`PartialOrd` trait 是必須的。

`Ord` trait 也讓你知道在一個帶註解型別上的任意兩個值存在有效順序。`Ord` trait 實現了 `cmp` 方法，它返回一個 `Ordering` 而不是 `Option<Ordering>`，因為總存在一個合法的順序。只可以在實現了 `PartialOrd` 和 `Eq`（`Eq` 依賴 `PartialEq`）的型別上使用 `Ord` trait。當在結構體或列舉上派生時，`cmp` 的行為與 `PartialOrd` 派生實現的 `partial_cmp` 相同。

例如，將值儲存到 `BTreeSet<T>` 中時，需要 `Ord` trait，因為該資料結構基於值的排序順序來儲存資料。

### 複製值的 `Clone` 和 `Copy`

`Clone` trait 可以明確地建立一個值的深複製（deep copy），複製過程可能包含任意程式碼的執行以及堆上資料的複製。查閱第四章 [“使用克隆的變數與資料互動”][variables-and-data-interacting-with-clone] 以獲取有關 `Clone` 的更多資訊。

派生 `Clone` 實現了 `clone` 方法，當其為整個型別實現時，會在型別的每一部分上呼叫 `clone` 方法。這意味著型別中所有欄位或值也必須實現了 `Clone`，這樣才能夠派生 `Clone` 。

例如，當在一個 slice 上呼叫 `to_vec` 方法時，`Clone` 是必須的。slice 並不擁有其包含的例項，但是從 `to_vec` 中返回的 vector 需要擁有它們的例項，因此 `to_vec` 在每個元素上呼叫 `clone`。所以儲存在切片中的型別必須實現 `Clone`。

`Copy` trait 允許你透過只複製儲存在棧上的位來複制值；無需執行額外的程式碼。查閱第四章 [“只在棧上的資料：複製”][stack-only-data-copy] 的部分來獲取有關 `Copy` 的更多資訊。

`Copy` trait 並未定義任何方法來阻止程式設計人員重寫這些方法或違反無需執行額外程式碼的假設。這樣，所有程式設計師都可以假定複製值會非常快速。

可以在型別內部全部實現 `Copy` trait 的任意型別上派生 `Copy`。一個實現了 `Copy` 的型別必須也實現了 `Clone`，因為一個實現了 `Copy` 的型別也簡單地實現了 `Clone`，其執行和 `Copy` 相同的任務。

`Copy` trait 很少是必需的；實現 `Copy` 的型別是有最佳化的，這意味著你無需呼叫 `clone`，這讓程式碼更簡潔。

任何使用 `Copy` 的程式碼都可以透過 `Clone` 實現，但程式碼可能會稍慢，或者不得不在程式碼中的許多位置上使用 `clone`。

### 固定大小的值到值對映的 `Hash`

`Hash` trait 可以例項化一個任意大小的型別，並且能夠用雜湊（hash）函式將該例項對映到一個固定大小的值上。派生 `Hash` 實現了 `hash` 方法。`hash` 方法的派生實現結合了在型別的每部分呼叫 `hash` 的結果，這意味著所有的欄位或值也必須實現了 `Hash`，這樣才能夠派生 `Hash`。

例如，在 `HashMap<K, V>` 上儲存資料，存放 key 的時候，`Hash` 是必須的。

一個 `Hash` 是必須的例子是在 `HashMap<K, V>` 中儲存鍵來高效地儲存資料。

### 預設值的 `Default`

`Default` trait 使你建立一個型別的預設值。派生 `Default` 實現了 `default` 函式。`default` 函式的派生實現呼叫了型別每部分的 `default` 函式，這意味著型別中所有的欄位或值也必須實現了 `Default`，這樣才能夠派生 `Default` 。

`Default::default` 函式通常結合結構體更新語法一起使用，這在第五章的 [“使用結構體更新語法建立例項”][creating-instances-from-other-instances-with-struct-update-syntax] 部分有討論。可以自定義一個結構體的一小部分欄位而剩餘欄位則使用 `..Default::default()` 來設定預設值。

例如，當你在 `Option<T>` 例項上使用 `unwrap_or_default` 方法時，`Default` trait 是必須的。如果 `Option<T>` 是 `None` 的話，`unwrap_or_default` 方法將返回儲存在 `Option<T>` 中 `T` 型別的 `Default::default` 的結果。

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-defining-structs.html#使用結構體更新語法建立例項
[stack-only-data-copy]: ch04-01-what-is-ownership.html#只在棧上的資料複製
[variables-and-data-interacting-with-clone]: ch04-01-what-is-ownership.html#使用克隆的變數與資料互動
[custom-derive-macros]: ch20-05-macros.html#自定義-derive-宏
