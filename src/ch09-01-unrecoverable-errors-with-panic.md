## 用 `panic!` 處理不可恢復的錯誤

[ch09-01-unrecoverable-errors-with-panic.md](https://github.com/rust-lang/book/blob/d46785983db2d2f94ca3d571db2cfbad0f5ad3e6/src/ch09-01-unrecoverable-errors-with-panic.md)

有時，你的程式碼裡會發生一些糟糕的事情，而且你對此無能為力。在這種情況下，Rust 提供了 `panic!` 宏。實際中有兩種方式會導致 panic：一種是執行了會讓程式碼 panic 的操作，比如訪問超出陣列結尾的位置；另一種是顯式呼叫 `panic!` 宏。這兩種情況都會讓程式 panic。預設情況下，這些 panic 會列印失敗資訊、展開棧、清理棧資料，然後退出。你還可以透過環境變數，讓 Rust 在 panic 發生時顯示呼叫棧（call stack），以便更容易追蹤 panic 的來源。

> ### 響應 panic 時的棧展開或終止
>
> 當出現 panic 時，程式預設會開始 **展開**（*unwinding*），這意味著 Rust 會回溯棧並清理它遇到的每一個函式的資料，不過這個回溯並清理的過程有很多工作。另一種選擇是直接 **終止**（*abort*），這會不清理資料就退出程式。
>
> 那麼程式所使用的記憶體需要由作業系統來清理。如果你需要專案的最終二進位制檔案越小越好，panic 時透過在 *Cargo.toml* 的 `[profile]` 部分增加 `panic = 'abort'`，可以由展開切換為終止。例如，如果你想要在 release 模式中 panic 時直接終止，可新增：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

讓我們在一個簡單的程式中呼叫 `panic!`：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

執行程式將會出現類似這樣的輸出：

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

最後兩行包含 `panic!` 呼叫造成的錯誤資訊。第一行顯示了 panic 提供的資訊並指明瞭原始碼中 panic 出現的位置：*src/main.rs:2:5* 表明這是 *src/main.rs* 檔案的第二行第五個字元。

在這個例子中，被指明的那一行是我們程式碼的一部分，如果跳轉到該行，就會發現 `panic!` 宏的呼叫。在其它情況下，`panic!` 可能會出現在我們的程式碼所呼叫的程式碼中。錯誤資訊報告的檔名和行號可能指向別人程式碼中的 `panic!` 宏呼叫，而不是我們程式碼中最終導致 `panic!` 的那一行。

<a id="using-a-panic-backtrace"></a>

### 使用 `panic!` 的 backtrace

我們可以利用觸發 `panic!` 的函式 backtrace，找出程式碼裡到底是哪一部分出了問題。為了理解如何使用 `panic!` 的 backtrace，讓我們再看一個例子：這次 `panic!` 呼叫不是來自我們直接呼叫宏，而是因為我們程式碼裡的 bug 觸發了庫中的 `panic!`。示例 9-1 展示了一段嘗試訪問 vector 有效索引範圍之外元素的程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">示例 9-1：嘗試訪問超越 vector 結尾的元素，這會造成 `panic!`</span>

這裡嘗試訪問 vector 的第 100 個元素（這裡的索引是 99 因為索引從 0 開始），不過它只有三個元素。這種情況下 Rust 會 panic。`[]` 應當返回一個元素，不過如果傳遞了一個無效索引，就沒有可供 Rust 返回的正確元素。

C 語言中，嘗試讀取資料結構末尾之後的內容屬於未定義行為（undefined behavior）。你可能會讀到資料結構中對應那個位置的記憶體裡的任意值，即使那塊記憶體根本不屬於這個資料結構。這被稱為**緩衝區過讀**（*buffer overread*），並可能導致安全漏洞；例如，攻擊者也許能透過操縱索引，讀取本不該被讀取、但恰好儲存在該資料結構之後的資料。

為了保護程式不受此類漏洞的影響，如果嘗試讀取一個索引不存在的元素，Rust 會停止執行並拒絕繼續。讓我們來試一試，看看結果：

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

這個錯誤指向了 *main.rs* 的第 4 行，也就是我們試圖訪問向量 `v` 中索引 `99` 的地方。


`note:` 這一行告訴我們，可以設定 `RUST_BACKTRACE` 環境變數來得到 backtrace。*backtrace* 是一份到達當前執行點之前所有被呼叫函式的列表。Rust 中的 backtrace 和其他語言裡的工作方式一樣：閱讀 backtrace 的關鍵，是從最上面開始往下讀，直到看到你自己寫的檔案。那一處就是問題開始的地方。它上面的那些行，是你的程式碼呼叫過的程式碼；下面的那些行，則是呼叫了你程式碼的程式碼。這些前前後後的行，可能包括 Rust 核心程式碼、標準庫程式碼，或你正在使用的 crate。現在把 `RUST_BACKTRACE` 環境變數設成除 `0` 之外的任意值，來看看 backtrace。示例 9-2 展示了類似下面這樣的輸出：

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">示例 9-2：當設定 `RUST_BACKTRACE` 環境變數時 `panic!` 呼叫所生成的 backtrace 資訊</span>

這裡的輸出很多！你實際看到的內容可能會因為作業系統和 Rust 版本不同而有所區別。要獲得帶有這些資訊的 backtrace，必須啟用除錯符號（debug symbols）。當像這裡這樣，不帶 `--release` 引數執行 `cargo build` 或 `cargo run` 時，除錯符號預設就是啟用的。

示例 9-2 的輸出中，backtrace 的第 6 行指向了我們專案中造成問題的行：*src/main.rs* 的第 4 行。如果你不希望程式 panic，就應當從第一個提到我們自己編寫的檔案的那一行開始調查。在示例 9-1 中，我們故意編寫了會導致 panic 的程式碼，修復這個 panic 的方法就是不要嘗試在一個只包含三個項的 vector 中請求索引是 100 的元素。當將來你的程式碼出現了 panic，你需要搞清楚在這特定的場景下程式碼中執行了什麼操作和什麼值導致了 panic，以及應當如何處理才能避免該問題。

本章後面的小節 [“要不要 panic!”][to-panic-or-not-to-panic] 會再次回到 `panic!` 並講解何時應該、何時不應該使用 `panic!` 來處理錯誤情況。接下來，我們來看看如何使用 `Result` 來從錯誤中恢復。

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.html#要不要-panic
