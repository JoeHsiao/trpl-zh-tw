## 宏

[ch20-05-macros.md](https://github.com/rust-lang/book/blob/57ff62db22b006e6b319e2e35c9364d932a8b4e5/src/ch20-05-macros.md)

我們已經在本書中使用過像 `println!` 這樣的宏了，不過尚未深入探討什麼是宏以及它是如何工作的。**宏**（*Macro*）指的是 Rust 中一系列的功能：使用 `macro_rules!` 的 **宣告宏**（*declarative macro*），和三種 **過程宏**（*procedural macro*）：

* 自定義 `#[derive]` 宏，用於在結構體和列舉上通過新增 `derive` 屬性生成程式碼
* 類屬性宏，定義可用於任意項的自定義屬性
* 類函式宏，看起來像函式，但操作的是作為其引數傳遞的 token

我們會依次討論每一種宏，不過首要的是，讓我們看看為什麼已經有了函式還需要宏呢？

### 宏和函式的區別

從根本上來說，宏是一種為寫其他程式碼而寫程式碼的方式，即所謂的 **超程式設計**（*metaprogramming*）。在附錄 C 中會探討 `derive` 屬性，其生成各種 trait 的實現。我們也在本書中一直使用 `println!` 宏和 `vec!` 宏。所有的這些宏以 **展開** 的方式來生成比你所手寫出的更多的程式碼。

超程式設計對於減少大量編寫和維護的程式碼是非常有用的，它也扮演了函式所扮演的角色。但宏有一些函式所沒有的附加能力。

一個函式簽名必須宣告函式引數的數量和型別。相比之下，宏能夠接收可變數量的引數：用一個引數呼叫 `println!("hello")` 或用兩個引數呼叫 `println!("hello {}", name)` 。而且，宏可以在編譯器解析程式碼前展開，例如，宏可以在一個給定型別上實現 trait。而函式則不行，因為函式是在執行時被呼叫，而 trait 需要在編譯時實現。

實現宏的缺點是與函式的定義相比宏的定義更復雜，因為你正在編寫生成 Rust 程式碼的 Rust 程式碼。由於這樣的間接性，巨集定義通常要比函式定義更難閱讀、理解和維護。

宏和函式的最後一個重要的區別是：在一個檔案裡呼叫宏 **之前** 必須定義它，或將其引入作用域，而函式則可以在任何地方定義和呼叫。

### 用 `macro_rules!` 編寫用於通用超程式設計的宣告宏

Rust 最常用的宏形式是 **宣告宏**（*declarative macros*）。它們有時也被稱為 “macros by example”、“`macro_rules!` 宏” 或者就是 “macros”。其核心概念是，宣告宏允許我們編寫一些類似 Rust `match` 表示式的程式碼。正如在第六章討論的那樣，`match` 表示式是一種控制結構，其接收一個表示式，與表示式的結果進行模式匹配，然後根據模式匹配執行相關程式碼。宏也將一個值和包含相關程式碼的模式進行比較：此種情況下，該值是傳遞給宏的 Rust 原始碼字面值；模式用於和前面提到的原始碼字面值進行比較，一旦匹配成功，每個模式的相關程式碼會替換傳遞給宏的程式碼。所有這一切都發生於編譯時。

可以使用 `macro_rules!` 來定義宏。讓我們通過檢視 `vec!` 巨集定義來探索如何使用 `macro_rules!` 結構。第八章講述瞭如何使用 `vec!` 宏來生成一個給定值的新 vector。例如，下面的宏用三個整數建立一個 vector：

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

也可以使用 `vec!` 宏來構造兩個整數的 vector 或五個字串 slice 的 vector。但卻無法使用函式做相同的事情，因為我們無法預先知道引數值的數量和型別。

在示例 20-35 中展示了 `vec!` 宏的一個稍微簡化的定義。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

<span class="caption">示例 20-35: 一個 `vec!` 巨集定義的簡化版本</span>

> 注意：標準庫中實際定義的 `vec!` 包括預分配正確數量記憶體的程式碼。這部分為程式碼最佳化，為了讓示例簡化，此處並沒有包含在內。

`#[macro_export]` 註解表明只要匯入了定義這個宏的 crate，該宏就應該是可用的。如果沒有該註解，這個宏不能被引入作用域。

接著使用 `macro_rules!` 和宏名稱開始巨集定義，且所定義的宏並 **不帶** 感嘆號。名字後跟大括號表示巨集定義體，在該例中宏名稱是 `vec` 。

`vec!` 宏的結構和 `match` 表示式的結構類似。此處有一個分支模式 `( $( $x:expr ),* )` ，後跟 `=>` 以及和模式相關的程式碼塊。如果模式匹配，該相關程式碼塊將被展開。鑑於這個宏只有一個模式，那就只有一個有效匹配方式，其他任何模式方向（譯者注：不匹配這個模式）都會導致錯誤。更復雜的宏會有不止一個分支。

巨集定義中有效模式語法和在第十九章提及的模式語法是不同的，因為宏模式所匹配的是 Rust 程式碼結構而不是值。回過頭來檢查下示例 20-29 中模式片段什麼意思。有關完整的宏模式語法，請查閱 [Rust 參考][ref]。

首先，一對括號包含了整個模式。我們使用美元符號（`$`）在宏系統中宣告一個變數來包含匹配該模式的 Rust 程式碼。美元符號明確表明這是一個宏變數而不是普通 Rust 變數。之後是一對括號，其捕獲了符合括號內模式的值用以在替代程式碼中使用。`$()` 內則是 `$x:expr` ，其匹配 Rust 的任意表達式，並將該表示式命名為 `$x`。

在 `$()` 之後的逗號表示在每個與 `$()` 內程式碼匹配的例項之間必須出現一個字面量逗號分隔符。緊隨逗號之後的 `*` 說明該模式匹配零個或更多個 `*` 之前的任何模式。

當以 `vec![1, 2, 3];` 呼叫宏時，`$x` 模式與三個表示式 `1`、`2` 和 `3` 對應進行了三次匹配。

現在讓我們來看看與此分支模式相關聯的程式碼塊中的模式：在 `$()*` 部分，`temp_vec.push($x)` 會針對模式中每次匹配到 `$()` 的部分，生成零次或多次，取決於模式匹配到多少次。`$x` 由每個與之相匹配的表示式所替換。當以 `vec![1, 2, 3];` 呼叫該宏時，替換該巨集呼叫所生成的程式碼會是下面這樣：

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

我們已經定義了一個宏，其可以接收任意數量和型別的引數，同時可以生成能夠建立包含指定元素的 vector 的程式碼。

要了解更多關於如何編寫宏的資訊，請查閱線上文件或其他資源，如由 Daniel Keep 發起、Lukas Wirth 繼續維護的 [“The Little Book of Rust Macros”][tlborm]。

### 用於從屬性生成程式碼的過程宏

第二種形式的宏被稱為 **過程宏**（*procedural macros*），因為它們更像函式（一種型別的過程）。過程宏接收 Rust 程式碼作為輸入，在這些程式碼上進行操作，然後產生另一些程式碼作為輸出，而非像宣告式宏那樣匹配對應模式然後以另一部分程式碼替換當前程式碼。有三種類型的過程宏，自定義派生（derive），類屬性和類函式，它們的工作原理都類似。

建立過程宏時，其定義必須駐留在它們自己的具有特殊 crate 型別的 crate 中。這麼做出於一些複雜的技術原因，將來我們希望能夠消除這些限制。在示例 20-36 中展示瞭如何定義過程宏，其中 `some_attribute` 是一個使用特定宏變體的佔位符。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

<span class="caption">示例 20-36: 一個定義過程宏的例子</span>

定義過程宏的函式接收一個 `TokenStream` 作為輸入並生成 `TokenStream` 作為輸出。`TokenStream` 是定義於 `proc_macro` crate 裡代表一系列 token 的型別，Rust 預設攜帶了`proc_macro` crate。這就是宏的核心：宏所處理的原始碼組成了輸入 `TokenStream`，宏生成的程式碼是輸出 `TokenStream`。函式上還有一個屬性；這個屬性指明瞭我們建立的過程宏的型別。在同一 crate 中可以有多種的過程宏。

讓我們看看不同種類的程式宏。我們將從一個自定義的派生宏開始，然後解釋使其他形式不同的小差異。

### 自定義 `derive` 宏

讓我們建立一個 `hello_macro` crate，其包含名為 `HelloMacro` 的 trait 和關聯函式 `hello_macro`。不同於讓使用者為其每一個型別實現 `HelloMacro` trait，我們將會提供一個過程式宏以便使用者可以使用 `#[derive(HelloMacro)]` 註解它們的型別來得到 `hello_macro` 函式的預設實現。該預設實現會列印 `Hello, Macro! My name is TypeName!`，其中 `TypeName` 為定義了 trait 的型別名。換言之，我們會建立一個 crate，使程式設計師能夠寫類似示例 20-37 中的程式碼。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

<span class="caption">示例 20-37: 我們 crate 的使用者所寫的能夠使用過程式宏的程式碼</span>

執行該程式碼將會列印 `Hello, Macro! My name is Pancakes!` 第一步是像下面這樣新建一個庫 crate：

```console
$ cargo new hello_macro --lib
```

接下來，會定義 `HelloMacro` trait 以及其關聯函式：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/hello_macro/src/lib.rs}}
```

<span class="caption">示例 20-38: 一個我們會用於 `derive` 宏的簡單 trait</span>

現在有了一個 trait 及其相關函式。此時，crate 使用者可以像示例 20-39 那樣實現該 trait 來達到期望的功能，像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancakes/src/main.rs}}
```

<span class="caption">示例 20-39: 如果使用者手動編寫了一個 `HelloMacro` trait 實現看起來如何</span>

然而，他們需要為每一個想要與 `hello_macro` 一同使用的型別編寫實現的程式碼塊。我們希望免去他們的這份工作。

另外，我們也無法為 `hello_macro` 函式提供一個能夠列印實現了該 trait 的型別的名字的預設實現：Rust 沒有反射能力，因此其無法在執行時獲取型別名。我們需要一個在編譯時生成程式碼的宏。

下一步是定義過程宏。在編寫本部分時，過程式宏必須在其自己的 crate 內。該限制最終可能被取消。crate 及其宏 crate 的結構慣例如下：對於一個名為 `foo` 的 crate，其自定義 derive 過程宏 crate 通常命名為 `foo_derive`。讓我們在 `hello_macro` 專案中，新建一個名為 `hello_macro_derive` 的 crate。

```console
$ cargo new hello_macro_derive --lib
```

由於兩個 crate 緊密相關，因此在 `hello_macro` 包的目錄下建立過程式宏的 crate。如果改變在 `hello_macro` 中定義的 trait，同時也必須改變在 `hello_macro_derive` 中過程宏的實現。這兩個包需要分別釋出，程式設計人員如果使用這些包，則需要同時新增這兩個依賴並將其引入作用域。我們也可以只用 `hello_macro` 包而將 `hello_macro_derive` 作為一個依賴，並重匯出過程式宏的程式碼。但現在我們組織專案的方式使程式設計人員在無需 `derive` 功能時也能夠單獨使用 `hello_macro`。

我們需要宣告 `hello_macro_derive` crate 為過程宏 (proc-macro) crate。我們還需要 `syn` 和 `quote` crate 中的功能，正如你即將看到的，需要將它們加到依賴中。將下面的程式碼加入到 `hello_macro_derive` 的 *Cargo.toml* 檔案中。

<span class="filename">檔名：hello_macro_derive/Cargo.toml</span>

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

為定義一個過程式宏，請將示例 20-40 中的程式碼放在 `hello_macro_derive` crate 的 *src/lib.rs* 檔案裡面。注意這段程式碼在我們新增 `impl_hello_macro` 函式的定義之前是無法編譯的。

<span class="filename">檔名：hello_macro_derive/src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/src/lib.rs}}
```

<span class="caption">示例 20-40: 大多數過程式巨集處理 Rust 程式碼時所需的程式碼</span>

注意我們將程式碼分成了 `hello_macro_derive` 和 `impl_hello_macro` 兩個函式，前者負責解析 `TokenStream`，後者負責轉換語法樹：這使得編寫過程宏更加方便。幾乎你看到或者建立的每一個過程宏的外部函式（這裡是 `hello_macro_derive`）中的程式碼都跟這裡是一樣的。你放入內部函式（這裡是 `impl_hello_macro`）中的程式碼根據你的過程宏的設計目的會有所不同。

現在，我們已經引入了三個新的 crate：`proc_macro`、[syn] 和 [quote]。Rust 自帶 `proc_macro` crate，因此無需將其加到 *Cargo.toml* 檔案的依賴中。`proc_macro` crate 是編譯器提供的 API，讓我們能夠在自己的程式碼中讀取並操作 Rust 程式碼。

`syn` crate 將字串中的 Rust 程式碼解析成為一個可以操作的資料結構。`quote` crate 則將 `syn` 解析的資料結構轉換回 Rust 程式碼。這些 crate 讓解析任何我們所要處理的 Rust 程式碼變得更加簡單：為 Rust 編寫完整的解析器並不是一件簡單的工作。

當用戶在一個型別上指定 `#[derive(HelloMacro)]` 時，`hello_macro_derive` 函式將會被呼叫。我們已使用 `proc_macro_derive` 註解該函式並指定名稱 `HelloMacro`，該名稱與我們的 trait 名稱相匹配；這是大多數過程宏遵循的慣例。

該函式首先將來自 `TokenStream` 的 `input` 轉換為一個我們可以解釋和操作的資料結構。這正是 `syn` 派上用場的地方。`syn` 中的 `parse` 函式獲取一個 `TokenStream` 並返回一個表示解析出的 Rust 程式碼的 `DeriveInput` 結構體。示例 20-41 展示了從字串 `struct Pancakes;` 中解析出來的 `DeriveInput` 結構體的相關部分：

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<span class="caption">示例 20-41: 解析示例 20-37 中帶有宏屬性的程式碼時得到的 `DeriveInput` 例項</span>

該結構體的欄位表明：我們解析出的 Rust 程式碼是一個類單元結構體，它的 `ident`（識別符號，也就是名稱）是 `Pancakes`。這個結構體裡還有更多欄位，用來描述各種 Rust 程式碼；更多資訊請參見 [syn 中 `DeriveInput` 的文件][syn-docs]。

很快我們將定義 `impl_hello_macro` 函式，其用於構建所要包含在內的 Rust 新程式碼。但在此之前，注意其輸出也是 `TokenStream`。所返回的 `TokenStream` 會被加到我們的 crate 使用者所寫的程式碼中，因此，當用戶編譯他們的 crate 時，他們會通過修改後的 `TokenStream` 獲取到我們所提供的額外功能。

你可能也注意到了，當呼叫 `syn::parse` 函式失敗時，我們用 `unwrap` 來使 `hello_macro_derive` 函式 panic。在錯誤時 panic 對過程宏來說是必須的，因為 `proc_macro_derive` 函式必須返回 `TokenStream` 而不是 `Result`，以此來符合過程宏的 API。這裡選擇用 `unwrap` 來簡化了這個例子；在生產程式碼中，則應該通過 `panic!` 或 `expect` 來提供關於發生何種錯誤的更加明確的錯誤資訊。

現在我們有了將註解的 Rust 程式碼從 `TokenStream` 轉換為 `DeriveInput` 例項的程式碼，讓我們來建立在註解型別上實現 `HelloMacro` trait 的程式碼，如示例 20-42 所示。

<span class="filename">檔名：hello_macro_derive/src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

<span class="caption">示例 20-42: 使用解析過的 Rust 程式碼實現 `HelloMacro` trait</span>

我們會得到一個 `Ident` 結構體例項，其中包含了註解型別的名字（識別符號），這個值來自 `ast.ident`。示例 20-41 中的結構體表明：當 `impl_hello_macro` 函式作用於示例 20-37 中的程式碼時，`ident` 欄位的值就是 `"Pancakes"`。因此，在示例 20-42 中，`name` 變數會包含一個 `Ident` 結構體例項；當它被打印出來時，就是字串 `"Pancakes"`，也就是示例 20-37 中那個結構體的名稱。

`quote!` 宏能讓我們編寫希望返回的 Rust 程式碼。`quote!` 宏執行的直接結果並不是編譯器所期望的所以需要轉換為 `TokenStream`。為此需要呼叫 `into` 方法，它會消費這個中間表示（intermediate representation，IR）並返回所需的 `TokenStream` 型別值。

這個宏還提供了一套很方便的模板機制；我們可以寫 `#name`，然後 `quote!` 會把它替換成名為 `name` 的變數的值。你甚至還可以像常規宏那樣做一些重複程式碼生成。完整介紹請參見 [quote crate 的文件][quote-docs]。

我們期望我們的過程式宏能夠為通過 `#name` 獲取到的使用者註解型別生成 `HelloMacro` trait 的實現。該 trait 的實現有一個函式 `hello_macro` ，其函式體包括了我們期望提供的功能：列印 `Hello, Macro! My name is` 和註解的型別名。

此處所使用的 `stringify!` 為 Rust 內建宏。其接收一個 Rust 表示式，如 `1 + 2` ，然後在編譯時將表示式轉換為一個字串常量，如 `"1 + 2"` 。這與計算表示式並接著將結果轉換為 `String` 的 `format!` 或 `println!` 不同。有一種可能的情況是，所輸入的 `#name` 可能是一個需要列印的表示式，因此我們用 `stringify!` 。`stringify!` 也能通過在編譯時將 `#name` 轉換為字串字面值來節省一次記憶體分配。

此時，`cargo build` 應該都能成功編譯 `hello_macro` 和 `hello_macro_derive` 。我們將這些 crate 連線到示例 20-31 的程式碼中來看看過程宏的行為！在 *projects* 目錄下用 `cargo new pancakes` 命令新建一個二進位制專案。需要將 `hello_macro` 和 `hello_macro_derive` 作為依賴加到 `pancakes` 包的 *Cargo.toml* 檔案中去。如果你正將 `hello_macro` 和 `hello_macro_derive` 的版本釋出到 [crates.io](https://crates.io/) 上，它們將是常規依賴；否則，則可以像下面這樣將其指定為 `path` 依賴：

```toml
{{#include ../listings/ch20-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

把示例 20-37 中的程式碼放在 *src/main.rs* ，然後執行 `cargo run`：其應該列印 `Hello, Macro! My name is Pancakes!`。其包含了該過程宏中 `HelloMacro` trait 的實現，而無需 `pancakes` crate 實現它；`#[derive(HelloMacro)]` 增加了該 trait 實現。

接下來，讓我們探索一下其他型別的過程宏與自定義 `derive` 宏有何區別。

### 類屬性宏

類屬性（Attribute-Like）宏與自定義 `derive` 宏相似，不同之處在於它們不是為 `derive` 屬性生成程式碼，而是允許你建立新的屬性。它們也更為靈活；`derive` 只能用於結構體和列舉；屬性還可以用於其它的項，比如函式。作為一個使用類屬性宏的例子，可以建立一個名為 `route` 的屬性用於註解 web 應用程式框架（web application framework）的函式：

```rust,ignore
#[route(GET, "/")]
fn index() {
```

`#[route]` 屬性將由框架本身定義為一個過程宏。其巨集定義的函式簽名看起來像這樣：

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

這裡有兩個 `TokenStream` 型別的引數；第一個用於屬性內容本身，也就是 `GET, "/"` 部分。第二個是屬性所標記的項：在本例中，是 `fn index() {}` 和剩下的函式體。

除此之外，類屬性宏與自定義派生宏工作方式一致：建立 `proc-macro` crate 型別的 crate 並實現生成所希望的程式碼的函式！

### 類函式宏

類函式（Function-like）宏的定義看起來像函式呼叫的宏。類似於 `macro_rules!`，它們比函式更靈活；例如，可以接受未知數量的引數。然而 `macro_rules!` 宏只能使用之前 [“使用 `macro_rules!` 的宣告宏用於通用超程式設計”][decl] 介紹的類匹配的語法定義。類函式宏獲取 `TokenStream` 引數，其定義使用 Rust 程式碼操縱 `TokenStream`，就像另兩種過程宏一樣。一個類函式宏例子是可以像這樣被呼叫的 `sql!` 宏：

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

這個宏會解析其中的 SQL 語句並檢查其是否是句法正確的，這是比 `macro_rules!` 可以做到的更為複雜的處理。`sql!` 宏會被類似於像這樣定義：

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

這類似於自定義 `derive` 宏的簽名：獲取括號中的 token，並返回希望生成的程式碼。

## 總結

呼！現在你的工具箱中有了一些 Rust 特性，雖然你可能不會經常使用它們，但在非常特定的情況下你會知道它們可用。我們介紹了幾個複雜的主題，以便當你在錯誤資訊建議或他人程式碼中遇到它們時，能夠識別這些概念和語法。本章可作為查詢解決方案的參考。

接下來，我們將再開始一個專案，將本書所學的所有內容付諸實踐！

[ref]: https://doc.rust-lang.org/reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[syn]: https://crates.io/crates/syn
[quote]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #用-macro_rules-編寫用於通用超程式設計的宣告宏
