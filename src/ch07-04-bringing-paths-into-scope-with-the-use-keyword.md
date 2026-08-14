## 使用 `use` 關鍵字將路徑引入作用域

[ch07-04-bringing-paths-into-scope-with-the-use-keyword.md](https://github.com/rust-lang/book/blob/79b9d15410a7b15a65cb86bcb40cbea99198a9e4/src/ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)

不得不編寫路徑來呼叫函式顯得繁瑣且重複。在示例 7-7 中，無論我們選擇 `add_to_waitlist` 函式的絕對路徑還是相對路徑，每次我們想要呼叫 `add_to_waitlist` 時，都必須指定`front_of_house` 和 `hosting`。幸運的是，有一種方法可以簡化這個過程。我們可以使用 `use` 關鍵字建立一個捷徑，然後就可以在作用域中的任何地方使用這個更短的名字。

在示例 7-11 中，我們將 `crate::front_of_house::hosting` 模組引入了 `eat_at_restaurant` 函式的作用域，而我們只需要指定 `hosting::add_to_waitlist` 即可在 `eat_at_restaurant` 中呼叫 `add_to_waitlist` 函式。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">示例 7-11: 使用 `use` 將模組引入作用域</span>

在作用域中增加 `use` 和路徑類似於在檔案系統中建立軟連線（符號連線，symbolic link）。通過在 crate 根增加 `use crate::front_of_house::hosting`，現在 `hosting` 在作用域中就是有效的名稱了，如同 `hosting` 模組被定義於 crate 根一樣。通過 `use` 引入作用域的路徑也會檢查私有性，同其它路徑一樣。

注意 `use` 只能建立 `use` 所在的特定作用域內的捷徑。示例 7-12 將 `eat_at_restaurant` 函式移動到了一個叫 `customer` 的子模組，這又是一個不同於 `use` 語句的作用域，所以函式體不能編譯。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">示例 7-12: `use` 語句只適用於其所在的作用域</span>

編譯器錯誤顯示捷徑不再適用於 `customer` 模組中：

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

注意這裡還有一個警告說 `use` 在其作用域內不再被使用！為了修復這個問題，可以將 `use` 移動到 `customer` 模組內，或者在子模組 `customer` 內通過 `super::hosting` 引用父模組中的這個捷徑。

### 建立慣用的 `use` 路徑

在示例 7-11 中，你可能會比較疑惑，為什麼我們是指定 `use crate::front_of_house::hosting`，然後在 `eat_at_restaurant` 中呼叫 `hosting::add_to_waitlist` ，而不是通過指定一直到 `add_to_waitlist` 函式的 `use` 路徑來得到相同的結果，如示例 7-13 所示。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">示例 7-13: 使用 `use` 將 `add_to_waitlist` 函式引入作用域，這並不符合習慣</span>

雖然示例 7-11 和 7-13 都完成了相同的任務，但示例 7-11 是使用 `use` 將函式引入作用域的習慣用法。要想使用 `use` 將函式的父模組引入作用域，我們必須在呼叫函式時指定父模組，這樣可以清晰地表明函式不是在本地定義的，同時使完整路徑的重複度最小化。示例 7-13 中的程式碼不清楚 `add_to_waitlist` 是在哪裡被定義的。

另一方面，使用 `use` 引入結構體、列舉和其他項時，習慣是指定它們的完整路徑。示例 7-14 展示了將 `HashMap` 結構體引入二進位制 crate 作用域的習慣用法。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">示例 7-14: 將 `HashMap` 引入作用域的習慣用法</span>

這種習慣用法背後沒有什麼硬性要求：它只是一種慣例，人們已經習慣了以這種方式閱讀和編寫 Rust 程式碼。

這個習慣用法有一個例外，那就是我們想使用 `use` 語句將兩個具有相同名稱的項帶入作用域，因為 Rust 不允許這樣做。示例 7-15 展示瞭如何將兩個具有相同名稱但不同父模組的 `Result` 型別引入作用域，以及如何引用它們。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">示例 7-15: 使用父模組將兩個具有相同名稱的型別引入同一作用域</span>

如你所見，使用父模組可以區分這兩個 `Result` 型別。如果我們是指定 `use std::fmt::Result` 和 `use std::io::Result`，我們將在同一作用域擁有了兩個 `Result` 型別，當我們使用 `Result` 時，Rust 則不知道我們要用的是哪個。

### 使用 `as` 關鍵字提供新的名稱

使用 `use` 將兩個同名型別引入同一作用域這個問題還有另一個解決辦法：在這個型別的路徑後面，我們使用 `as` 指定一個新的本地名稱或者**別名**。示例 7-16 展示了另一個編寫示例 7-15 中程式碼的方法，通過 `as` 重新命名其中一個 `Result` 型別。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">示例 7-16: 使用 `as` 關鍵字重新命名引入作用域的型別</span>

在第二個 `use` 語句中，我們選擇 `IoResult` 作為 `std::io::Result` 的新名稱，它與從 `std::fmt` 引入作用域的 `Result` 並不衝突。示例 7-15 和示例 7-16 都是慣用寫法，如何選擇都取決於你！

### 使用 `pub use` 重匯出名稱

使用 `use` 關鍵字，將某個名稱匯入當前作用域後，該名稱對此作用域之外還是私有的。若要讓作用域之外的程式碼能夠像在當前作用域中一樣使用該名稱，可以將 `pub` 與 `use` 組合使用。這種技術被稱為**重匯出**（*re-exporting*），因為在把某個專案匯入當前作用域的同時，也將其暴露給其他作用域。

示例 7-17 將示例 7-11 根模組中的 `use` 改為 `pub use` 的程式碼。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">示例 7-17: 通過 `pub use` 使名稱可從新作用域中被匯入至任何程式碼</span>

在這個修改之前，外部程式碼需要使用路徑 `restaurant::front_of_house::hosting::add_to_waitlist()` 來呼叫 `add_to_waitlist` 函式，並且還需要將 `front_of_house` 模組標記為 `pub`。現在這個 `pub use` 從根模組重匯出了 `hosting` 模組，外部程式碼現在可以使用路徑 `restaurant::hosting::add_to_waitlist`。

當你程式碼的內部結構與呼叫你程式碼的程式設計師所想像的結構不同時，重匯出會很有用。例如，在這個餐館的比喻中，經營餐館的人會想到“前臺”和“後臺”。但顧客在光顧一家餐館時，可能不會以這些術語來考慮餐館的各個部分。使用 `pub use`，我們可以使用一種結構編寫程式碼，卻將不同的結構形式暴露出來。這樣做使我們的庫井井有條，也使開發這個庫的程式設計師和呼叫這個庫的程式設計師都更加方便。在[“匯出實用的公有 API”][ch14-pub-use]部分讓我們再看另一個 `pub use` 的例子來了解這如何影響 crate 的文件。

### 使用外部包

在第二章中我們編寫了一個猜猜看遊戲。那個專案使用了一個外部包 `rand` 來生成隨機數。為了在專案中使用 `rand`，在 *Cargo.toml* 中加入瞭如下行：

<span class="filename">檔名：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

在 *Cargo.toml* 中加入 `rand` 依賴告訴了 Cargo 要從 [crates.io](https://crates.io) 下載 `rand` 和其依賴，並使其可在專案程式碼中使用。

接著，為了將 `rand` 定義引入專案包的作用域，我們加入一行 `use` 起始的包名，它以 `rand` 包名開頭並列出了需要引入作用域的項。回憶一下第二章的“生成一個隨機數”部分，我們曾將 `Rng` trait 引入作用域並呼叫了 `rand::thread_rng` 函式：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

[crates.io](https://crates.io) 上有很多 Rust 社群成員釋出的包，將其引入你自己的專案都需要一道相同的步驟：在 *Cargo.toml* 列出它們並通過 `use` 將其中定義的項引入專案包的作用域中。

注意 `std` 標準庫對於你的包來說也是外部 crate。因為標準庫隨 Rust 語言一同分發，無需修改 *Cargo.toml* 來引入 `std`，不過需要通過 `use` 將標準庫中定義的項引入專案包的作用域中來引用它們。例如，對於 `HashMap`，我們會使用以下語句：

```rust
use std::collections::HashMap;
```

這是一個以標準庫 crate 名 `std` 開頭的絕對路徑。

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>

### 使用巢狀路徑來清理大量的 `use` 列表

當需要引入很多定義於相同包或相同模組的項時，為每一項單獨列出一行會佔用原始碼大量的垂直空間。例如猜猜看章節示例 2-4 中有兩行 `use` 語句都從 `std` 引入項到作用域：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

相反，我們可以使用巢狀路徑將相同的項在一行中引入作用域。這麼做需要指定路徑的相同部分，接著是兩個冒號，接著是大括號中的各自不同的路徑部分，如示例 7-18 所示。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">示例 7-18: 指定巢狀的路徑在一行中將多個帶有相同字首的項引入作用域</span>

在較大的程式中，使用巢狀路徑從相同包或模組中引入很多項，可以顯著減少所需的獨立 `use` 語句的數量！

我們可以在路徑的任何層級使用巢狀路徑，這在組合兩個共享子路徑的 `use` 語句時非常有用。例如，示例 7-19 中展示了兩個 `use` 語句：一個將 `std::io` 引入作用域，另一個將 `std::io::Write` 引入作用域：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">示例 7-19: 通過兩行 `use` 語句引入兩個路徑，其中一個是另一個的子路徑</span>

兩個路徑的相同部分是 `std::io`，這正是第一個路徑。為了在一行 `use` 語句中引入這兩個路徑，可以在巢狀路徑中使用 `self`，如示例 7-20 所示。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">示例 7-20: 將示例 7-19 中部分重複的路徑合併為一個 `use` 語句</span>

這一行便將 `std::io` 和 `std::io::Write` 同時引入作用域。

<a id="the-glob-operator"></a>

### 通過 glob 運算子匯入項

如果希望將一個路徑下**所有**公有項引入作用域，可以指定路徑後跟 `*` glob 運算子：

```rust
use std::collections::*;
```

這個 `use` 語句將 `std::collections` 中定義的所有公有項引入當前作用域。使用 glob 運算子時請多加小心！Glob 會使得我們難以推導作用域中有什麼名稱和它們是在何處定義的。

glob 運算子經常用於測試模組 `tests` 中，這時會將所有內容引入作用域；我們將在第十一章[“如何編寫測試”][writing-tests]部分講解。glob 運算子有時也用於 prelude 模式；檢視[標準庫中的文件](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)瞭解這個模式的更多細節。

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#匯出實用的公有-api
[rand]: ch02-00-guessing-game-tutorial.html#生成一個隨機數
[writing-tests]: ch11-01-writing-tests.html#如何編寫測試
