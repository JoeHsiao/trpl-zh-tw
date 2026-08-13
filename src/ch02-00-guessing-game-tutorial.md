# 編寫一個猜數字遊戲

[ch02-00-guessing-game-tutorial.md](https://github.com/rust-lang/book/blob/95dd8a023b2c736d12377a713bb6b8ed77e48678/src/ch02-00-guessing-game-tutorial.md)

讓我們一起動手完成一個專案來快速上手 Rust！本章將介紹一些 Rust 中常見的概念，並透過真實的程式來展示如何運用它們。你將會學到 `let`、`match`、方法（methods）、關聯函式（associated functions）、外部 crate 等知識！後續章節會深入探討這些概念的細節。在這一章，我們將主要練習基礎內容。

我們會實現一個經典的新手程式設計問題：猜數字遊戲。遊戲的規則如下：程式將會生成一個 1 到 100 之間的隨機整數。然後提示玩家輸入一個猜測值。輸入後，程式會指示該猜測是太低還是太高。如果猜對了，遊戲會列印祝賀資訊並退出。

## 準備一個新專案

要建立一個新專案，進入第一章中建立的 _projects_ 目錄，使用 Cargo 新建一個專案，如下：

```console
$ cargo new guessing_game
$ cd guessing_game
```

第一條命令 `cargo new` 將專案名稱（`guessing_game`）作為第一個引數。第二條命令則進入新建立的專案目錄。

看看生成的 _Cargo.toml_ 檔案：

<span class="filename">檔名：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

正如第一章那樣，`cargo new` 生成了一個 “Hello, world!” 程式。檢視 _src/main.rs_ 檔案：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

現在使用 `cargo run` 命令，一步完成 “Hello, world!” 程式的編譯和執行：

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

當你需要在專案中快速迭代時，`run` 命令就能派上用場，正如我們在這個遊戲專案中做的，在下一次迭代之前快速測試每一次迭代。

重新開啟 _src/main.rs_ 檔案。我們將會在這個檔案中編寫全部的程式碼。

## 處理一次猜測

猜數字程式的第一部分請求和處理使用者輸入，並檢查輸入是否符合預期的格式。首先，我們會允許玩家輸入一個猜測。在 _src/main.rs_ 中輸入示例 2-1 中的程式碼。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<figcaption>示例 2-1：獲取使用者猜測並列印的程式碼</figcaption>

</figure>

這些程式碼包含很多資訊，我們一行一行地講解。為了獲取使用者輸入並將結果列印為輸出，我們需要把輸入/輸出庫 `io` 引入當前作用域。`io` 庫來自標準庫，也被稱為 `std`：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

預設情況下，Rust 會把標準庫中的一組內容自動帶入每個程式的作用域，這組內容被稱為 *預匯入（prelude）*。你可以在[標準庫文件][prelude]中檢視 prelude 裡都有哪些內容。

如果你需要的型別不在 prelude 中，就必須使用 `use` 語句顯式地將其引入作用域。`std::io` 庫提供了許多有用的功能，其中包括接收使用者輸入。

如第一章所提及，`main` 函式是程式的入口點：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` 語法聲明瞭一個新函式，小括號 `()` 表明沒有引數，大括號 `{` 作為函式體的開始。

第一章也提及了 `println!` 是一個在螢幕上列印字串的宏：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

這些程式碼僅僅列印提示，介紹遊戲的內容然後請求使用者輸入。

### 使用變數儲存值

接下來，建立一個 **變數**（_variable_）來儲存使用者輸入，像這樣：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

現在程式開始變得有意思了！這一小行程式碼發生了很多事。我們使用 `let` 語句來建立變數。這裡是另外一個例子：

```rust,ignore
let apples = 5;
```

這行程式碼新建了一個叫做 `apples` 的變數並把它繫結到值 `5` 上。在 Rust 中，變數預設是不可變的，這意味著一旦我們給變數賦值，這個值就不可以再修改了。我們將會在第三章的 [“變數與可變性”][variables-and-mutability] 部分詳細討論這個概念。下面的例子展示瞭如何在變數名前使用 `mut` 來使一個變數可變：

```rust,ignore
let apples = 5; // 不可變
let mut bananas = 5; // 可變
```

> 注意：`//` 語法開始一個註釋，持續到行尾。Rust 忽略註釋中的所有內容，[第三章][comments]將會詳細介紹註釋。

回到猜數字程式中。現在我們知道了 `let mut guess` 會引入一個叫做 `guess` 的可變變數。等號（`=`）告訴 Rust 我們現在想將某個值繫結在變數上。等號的右邊是 `guess` 所繫結的值，它是 `String::new` 的結果，這個函式會返回一個 `String` 的新例項。[`String`][string]<!-- ignore --> 是一個標準庫提供的字串型別，它是 UTF-8 編碼的可增長文字塊。

`::new` 那一行的 `::` 語法表明 `new` 是 `String` 型別的一個 **關聯函式**（_associated function_）。關聯函式是針對某個型別實現的函式，在這個例子中是 `String`。這個 `new` 函式建立了一個新的空字串。你會發現許多型別上都有一個 `new` 函式，因為這是為某種型別建立新值的常用函式名。

總的來說，`let mut guess = String::new();` 這一行建立了一個可變變數，當前它繫結到一個新的 `String` 空例項上。呼！

### 接收使用者輸入

回憶一下，我們在程式的第一行使用 `use std::io;` 從標準庫中引入了輸入/輸出功能。現在呼叫 `io` 庫中的函式 `stdin`，這允許我們處理使用者輸入：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

如果程式的開頭沒有使用 `use std::io;` 引入 `io` 庫，我們仍可以透過把函式呼叫寫成 `std::io::stdin` 來使用該函式。`stdin` 函式返回一個 [`std::io::Stdin`][iostdin]<!-- ignore --> 的例項，這是一種代表終端標準輸入控制代碼的型別。

接下來，程式碼中的 `.read_line(&mut guess)` 呼叫了標準輸入控制代碼上的 [`read_line`][read_line]<!-- ignore --> 方法，以獲取使用者輸入。我們還將 `&mut guess` 作為引數傳遞給 `read_line` 函式，讓其將使用者輸入儲存到這個字串中。`read_line` 的工作是，無論使用者在標準輸入中鍵入什麼內容，都將其追加（不會覆蓋其原有內容）到一個字串中，因此它需要字串作為引數。這個字串引數應該是可變的，以便 `read_line` 將使用者輸入附加上去。

`&` 表示這個引數是一個 **引用**（_reference_），它允許多處程式碼訪問同一處資料，而無需在記憶體中多次複製。引用是一個複雜的特性，Rust 的一個主要優勢就是安全而簡單的操縱引用。完成當前程式並不需要了解如此多細節。現在，我們只需知道它像變數一樣，預設是不可變的。因此，需要寫成 `&mut guess` 來使其可變，而不是 `&guess`。（第四章會更全面地講解引用。）

<a id="handling-potential-failure-with-the-result-type"></a>

### 使用 `Result` 型別來處理潛在的錯誤

我們還沒有完全分析完這行程式碼。雖然我們已經講到了第三行文字，但要注意：它仍然屬於同一條邏輯程式碼。接下來的部分是這個方法（method）：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

我們也可以將程式碼這樣寫：

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

不過，過長的程式碼行難以閱讀，所以最好拆開來寫。通常來說，當使用 `.method_name()` 語法呼叫方法時，適當加入換行和空白來拆分長程式碼行是很明智的。現在來看看這行程式碼到底做了什麼。

之前提到了 `read_line` 會將使用者輸入附加到傳遞給它的字串中，不過它也會返回一個型別為 `Result` 的值。[`Result`][result]<!-- ignore --> 是一種[*列舉型別*][enums]<!-- ignore -->，通常也寫作 *enum*，它可以是多種可能狀態中的一個。我們把每種可能的狀態稱為一種 **列舉成員**（*variant*）。

[第六章][enums]將介紹列舉的更多細節。這裡的 `Result` 型別將用來編碼錯誤處理的資訊。

`Result` 的成員是 `Ok` 和 `Err`，`Ok` 成員表示操作成功，內部包含成功時產生的值。`Err` 成員則意味著操作失敗，並且 `Err` 中包含有關操作失敗的原因或方式的資訊。

`Result` 型別的值，像其他型別一樣，擁有定義於其例項上的方法。`Result` 的例項擁有 [`expect` 方法][expect]<!-- ignore -->。如果 `io::Result` 例項的值是 `Err`，`expect` 會導致程式崩潰，並輸出當做引數傳遞給 `expect` 的資訊。所以當 `read_line` 方法返回 `Err`，則可能是來源於底層作業系統錯誤的結果。如果 `Result` 例項的值是 `Ok`，`expect` 會獲取 `Ok` 中的值並原樣返回。在本例中，這個值是使用者輸入到標準輸入中的位元組數。

如果不呼叫 `expect`，程式也能編譯，不過會出現一個警告：

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust 警告我們沒有使用 `read_line` 的返回值 `Result`，說明有一個可能的錯誤沒有處理。

消除警告的正確做法是實際去編寫錯誤處理程式碼，不過由於我們就是希望程式在出現問題時立即崩潰，所以直接使用 `expect`。[第九章][recover] 會學習如何從錯誤中恢復。

### 使用 `println!` 佔位符列印值

除了位於結尾的右花括號，目前為止就只有這一行程式碼值得討論一下了：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

這行程式碼現在列印了儲存使用者輸入的字串。`{}` 這對大括號是一個佔位符：把 `{}` 想象成小蟹鉗，可以夾住合適的值。當列印變數的值時，變數名可以寫進大括號中。當打印表達式的執行結果時，格式化字串（format string）中大括號中留空，格式化字串後跟逗號分隔的需要列印的表示式列表，其順序與每一個空大括號佔位符的順序一致。在一個 `println!` 呼叫中列印變數和表示式的值看起來像這樣：

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

這行程式碼會打印出 `x = 5 and y + 2 = 12`。

### 測試第一部分程式碼

讓我們來測試下猜數字遊戲的第一部分。使用 `cargo run` 執行：

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

至此為止，遊戲的第一部分已經完成：我們從鍵盤獲取輸入並列印了出來。

## 生成一個秘密數字

接下來，需要生成一個秘密數字，好讓使用者來猜。秘密數字應該每次都不同，這樣重複玩才不會乏味；範圍應該在 1 到 100 之間，這樣才不會太困難。Rust 標準庫中尚未包含隨機數功能。然而，Rust 團隊還是提供了一個包含上述功能的 [`rand` crate][randcrate]。

<a id="using-a-crate-to-get-more-functionality"></a>

### 使用 crate 來增加更多功能

記住，crate 是一組 Rust 原始碼檔案。我們正在構建的專案是一個 *二進位制 crate*，它會生成一個可執行檔案。`rand` crate 則是一個 *庫 crate*，庫 crate 可以包含任意能被其他程式使用的程式碼，但不能獨立執行。

Cargo 在管理外部 crate 方面的能力正是其真正大放異彩的地方。在我們使用 `rand` 編寫程式碼之前，需要修改 *Cargo.toml* 檔案，把 `rand` 加為一個依賴。現在開啟這個檔案，並將下面這一行新增到 `[dependencies]` section 標題之下。在當前版本下，請務必按這裡的方式指定 `rand`，否則本教程中的示例程式碼可能無法執行。

<span class="filename">檔名：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

在 _Cargo.toml_ 檔案中，一個標題以及其後的內容都屬於同一個 section，直到出現下一個標題才會開始新的 section。`[dependencies]` section 用來告訴 Cargo：這個專案依賴哪些外部 crate，以及它們的版本要求。在本例中，我們使用語義化版本 `0.8.5` 來指定 `rand` crate。Cargo 理解 [語義化版本（Semantic Versioning）][semver]<!-- ignore -->，也常簡稱為 _SemVer_，它是一種編寫版本號的標準。`0.8.5` 實際上是 `^0.8.5` 的簡寫，表示任何至少為 `0.8.5` 但小於 `0.9.0` 的版本。

Cargo 認為這些版本與 `0.8.5` 的公開 API 相容，因此這樣的版本約束可以確保我們獲得仍能編譯本章程式碼的最新補丁（patch）版本。任何大於等於 `0.9.0` 的版本，都不能保證仍然使用與後續示例相同的 API。

現在，不修改任何程式碼，構建專案，如示例 2-2 所示。

<figure class="listing">

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

<figcaption>示例 2-2：將 rand crate 新增為依賴之後執行 `cargo build` 的輸出</figcaption>

</figure>

可能會出現不同的版本號（多虧了語義化版本，它們與程式碼是相容的！），並且顯示的行數可能會有所不同（取決於作業系統），行的順序也可能會不同。

當我們加入一個外部依賴時，Cargo 會從 _registry_ 獲取該依賴所需內容的最新版本資訊；這個 _registry_ 是來自 [Crates.io][cratesio] 資料的一份副本。Crates.io 是 Rust 生態系統中人們釋出開源 Rust 專案、供他人使用的平臺。

在更新完 _registry_ 之後，Cargo 會檢查 `[dependencies]` section，並下載其中列出但尚未下載的 crate。本例中，雖然我們只聲明瞭 `rand` 這一個依賴，Cargo 還是額外獲取了 `rand` 正常工作所依賴的其他 crate。下載完成後，Rust 會先編譯這些依賴，再在這些依賴可用的情況下編譯專案本身。

如果不做任何修改，立刻再次執行 `cargo build`，除了 `Finished` 那一行之外，你不會看到任何輸出。Cargo 知道它已經下載並編譯了這些依賴，而且 _Cargo.toml_ 檔案也沒有發生變化。Cargo 還知道你的程式碼也沒有改動，所以它也不會重新編譯程式碼。由於沒有事情可做，它就會直接退出。

如果開啟 _src/main.rs_ 檔案，做一些無關緊要的修改，儲存並再次構建，你將只會看到兩行輸出：

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

這表明 Cargo 只因為你對 _src/main.rs_ 做了微小修改而更新了構建。依賴並沒有變化，所以 Cargo 知道它可以複用之前已經下載並編譯好的那些程式碼。

<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

#### _Cargo.lock_ 檔案確保可重現構建

Cargo 有一個機制，可以確保無論是你還是其他人在任何時候重新構建程式碼，都會得到相同的構建產物：Cargo 只會使用已經確定下來的依賴版本，除非你明確要求它更新。例如，假設下週 `rand` crate 釋出了 `0.8.6` 版本，其中修復了一個重要 bug，但同時也引入了一個會破壞你程式碼的迴歸問題。為了解決這類問題，Cargo 會在你第一次執行 `cargo build` 時建立 *Cargo.lock* 檔案，我們現在就能在 *guessing_game* 目錄中看到它。

當你第一次構建專案時，Cargo 會計算出所有符合要求的依賴版本，並把它們寫入 *Cargo.lock* 檔案。之後再次構建專案時，Cargo 會發現 *Cargo.lock* 已經存在，於是直接使用其中記錄的版本，而不是重新計算一遍。這就讓你自動擁有了可重現構建（reproducible build）。換句話說，專案會一直停留在 `0.8.5`，直到你顯式升級它，這都要歸功於 *Cargo.lock* 檔案。由於 *Cargo.lock* 對可重現構建非常重要，所以它通常會和專案中的其餘程式碼一起提交到版本控制系統中。

#### 更新 crate 到一個新版本

當你 **確實** 想更新某個 crate 時，Cargo 提供了 `update` 命令，它會忽略 *Cargo.lock* 檔案，並重新計算所有符合 *Cargo.toml* 中宣告要求的最新版本。然後，Cargo 會把這些版本寫回 *Cargo.lock* 檔案。不過，預設情況下，Cargo 只會查詢大於 `0.8.5` 且小於 `0.9.0` 的版本。如果 `rand` crate 釋出了兩個新版本：`0.8.6` 和 `0.999.0`，那麼執行 `cargo update` 時你會看到如下輸出：

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

Cargo 會忽略 `0.999.0` 這個版本。這時，你也會注意到 *Cargo.lock* 檔案發生了變化：你現在使用的 `rand` crate 版本變成了 `0.8.6`。如果你想使用 `rand` 的 `0.999.0` 版本，或者 `0.999.x` 系列中的任何版本，就必須像下面這樣更新 *Cargo.toml* 檔案：

```toml
[dependencies]
rand = "0.999.0"
```

下一次執行 `cargo build` 時，Cargo 會更新可用 crate 的 registry，並根據你指定的新版本重新評估 `rand` 的版本要求。

第十四章會介紹 [Cargo][doccargo]<!-- ignore --> 及其[生態系統][doccratesio]<!-- ignore --> 的更多內容，不過目前知道這些就足夠了。Cargo 讓複用庫變得非常容易，因此 Rustaceans 能夠編寫出由多個包組合而成、更小巧的專案。

### 生成一個隨機數

讓我們開始使用 `rand` 來生成一個要猜測的數字。下一步是更新 *src/main.rs*，如示例 2-3 所示。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<figcaption>示例 2-3：新增生成隨機數的程式碼</figcaption>

</figure>

首先，我們新增了一行 `use rand::Rng;`。`Rng` 是一個 trait，它定義了隨機數生成器應實現的方法，想使用這些方法的話，此 trait 必須在作用域中。第十章會詳細介紹 trait。

接下來，我們又在中間加了兩行。第一行呼叫 `rand::thread_rng` 函式，得到我們將要使用的那個特定隨機數生成器：它與當前執行執行緒區域性相關，並由作業系統提供種子（seed）。然後，我們在這個隨機數生成器上呼叫 `gen_range` 方法。這個方法由透過 `use rand::Rng` 語句引入作用域的 `Rng` trait 定義。`gen_range` 方法接收一個範圍表示式（range expression）作為引數，並在該範圍內生成一個隨機數。這裡我們使用的範圍表示式形式是 `start..=end`，它在上下邊界上都是閉區間，所以我們需要寫 `1..=100`，來請求一個 1 到 100 之間的數。

> 注意：你不可能憑空知道應該 `use` 哪個 trait，或者應該從 crate 中呼叫哪些方法和函式，因此每個 crate 都帶有使用說明文件。Cargo 的另一個很棒的功能是，執行 `cargo doc --open` 命令會在本地構建所有依賴提供的文件，並在瀏覽器中開啟。例如，假設你對 `rand` crate 的其他功能感興趣，你可以執行 `cargo doc --open`，然後點選左側邊欄中的 `rand`。

新增加的第二行程式碼打印出了秘密數字。這在開發程式時很有用，因為可以測試它，不過在最終版本中會刪掉它。如果遊戲一開始就打印出結果就沒什麼可玩的了！

嘗試執行程式幾次：

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

你應該能得到不同的隨機數，同時它們應該都是在 1 和 100 之間的。幹得漂亮！

## 比較猜測的數字和秘密數字

現在有了使用者輸入和一個隨機數，我們可以比較它們。這個步驟如示例 2-4 所示。注意這段程式碼還不能透過編譯，我們稍後會解釋。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<figcaption>示例 2-4：處理比較兩個數字可能的返回值</figcaption>

</figure>

首先，我們增加了另一條 `use` 宣告，把標準庫中的 `std::cmp::Ordering` 型別引入作用域。`Ordering` 也是一個列舉，它的成員分別是 `Less`、`Greater` 和 `Equal`。這正是比較兩個值時可能出現的三種結果。

接著，我們在底部添加了五行新程式碼，用到了 `Ordering` 型別。`cmp` 方法用於比較兩個值，並且可以在任何可比較的值上呼叫。它接收一個對比物件的引用：這裡就是把 `guess` 和 `secret_number` 作比較。然後，它會返回一個我們剛剛透過 `use` 引入作用域的 `Ordering` 列舉成員。我們使用 [`match`][match]<!-- ignore --> 表示式，根據對 `guess` 和 `secret_number` 呼叫 `cmp` 後返回的是哪個 `Ordering` 成員，來決定下一步該做什麼。

一個 `match` 表示式由 **分支（arms）** 構成。一個分支包含一個 **模式**（*pattern*）和表示式開頭的值與分支模式相匹配時應該執行的程式碼。Rust 獲取提供給 `match` 的值並挨個檢查每個分支的模式。`match` 結構和模式是 Rust 中強大的功能，它體現了程式碼可能遇到的多種情形，並確保對所有情況作出處理。這些功能將分別在第六章和第十九章詳細介紹。

讓我們看看使用 `match` 表示式的例子。假設使用者猜了 50，這時隨機生成的秘密數字是 38。

當代碼比較 50 和 38 時，因為 50 大於 38，`cmp` 方法會返回 `Ordering::Greater`。`match` 表示式拿到的就是這個 `Ordering::Greater` 值。它會先檢查第一個分支的模式 `Ordering::Less`，發現並不匹配，於是忽略這個分支中的程式碼並繼續看下一個分支。下一個分支的模式是 `Ordering::Greater`，它**確實**匹配！於是該分支關聯的程式碼會被執行，並在螢幕上列印 `Too big!`。由於 `match` 表示式會在第一次成功匹配後結束，所以在這種情況下它不會再去看最後一個分支。

然而，示例 2-4 的程式碼目前並不能編譯，可以嘗試一下：

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

錯誤的核心表明這裡有 **不匹配的型別**（_mismatched types_）。Rust 有一個靜態強型別系統，同時也有型別推斷。當我們寫出 `let guess = String::new()` 時，Rust 推斷出 `guess` 應該是 `String` 型別，並不需要我們寫出型別。另一方面，`secret_number`，是數字型別。幾個數字型別擁有 1 到 100 之間的值：32 位數字 `i32`；32 位無符號數字 `u32`；64 位數字 `i64` 等等。Rust 預設使用 `i32`，所以它是 `secret_number` 的型別，除非增加型別資訊，或任何能讓 Rust 推斷出不同數值型別的資訊。這裡錯誤的原因在於 Rust 不會比較字串型別和數字型別。

所以我們必須把從輸入中讀取到的 `String` 轉換為一個數字型別，才好與秘密數字進行比較。這可以透過在 `main` 函式體中增加如下程式碼來實現：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

這行新程式碼是：

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

這裡建立了一個名為 `guess` 的變數。不過等等，不是已經有一個叫做 `guess` 的變量了嗎？確實如此，但 Rust 很貼心地允許我們用一個新值來 **遮蔽**（_shadowing_）之前的 `guess`。這樣一來，我們就可以繼續複用 `guess` 這個變數名，而不必被迫建立兩個不同的變數，比如 `guess_str` 和 `guess`。[第三章][shadowing]會更詳細地介紹 shadowing；現在你只需要知道，這個特性經常被用來把一個值從一種型別轉換為另一種型別。

我們把這個新變數繫結到 `guess.trim().parse()` 這個表示式上。表示式裡的 `guess` 指向的是原先那個儲存使用者輸入字串的 `guess` 變數。`String` 例項上的 `trim` 方法會去掉字串開頭和結尾的空白字元；在把字串轉換為 `u32` 之前，我們必須先這麼做，因為 `u32` 只能包含數值資料。使用者必須按下 <kbd>enter</kbd> 才能讓 `read_line` 返回並提交他們的猜測，這會在字串中附加一個換行符（newline）。例如，如果使用者輸入 <kbd>5</kbd> 並按下 <kbd>enter</kbd>，`guess` 實際上會變成 `5\n`。（在 Windows 上，按下 <kbd>enter</kbd> 會得到回車加換行，也就是 `\r\n`。）`trim` 方法會去除 `\n` 或 `\r\n`，最後只留下 `5`。

[字串的 `parse` 方法][parse]<!-- ignore --> 將字串轉換成其他型別。這裡用它來把字串轉換為數值。我們需要告訴 Rust 具體的數字型別，這裡透過 `let guess: u32` 指定。`guess` 後面的冒號（`:`）告訴 Rust 我們指定了變數的型別。Rust 有一些內建的數字型別；`u32` 是一個無符號的 32 位整型。對於不大的正整數來說，它是不錯的預設型別，[第三章][integers]還會講到其他數字型別。

另外，程式中的 `u32` 註解以及與 `secret_number` 的比較，意味著 Rust 會推斷出 `secret_number` 也是 `u32` 型別。現在可以使用相同型別比較兩個值了！

`parse` 方法只會在字元邏輯上確實可以轉換為數字時才成功，因此很容易失敗。例如，如果字串裡包含 `A👍%`，那就根本不可能把它轉換成數字。正因如此，`parse` 方法會返回一個 `Result` 型別，就像前面在 [“使用 `Result` 型別來處理潛在的錯誤”](#使用-result-型別來處理潛在的錯誤) 中討論過的 `read_line` 方法一樣。這裡我們再次使用 `expect` 方法來處理它。如果 `parse` 無法從字串中生成數字，並返回 `Result` 的 `Err` 成員，`expect` 就會讓遊戲崩潰，並列印我們提供的訊息。如果 `parse` 成功把字串轉換為數字，它就會返回 `Result` 的 `Ok` 成員，而 `expect` 會把 `Ok` 裡儲存的數字返回給我們。

現在讓我們執行程式！

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

很好！即使在猜測前面加了空格，程式仍然能判斷出使用者猜的是 76。你可以多執行幾次程式，用不同型別的輸入來驗證不同的行為：猜中正確數字、猜一個過大的數字，以及猜一個過小的數字。

現在遊戲已經大體上能玩了，不過使用者只能猜一次。增加一個迴圈來改變它吧！

## 使用迴圈來允許多次猜測

`loop` 關鍵字建立了一個無限迴圈。我們會增加迴圈來給使用者更多機會猜數字：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

如你所見，我們把從提示使用者輸入猜測開始往後的所有內容都移進了迴圈中。請確保迴圈中的程式碼都額外縮排四個空格，然後再次執行程式。程式現在會不斷要求使用者輸入新的猜測，但這也引入了一個新問題。使用者似乎沒法退出了！

使用者當然總可以用鍵盤快捷鍵 <kbd>ctrl</kbd>-<kbd>C</kbd> 中斷程式。不過還有另一種辦法能逃離這個貪得無厭的怪物，正如我們在 [“比較猜測的數字和秘密數字”](#比較猜測的數字和秘密數字) 一節討論 `parse` 時提到的那樣：如果使用者輸入的不是數字，程式就會崩潰。我們可以利用這一點來讓使用者退出，如下所示：

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

輸入 `quit` 的確會退出遊戲，但你也會注意到，輸入任何其他非數字內容也一樣會退出。這種體驗至少可以說並不理想；我們希望遊戲在猜中正確數字時也能停止。

### 猜測正確後退出

讓我們增加一個 `break` 語句，在使用者猜對時退出遊戲：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

在 `You win!` 之後加上一行 `break`，程式就會在使用者正確猜出秘密數字時退出迴圈。退出迴圈也就意味著退出程式，因為這個迴圈就是 `main` 的最後一部分。

### 處理無效輸入

為了進一步改進遊戲體驗，我們不希望程式在使用者輸入非數字時崩潰；相反，我們希望它忽略這些輸入，讓使用者繼續猜。我們可以透過修改將 `guess` 從 `String` 轉換為 `u32` 的那行程式碼來做到這一點，如示例 2-5 所示：

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<figcaption>示例 2-5：忽略非數字的猜測並重新請求數字而不是讓程式崩潰</figcaption>

</figure>

我們將 `expect` 呼叫換成 `match` 語句，以從遇到錯誤就崩潰轉換為處理錯誤。須知 `parse` 返回的是一個 `Result` 型別，而 `Result` 是一個擁有 `Ok` 和 `Err` 成員的列舉。這裡使用的 `match` 表示式，和我們之前處理 `cmp` 方法返回的 `Ordering` 時用的是同一種方式。

如果 `parse` 能夠成功地將字串轉換為一個數字，它會返回一個包含結果數字的 `Ok`。這個 `Ok` 值與 `match` 第一個分支的模式相匹配，該分支對應的動作返回 `Ok` 值中的數字 `num`，最後如願變成新建立的 `guess` 變數。

如果 `parse` **不能**把字串轉換為數字，它就會返回一個包含更多錯誤資訊的 `Err`。這個 `Err` 值無法匹配第一個 `match` 分支中的 `Ok(num)` 模式，但會匹配第二個分支裡的 `Err(_)` 模式。下劃線 `_` 是一個通配值；在這個例子裡，它表示我們要匹配所有 `Err` 值，而不關心其中具體包含什麼資訊。因此，程式會執行第二個分支中的程式碼 `continue`，告訴程式進入 `loop` 的下一次迭代，並請求另一個猜測。於是，程式就有效地忽略了 `parse` 可能遇到的所有錯誤！

現在程式中的一切都應該如預期般工作了。讓我們試試吧：

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

太棒了！再做最後一個小改動，猜數字遊戲就完成了：別忘了程式現在仍然會打印出秘密數字。測試時這很方便，但正式釋出時會破壞遊戲體驗。把輸出秘密數字的 `println!` 刪掉吧。示例 2-6 展示了最終程式碼：

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<figcaption>示例 2-6：猜數字遊戲的完整程式碼</figcaption>

</figure>

到這裡，你已經成功構建出了猜數字遊戲。恭喜！

## 總結

這個專案透過動手實踐的方式，向你介紹了許多 Rust 新概念：`let`、`match`、函式、使用外部 crate，等等。在接下來的幾章中，你會更深入地學習這些概念。第三章介紹大多數程式語言共有的概念，例如變數、資料型別和函式，並展示如何在 Rust 中使用它們。第四章將探索所有權（ownership），這是 Rust 區別於其他語言的一項特性。第五章討論結構體與方法語法，第六章則解釋列舉是如何工作的。

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#變數和可變性
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#遮蔽
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#整型
