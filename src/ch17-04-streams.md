## Stream：按順序出現的 Future

[ch17-04-streams.md](https://github.com/rust-lang/book/blob/2c56b53dfb99d0c3eca23b36e37fb0638eb57dbe/src/ch17-04-streams.md)

回憶一下本章前面在[“通過訊息傳遞在兩個任務之間傳送資料”][17-02-messages]<!-- ignore -->一節中，我們是如何使用非同步通道的接收端的。非同步版的 `recv` 方法會隨著時間推移產出一系列條目。這正是一種更普遍模式的例項，通常稱為 *流*（*stream*）。很多概念都很自然地適合表示成 stream：佇列中逐步變得可用的項、當完整資料集太大而無法一次裝入記憶體時從檔案系統中逐塊拉取的資料、或者隨著時間逐漸從網路到達的資料。由於 stream 本身也和 future 密切相關，我們可以把它和其他型別的 future 一起使用，並以有趣的方式進行組合。比如，我們可以把事件分批處理，以避免觸發過多網路呼叫；可以為一串長時間執行的操作設定超時；也可以對 UI 事件進行節流，避免做無謂的工作。

我們在第十三章[“Iterator trait 和 `next` 方法”][iterator-trait]<!-- ignore -->一節中已經見過“按順序產生一系列項”這回事，但迭代器和非同步通道接收端之間有兩個區別。第一個區別是時間：迭代器是同步的，而通道接收端是非同步的。第二個區別是 API。直接處理 `Iterator` 時，我們會呼叫同步的 `next` 方法；而對於 `trpl::Receiver` 這個具體的 stream 來說，我們呼叫的是非同步的 `recv` 方法。除此之外，這些 API 給人的感覺非常相似，而這種相似並非巧合。stream 就像迭代的一種非同步形式。不過，`trpl::Receiver` 專門用於等待接收訊息，而更通用的 stream API 則寬泛得多：它像 `Iterator` 一樣提供“下一個條目”，只不過是以非同步方式來做。

Rust 中迭代器和 stream 的這種相似性意味著，我們實際上可以從任意迭代器建立一個 stream。和使用迭代器一樣，我們也可以通過呼叫 stream 的 `next` 方法，再等待其輸出，來處理它，如示例 17-21 所示。不過這段程式碼暫時還編譯不過。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

<figcaption>示例 17-21：從迭代器建立一個 stream，並打印出其中的值</figcaption>

</figure>

我們從一個數字陣列開始，把它轉換為迭代器，然後呼叫 `map` 把其中所有值都翻倍。接著再使用 `trpl::stream_from_iter` 函式，把這個迭代器轉換成一個 stream。最後，我們用 `while let` 迴圈，在 stream 中的值陸續到達時逐個處理它們。

遺憾的是，當我們嘗試執行這段程式碼時，編譯器並不會通過，而是報告說沒有可用的 `next` 方法：

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

正如這段輸出解釋的那樣，編譯錯誤的原因是：我們需要把正確的 trait 放進作用域，才能使用 `next` 方法。根據前面的討論，你很可能會合理地猜測這個 trait 應該是 `Stream`，但實際上它是 `StreamExt`。這裡的 `Ext` 是 *extension* 的縮寫；在 Rust 社群裡，用一個 trait 去擴充套件另一個 trait，是非常常見的模式。

`Stream` trait 定義的是一個底層介面，它實際上把 `Iterator` 和 `Future` trait 的特徵結合在了一起。`StreamExt` 則在 `Stream` 之上提供了一組更高層的 API，其中包括 `next` 方法，以及其他一些和 `Iterator` trait 提供的工具方法相似的輔助方法。`Stream` 和 `StreamExt` 目前都還不是 Rust 標準庫的一部分，不過生態系統中的大多數 crate 都使用相似的定義。

修復這個編譯錯誤的方式，就是像示例 17-22 那樣，新增一條 `use trpl::StreamExt` 語句。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

<figcaption>示例 17-22：成功把迭代器作為 stream 的基礎來使用</figcaption>

</figure>

把這些部分拼起來之後，這段程式碼就會按我們想要的方式工作！更重要的是，既然我們已經把 `StreamExt` 引入作用域，就也能像使用迭代器時那樣，使用它提供的整套工具方法。

[17-02-messages]: ch17-02-concurrency-with-async.html#通過訊息傳遞在兩個任務之間傳送資料
[iterator-trait]: ch13-02-iterators.html#iterator-trait-和-next-方法
