## 使用執行緒同時執行程式碼

[ch16-01-threads.md](https://github.com/rust-lang/book/blob/9bd32402af8d3103302650895ec9d129ebfa47e1/src/ch16-01-threads.md)

在大多數當前的作業系統中，一個執行中的程式程式碼會在一個**程序**（*process*）中執行，而作業系統會同時管理多個程序。在一個程式內部，也可以存在彼此獨立、同時執行的多個部分。執行這些獨立部分的功能被稱為**執行緒**（*threads*）。例如，一個 web 伺服器可以擁有多個執行緒，以便同時響應多個請求。

將程式中的計算拆分進多個執行緒可以改善效能，因為程式可以同時進行多個任務，不過這也會增加複雜性。因為執行緒是同時執行的，所以無法預先保證不同執行緒中的程式碼的執行順序。這會導致諸如此類的問題：

- 競態條件（Race conditions），多個執行緒以不一致的順序訪問資料或資源
- 死鎖（Deadlocks），兩個執行緒相互等待對方，這會阻止兩者繼續執行
- 只會發生在特定情況且難以穩定重現和修復的 bug

Rust 嘗試減輕使用執行緒的負面影響。不過在多執行緒上下文中程式設計仍需格外小心，同時其所要求的程式碼結構也不同於運行於單執行緒的程式。

程式語言實現執行緒的方式各不相同，許多作業系統都提供了可供程式語言呼叫、用來建立新執行緒的 API。Rust 標準庫使用的是執行緒實現的 *1:1* 模型，也就是程式中的每一個語言級執行緒都對應一個作業系統執行緒。也有一些 crate 實現了其他執行緒模型，它們相對於 1:1 模型有著不同的取捨。（我們將在下一章看到的 Rust async 系統，也提供了另一種併發方式。）

### 使用 `spawn` 建立新執行緒

為了建立一個新執行緒，需要呼叫 `thread::spawn` 函式並傳遞一個閉包（第十三章學習了閉包），並在其中包含希望在新執行緒執行的程式碼。示例 16-1 中的例子在主執行緒列印了一些文字而另一些文字則由新執行緒列印：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

<span class="caption">示例 16-1: 建立一個列印某些內容的新執行緒同時主執行緒列印其它內容</span>

注意當 Rust 程式的主執行緒結束時，所有新執行緒也會結束，而不管其是否執行完畢。這個程式的輸出可能每次都略有不同，不過它大體上看起來像這樣：

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep` 呼叫強制執行緒停止執行一小段時間，這會允許其他不同的執行緒執行。這些執行緒可能會輪流執行，不過並不保證如此：這依賴作業系統如何排程執行緒。在這裡，主執行緒首先列印，即便新建立執行緒的列印語句位於程式的開頭，甚至即便我們告訴新建的執行緒列印直到 `i` 等於 9，它在主執行緒結束之前也只打印到了 5。

如果執行程式碼只看到了主執行緒的輸出，或沒有出現重疊列印的現象，嘗試增大區間 (變數 `i` 的範圍) 來增加作業系統切換執行緒的機會。

### 等待所有執行緒結束

由於主執行緒結束，示例 16-1 中的程式碼大部分時候不光會提早結束新建執行緒，因為無法保證執行緒執行的順序，我們甚至不能實際保證新建執行緒會被執行！

可以透過將 `thread::spawn` 的返回值儲存在變數中來修復新建執行緒部分沒有執行或者完全沒有執行的問題。`thread::spawn` 的返回值型別是 `JoinHandle<T>`。`JoinHandle<T>` 是一個擁有所有權的值，當對其呼叫 `join` 方法時，它會等待其執行緒結束。示例 16-2 展示瞭如何使用示例 16-1 中建立的執行緒的 `JoinHandle<T>` 並呼叫 `join` 來確保新建執行緒在 `main` 退出前結束執行。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

<span class="caption">示例 16-2: 從 `thread::spawn` 儲存一個 `JoinHandle<T>` 以確保該執行緒能夠執行至結束</span>

對控制代碼呼叫 `join` 會阻塞當前正在執行的執行緒，直到該控制代碼所代表的執行緒結束。**阻塞**（*blocking*）一個執行緒，意味著這個執行緒會被阻止繼續工作或退出。因為我們把 `join` 呼叫放在主執行緒的 `for` 迴圈之後，執行示例 16-2 時應該會得到類似如下的輸出：

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

這兩個執行緒仍然會交替執行，不過主執行緒會由於 `handle.join()` 呼叫而不會結束直到新建執行緒執行完畢。

不過讓我們看看將 `handle.join()` 移動到 `main` 中 `for` 迴圈之前會發生什麼，如下：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

主執行緒會等待直到新建執行緒執行完畢之後才開始執行 `for` 迴圈，所以輸出將不會交替出現，如下所示：

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

諸如將 `join` 放置於何處這樣的小細節，會影響執行緒是否同時執行。

### 將 `move` 閉包與執行緒一同使用

`move` 關鍵字經常用於傳遞給 `thread::spawn` 的閉包，因為閉包會獲取從環境中取得的值的所有權，因此會將這些值的所有權從一個執行緒傳送到另一個執行緒。在第十三章[“捕獲環境”][capture]部分討論了閉包上下文中的 `move`。現在我們會更專注於 `move` 和 `thread::spawn` 之間的互動。

注意示例 16-1 中傳遞給 `thread::spawn` 的閉包並沒有任何引數：並沒有在新建執行緒程式碼中使用任何主執行緒的資料。為了在新建執行緒中使用來自於主執行緒的資料，需要新建執行緒的閉包獲取它需要的值。示例 16-3 展示了一個嘗試在主執行緒中建立一個 vector 並用於新建執行緒的例子，不過這麼寫還不能工作，如下所示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

<span class="caption">示例 16-3: 嘗試在另一個執行緒使用主執行緒建立的 vector</span>

閉包使用了 `v`，所以閉包會捕獲 `v` 並使其成為閉包環境的一部分。因為 `thread::spawn` 在一個新執行緒中執行這個閉包，所以可以在新執行緒中訪問 `v`。然而當編譯這個例子時，會得到如下錯誤：

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust 會**推斷**如何捕獲 `v`，因為 `println!` 只需要 `v` 的引用，閉包嘗試借用 `v`。然而這有一個問題：Rust 不知道這個新建執行緒會執行多久，所以無法知曉對 `v` 的引用是否一直有效。

示例 16-4 展示了一個 `v` 的引用很有可能不再有效的場景：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

<span class="caption">示例 16-4: 一個具有閉包的執行緒，嘗試使用一個在主執行緒中被回收的引用 `v`</span>

如果 Rust 允許這段程式碼執行，則新建執行緒可能會立刻被轉移到後臺並完全沒有機會執行。新建執行緒內部有一個 `v` 的引用，不過主執行緒立刻就使用第十五章討論的 `drop` 丟棄了 `v`。接著當新建執行緒開始執行，`v` 已不再有效，所以其引用也是無效的。噢，這太糟了！

為了修復示例 16-3 的編譯錯誤，我們可以聽取錯誤資訊的建議：

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

透過在閉包之前增加 `move` 關鍵字，我們強制閉包獲取其使用的值的所有權，而不是任由 Rust 推斷它應該借用值。示例 16-5 中展示的對示例 16-3 程式碼的修改，就能按預期編譯並執行：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

<span class="caption">示例 16-5: 使用 `move` 關鍵字強制獲取它使用的值的所有權</span>

我們可能希望嘗試同樣的方法來修復示例 16-4 中的程式碼，其主執行緒使用 `move` 閉包呼叫了 `drop`。然而這個修復行不通，因為示例 16-4 所嘗試的操作由於一個不同的原因而不被允許。如果為閉包增加 `move`，將會把 `v` 移動進閉包的環境中，如此將不能在主執行緒中對其呼叫 `drop` 了。我們會得到如下不同的編譯錯誤：

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust 的所有權規則又一次幫助了我們！示例 16-3 中的錯誤是因為 Rust 是保守的並只會為執行緒借用 `v`，這意味著主執行緒理論上可能使新建執行緒的引用無效。透過告訴 Rust 將 `v` 的所有權移動到新建執行緒，我們向 Rust 保證主執行緒不會再使用 `v`。如果對示例 16-4 也做出如此修改，那麼當在主執行緒中使用 `v` 時就會違反所有權規則。`move` 關鍵字覆蓋了 Rust 預設保守的借用，但它不允許我們違反所有權規則。

現在我們已經瞭解了執行緒的概念以及執行緒 API 提供的方法，下面讓我們看看在什麼情況下可以使用執行緒。

[capture]: ch13-01-closures.html#捕獲環境
