## Future 與 async 語法

[ch17-01-futures-and-syntax.md](https://github.com/rust-lang/book/blob/0d5a0dd395aba1f82d7e5aaf6dbb59b2b843ad2c/src/ch17-01-futures-and-syntax.md)

Rust 非同步程式設計的關鍵元素是 *futures* 和 Rust 的 `async` 與 `await` 關鍵字。

*future* 是一個現在也許還沒準備好，但會在將來某個時刻準備好的值。（這個概念在很多語言裡都存在，只是有時會用 *task* 或 *promise* 之類的名字。）Rust 提供了 `Future` trait 作為基礎構件，讓不同的非同步操作可以用不同的資料結構來實現，同時又擁有統一的介面。在 Rust 中，future 就是那些實現了 `Future` trait 的型別。每個 future 都儲存了自身的進度資訊，以及“就緒”到底意味著什麼。

`async` 關鍵字可以用於程式碼塊和函式，表示它們可以被中斷和恢復。在 async 塊或 async 函式中，你可以使用 `await` 關鍵字來 *await 一個 future*，也就是等待它變為就緒。在 async 塊或函數里，每個等待 future 的位置，都是這個塊或函式可能暫停並隨後恢復的點。檢查 future、看看它的值是否已經可用，這個過程稱為 *polling*（輪詢）。

其他一些語言，例如 C# 和 JavaScript，也用 `async` 和 `await` 關鍵字進行非同步程式設計。如果你熟悉這些語言，可能會注意到 Rust 在語法處理上存在一些明顯差異。我們會看到，這樣設計是有充分理由的。

編寫非同步 Rust 時，大多數時候我們直接使用 `async` 和 `await` 關鍵字。Rust 會把它們編譯成等價的、基於 `Future` trait 的程式碼，就像它把 `for` 迴圈編譯成基於 `Iterator` trait 的等價程式碼一樣。不過，既然 Rust 提供了 `Future` trait，你在需要時也可以為自己的資料型別實現它。本章中我們會見到很多函式，它們都返回擁有各自 `Future` 實現的型別。我們會在本章結尾回到這個 trait 的定義，進一步深入理解它的工作原理；不過眼下這些細節已經足夠讓我們繼續前進。

這些內容可能仍然有些抽象，所以我們來寫第一個非同步程式：一個小型網頁抓取器。我們會從命令列傳入兩個 URL，併發地抓取它們，然後返回那個最先完成的結果。這個例子會帶來不少新語法，不過不用擔心，我們會一路把需要知道的內容都解釋清楚。

## 第一個非同步程式

為了讓本章專注於學習 async，而不是在生態系統的各種元件之間來回切換，我們準備了一個 `trpl` crate（`trpl` 是 “The Rust Programming Language” 的縮寫）。它重新匯出了本章需要的所有型別、trait 和函式，主要來自 [`futures`][futures-crate] 和 [`tokio`][tokio] crate。`futures` crate 是 Rust 非同步程式碼實驗的官方陣地，`Future` trait 最初就是在那裡設計出來的。Tokio 則是目前 Rust 中使用最廣泛的非同步執行時（async runtime），尤其常見於 Web 應用。生態中也還有其他很優秀的執行時，而且它們可能更適合你的實際用途。我們在 `trpl` 的底層使用 `tokio`，是因為它經過了充分測試，也足夠常用。

在某些場景下，`trpl` 還會對原始 API 進行重新命名或包裝，好讓你把注意力集中在本章相關的細節上。如果你想了解這個 crate 實際做了什麼，我們建議你看看[它的原始碼][crate-source]。你可以從中看到每個重匯出項究竟來自哪個 crate，我們也留下了很多註釋來解釋這個 crate 的行為。

建立一個名為 `hello-async` 的二進位制專案並將 `trpl` crate 作為一個依賴新增：

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

現在我們可以利用 `trpl` 提供的各種元件來編寫第一個非同步程式。我們要構建一個小型命令列工具：抓取兩個網頁，從各自頁面中提取 `<title>` 元素，然後打印出那個最先完成整套流程的頁面標題。

### 定義 page_title 函式

讓我們開始編寫一個函式，它獲取一個網頁 URL 作為引數，請求該 URL 並返回標題元素的文字（見示例 17-1）。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

<figcaption>示例 17-1：定義一個 async 函式來獲取一個 HTML 頁面的標題元素</figcaption>

</figure>

首先，我們定義了一個名為 `page_title` 的函式，並用 `async` 關鍵字標記它。然後使用 `trpl::get` 函式抓取傳入的 URL，再用 `await` 關鍵字等待響應。為了得到 `response` 的文字，我們呼叫它的 `text` 方法，並再次使用 `await` 進行等待。這兩個步驟都是非同步的。對於 `get` 函式來說，我們必須等待伺服器先把響應的第一部分發回來，其中包括 HTTP headers、cookies 等，這些內容可以和響應體分開發送。尤其當響應體很大時，全部資料到達可能要花上一些時間。由於我們必須等待響應*完整*到達，`text` 方法自然也是 async 的。

我們必須顯式地等待這兩個 future，因為 Rust 中的 future 是 *lazy* 的：在你用 `await` 請求它之前，它什麼都不會做。（實際上，如果你建立了 future 卻不使用它，Rust 還會給出編譯器警告。）這大概會讓你想起第十三章[“使用迭代器處理元素序列”][iterators-lazy]中的討論。迭代器只有在你呼叫 `next` 方法時才會工作，無論是直接呼叫，還是透過 `for` 迴圈，或者藉助像 `map` 這樣底層會呼叫 `next` 的方法。future 也是一樣，只有你顯式要求它執行時，它才會開始工作。這種惰性讓 Rust 能夠避免在真正需要之前就執行非同步程式碼。

> 注意：這和我們在第十六章[“使用 spawn 建立新執行緒”][thread-spawn]裡看到的 `thread::spawn` 的行為不同，在那裡我們傳給新執行緒的閉包會立刻開始執行。它也和許多其他語言處理 async 的方式不同。但這對於 Rust 提供它一貫的效能保證很重要，正如迭代器也是如此。

有了 `response_text` 之後，我們就可以用 `Html::parse` 把它解析成 `Html` 型別的例項。這樣一來，我們得到的就不再是原始字串，而是一個可以把 HTML 當作更豐富資料結構來操作的型別。特別是，我們可以用 `select_first` 方法找到給定 CSS selector 的第一個匹配項。傳入字串 `"title"` 後，我們就能拿到文件中的第一個 `<title>` 元素，如果它存在的話。因為也可能根本沒有匹配項，所以 `select_first` 返回的是 `Option<ElementRef>`。最後，我們使用 `Option::map` 方法：如果 `Option` 中有值，它就會對其中的值進行處理；如果沒有，就什麼都不做。（這裡當然也可以使用 `match` 表示式，不過 `map` 更符合慣用寫法。）在我們傳給 `map` 的閉包裡，會對 `title` 呼叫 `inner_html` 來獲取其中的內容，它是一個 `String`。到這裡，我們最終得到的就是一個 `Option<String>`。

注意，Rust 的 `await` 關鍵字放在要等待的表示式*後面*，而不是前面。也就是說，它是一個 *postfix keyword*（字尾關鍵字）。如果你在其他語言裡用過 async，這一點可能和你的習慣不同；但在 Rust 中，這種設計會讓鏈式方法呼叫更易讀。因此，我們可以把 `page_title` 的函式體改寫成在 `trpl::get` 和 `text` 呼叫之間插入 `await` 的鏈式寫法，如示例 17-2 所示：

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

<figcaption>示例 17-2：使用 `await` 關鍵字的鏈式呼叫</figcaption>

</figure>

這樣我們就成功編寫了第一個非同步函式！在我們向 `main` 加入一些程式碼呼叫它之前，讓我們再多瞭解下我們寫了什麼以及它的意義。

當 Rust 遇到一個 `async` 關鍵字標記的程式碼塊時，會將其編譯為一個實現了 `Future` trait 的唯一的、匿名的資料型別。當 Rust 遇到一個被標記為 `async` 的函式時，會將其編譯成一個函式體是非同步程式碼塊的非非同步函式。非同步函式的返回值型別是編譯器為非同步程式碼塊所建立的匿名資料型別。

因此，編寫 `async fn` 就等同於編寫一個返回型別為 *future* 的函式。當編譯器遇到類似示例 17-1 中 `async fn page_title` 的函式定義時，它等價於以下定義的非非同步函式：

```rust
# extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

讓我們挨個看一下轉換後版本的每一個部分：

- 它使用了之前第十章 [“trait 作為引數”][impl-trait] 部分討論過的 `impl Trait` 語法。
- 它返回的值實現了 `Future` trait，並且這個 trait 有一個關聯型別 `Output`。注意 `Output` 的型別是 `Option<String>`，這和 `async fn` 版本的 `page_title` 的原始返回型別一致。
- 原始函式體中的所有程式碼都被包進了一個 `async move` 塊。回憶一下，程式碼塊本身就是表示式。整個塊就是函式返回的那個表示式。
- 如上所述，這個非同步程式碼塊產生一個 `Option<String>` 型別的值。這個值與返回型別中的 `Output` 型別一致。這正類似於你已經見過的其它程式碼塊。
- 這個新函式體之所以是 `async move` 塊，是由它使用 `url` 引數的方式決定的。（本章後面會更詳細地討論 `async` 和 `async move` 的區別。）

現在我們可以在 `main` 中呼叫 `page_title`。

### 使用執行時執行非同步函式

首先，我們只獲取單個頁面的標題，如示例 17-3 所示。不幸的是，這段程式碼還不能編譯。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

<figcaption>示例 17-3：在 `main` 中透過一個使用者提供的引數呼叫 `page_title` 函式</figcaption>

</figure>

我們沿用了第十二章[“接受命令列引數”][cli-args]一節中獲取命令列引數的模式。然後把 URL 引數傳給 `page_title`，再等待它的結果。由於 future 產出的值是 `Option<String>`，我們使用 `match` 表示式來根據頁面是否含有 `<title>` 列印不同的資訊。

唯一能使用 `await` 關鍵字的地方，是 async 函式或 async 程式碼塊中，而 Rust 又不允許我們把特殊的 `main` 函式標記為 `async`。

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main` 不能標記為 `async` 的原因是非同步程式碼需要一個 *執行時*：即一個管理執行非同步程式碼細節的 Rust crate。一個程式的 `main` 函式可以 *初始化* 一個執行時，但是其 *自身* 並不是一個執行時。（稍後我們會進一步解釋原因。）每一個執行非同步程式碼的 Rust 程式必須至少有一個設定執行時並執行 futures 的地方。

大多數支援 async 的語言都會自帶執行時，但 Rust 不會。相反，Rust 有很多不同的非同步執行時可供選擇，每一種都針對自己的目標用例做了不同權衡。比如，一個擁有許多 CPU 核心和大量 RAM 的高吞吐 Web 伺服器，和一個單核、RAM 很小、甚至不能進行堆分配的微控制器，需求就截然不同。提供這些執行時的 crate 往往也會一併提供檔案或網路 I/O 等常見功能的非同步版本。

在這裡，以及本章餘下的部分，我們會使用 `trpl` crate 提供的 `block_on` 函式。它接受一個 future 作為引數，並阻塞當前執行緒，直到這個 future 執行完成為止。在內部，呼叫 `block_on` 會藉助 `tokio` crate 設定一個執行時，用來執行傳入的 future（`trpl` 的 `block_on` 和其他執行時 crate 提供的同名函式行為類似）。一旦 future 完成，`block_on` 就會返回 future 產生的值。

我們當然可以把 `page_title` 返回的 future 直接傳給 `block_on`，並在它完成後對得到的 `Option<String>` 進行匹配，就像我們在示例 17-3 中本來打算做的那樣。不過，本章的大部分例子裡（以及現實中的大多數 async 程式碼裡），我們都不止會進行一次非同步函式呼叫，因此我們改為傳入一個 `async` 塊，並在其中顯式等待 `page_title` 的結果，如示例 17-4 所示。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

<figcaption>示例 17-4：使用 `trpl::block_on` 等待一個 async 程式碼塊</figcaption>

</figure>

當我們執行這段程式碼時，就會得到一開始期待的行為：

```console
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

我們終於有了一些可以正常工作的非同步程式碼！不過在我們新增程式碼讓兩個網址進行競爭之前，讓我們簡要地回顧一下 future 是如何工作的。

每一個 *await point*，也就是程式碼使用 `await` 關鍵字的地方，代表將控制權交還給執行時的地方。為此 Rust 需要記錄非同步程式碼塊中涉及的狀態，這樣執行時可以去執行其他工作，並在準備好時回來繼續推進當前的任務。這就像你透過編寫一個列舉來儲存每一個 `await` point 的狀態一樣：

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

編寫程式碼來手動控制不同狀態之間的轉換是非常乏味且容易出錯的，特別是之後增加了更多功能和狀態的時候。相反，Rust 編譯器自動建立並管理非同步程式碼的狀態機資料結構。如果你感興趣的話：是的，正常的借用和所有權也全部適用於這些資料結構。幸運的是，編譯器也會為我們處理這些檢查，並提供友好的錯誤資訊。本章稍後會講解一些相關內容！

最終，總得有某個元件來執行這個狀態機，而那個元件就是執行時。（這也是為什麼在瞭解執行時時，你可能會看到 *executor* 這個詞：executor 是執行時中負責執行非同步程式碼的那一部分。）

現在你就能理解，為什麼編譯器會在示例 17-3 中阻止我們把 `main` 本身寫成非同步函數了。如果 `main` 是 async 函式，那麼就必須有別的東西來管理 `main` 返回的 future 對應的狀態機；可 `main` 本身就是程式的入口點！因此，我們改為在 `main` 中呼叫 `trpl::block_on`，讓它設定好執行時，並執行 `async` 塊返回的 future，直到執行完成。

> 注意：有些執行時會提供宏，因此你*確實可以*寫非同步版的 `main` 函式。這些宏會把 `async fn main() { ... }` 重寫成普通的 `fn main`，其邏輯和我們在示例 17-4 中手動做的事情一樣：呼叫一個像 `trpl::block_on` 這樣的函式，把 future 跑到完成為止。

現在讓我們把這些部分組合起來，看看如何編寫併發程式碼。

### 讓兩個 URL 併發競爭

在示例 17-5 中，我們會對從命令列傳入的兩個不同 URL 分別呼叫 `page_title`，並選出最先完成的那個 future。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

<figcaption>示例 17-5：對兩個 URL 呼叫 `page_title`，看誰先返回</figcaption>

</figure>

我們首先分別對使用者提供的兩個 URL 呼叫 `page_title`。隨後把得到的 future 儲存到 `title_fut_1` 和 `title_fut_2` 中。記住，它們此時還什麼都沒做，因為 future 是惰性的，而我們也還沒有等待它們。接著我們把這些 future 傳給 `trpl::select`，它會返回一個值，用來表明傳入的 future 中哪一個最先完成。

> 注意：在底層，`trpl::select` 建立在 `futures` crate 中更通用的 `select` 函式之上。`futures` crate 的 `select` 函式能做很多 `trpl::select` 做不到的事，不過它也帶來了一些額外複雜性，所以我們暫時先跳過。

任意一個 future 都有可能“獲勝”，因此這裡返回 `Result` 並不合理。相反，`trpl::select` 返回的是一個我們之前還沒見過的型別：`trpl::Either`。`Either` 在某種程度上有點像 `Result`，也有兩個分支；但不同的是，它並沒有內建“成功”或“失敗”的語義，而是用 `Left` 和 `Right` 來表示“這個或那個”。

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

如果第一個引數先完成，`select` 就返回 `Left`，其中包含該 future 的輸出；如果第二個 future 先完成，則返回 `Right`，其中包含第二個 future 的輸出。這正好對應函式呼叫時引數的順序：第一個引數位於第二個引數的左邊。

我們還更新了 `page_title`，讓它把傳入的 URL 一併返回。這樣一來，即使最先返回的頁面無法解析出 `<title>`，我們仍然可以打印出一條有意義的資訊。有了這些資料之後，我們最後再調整 `println!` 的輸出，讓它既能顯示哪個 URL 最先完成，也能在頁面存在 `<title>` 時打印出標題內容。

至此，你已經構建出了一個可以工作的迷你網頁抓取器！隨便選兩個 URL 執行一下這個命令列工具吧。你會發現有些站點總是比另一些更快，而另一些情況下則每次執行誰快誰慢都不一定。更重要的是，你已經掌握了使用 future 的基礎知識，所以現在我們可以繼續深入，看看 async 還能做些什麼。

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs

[impl-trait]: ch10-02-traits.html#使用-trait-作為引數
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#使用-spawn-建立新執行緒
[cli-args]: ch12-01-accepting-command-line-arguments.html
