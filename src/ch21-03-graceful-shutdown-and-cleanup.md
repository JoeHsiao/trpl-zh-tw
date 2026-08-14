## 優雅停機與清理

[ch21-03-graceful-shutdown-and-cleanup.md](https://github.com/rust-lang/book/blob/8aa0d003e6499d733d639de32d70f590efa48657/src/ch21-03-graceful-shutdown-and-cleanup.md)

示例 21-20 中的程式碼如我們所願，藉助執行緒池非同步地響應請求。這裡有一些關於 `workers`、`id` 和 `thread` 欄位沒有被直接使用的警告，這提醒了我們還有一些東西沒有清理。當我們用不那麼優雅的 <kbd>ctrl</kbd>-<kbd>c</kbd> 方式終止主執行緒時，其他所有執行緒也都會立刻停止，即便它們正處於處理請求的過程中。

接下來，我們要實現 `Drop` trait，在其中對執行緒池裡的每個執行緒呼叫 `join`，這樣它們就能在關閉前完成手頭正在處理的請求。然後，我們還會實現一種方式，通知這些執行緒停止接收新請求並關閉。為了觀察這些程式碼的實際效果，我們會修改 server，讓它在優雅停機之前只接受兩個請求。

這裡有一點需要先注意：這一切都不會影響執行閉包的那部分程式碼，因此如果我們是在 async 執行時裡使用執行緒池，這一節中的內容也完全相同。

### 為 `ThreadPool` 實現 `Drop` Trait

現在開始為執行緒池實現 `Drop`。當執行緒池被丟棄時，應該 join 所有執行緒以確保它們完成其操作。示例 21-22 展示了 `Drop` 實現的第一次嘗試；這些程式碼還不能夠編譯：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

<span class="caption">示例 21-22: 當執行緒池離開作用域時 join 每個執行緒</span>

這裡首先遍歷執行緒池中的每個 `worker`。之所以使用 `&mut`，是因為 `self` 是一個可變引用，而且我們還需要修改 `worker`。對於每個 `worker`，我們都會列印一條訊息，說明該 `Worker` 例項正在關閉，然後對這個 `Worker` 例項中的執行緒呼叫 `join`。如果 `join` 呼叫失敗，就用 `unwrap` 讓 Rust panic，進入一種不夠優雅的關閉方式。

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

這裡的錯誤告訴我們不能呼叫 `join`，因為我們手裡只有每個 `worker` 的可變借用，而 `join` 需要拿走其引數的所有權。要解決這個問題，我們需要把 `thread` 從擁有它的 `Worker` 例項中移出來，這樣 `join` 才能消費這個執行緒。一種做法和示例 18-15 中類似：如果 `Worker` 存放的是 `Option<thread::JoinHandle<()>>`，就可以在這個 `Option` 上呼叫 `take` 方法，把值從 `Some` 變體中移出來，並在原地留下一個 `None`。換句話說，正在執行的 `Worker` 會在 `thread` 欄位中持有一個 `Some`，而當我們想清理這個 `Worker` 時，就把 `Some` 替換成 `None`，這樣這個 `Worker` 就不再持有可執行的執行緒。

然而，這種情況**只**會在丟棄 `Worker` 時出現。相應地，我們必須在任何訪問 `worker.thread` 時處理 `Option<thread::JoinHandle<()>>`。在慣用的 Rust 程式碼中 `Option` 用的很多，但當你發現自己總是知道 `Option` 中一定會有值，卻還要將其包裝在 `Option` 中來應對這一場景時，就應該考慮其他更優雅的方法了。

在這個例子中，存在一個更好的替代方案：`Vec::drain` 方法。它接受一個 range 引數來指定哪些項要從 `Vec` 中移除，並返回一個這些項的迭代器。使用 `..` range 語法會從 `Vec` 中移除**所有**值。

因此我們需要像下面這樣更新 `ThreadPool` 的 `drop` 實現：

<span class="filename">檔名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

這解決了編譯器錯誤，而且不需要對我們的程式碼做任何其他修改。注意，因為 `drop` 可能在 panic 過程中被呼叫，這裡的 `unwrap` 也可能再次 panic，造成雙重 panic，並立刻讓程式崩潰，中斷正在進行的清理。對於示例程式來說這沒問題，但在生產程式碼中並不推薦這樣做。

### 向執行緒發出訊號，讓它們停止接收任務

有了這些修改後，我們的程式碼已經可以無警告編譯。然而壞訊息是，它還不能按我們期望的方式工作。關鍵在於 `Worker` 例項中執行緒所執行的閉包邏輯：目前我們確實呼叫了 `join`，但這並不會讓執行緒停止，因為它們會永遠 `loop` 下去尋找新任務。如果用當前這個 `drop` 實現去丟棄 `ThreadPool`，主執行緒會永遠阻塞，等待第一個執行緒結束。

為了修復這個問題，我們將修改 `ThreadPool` 的 `drop` 實現並修改 `Worker` 迴圈。

首先修改 `ThreadPool` 的 `drop` 實現在等待執行緒結束前顯式地丟棄 `sender`。示例 21-23 展示了 `ThreadPool` 顯式丟棄 `sender` 所作的修改。與處理執行緒時不同，這裡**確實**需要使用 `Option`，以便能夠使用 `Option::take` 將 `sender` 從 `ThreadPool` 中移出。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

<span class="caption">示例 21-23: 在 join `Worker` 執行緒之前顯式丟棄 `sender`</span>

丟棄 `sender` 會關閉通道，這表明不會有更多的訊息被髮送。這時 `Worker` 例項中的無限迴圈中的所有 `recv` 呼叫都會返回錯誤。在示例 21-24 中，我們修改 `Worker` 迴圈在這種情況下優雅地退出，這意味著當 `ThreadPool` 的 `drop` 實現呼叫 `join` 時執行緒會結束。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

<span class="caption">示例 21-24: 當 `recv` 返回錯誤時顯式跳出迴圈</span>

為了實踐這些程式碼，如示例 21-25 所示修改 `main` 在優雅停機服務端之前只接受兩個請求：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

<span class="caption">示例 21-25: 在處理兩個請求之後通過退出迴圈來停止服務端</span>

你不會希望真實世界的 web 服務端只處理兩次請求就停機了，這只是為了展示優雅停機和清理處於正常工作狀態。

`take` 方法定義在 `Iterator` trait 上，它會把迭代限制為至多前兩個元素。`ThreadPool` 會在 `main` 結束時離開作用域，隨後 `drop` 實現就會執行。

使用 `cargo run` 啟動服務端，併發起三個請求。第三個請求應該會失敗，而終端的輸出應該看起來像這樣：

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

你看到的 `Worker` ID 和列印訊息順序可能會有所不同。我們可以從這些訊息中看出程式碼是如何工作的：`Worker` 例項 0 和 3 處理了前兩個請求。server 在接收到第二個連線後就停止接受新連線，而 `ThreadPool` 上的 `Drop` 實現甚至會在 `Worker` 3 真正開始處理任務之前就開始執行。丟棄 `sender` 會讓所有 `Worker` 例項斷開連線，並通知它們關閉。每個 `Worker` 例項在斷開時都會列印一條訊息，然後執行緒池會呼叫 `join`，等待每個 `Worker` 執行緒結束。

注意這個具體執行過程裡有個有意思的地方：`ThreadPool` 丟棄了 `sender` 之後，在任何一個 `Worker` 收到錯誤之前，就先嚐試去 join `Worker` 0。此時 `Worker` 0 還沒有從 `recv` 得到錯誤，所以主執行緒會阻塞，等待 `Worker` 0 結束。與此同時，`Worker` 3 收到了一個任務，然後所有執行緒都會收到錯誤。當 `Worker` 0 完成後，主執行緒再等待其餘 `Worker` 例項結束。那時，它們都已經退出迴圈並停止了。

恭喜！現在我們完成了這個專案，也有了一個使用執行緒池非同步響應請求的基礎 web 服務端。我們能對服務端執行優雅停機，它會清理執行緒池中的所有執行緒。

如下是完整的程式碼參考：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

我們還能做得更多！如果你希望繼續增強這個專案，如下是一些點子：

- 為 `ThreadPool` 和它的公有方法補充更多文件。
- 為這個庫的功能新增測試。
- 把對 `unwrap` 的呼叫改成更健壯的錯誤處理。
- 用 `ThreadPool` 來執行除處理 web 請求之外的其他任務。
- 在 [crates.io](https://crates.io/) 上找一個執行緒池 crate，用它實現一個類似的 web server，然後比較它的 API 和魯棒性與我們實現的執行緒池有何不同。

## 總結

好極了！你已經完成了本書的學習！由衷感謝你與我們一道踏上這段 Rust 之旅。現在你已經準備好實現自己的 Rust 專案並幫助他人了。請不要忘記我們的社群，這裡有其他 Rustaceans 正樂於幫助你迎接 Rust 之路上的任何挑戰。
