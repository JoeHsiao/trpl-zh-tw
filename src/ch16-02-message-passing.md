## 使用訊息傳遞線上程間傳送資料

[ch16-02-message-passing.md](https://github.com/rust-lang/book/blob/9bd32402af8d3103302650895ec9d129ebfa47e1/src/ch16-02-message-passing.md)

一個日益流行的確保安全併發的方式是**訊息傳遞**（_message passing_），這裡執行緒或 actor 通過傳送包含資料的訊息來相互溝通。這個思想來源於 [Go 程式語言文件](https://golang.org/doc/effective_go.html#concurrency) 中的口號：“不要通過共享記憶體來通訊；而要通過通訊來共享記憶體。”（“Do not communicate by sharing memory; instead, share memory by communicating.”）

為了實現訊息傳遞併發，Rust 標準庫提供了一個**通道**（_channel_）實現。通道是一個通用程式設計概念，表示資料從一個執行緒傳送到另一個執行緒。

你可以將程式設計中的通道想像為一個水流的渠道，比如河流或小溪。如果你將諸如橡皮鴨之類的東西放入其中，它們會順流而下到達下游。

通道有兩個組成部分：一個傳送端（transmitter）和一個接收端（receiver）。傳送端位於上游位置，在這裡可以將橡皮鴨放入河中，接收端則位於下游，橡皮鴨最終會漂流至此。程式碼中的一部分呼叫傳送端的方法以及希望傳送的資料，另一部分則檢查接收端收到的訊息。當傳送端或接收端任一被丟棄時可以認為通道被**關閉**（_closed_）了。

這裡，我們將開發一個程式，它會在一個執行緒生成值向通道傳送，而在另一個執行緒會接收值並打印出來。這裡會通過通道線上程間傳送簡單值來演示這個功能。一旦你熟悉了這項技術，你就可以將通道用於任何相互通訊的任何執行緒，例如一個聊天系統，或利用很多執行緒進行分散式計算並將部分計算結果傳送給一個執行緒進行聚合。

首先，在示例 16-6 中，建立了一個通道但沒有做任何事。注意這還不能編譯，因為 Rust 不知道我們想要在通道中傳送什麼型別：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">示例 16-6: 建立一個通道，並將其兩端賦值給 `tx` 和 `rx`</span>

這裡使用 `mpsc::channel` 函式建立一個新的通道；`mpsc` 是 **多生產者，單消費者**（_multiple producer, single consumer_）的縮寫。簡而言之，Rust 標準庫實現通道的方式意味著一個通道可以有多個產生值的 **傳送端**（_sending_），但只能有一個消費這些值的**接收端**（_receiving_）。想像一下多條小河小溪最終匯聚成大河：所有通過這些小河發出的東西最後都會來到下游的大河。目前我們以單個生產者開始，但是當示例可以工作後會增加多個生產者。

`mpsc::channel` 函式返回一個元組，其中第一個元素是傳送端，第二個元素是接收端。`tx` 和 `rx` 這兩個縮寫在許多領域裡傳統上分別表示 **傳送端**（*transmitter*）和 **接收端**（*receiver*），因此我們就用它們來給這兩端命名。這裡我們使用了帶模式的 `let` 語句來解構這個元組；第十九章會討論在 `let` 語句中使用模式以及解構。現在只要知道，這樣使用 `let` 是一種從 `mpsc::channel` 返回的元組中方便地取出各個部分的方法即可。

讓我們將傳送端移動到一個新建執行緒中併發送一個字串，這樣新建執行緒就可以和主執行緒通訊了，如示例 16-7 所示。這類似於在河的上游扔下一隻橡皮鴨或從一個執行緒向另一個執行緒傳送聊天資訊：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<span class="caption">示例 16-7: 將 `tx` 移動到一個新建的執行緒中併發送 `"hi"`</span>

這裡再次使用 `thread::spawn` 來建立一個新執行緒並使用 `move` 將 `tx` 移動到閉包中這樣新建執行緒就擁有 `tx` 了。新建執行緒需要擁有通道的傳送端以便能向通道傳送訊息。通道的傳送端有一個 `send` 方法用來獲取需要放入通道的值。`send` 方法返回一個 `Result<T, E>` 型別，所以如果接收端已經被丟棄了，將沒有傳送值的目標，所以傳送操作會返回錯誤。在這個例子中，出錯的時候呼叫 `unwrap` 產生 panic。不過在一個真實應用中，需要合理地處理它：回到第九章複習正確處理錯誤的策略。

在示例 16-8 中，我們在主執行緒中從通道的接收端獲取值。這類似於在河的下游撈起橡皮鴨或接收聊天資訊：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<span class="caption">示例 16-8: 在主執行緒中接收並列印內容 `"hi"`</span>

通道的接收端有兩個有用的方法：`recv` 和 `try_recv`。這裡，我們使用了 `recv`，它是 _receive_ 的縮寫，這會阻塞主執行緒執行直到從通道中接收一個值。一旦傳送了一個值，`recv` 會在一個 `Result<T, E>` 中返回它。當通道傳送端關閉，`recv` 會返回一個錯誤表明不會再有新的值到來了。

`try_recv` 不會阻塞，相反它立刻返回一個 `Result<T, E>`：`Ok` 值包含可用的資訊，而 `Err` 值代表此時沒有任何訊息。如果執行緒在等待訊息過程中還有其他工作時使用 `try_recv` 很有用：可以編寫一個迴圈來頻繁呼叫 `try_recv`，在有可用訊息時進行處理，其餘時候則處理一會其他工作直到再次檢查。

出於簡單的考慮，這個例子使用了 `recv`；主執行緒中除了等待訊息之外沒有任何其他工作，所以阻塞主執行緒是合適的。

執行示例 16-8 中的程式碼時，我們將會看到主執行緒打印出這個值：

```text
Got: hi
```

完美！

### 通過通道轉移所有權

所有權規則在訊息傳遞中扮演了重要角色，其有助於我們編寫安全的併發程式碼。防止併發程式設計中的錯誤是在 Rust 程式中考慮所有權的一大優勢。現在讓我們做一個實驗來看看通道與所有權如何一同協作以避免產生問題：我們將嘗試在新建執行緒中的通道中傳送完 `val` 值**之後**再使用它。嘗試編譯示例 16-9 中的程式碼並看看為何這是不允許的：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<span class="caption">示例 16-9: 在我們已經發送到通道中後，嘗試使用 `val` 引用</span>

這裡嘗試在通過 `tx.send` 傳送 `val` 到通道中之後將其打印出來。允許這麼做是一個壞主意：一旦將值傳送到另一個執行緒後，那個執行緒可能會在我們再次使用它之前就將其修改或者丟棄。其他執行緒對值可能的修改會由於不一致或不存在的資料而導致錯誤或意外的結果。然而，嘗試編譯示例 16-9 的程式碼時，Rust 會給出一個錯誤：

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

我們的併發錯誤會造成一個編譯時錯誤。`send` 函式獲取其引數的所有權並移動這個值歸接收端所有。這可以防止在傳送後意外地再次使用這個值；所有權系統檢查一切是否合乎規則。

### 傳送多個值

示例 16-8 中的程式碼可以編譯和執行，不過它並沒有明確的告訴我們兩個獨立的執行緒通過通道相互通訊。示例 16-10 則有一些改進會證明示例 16-8 中的程式碼是併發執行的：新建執行緒現在會發送多個訊息並在每個訊息之間暫停一秒鐘。

<span class="filename">檔名：src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<span class="caption">示例 16-10: 傳送多個訊息，並在每次傳送後暫停一段時間</span>

這一次，在新建執行緒中有一個字串 vector 希望傳送到主執行緒。我們遍歷它們，單獨傳送每一個字串並通過一個 `Duration` 值呼叫 `thread::sleep` 函式來暫停一秒。

在主執行緒中，不再顯式呼叫 `recv` 函式：而是將 `rx` 當作一個迭代器。對於每一個接收到的值，我們將其打印出來。當通道被關閉時，迭代器也將結束。

當執行示例 16-10 中的程式碼時，將看到如下輸出，每一行都會暫停一秒：

```text
Got: hi
Got: from
Got: the
Got: thread
```

因為主執行緒中的 `for` 迴圈裡並沒有任何暫停或等待的程式碼，所以可以說主執行緒是在等待從新建執行緒中接收值。

### 建立多個生產者

之前我們提到過，`mpsc` 是 *multiple producer, single consumer* 的縮寫。現在就來實際用用這個特性，把示例 16-10 中的程式碼擴充套件為：建立多個執行緒，並都把值傳送給同一個接收端。我們可以通過克隆傳送端來做到這一點，如示例 16-11 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<span class="caption">示例 16-11: 從多個生產者傳送多個訊息</span>

這一次，在建立新執行緒之前，我們對傳送端呼叫了 `clone` 方法。這會給我們一個可以傳遞給第一個新建執行緒的傳送端控制代碼。我們會將原始的通道傳送端傳遞給第二個新建執行緒。這樣就會有兩個執行緒，每個執行緒將向通道的接收端傳送不同的訊息。

執行程式碼時，輸出應該看起來類似如下：

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

雖然你可能會看到這些值以不同的順序出現；這依賴於你的系統。這也就是併發既有趣又困難的原因。如果通過 `thread::sleep` 做實驗，在不同的執行緒中提供不同的值，就會發現它們的執行更加不確定，且每次都會產生不同的輸出。

現在我們見識過了通道如何工作，再看看另一種不同的併發方式吧。
