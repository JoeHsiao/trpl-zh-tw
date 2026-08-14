## 使用 async 實現併發

[ch17-02-concurrency-with-async.md](https://github.com/rust-lang/book/blob/f78ab89d7545ac17780e6a367055cc089f4cd2ec/src/ch17-02-concurrency-with-async.md)

在這一部分，我們將使用非同步來應對一些與第十六章中透過執行緒解決的相同的併發問題。因為之前我們已經討論了很多關鍵理念了，這一部分我們會專注於執行緒與 future 的區別。

在很多情況下，使用非同步處理併發的 API 與使用執行緒的非常相似。在其它的一些情況，它們則非常不同。即便執行緒與非同步的 API *看起來* 很類似，通常它們有著不同的行為，同時它們幾乎總是有著不同的效能特點。

### 使用 `spawn_task` 建立新任務

第十六章中我們應付的第一個任務是在兩個不同的執行緒中計數。讓我們用非同步來完成相同的任務。`trpl` crate 提供了一個 `spawn_task` 函式，它看起來非常像 `thread::spawn` API，和一個 `sleep` 函式，這是 `thread::sleep` API 的非同步版本。我們可以將它們結合使用，實現與執行緒示例相同的計數功能，如示例 17-6 所示。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

<figcaption>示例 17-6：建立一個新任務，在主任務列印內容的同時列印另一組內容</figcaption>

</figure>

作為起點，我們在 `main` 函式中使用 `trpl::block_on`，這樣頂層函式就可以寫成 async 風格。

> 注意：從這裡開始，本章中的每個示例在 `main` 中都會包含這段幾乎完全一樣的 `trpl::block_on` 包裝程式碼，所以之後我們通常會像省略 `main` 一樣把它省掉。記得在你自己的程式碼裡補上它！

然後我們在這個程式碼塊裡寫了兩個迴圈，每個迴圈中都呼叫了 `trpl::sleep`，在輸出下一條訊息之前等待半秒（500 毫秒）。其中一個迴圈放在 `trpl::spawn_task` 的函式體裡，另一個則放在頂層的 `for` 迴圈中。我們還在 `sleep` 呼叫後加上了 `await`。

這段程式碼的行為和執行緒版實現很像，包括當你親自執行時，終端中的訊息順序可能和這裡不完全一樣：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

這個版本會在主 async 塊中的 `for` 迴圈一結束就停止，因為當 `main` 函式結束時，由 `spawn_task` 生成的任務也會被關閉。如果你想讓它一直執行到任務自身完成，就需要使用 join handle 來等待第一個任務結束。線上程的版本中，我們使用 `join` 方法“阻塞”等待執行緒執行結束。在示例 17-7 中，我們可以使用 `await` 做同樣的事，因為任務控制代碼本身就是一個 future。它的 `Output` 型別是 `Result`，所以在等待之後還要再 `unwrap` 一次。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

<figcaption>示例 17-7：在 join 控制代碼上使用 `await`，讓任務執行到完成</figcaption>

</figure>

更新後的版本會一直執行到*兩個*迴圈都完成為止：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

到目前為止，看起來 async 和執行緒只是用不同語法實現了相似效果：在 join handle 上使用 `await`，而不是呼叫 `join`；同時對 `sleep` 呼叫也使用 `await`。

更大的不同在於，我們根本不需要再建立另一個作業系統執行緒來做這件事。實際上，這裡甚至連任務都不一定要建立。因為 async 程式碼塊會被編譯成匿名 future，我們可以把每個迴圈都放進一個 async 程式碼塊裡，然後讓執行時使用 `trpl::join` 讓它們都執行到完成。

在第十六章[“等待所有執行緒完成”][join-handles]一節中，我們展示瞭如何對 `std::thread::spawn` 返回的 `JoinHandle` 呼叫 `join` 方法。`trpl::join` 與之類似，不過它面向的是 future。當你把兩個 future 傳給它時，它會生成一個新的 future；等到*兩個*傳入的 future 都完成時，這個新 future 的輸出就是一個包含它們各自輸出值的元組。因此，在示例 17-8 中，我們用 `trpl::join` 來等待 `fut1` 和 `fut2` 完成。我們*不會*分別等待 `fut1` 和 `fut2`，而是等待 `trpl::join` 生成的那個新 future。這裡我們忽略它的輸出，因為那不過是一個包含兩個 unit 值的元組。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

<figcaption>示例 17-8：使用 `trpl::join` 等待兩個匿名 future</figcaption>

</figure>

執行後，我們會看到兩個 future 都執行到了結束：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

現在你會發現，每次執行時順序都完全一樣，這和執行緒版本以及示例 17-7 中使用 `trpl::spawn_task` 的情況非常不同。這是因為 `trpl::join` 是 *fair* 的，也就是它會以同樣的頻率檢查每一個 future，在它們之間交替進行；只要另一個 future 已經就緒，它就不會讓其中一個一路領先。線上程模型下，由作業系統決定先檢查哪個執行緒、讓它執行多久。對於 async Rust，則由執行時決定先檢查哪個任務。（在實踐中，細節會複雜得多，因為非同步執行時可能會在底層藉助作業系統執行緒來實現併發，因此要保證公平性，對執行時來說可能意味著更多工作，但這仍然是可能做到的。）執行時並不一定會為任何給定操作都保證公平性，而且它們通常會提供不同的 API，讓你自行決定是否需要公平性。

嘗試這些不同的 await future 的變體來觀察它們的效果：

- 去掉一個或者兩個迴圈外的非同步程式碼塊。
- 在定義兩個非同步程式碼塊後立刻 await 它們。
- 只將第一個迴圈封裝進非同步程式碼塊，並在第二個迴圈體之後 await 作為結果的 future。

作為額外的挑戰，看看你能否在執行程式碼 *之前* 想出每個情況下的輸出！

<!-- Old headings. Do not remove or links may break. -->

<a id="message-passing"></a>
<a id="counting-up-on-two-tasks-using-message-passing"></a>

### 透過訊息傳遞在兩個任務之間傳送資料

在 future 之間共享資料的方式也會讓你感到熟悉：我們再次使用訊息傳遞，只不過這次使用的是非同步版本的型別和函式。為了展示基於執行緒的併發和基於 future 的併發之間的一些關鍵差別，我們會和第十六章[“透過訊息傳遞線上程間傳送資料”][message-passing-threads]一節稍微走一條不一樣的路線。在示例 17-9 中，我們先只使用一個 async 程式碼塊，而*不*像之前那樣顯式地建立一個獨立任務。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

<figcaption>示例 17-9：建立一個非同步通道（async channel）並賦值其兩端為 `tx` 和 `rx`</figcaption>

</figure>

這裡我們使用了 `trpl::channel`，一個第十六章用於執行緒的多生產者、單消費者通道 API 的非同步版本。非同步版本的 API 與基於執行緒的版本只有一點微小的區別：它使用一個可變的而不是不可變的 `rx`，並且它的 `recv` 方法產生一個需要 await 的 future 而不是直接返回值。現在我們可以傳送端向接收端傳送訊息了。注意我們無需產生一個獨立的執行緒或者任務；只需等待（await） `rx.recv` 呼叫。

`std::mpsc::channel` 中的同步 `Receiver::recv` 方法阻塞執行直到它接收一個訊息。`trpl::Receiver::recv` 則不會阻塞，因為它是非同步的。不同於阻塞，它將控制權交還給執行時，直到接收到一個訊息或者通道的傳送端關閉。相比之下，我們不用 await `send`，因為它不會阻塞。也無需阻塞，因為通道的傳送端的數量是沒有限制的。

> 注意：因為這些 async 程式碼都執行在傳給 `trpl::block_on` 的 async 程式碼塊裡，所以塊中的所有內容都可以避免阻塞。不過，塊*外部*的程式碼則會阻塞，直到 `block_on` 返回為止。這正是 `trpl::block_on` 的意義所在：它讓你可以*選擇*在哪一處對一組 async 程式碼進行阻塞，從而也就決定了在什麼地方切換同步和非同步程式碼。

請注意這個示例中的兩個地方：首先，訊息立刻就會到達！其次，雖然我們使用了 future，但是這裡還沒有併發。示例中的所有事情都是順序發生的，就像沒涉及到 future 時一樣。

讓我們透過傳送一系列訊息並在之間休眠來解決第一個問題，如示例 17-10 所示：

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

<figcaption>示例 17-10：透過非同步通道傳送和接收多個訊息並在每個訊息之間透過 `await` 休眠</figcaption>

</figure>

除了傳送訊息之外，我們還需要接收它們。在這個例子中我們可以手動接收，就是呼叫四次 `rx.recv().await`，因為我們知道進來了多少條訊息。然而，在現實世界中，我們通常會等待 *未知* 數量的訊息。這時我們需要一直等待直到可以確認沒有更多訊息了為止。

在示例 16-10 中，我們使用 `for` 迴圈處理從同步通道接收到的所有條目。不過，Rust 目前還沒有辦法對*非同步產生的*一系列條目使用 `for` 迴圈。因此，我們需要一種前面還沒見過的迴圈：`while let` 條件迴圈。它正是我們在第六章[“使用 `if let` 和 `let...else` 實現簡潔控制流”][if-let]中見過的 `if let` 結構的迴圈版本。只要它指定的模式還在持續匹配，迴圈就會繼續執行。

`rx.recv` 呼叫產生一個 `Future`，我們會 await 它。執行時會暫停 `Future` 直到它就緒。一旦訊息到達，future 會解析為 `Some(message)`，每次訊息到達時都會如此。當通道關閉時，不管是否有 *任何* 訊息到達，future 都會解析為 `None` 來表明沒有更多的值了，我們也就應該停止輪詢，也就是停止等待。

`while let` 迴圈將上述邏輯整合在一起。如果 `rx.recv().await` 呼叫的結果是 `Some(message)`，我們會得到訊息並可以在迴圈體中使用它，就像使用 `if let` 一樣。如果結果是 `None`，則迴圈停止。每次迴圈執行完畢，它會再次觸發 await point，如此執行時會再次暫停直到另一條訊息到達。

現在程式碼可以成功傳送和接收所有的訊息了。不幸的是，這裡還有一些問題。首先，訊息並不是按照半秒的間隔到達的。它們在程式啟動後兩秒（2000 毫秒）後立刻一起到達。其次，程式永遠也不會退出！相反它會永遠等待新訊息。你會需要使用 <span class="keystroke">ctrl-c</span> 來關閉它。

#### 一個 async 程式碼塊中的程式碼會線性執行

先來看為什麼這些訊息會在完整延遲之後一起到達，而不是在每次延遲之後逐條到達。在一個給定的 async 程式碼塊裡，程式碼中 `await` 出現的順序，也就是程式執行時它們執行的順序。

示例 17-10 中只有一個 async 程式碼塊，所以裡面的一切都按線性順序執行。這裡依然沒有併發。所有 `tx.send` 呼叫，連同 `trpl::sleep` 呼叫及其相應的 await 點，都會先全部依次發生。只有在那之後，`while let` 迴圈才有機會開始執行 `recv` 呼叫上的那些 await 點。

為了得到我們真正想要的行為，也就是在每條訊息之間都出現休眠間隔，我們需要把 `tx` 和 `rx` 的操作分別放進各自的 async 程式碼塊中，如示例 17-11 所示。這樣執行時就可以像示例 17-8 那樣，使用 `trpl::join` 分別執行它們。我們再次等待的是 `trpl::join` 呼叫的結果，而不是分別等待每個 future。要是依次等待它們，我們就又回到了順序執行的流程，這正是我們*不*想要的。

<!-- We cannot test this one because it never stops! -->

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

<figcaption>示例 17-11：將 `send` 和 `recv` 分隔到其各自的 `async` 程式碼塊中並 await 這些程式碼塊的 future</figcaption>

</figure>

使用示例 17-11 中更新後的程式碼後，訊息就會以 500 毫秒的間隔輸出，而不是在 2 秒之後一次性全部打印出來。

#### 將所有權移入 async 程式碼塊

但是程式仍然永遠也不會退出，這是由於 `while let` 迴圈與 `trpl::join` 的互動方式所致：

- `trpl::join` 返回的 future 只會完成一次，即傳遞的 *兩個* future 都完成的時候。
- `tx_fut` future 會在傳送完 `vals` 中最後一條訊息後，再完成最後一次休眠之後結束。
- `rx_fut` future 則要等到 `while let` 迴圈結束時才會結束。
- 只有當等待 `rx.recv` 的結果變成 `None` 時，`while let` 迴圈才會結束。
- 只有在通道另一端關閉後，等待 `rx.recv` 才會返回 `None`。
- 只有在我們呼叫 `rx.close`，或者傳送端 `tx` 被 drop 時，通道才會關閉。
- 我們根本沒有呼叫 `rx.close`，而 `tx` 也要等到傳給 `trpl::block_on` 的最外層 async 程式碼塊結束後才會被 drop。
- 但那個最外層 async 程式碼塊又必須等 `trpl::join` 完成才能結束，於是我們就又回到了這個列表的起點。

目前，傳送訊息的那個 async 程式碼塊只是*借用*了 `tx`，因為傳送訊息並不需要取得它的所有權。但如果我們能把 `tx` *move* 進那個 async 程式碼塊裡，那麼一旦該程式碼塊結束，`tx` 就會被 drop。在第十三章[“捕獲引用或移動所有權”][capture-or-move]中，你學過如何在閉包上使用 `move` 關鍵字；而正如第十六章[“將 `move` 閉包與執行緒一同使用”][move-threads]一節提到的那樣，線上程場景下我們也經常需要把資料 move 進閉包。相同的基本原理也適用於 async 程式碼塊，因此 `move` 關鍵字同樣可以和 async 程式碼塊一起使用。

在示例 17-12 中，我們把傳送訊息用的程式碼塊從 `async` 改為 `async move`。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

<figcaption>示例 17-12：對示例 17-11 的修改版本，它會在完成後正確關閉</figcaption>

</figure>

執行*這個*版本的程式碼後，它就會在最後一條訊息傳送並接收完之後正常退出。接下來，我們來看看，如果要從多個 future 傳送資料，又需要做哪些變化。

#### 使用 `join!` 宏合併多個 future

這個非同步通道同樣也是多生產者通道，因此如果我們希望從多個 future 傳送訊息，就可以對 `tx` 呼叫 `clone`，如示例 17-13 所示。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

<figcaption>示例 17-13：在 async 程式碼塊中使用多個生產者</figcaption>

</figure>

首先，我們克隆 `tx`，在第一個 async 程式碼塊外創建出 `tx1`。然後像之前處理 `tx` 那樣，把 `tx1` move 進這個程式碼塊裡。隨後，我們再把原始的 `tx` move 進一個*新的* async 程式碼塊，在那裡以稍慢一點的節奏繼續傳送更多訊息。這裡我們把這個新 async 程式碼塊放在接收訊息的 async 程式碼塊後面，不過放在前面也同樣可以。關鍵在於 future 被等待的順序，而不是它們被建立的順序。

兩個負責傳送訊息的 async 程式碼塊都必須寫成 `async move`，這樣當代碼塊結束時，`tx` 和 `tx1` 都會被 drop。否則，我們又會回到一開始那個無限迴圈的問題。

最後，我們從 `trpl::join` 切換為 `trpl::join!` 來處理新增的 future。`join!` 宏可以在 future 數量已知於編譯期的情況下，等待任意數量的 future。本章稍後我們還會討論，如何等待一個數量事先未知的 future 集合。

現在我們就能看到來自兩個傳送 future 的所有訊息了。由於這兩個傳送 future 在傳送後使用了略微不同的延遲，接收到這些訊息的時間間隔也會相應不同：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

我們已經探索瞭如何用訊息傳遞在 future 之間傳送資料、一個 async 程式碼塊中的程式碼如何按順序執行、如何將所有權 move 進 async 程式碼塊，以及如何合併多個 future。接下來，我們來討論一下，為什麼以及如何告訴執行時：它現在可以切換去執行別的任務了。

[thread-spawn]: ch16-01-threads.html#使用-spawn-建立新執行緒
[join-handles]: ch16-01-threads.html#等待所有執行緒結束
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#捕獲引用或移動所有權
[move-threads]: ch16-01-threads.html#將-move-閉包與執行緒一同使用
