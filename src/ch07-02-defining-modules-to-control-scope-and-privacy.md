<a id="defining-modules-to-control-scope-and-privacy"></a>

## 定義模組來控制作用域與私有性

[ch07-02-defining-modules-to-control-scope-and-privacy.md](https://github.com/rust-lang/book/blob/8c0eacd5c4acbb650497454f3a58c9e8083202a4/src/ch07-02-defining-modules-to-control-scope-and-privacy.md)

在本節，我們將討論模組和其它一些關於模組系統的部分，如允許你命名項的 *路徑*（*paths*）；用來將路徑引入作用域的 `use` 關鍵字；以及使項變為公有的 `pub` 關鍵字。我們還將討論 `as` 關鍵字、外部包（external packages）和 glob 運算子（glob operator）。

首先，我們將從一系列的規則開始，在你未來組織程式碼的時候，這些規則可被用作簡單的參考。接下來我們將會詳細的解釋每條規則。

## 模組小抄（Cheat Sheet）

在深入瞭解模組和路徑的細節之前，這裡提供一個簡單的參考，用來解釋模組、路徑、`use`關鍵詞和`pub`關鍵詞如何在編譯器中工作，以及大部分開發者如何組織他們的程式碼。我們將在本章中舉例說明每條規則，但這是回顧模組工作原理的絕佳參考。

- **從 crate 根節點開始**: 當編譯一個 crate, 編譯器首先在 crate 根檔案（通常，對於一個庫 crate 而言是 *src/lib.rs*，對於一個二進位制 crate 而言是 *src/main.rs*）中尋找需要被編譯的程式碼。
- **宣告模組**: 在 crate 根檔案中，你可以宣告一個新模組；比如，用 `mod garden;` 聲明瞭一個叫做 `garden` 的模組。編譯器會在下列路徑中尋找模組程式碼：
  - 內聯，用大括號替換 `mod garden` 後跟的分號
  - 在檔案 *src/garden.rs*
  - 在檔案 *src/garden/mod.rs*
- **宣告子模組**: 在除了 crate 根節點以外的任何檔案中，你可以定義子模組。比如，你可能在 *src/garden.rs* 中宣告 `mod vegetables;`。編譯器會在以父模組命名的目錄中尋找子模組程式碼：
  - 內聯，直接在 `mod vegetables` 後方不是一個分號而是一個大括號
  - 在檔案 *src/garden/vegetables.rs*
  - 在檔案 *src/garden/vegetables/mod.rs*
- **模組中的程式碼路徑**: 一旦一個模組是你 crate 的一部分，你可以在隱私規則允許的前提下，從同一個 crate 內的任意地方，通過程式碼路徑引用該模組的程式碼。舉例而言，一個 garden vegetables 模組下的 `Asparagus` 型別可以通過 `crate::garden::vegetables::Asparagus` 訪問。
- **私有 vs 公用**: 一個模組裡的程式碼預設對其父模組私有。為了使一個模組公用，應當在宣告時使用 `pub mod` 替代 `mod`。為了使一個公用模組內部的成員公用，應當在宣告前使用`pub`。
- **`use` 關鍵字**: 在一個作用域內，`use`關鍵字建立了一個項的快捷方式，用來減少長路徑的重複。在任何可以引用 `crate::garden::vegetables::Asparagus` 的作用域，你可以通過 `use crate::garden::vegetables::Asparagus;` 建立一個快捷方式，然後你就可以在作用域中只寫 `Asparagus` 來使用該型別。

這裡我們建立一個名為`backyard`的二進位制 crate 來說明這些規則。該 crate 的路徑同樣命名為`backyard`，該路徑包含了這些檔案和目錄：

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

這個例子中的 crate 根檔案是 *src/main.rs*，該檔案包含了：

<span class="filename">檔名：src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

`pub mod garden;` 行告訴編譯器將 *src/garden.rs* 中發現的程式碼包含進來：

<span class="filename">檔名：src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

在此處，`pub mod vegetables;` 意味著在 *src/garden/vegetables.rs* 中的程式碼也應該被包含。這些程式碼是：

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

現在讓我們深入瞭解這些規則的細節並在實踐中演示它們！

### 在模組中對相關程式碼進行分組

**模組**讓我們可以將一個 crate 中的程式碼進行分組，以提高可讀性與重用性。因為一個模組中的程式碼預設是私有的，所以還可以利用模組控制項的**私有性**（*privacy*）。私有項是不可為外部使用的內在詳細實現。我們也可以將模組和它其中的項標記為公開的，這樣，外部程式碼就可以使用並依賴於它們。

作為示例，讓我們編寫一個提供餐廳功能的庫 `crate`。我們將定義函式的簽名，但將其函式體留空以便將注意力集中在程式碼的組織結構上而不是餐廳實現的細節。

在餐飲業，餐館中會有一些地方被稱之為**前臺**（*front of house*），還有另外一些地方被稱之為**後臺**（*back of house*）。前臺是招待顧客的地方；這包括接待員為顧客安排座位、服務員接受點單和付款、調酒師製作飲品的地方。後臺則是廚師和烹飪人員在廚房工作、洗碗工清理餐具，以及經理處理行政事務的區域。

為了以這種方式構建我們的 `crate`，我們可以將其功能組織到巢狀模組中。通過執行 `cargo new restaurant --lib` 來建立一個新的名為 `restaurant` 的庫。然後將示例 7-1 中所羅列出來的程式碼放入 *src/lib.rs* 中，來定義一些模組和函式簽名；這段程式碼即為前臺部分。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">示例 7-1：一個包含了其他內建了函式的模組的 `front_of_house` 模組</span>

我們使用 `mod` 關鍵字來定義模組，後跟模組名（本例中叫做 `front_of_house`），並且用花括號包圍模組的主體。在模組內，我們還可以定義其它的模組，就像本例中的 `hosting` 和 `serving` 模組。模組還可以儲存一些定義的其它項，比如結構體、列舉、常量、trait、或者如示例 7-1 所示的函式。

通過使用模組，我們可以將相關的定義分組到一起，並指出它們為什麼相關。程式設計師可以通過使用這段程式碼，更加容易地找到他們想要的定義，因為他們可以基於分組來對程式碼進行導航，而不需要閱讀所有的定義。程式設計師向這段程式碼中新增一個新的功能時，他們也會知道程式碼應該放置在何處，可以保持程式的組織性。

在前面我們提到了，`src/main.rs` 和 `src/lib.rs` 叫做 crate 根。之所以這樣叫它們是因為這兩個檔案的內容都分別在 crate 模組結構的根組成了一個名為 `crate` 的模組，該結構被稱為**模組樹**（*module tree*）。

示例 7-2 展示了示例 7-1 中模組樹的結構。

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">示例 7-2: 示例 7-1 中程式碼的模組樹</span>

這個樹展示了一些模組是如何被嵌入到另一個模組的（例如，`hosting` 巢狀在 `front_of_house` 中）。這個樹還展示了一些模組是互為**兄弟**（*siblings*）的，這意味著它們定義在同一模組中；`hosting` 和 `serving` 被一起定義在 `front_of_house` 中。繼續沿用家庭關係的比喻，如果一個模組 A 被包含在模組 B 中，我們將模組 A 稱為模組 B 的 **子**（*child*）模組，模組 B 則是模組 A 的 **父**（*parent*）模組。注意，整個模組樹都植根於名為 `crate` 的隱式模組下。

這個模組樹可能會令你想起電腦上檔案系統的目錄樹；這是一個非常恰當的類比！就像檔案系統的目錄，你可以使用模組來組織你的程式碼。並且，就像目錄中的檔案，我們需要一種方法來找到模組。
