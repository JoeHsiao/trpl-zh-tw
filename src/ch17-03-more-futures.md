### 將控制權交還給執行時

[ch17-03-more-futures.md](https://github.com/rust-lang/book/blob/dec931300f1a00e6e26021907192b0634980950c/src/ch17-03-more-futures.md)

回憶一下我們在[“第一個非同步程式”][async-program]<!-- ignore -->一節中提到的內容：在每個 await 點，如果被等待的 future 還沒準備好，Rust 就會給執行時一個機會來暫停當前任務並切換到其他任務。反過來也成立：Rust *只會* 在 await 點暫停 async 程式碼塊，並把控制權交還給執行時。await 點之間的所有內容都是同步執行的。

這意味著，如果你在一個 async 程式碼塊中做了大量工作，卻沒有任何 await 點，那麼這個 future 就會阻止其他 future 取得進展。有時你會聽到人們把這稱為一個 future 讓其他 future *starve*（飢餓）。在某些場景下，這也許不是什麼大問題；但如果你在做昂貴的初始化、長時間執行的工作，或者你有一個會無限執行某項任務的 future，就需要認真考慮該在何時、何地把控制權交還給執行時。

讓我們透過模擬一個長時間執行的操作來展示這種“飢餓”問題，再看看該如何解決。示例 17-14 引入了一個 `slow` 函式。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

<figcaption>示例 17-14：使用 `thread::sleep` 來模擬緩慢的操作</figcaption>

</figure>

這段程式碼使用的是 `std::thread::sleep`，而不是 `trpl::sleep`，因此呼叫 `slow` 會讓當前執行緒阻塞若干毫秒。我們可以把 `slow` 看作現實世界中那些既耗時又會阻塞的操作的替身。

在示例 17-15 中，我們用 `slow` 來模擬在一對 future 中執行這類 CPU 密集型工作。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

<figcaption>示例 17-15：呼叫 `slow` 函式來模擬緩慢操作</figcaption>

</figure>

每個 future 都會在完成一大串緩慢操作之後，才把控制權交還給執行時。如果你執行這段程式碼，就會看到如下輸出：

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

和示例 17-5 中用 `trpl::select` 讓兩個 URL 獲取任務競爭時一樣，`select` 仍然會在 `a` 完成時立刻結束。不過，這兩個 future 裡的 `slow` 呼叫之間完全沒有交錯執行。`a` future 會一路把自己的工作做完，直到等待 `trpl::sleep` 呼叫；接著 `b` future 又一路做完自己的工作，直到它自己的 `trpl::sleep` 被等待；最後 `a` future 才完成。要想讓兩個 future 在這些緩慢任務之間都取得進展，我們就需要一些 await 點，好把控制權交還給執行時。也就是說，我們得有某種可以被 await 的東西！

實際上，我們已經能在示例 17-15 中看到這種“交接”是如何發生的：如果去掉 `a` future 末尾的 `trpl::sleep`，那麼它會直接完成，而 `b` future *根本不會執行*。讓我們先從 `trpl::sleep` 入手，試著讓這些操作能夠輪流取得進展，如示例 17-16 所示。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

<figcaption>示例 17-16：使用 `trpl::sleep` 讓操作輪流推進</figcaption>

</figure>

我們在每次呼叫 `slow` 之間都插入了 `trpl::sleep` 呼叫和 await 點。現在，兩個 future 的工作就交錯在一起了：

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

`a` future 仍然會在第一次把控制權交給 `b` 之前先執行一陣，因為它是在第一次呼叫 `trpl::sleep` 之前先執行了 `slow`；但在那之後，每當其中一個 future 命中 await 點，它們就會來回切換。在這個例子中，我們是在每次 `slow` 之後這麼做的，不過實際上也可以按任何對你最合理的方式來拆分工作。

但我們其實並不是真的想在這裡*休眠*（*sleep*）；我們只是希望程式儘可能快地前進。我們真正需要的只是把控制權交還給執行時。可以直接透過 `trpl::yield_now` 來做到這一點。在示例 17-17 中，我們把前面的 `trpl::sleep` 全部替換成 `trpl::yield_now`。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

<figcaption>示例 17-17：使用 `yield_now` 讓操作輪流推進</figcaption>

</figure>

這段程式碼不僅更清楚地表達了真實意圖，而且通常也會比使用 `sleep` 快得多，因為像 `sleep` 使用的那類計時器，往往都會受到最小粒度限制。比如我們這裡使用的 `sleep`，即使你傳入的是 1 納秒的 `Duration`，它也至少會睡眠 1 毫秒。再說一次，現代計算機是非常*快*的：1 毫秒裡已經能完成大量工作了！

這說明：即使面對計算密集型任務，async 依然可能是有用的，具體取決於你的程式還在做什麼，因為它提供了一種很實用的手段，用來組織程式不同部分之間的關係（當然代價是 async 狀態機本身也有一定開銷）。這是一種 *協作式多工*（*cooperative multitasking*）：每個 future 都可以透過 await 點來決定何時交出控制權，因此每個 future 也都負有避免長時間阻塞的責任。在某些基於 Rust 的嵌入式作業系統中，這甚至是*唯一*的多工形式！

當然，在真實程式碼裡，你通常不會在每一行之間都交替插入函式呼叫和 await 點。像這樣主動交出控制權雖然相對便宜，但並不是沒有代價。在很多情況下，試圖把一個計算密集型任務切得太碎，反而可能顯著拖慢它的執行速度，所以有時為了*整體*效能，讓某個操作短暫阻塞一下反而更好。還是那句老話：一定要靠測量來確認程式碼真正的效能瓶頸。不過，如果你發現原本預期會併發執行的工作，實際上卻大量序列發生，那麼就要記住這裡的底層機制。

### 構建我們自己的非同步抽象

我們還可以把多個 future 組合在一起，創造出新的模式。比如，完全可以用手頭已有的 async 構件來寫一個 `timeout` 函式。等我們做完，它本身就會成為另一個可以繼續拿來構建更多 async 抽象的基礎模組。

示例 17-18 展示了我們希望這個 `timeout` 在面對一個緩慢 future 時應該如何工作。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

<figcaption>示例 17-18：使用我們設想的 `timeout` 為一個緩慢操作設定時間限制</figcaption>

</figure>

讓我們來實現它。首先先想想 `timeout` 的 API：

- 它本身需要是一個 async 函式，這樣我們才能等待它。
- 它的第一個引數應該是一個要執行的 future。我們可以把它設計成泛型，從而支援任意 future。
- 它的第二個引數應該是最大等待時間。如果用 `Duration`，就能很方便地直接傳給 `trpl::sleep`。
- 它應該返回一個 `Result`。如果傳入的 future 成功完成，結果應當是 `Ok`，內部帶上該 future 產生的值；如果先發生超時，結果就應當是 `Err`，內部帶上等待的時長。

示例 17-19 展示了這個宣告。

<!-- This is not tested because it intentionally does not compile. -->

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

<figcaption>示例 17-19：定義 `timeout` 的簽名</figcaption>

</figure>

這樣型別層面的目標就滿足了。接下來想想我們需要的*行為*：我們希望讓傳入的 future 和這個時長“競爭”。可以用 `trpl::sleep` 根據這個時長構造一個計時器 future，再用 `trpl::select` 讓計時器和呼叫者傳入的 future 一起執行。

在示例 17-20 中，我們透過匹配 `trpl::select` 的等待結果來實現 `timeout`。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

<figcaption>示例 17-20：使用 `select` 和 `sleep` 定義 `timeout`</figcaption>

</figure>

`trpl::select` 的實現並不是公平的：它總是按引數傳入的順序進行輪詢（其他一些 `select` 實現會隨機選擇先輪詢哪個引數）。因此，我們把 `future_to_try` 作為第一個引數傳給 `select`，好讓它即使在 `max_time` 很短的情況下，也仍然有機會先完成。如果 `future_to_try` 先完成，`select` 會返回 `Left`，其中包含 `future_to_try` 的輸出；如果 `timer` 先完成，`select` 就會返回 `Right`，其中包含計時器的輸出 `()`。

如果 `future_to_try` 成功完成，並且我們得到了 `Left(output)`，那麼就返回 `Ok(output)`。如果相反是睡眠計時器先結束，我們得到 `Right(())`，那就用 `_` 忽略這個 `()`，並返回 `Err(max_time)`。

這樣一來，我們就用另外兩個 async 小工具拼出了一個可工作的 `timeout`。如果執行程式碼，它會在超時後打印出失敗資訊：

```text
Failed after 2 seconds
```

因為 future 可以和其他 future 組合，你就能利用更小的 async 構件構建出非常強大的工具。比如，完全可以用同樣的方法把 timeout 和 retry 組合起來，再進一步把它們用在網路請求一類的操作上（例如示例 17-5 裡的那些）。

在實踐中，你通常會主要直接使用 `async` 和 `await`，其次才是像 `select` 這樣的函式，以及像 `join!` 這樣的宏，來控制最外層的 future 應該如何執行。

到這裡，我們已經看過好幾種同時處理多個 future 的方式了。接下來，我們將看看如何藉助 *stream*，按照時間順序處理一串 future。

[async-program]: ch17-01-futures-and-syntax.html#第一個非同步程式
