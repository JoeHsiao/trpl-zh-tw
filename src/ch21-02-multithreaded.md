## 將單執行緒 server 變為多執行緒 server

[ch21-02-multithreaded.md](https://github.com/rust-lang/book/blob/8aa0d003e6499d733d639de32d70f590efa48657/src/ch21-02-multithreaded.md)

目前服務端會依次處理每一個請求，意味著它在完成第一個連線的處理之前不會處理第二個連線。如果服務端正接收越來越多的請求，這類序列操作會使效能越來越差。如果一個請求花費很長時間來處理，隨後而來的請求則不得不等待這個長請求結束，即便這些新請求可以很快就處理完。我們需要修復這種情況，不過首先讓我們實際嘗試一下這個問題。

### 模擬慢請求

讓我們看看一個慢請求如何影響當前服務端實現中的其他請求。示例 21-10 透過模擬慢響應實現了 */sleep* 請求處理，它會使服務端在響應之前休眠五秒。

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

<span class="caption">示例 21-10: 透過休眠五秒來模擬慢請求</span>

從 `if` 切換到 `match` 後現在有三個分支了。我們需要顯式地匹配一個 slice 的 `request_line` 以模式匹配字串字面值。`match` 不會像相等方法那樣自動引用和解引用。

第一個分支與示例 21-9 中的 `if` 程式碼塊相同。第二個分支匹配一個 */sleep* 請求。當接收到這個請求時，server 在渲染成功 HTML 頁面之前會先休眠五秒。第三個分支與示例 21-9 中的 `else` 程式碼塊相同。

現在就可以真切的看出我們的服務端有多麼的原始：真實的庫將會以更簡潔的方式處理多個請求的識別！

使用 `cargo run` 啟動服務端，並接著開啟兩個瀏覽器視窗：一個請求 *http://127.0.0.1:7878/* 而另一個請求 *http://127.0.0.1:7878/sleep* 。如果像之前一樣多次請求 */*，會發現響應的比較快速。不過如果請求 */sleep* 之後再請求 */*，就會看到 */* 會等待直到 `sleep` 休眠完五秒之後才響應。

有多種技術可以用來避免所有請求都排在慢請求之後，包括我們在第十七章中所使用的非同步；我們將要實現的一個便是執行緒池。

### 使用執行緒池改善吞吐量

**執行緒池**（*thread pool*）是一組預先分配的等待或準備處理任務的執行緒。當程式收到一個新任務，執行緒池中的一個執行緒會被分配該任務，並負責處理它。其餘執行緒在該執行緒處理任務的同時可以處理任何其他接收到的任務。當第一個執行緒處理完任務時，它會返回空閒執行緒池中等待處理新任務。執行緒池允許我們併發處理連線，提高服務端的吞吐量。

我們會將池中執行緒限制為較少的數量，以防拒絕服務（Denial of Service，DoS）攻擊；如果程式為每一個接收的請求都新建一個執行緒，某人向服務端發起千萬級的請求時會耗盡伺服器的資源並導致請求處理陷入停滯。

不同於分配無限的執行緒，執行緒池中將有固定數量的等待執行緒。當新進請求時，將請求傳送到執行緒池中做處理。執行緒池會維護一個接收請求的佇列。每一個執行緒會從佇列中取出一個請求，處理請求，接著向佇列獲取下一個請求。透過這種設計，則可以併發處理 *`N`* 個請求，其中 *`N`* 為執行緒數。如果每一個執行緒都在響應慢請求，之後的請求仍然會阻塞佇列，不過相比之前已經增加了能處理的慢請求的數量。

這個設計僅僅是多種改善 web 服務端吞吐量的方法之一。其他可供探索的方法有 fork/join 模型（fork/join model）、單執行緒非同步 I/O 模型（single-threaded async I/O model）或者多執行緒非同步 I/O 模型（multi-threaded async I/O model）。如果你對這個主題感興趣，則可以閱讀更多關於其他解決方案的內容並嘗試實現它們；對於一個像 Rust 這樣的底層語言，所有這些方法都是可行的。

在開始之前，讓我們討論一下執行緒池應用看起來如何。當嘗試設計程式碼時，首先編寫客戶端介面（client interface）有助於指導程式碼設計。以期望的呼叫方式來構建 API 程式碼的結構，接著在這個結構之內實現功能，而不是先實現功能再設計公有 API。

類似於第十二章專案中使用的測試驅動開發。這裡將要使用編譯器驅動開發（compiler-driven development）。我們將編寫呼叫所期望的函式的程式碼，接著觀察編譯器錯誤告訴我們接下來需要修改什麼使得程式碼可以工作。不過在開始之前，我們將探索不會作為起點使用的技術。

#### 為每一個請求分配執行緒

首先，讓我們探索一下如果為每一個連線都建立一個執行緒的程式碼看起來如何。這並不是最終方案，因為正如之前講到的它會潛在的分配無限的執行緒，不過這是一個可用的多執行緒服務端的起點。接著我們會增加執行緒池作為改進，這樣比較兩個方案將會更容易。示例 21-11 展示了 `main` 的改變，它在 `for` 迴圈中為每一個流分配了一個新執行緒進行處理：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

<span class="caption">示例 21-11: 為每一個流新建一個執行緒</span>

正如第十六章講到的，`thread::spawn` 會建立一個新執行緒並在其中執行閉包中的程式碼。如果執行這段程式碼並在在瀏覽器中載入 */sleep*，接著在另兩個瀏覽器標籤頁中載入 */*，確實會發現 */* 請求不必等待 */sleep* 結束。不過正如之前提到的，這最終會使系統崩潰因為我們會無限制地建立新執行緒。

你可能也會回想起第十七章中正是這一類情況才是 async 和 await 真正閃光的地方！在我們用執行緒池構建專案時請記住並思考這與非同步有什麼不同或相同的地方。

#### 建立有限數量的執行緒

我們期望執行緒池以類似且熟悉的方式工作，以便從執行緒切換到執行緒池並不會對使用該 API 的程式碼做出大幅修改。示例 21-12 展示我們希望用來替換 `thread::spawn` 的 `ThreadPool` 結構體的假想介面：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

<span class="caption">示例 21-12: 假想的 `ThreadPool` 介面</span>

我們使用 `ThreadPool::new` 建立一個具有可配置執行緒數的新執行緒池，在這裡是四。這樣在 `for` 迴圈中，`pool.execute` 有著類似 `thread::spawn` 的介面，它獲取一個執行緒池運行於每一個流的閉包。我們需要實現 `pool.execute`，使其能夠接收閉包並將其傳遞給執行緒池中的執行緒執行。這段程式碼還不能編譯，但我們可以嘗試讓編譯器指導我們如何修復它。

#### 採用編譯器驅動開發構建 `ThreadPool`

繼續並對示例 21-12 中的 *src/main.rs* 做出修改，並利用來自 `cargo check` 的編譯器錯誤來驅動開發。下面是我們得到的第一個錯誤：

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

太好了！這個錯誤告訴我們需要一個 `ThreadPool` 型別或模組，所以我們現在就來構建一個。`ThreadPool` 的實現會與 web 服務端的特定工作相獨立。所以讓我們從 `hello` crate 切換到存放 `ThreadPool` 實現的新庫 crate。切換為庫 crate 之後，我們就可以在任何工作中使用這個單獨的執行緒池庫，而不僅僅是處理網路請求。

建立 *src/lib.rs* 檔案，它包含了目前可用的最簡單的 `ThreadPool` 定義：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

接著編輯 *main.rs* 檔案透過在 *src/main.rs* 的開頭增加如下程式碼將 `ThreadPool` 從庫 crate 引入作用域：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

這仍然不能工作，再次嘗試執行來得到下一個需要解決的錯誤：

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

此錯誤表明下一步是為 `ThreadPool` 建立一個叫做 `new` 的關聯函式。我們還知道 `new` 需要有一個引數可以接受 `4`，而且 `new` 應該返回 `ThreadPool` 例項。讓我們實現擁有此特徵的最小化 `new` 函式：

<span class="filename">資料夾：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

這裡選擇 `usize` 作為 `size` 引數的型別，因為我們知道執行緒數為負沒有意義。我們還知道將使用 `4` 作為執行緒集合的元素數量，這也就是使用 `usize` 型別的原因，如第三章 [“整型”][integer-types] 部分所講。

再次編譯檢查這段程式碼：

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

這裡發生錯誤是因為並沒有 `ThreadPool` 上的 `execute` 方法。回憶 [“建立有限數量的執行緒”](#建立有限數量的執行緒) 部分，我們決定執行緒池應該有與 `thread::spawn` 類似的介面。此外，我們將實現 `execute` 函式，使其接收傳給它的閉包，並將其交給池中的某個空閒執行緒去執行。

我們會在 `ThreadPool` 上定義 `execute` 方法，讓它接收一個閉包作為引數。回憶第十三章的 [“將捕獲的值移出閉包”][moving-out-of-closures] 部分，我們可以用三種不同的 trait 來接收閉包引數：`Fn`、`FnMut` 和 `FnOnce`。我們需要決定這裡該用哪一種。我們知道最終會做出類似標準庫 `thread::spawn` 的實現，所以可以先看看 `thread::spawn` 的簽名對其引數施加了哪些約束。文件給出的簽名如下：

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`F` 是這裡我們關心的引數；`T` 與返回值有關所以我們並不關心。考慮到 `spawn` 使用 `FnOnce` 作為 `F` 的 trait 約束，這可能也是我們需要的，因為最終會將傳遞給 `execute` 的引數傳給 `spawn`。因為處理請求的執行緒只會執行閉包一次，這也進一步確認了 `FnOnce` 是我們需要的 trait，這裡符合 `FnOnce` 中 `Once` 的意思。

`F` 還有 trait 約束 `Send` 和生命週期繫結 `'static`，這對我們的情況也是有意義的：需要 `Send` 來將閉包從一個執行緒轉移到另一個執行緒，而 `'static` 是因為並不知道執行緒會執行多久。讓我們編寫一個使用帶有這些約束的泛型引數 `F` 的 `ThreadPool` 的 `execute` 方法：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

`FnOnce` trait 仍然需要之後的 `()`，因為這裡的 `FnOnce` 代表一個沒有引數也沒有返回值的閉包。正如函式的定義，返回值型別可以從簽名中省略，不過即便沒有引數也需要括號。

這裡再一次增加了 `execute` 方法的最小化實現：它沒有做任何工作，只是嘗試讓程式碼能夠編譯。再次進行檢查：

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

現在就只有警告了！這意味著能夠編譯了！注意如果嘗試 `cargo run` 執行程式並在瀏覽器中發起請求，仍會在瀏覽器中出現在本章開始時那樣的錯誤。這個庫實際上還沒有呼叫傳遞給 `execute` 的閉包！

> 一個你可能聽說過的關於像 Haskell 和 Rust 這樣有嚴格編譯器的語言的說法是 “如果程式碼能夠編譯，它就能工作”。不過這個說法並不是普適的。我們的專案可以編譯，不過它完全沒有做任何工作！如果構建一個真實且功能完整的專案，則需花費大量的時間來開始編寫單元測試來檢查程式碼能否編譯 **並且** 擁有期望的行為。

思考一下：如果這裡要執行的是一個 `future` 而不是閉包會有什麼不同？

#### 在 `new` 中驗證執行緒池的執行緒數量

這裡並沒有對 `new` 和 `execute` 的引數做任何操作。讓我們用期望的行為來實現這些函式。以考慮 `new` 作為開始。之前選擇使用無符號型別作為 `size` 引數的型別，因為執行緒數為負的執行緒池沒有意義。然而，執行緒數為零的執行緒池同樣沒有意義，不過零是一個完全有效的 `usize` 值。讓我們增加在返回 `ThreadPool` 例項之前檢查 `size` 是否大於零的程式碼，並使用 `assert!` 宏在得到零時 panic，如示例 21-13 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

<span class="caption">示例 21-13: 實現 `ThreadPool::new` 在 `size` 為零時 panic</span>

這裡也用文件註釋為 `ThreadPool` 增加了一些文件。注意這裡遵循了良好的文件實踐並增加了一個部分來提示函式會 panic 的情況，正如第十四章所討論的。嘗試執行 `cargo doc --open` 並點選 `ThreadPool` 結構體來檢視生成的 `new` 的文件看起來如何！

相比像這裡使用 `assert!` 宏，也可以讓 `new` 像之前 I/O 專案中示例 12-9 中 `Config::build` 那樣將 `new` 更改為 `build` 並返回一個 `Result`，不過在這裡我們選擇建立一個沒有任何執行緒的執行緒池應該是不可恢復的錯誤。如果你想做的更好，嘗試編寫一個採用如下簽名的名為 `build` 的函式來對比一下 `new` 函式：

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### 分配空間以儲存執行緒

現在我們已經有了一種方法來確保執行緒池中的執行緒數有效，就可以實際建立這些執行緒並在返回結構體之前將它們儲存在 `ThreadPool` 結構體中。不過如何 “儲存” 一個執行緒？讓我們再看看 `thread::spawn` 的簽名：

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` 返回 `JoinHandle<T>`，其中 `T` 是閉包返回的型別。嘗試使用 `JoinHandle` 來看看會發生什麼。在我們的情況中，傳遞給執行緒池的閉包會處理連線並不返回任何值，所以 `T` 將會是單元型別 `()`。

示例 21-14 中的程式碼可以編譯，不過實際上還並沒有建立任何執行緒。我們改變了 `ThreadPool` 的定義來存放一個 `thread::JoinHandle<()>` 的 vector 例項，使用 `size` 容量來初始化，並設定一個 `for` 迴圈來執行建立執行緒的程式碼，並返回包含這些執行緒的 `ThreadPool` 例項：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

<span class="caption">示例 21-14: 為 `ThreadPool` 建立一個 vector 來存放執行緒</span>

這裡將 `std::thread` 引入庫 crate 的作用域，因為使用了 `thread::JoinHandle` 作為 `ThreadPool` 中 vector 元素的型別。

在得到了有效的數量之後，`ThreadPool` 新建一個存放 `size` 個元素的 vector。`with_capacity` 函式與 `Vec::new` 做了同樣的工作，不過有一個重要的區別：它為 vector 預先分配空間。因為已經知道了 vector 中需要 `size` 個元素，預先進行分配比僅僅 `Vec::new` 要稍微有效率一些，因為 `Vec::new` 隨著插入元素而重新改變大小。

如果再次執行 `cargo check`，它應該會成功。

#### 將程式碼從 `ThreadPool` 傳送給執行緒

示例 21-14 的 `for` 迴圈中留下了一個關於建立執行緒的註釋。這裡，我們來看看如何實際建立執行緒。標準庫提供了 `thread::spawn` 作為建立執行緒的方法，`thread::spawn` 期望獲取一些一旦建立執行緒就應該執行的程式碼。然而，我們希望開始執行緒並使其等待稍後傳遞的程式碼。標準庫的執行緒實現並沒有包含這麼做的方法；我們必須手動實現。

我們將要實現的行為是建立執行緒並稍後傳送程式碼，這會在 `ThreadPool` 和執行緒間引入一個新資料型別來管理這種新行為。這個資料結構稱為 *Worker*，這是一個池實現中的常見概念。`Worker` 會獲取需要執行的程式碼，並在該 worker 的執行緒中執行該程式碼。

想象一下在餐館廚房工作的員工：員工等待來自顧客的訂單，他們負責接單並完成它們。

不同於線上程池中儲存一個 `JoinHandle<()>` 例項的 vector，我們會儲存 `Worker` 結構體的例項。每一個 `Worker` 會儲存一個單獨的 `JoinHandle<()>` 例項。接著會在 `Worker` 上實現一個方法，該方法將閉包傳送到已經執行的執行緒中執行。我們還會賦予每個 worker 一個 `id`，這樣就可以在日誌和除錯中區別執行緒池中的不同 `Worker` 的例項。

如下是建立 `ThreadPool` 時會發生的新過程。在透過如下方式設定完 `Worker` 之後，我們會實現向執行緒傳送閉包的程式碼：

1. 定義存放 `id` 和 `JoinHandle<()>` 的 `Worker` 結構體。
2. 修改 `ThreadPool` 存放一個 `Worker` 例項的 vector。
3. 定義 `Worker::new` 函式，它獲取一個 `id` 數字並返回一個帶有 `id` 和用空閉包分配的執行緒的 `Worker` 例項。
4. 在 `ThreadPool::new` 中，使用 `for` 迴圈計數生成 `id`，使用這個 `id` 新建 `Worker`，並儲存進 vector 中。

如果你渴望挑戰，在查示例 21-15 中的程式碼之前嘗試自己實現這些修改。

準備好了嗎？示例 21-15 就是一個做出了上述修改的例子：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

<span class="caption">示例 21-15: 修改 `ThreadPool` 存放 `Worker` 例項而不是直接存放執行緒</span>

這裡將 `ThreadPool` 中欄位名從 `threads` 改為 `workers`，因為它現在儲存 `Worker` 而不是 `JoinHandle<()>`。使用 `for` 迴圈中的計數作為 `Worker::new` 的引數，並將每一個新建的 `Worker` 儲存在叫做 `workers` 的 vector 中。

`Worker` 結構體和其 `new` 函式是私有的，因為外部程式碼（比如 *src/main.rs* 中的 server）並不需要知道關於 `ThreadPool` 中使用 `Worker` 結構體的實現細節。`Worker::new` 函式使用 `id` 引數並存儲了使用一個空閉包建立的 `JoinHandle<()>` 例項。

> 注意：如果作業系統因為沒有足夠的系統資源而無法建立執行緒時，`thread::spawn` 會 panic。這會導致整個 server panic，即使一些執行緒可能建立成功了。出於簡單的考慮，這個行為是可行的，不過在一個生產級別的執行緒池實現中，你可能會希望使用 [`std::thread::Builder`][builder] 和其 [`spawn`][builder-spawn] 方法來返回一個 `Result`。

這段程式碼能夠編譯並用指定給 `ThreadPool::new` 的引數建立儲存了一系列的 `Worker` 例項，不過 **仍然** 沒有處理 `execute` 中得到的閉包。讓我們聊聊接下來怎麼做。

#### 使用通道向執行緒傳送請求

下一個需要解決的問題是傳遞給 `thread::spawn` 的閉包完全沒有做任何工作。目前，我們在 `execute` 方法中獲得期望執行的閉包，不過在建立 `ThreadPool` 的過程中建立每一個 `Worker` 時需要向 `thread::spawn` 傳遞一個要執行的閉包。

我們希望剛建立的 `Worker` 結構體能夠從 `ThreadPool` 的佇列中獲取需要執行的程式碼，併發送到執行緒中執行。

在第十六章，我們學習了 **通道** —— 一個溝通兩個執行緒的簡單手段 —— 對於這個例子來說則是絕佳的選擇。這裡通道將充當任務佇列的作用，`execute` 將透過 `ThreadPool` 向其中執行緒正在尋找工作的 `Worker` 例項傳送任務。計劃如下：

1. `ThreadPool` 會建立一個通道並持有傳送端。
2. 每個 `Worker` 將持有接收端。
3. 新建一個 `Job` 結構體來存放用於向通道中傳送的閉包。
4. `execute` 方法會在傳送者發出期望執行的工作。
5. 線上程中，`Worker` 會遍歷接收者並執行任何接收到的工作。

讓我們以在 `ThreadPool::new` 中建立通道並讓 `ThreadPool` 例項充當傳送者開始，如示例 21-16 所示。`Job` 結構體目前為空，但它將作為我們透過通道傳送的型別：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

<span class="caption">示例 21-16: 修改 `ThreadPool` 來儲存一個傳輸 `Job` 例項的傳送者</span>

在 `ThreadPool::new` 中，新建了一個通道，並接著讓執行緒池持有傳送端。這段程式碼能夠成功編譯。

讓我們嘗試線上程池建立每個 worker 時將接收端傳遞給它們。須知我們希望在 worker 所分配的執行緒中使用接收者，所以將在閉包中引用 `receiver` 引數。示例 21-17 中展示的程式碼還不能編譯：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

<span class="caption">示例 21-17: 將通道的接收端傳遞給 worker</span>

這是一些簡單而直觀的修改：將接收端傳遞進了 `Worker::new`，並接著在閉包中使用它。

如果嘗試 check 程式碼，會得到這個錯誤：

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

這段程式碼嘗試將 `receiver` 傳遞給多個 `Worker` 例項。這是不行的，回憶第十六章：Rust 所提供的通道實現是多 **生產者**，單 **消費者** 的。這意味著不能簡單的克隆通道的消費端來解決問題。我們也不希望將一個訊息向多個消費者傳送多次；我們希望有一個訊息列表和多個 worker 這樣每個訊息就只會處理一次。

另外，從通道佇列中取出任務涉及到修改 `receiver`，所以這些執行緒需要一個能安全的共享和修改 `receiver` 的方式，否則可能導致競爭狀態（參考第十六章）。

回憶一下第十六章討論的執行緒安全智慧指標，為了在多個執行緒間共享所有權並允許執行緒修改其值，需要使用 `Arc<Mutex<T>>`。`Arc` 使得多個 `Worker` 例項擁有接收端，而 `Mutex` 則確保一次只有一個 `Worker` 能從接收端得到任務。示例 21-18 展示了所需的修改：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

<span class="caption">示例 21-18: 使用 `Arc` 和 `Mutex` 在 worker 間共享接收者</span>

在 `ThreadPool::new` 中，將接收端放入 `Arc` 和 `Mutex` 中。對於每一個新 `Worker` `Arc` 來增加引用計數，如此這些 `Worker` 例項就可以共享接收者的所有權了。

透過這些修改，程式碼可以編譯了！我們已經快完成了！

#### 實現 `execute` 方法

最後讓我們實現 `ThreadPool` 上的 `execute` 方法。同時也要修改 `Job` 結構體：它將不再是結構體，`Job` 將是一個有著 `execute` 接收到的閉包型別的 trait 物件的類型別名。第二十章 [“型別同義詞與類型別名”][type-aliases] 部分提到過，類型別名允許將長的型別變短。觀察示例 21-19：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

<span class="caption">示例 21-19: 為存放每一個閉包的 `Box` 建立一個 `Job` 類型別名，接著在通道中發出任務</span>

在使用 `execute` 得到的閉包新建 `Job` 例項之後，將這些任務從通道的傳送端發出。這裡呼叫 `send` 上的 `unwrap`，因為傳送可能會失敗，這可能發生於例如停止了所有執行緒執行的情況，這意味著接收端停止接收新訊息了。不過目前我們無法停止執行緒執行；只要執行緒池存在它們就會一直執行。使用 `unwrap` 是因為我們知道失敗不可能發生，不過編譯器不知道這些。

不過到此事情還沒有結束！在 `Worker` 中，傳遞給 `thread::spawn` 的閉包仍然還只是 **引用** 了通道的接收端。相反我們需要閉包一直迴圈，向通道的接收端請求任務，並在得到任務時執行它們。如示例 21-20 對 `Worker::new` 做出修改：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

<span class="caption">示例 21-20: 在 worker 執行緒中接收並執行任務</span>

這裡，首先在 `receiver` 上呼叫了 `lock` 來獲取互斥器，接著 `unwrap` 在出現任何錯誤時 panic。如果互斥器處於一種叫做 **被汙染**（*poisoned*）的狀態時獲取鎖可能會失敗，這可能發生於其他執行緒在持有鎖時 panic 了且沒有釋放鎖。在這種情況下，呼叫 `unwrap` 使其 panic 是正確的行為。請隨意將 `unwrap` 改為包含有意義錯誤資訊的 `expect`。

如果鎖定了互斥器，接著呼叫 `recv` 從通道中接收 `Job`。最後的 `unwrap` 也繞過了一些錯誤，這可能發生於持有通道傳送端的執行緒停止的情況，類似於如果接收端關閉時 `send` 方法如何返回 `Err` 一樣。

呼叫 `recv` 會阻塞當前執行緒，所以如果還沒有任務，其會等待直到有可用的任務。`Mutex<T>` 確保一次只有一個 `Worker` 執行緒嘗試請求任務。

現線上程池處於可以執行的狀態了！執行 `cargo run` 併發起一些請求：

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

成功了！現在我們有了一個可以非同步執行連線的執行緒池！它絕不會建立超過四個執行緒，所以當服務端收到大量請求時系統也不會負擔過重。如果請求 */sleep*，server 也能夠透過另外一個執行緒處理其他請求。

> 注意如果同時在多個瀏覽器視窗開啟 */sleep*，它們可能會彼此間隔地載入 5 秒，因為一些瀏覽器出於快取的原因會順序執行相同請求的多個例項。這些限制並不是由於我們的 web 服務端造成的。

現在正適合停下來想一想：如果示例 21-18、21-19 和 21-20 中，待執行的工作不是一個閉包而是 future，那麼這些程式碼會有哪些不同？哪些型別會變？方法簽名會不會變化？哪些部分又會保持不變？

在學習了第十七章和第十八章的 `while let` 迴圈之後，你可能會好奇為何不能如此編寫 worker 執行緒，如示例 21-21 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

<span class="caption">示例 21-21: 一個使用 `while let` 的 `Worker::new` 替代實現</span>

這段程式碼可以編譯和執行，但是並不會產生所期望的執行緒行為：一個慢請求仍然會導致其他請求等待執行。其原因有些微妙：`Mutex` 結構體沒有公有 `unlock` 方法，因為鎖的所有權依賴 `lock` 方法返回的 `LockResult<MutexGuard<T>>` 中 `MutexGuard<T>` 的生命週期。這允許借用檢查器在編譯時確保絕不會在沒有持有鎖的情況下訪問由 `Mutex` 守護的資源，不過如果沒有認真的思考 `MutexGuard<T>` 的生命週期的話，也可能會導致比預期更久的持有鎖。

示例 21-20 中的程式碼使用的 `let job = receiver.lock().unwrap().recv().unwrap();` 之所以可以工作是因為對於 `let` 來說，當 `let` 語句結束時任何表示式中等號右側使用的臨時值都會立即被丟棄。然而 `while let`（`if let` 和 `match`）直到相關的程式碼塊結束都不會丟棄臨時值。在示例 21-21 中，`job()` 呼叫期間鎖一直持續，這也意味著其他的 `Worker` 例項無法接收任務。

[type-aliases]: ch20-03-advanced-types.html#型別同義詞與類型別名
[integer-types]: ch03-02-data-types.html#整型
[fn-traits]: ch13-01-closures.html#將捕獲的值移出閉包
[builder]: https://doc.rust-lang.org/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn
