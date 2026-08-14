## 構建單執行緒 web server

[ch21-01-single-threaded.md](https://github.com/rust-lang/book/blob/8aa0d003e6499d733d639de32d70f590efa48657/src/ch21-01-single-threaded.md)

首先讓我們建立一個可執行的單執行緒 web server。在開始之前，我們將快速瞭解一下構建 web server 所涉及到的協議。這些協議的細節超出了本書的範疇，不過一個簡單的概括會提供我們所需的資訊。

web server 中涉及到的兩個主要協議是 **超文本傳輸協議**（*Hypertext Transfer Protocol*，*HTTP*）和 **傳輸控制協議**（*Transmission Control Protocol*，*TCP*）。這兩者都是 **請求 - 響應**（*request-response*）協議，也就是說，有 **客戶端**（*client*）來初始化請求，並有 **服務端**（*server*）監聽請求並向客戶端提供響應。請求與響應的內容由協議本身定義。

TCP 是一個底層協議，它描述了資訊如何從一個 server 到另一個的細節，不過其並不指定資訊是什麼。HTTP 構建於 TCP 之上，它定義了請求和響應的內容。從技術上講可將 HTTP 用於其他協議之上，不過對於絕大部分情況，HTTP 通過 TCP 傳輸資料。我們將要做的就是處理 TCP 和 HTTP 請求與響應的原始位元組資料。

### 監聽 TCP 連線

我們的 web server 所需做的第一件事，就是監聽 TCP 連線。標準庫提供了 `std::net` 模組處理這些功能。讓我們像往常一樣新建一個專案：

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

現在，在 `src/main.rs` 輸入示例 21-1 中的程式碼，作為一個開始。這段程式碼會在地址 `127.0.0.1:7878` 上監聽傳入的 TCP 流。當獲取到傳入的流，它會打印出 `Connection established!`：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

<span class="caption">示例 21-1: 監聽傳入的流並在接收到流時列印資訊</span>

`TcpListener` 用於監聽 TCP 連線。我們選擇監聽本地地址 `127.0.0.1:7878`。將這個地址拆開來看，冒號之前的部分是一個代表本機的 IP 地址（在每臺計算機上，這個地址都指本機，並不特指作者的計算機），而 `7878` 是埠。選擇這個端口出於兩個原因：通常 HTTP 伺服器不在這個埠上接受請求，所以它不太可能與你機器上執行的其它 web server 的埠衝突；而且 7878 在電話上打出來就是 "rust"（譯者注：九宮格鍵盤上的英文）。

在這個場景中 `bind` 函式類似於 `new` 函式，在這裡它返回一個新的 `TcpListener` 例項。這個函式叫做 `bind` 是因為，在網路領域，連線到要監聽的埠稱為“繫結到埠”（“binding to a port”）

`bind` 函式返回 `Result<T, E>`，這表明繫結可能會失敗。例如，監聽 80 埠需要管理員許可權（非管理員使用者只能監聽大於 1023 的埠），所以如果嘗試監聽 80 埠而沒有管理員許可權，則會繫結失敗。再比如，如果我們執行這個程式的兩個例項，並因此有兩個例項監聽同一個埠，那麼繫結也將失敗。我們是出於學習目的來編寫一個基礎的伺服器，不用關心處理這類錯誤，而僅僅使用 `unwrap` 在出現這些情況時直接停止程式。

`TcpListener` 的 `incoming` 方法返回一個迭代器，它提供了一系列的流（更準確的說是 `TcpStream` 型別的流）。**流**（*stream*）代表一個客戶端和服務端之間開啟的連線。**連線**（*connection*）代表客戶端連線服務端、服務端生成響應以及服務端關閉連線的整個請求 / 響應過程。為此，我們會從 `TcpStream` 讀取客戶端傳送了什麼並接著向流傳送響應以向客戶端發回資料。總體來說，這個 `for` 迴圈會依次處理每個連線併產生一系列的流供我們處理。

目前，處理流的程式碼中也有一個 `unwrap` 呼叫，如果 `stream` 出現任何錯誤會終止程式；如果沒有任何錯誤，程式會列印一條訊息。下一個示例中，我們將為成功的情況增加更多功能。當客戶端連線到服務端時，`incoming` 方法是可能返回錯誤的，因為我們實際上不是在遍歷連線，而是遍歷 **連線嘗試**（*connection attempts*）。連線的嘗試可能會因為多種原因不能成功，其中大部分是作業系統相關的。例如，很多系統限制它所能支援的同時開啟的連線數，超出數量限制的新連線嘗試會產生錯誤，直到一些現有的連線關閉為止。

讓我們試試這段程式碼！首先在終端執行 `cargo run`，接著在瀏覽器中開啟 `127.0.0.1:7878`。瀏覽器會顯示出看起來類似於“連線重置”（“Connection reset”）的錯誤資訊，因為 server 目前並沒響應任何資料。如果我們觀察終端，會發現當瀏覽器連線我們的服務端時，會打印出一系列的資訊！

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

有時，對於一次瀏覽器請求可能會打印出多條資訊；原因可能是瀏覽器不僅請求頁面，還請求其他資源，比如出現在瀏覽器標籤頁中的 *favicon.ico* 圖示。

這也可能是因為瀏覽器嘗試多次連線服務端，因為服務端沒有響應任何資料。當 `stream` 在迴圈結束時離開作用域並被丟棄，其連線將作為 `drop` 實現的一部分被關閉。瀏覽器有時通過重連來處理關閉的連線，因為這些問題可能是暫時的。

瀏覽器有時還會在不傳送任何請求的情況下開啟多個連線，以便在**稍後**傳送請求時能夠更快地開始。出現這種情況時，我們的服務端會看到每個連線，而不管該連線上是否有請求。例如，許多基於 Chrome 的瀏覽器版本都會這樣做；你可以通過使用私人瀏覽模式或更換其他瀏覽器來停用該最佳化。

重要的是，我們已經成功獲取了一個 TCP 連線的控制代碼！

記得當執行完特定版本的程式碼後，使用 <kbd>ctrl</kbd>-<kbd>c</kbd> 來停止程式。然後在你完成每次程式碼修改後，通過執行 `cargo run` 命令重新啟動程式，以確保你正在執行最新的程式碼。

### 讀取請求

讓我們實現讀取來自瀏覽器請求的功能！為了將首先獲取連線和接著對連線採取操作兩項職責分離，我們將開始寫一個新函式來處理連線。在這個新的 `handle_connection` 函式中，我們從 TCP 流中讀取資料，並打印出來，以便觀察瀏覽器傳送過來的資料。將程式碼修改為如示例 21-2 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

<span class="caption">示例 21-2: 讀取 `TcpStream` 並列印資料</span>

這裡將 `std::io::prelude` 和 `std::io::BufReader` 引入作用域，來獲取讀寫流所需的 trait 和型別。在 `main` 函式的 `for` 迴圈中，相比獲取到連線時列印資訊，現在呼叫新的 `handle_connection` 函式並向其傳遞 `stream`。

在 `handle_connection` 中，我們新建了一個 `BufReader` 例項來封裝一個 `stream` 的引用。`BufReader` 通過替我們管理 `std::io::Read` trait 方法的呼叫增加了緩衝。

我們建立了一個 `http_request` 變數來收集瀏覽器傳送給服務端的請求行。這裡增加了 `Vec<_>` 型別註解表明希望將這些行收集到一個 vector 中。

`BufReader` 實現了 `std::io::BufRead` trait，它提供了 `lines` 方法。`lines` 方法通過遇到換行符（newline）位元組就切分資料流來返回一個 `Result<String, std::io::Error>` 的迭代器。為了獲取每一個 `String`，我們通過 map 並 `unwrap` 每一個 `Result`。如果資料不是有效的 UTF-8 編碼或者讀取流遇到問題時，`Result` 可能是一個錯誤。同理，用於生產環境的程式應該更優雅地處理這些錯誤，不過出於簡單的目的我們選擇在錯誤情況下停止程式。

瀏覽器通過連續傳送兩個換行符來代表一個 HTTP 請求的結束，所以為了從流中獲取一個請求，我們會讀取行直到遇到一個空字串的行。一旦將這些行收集進 vector，就可以使用友好的 debug 格式化列印它們，以便看看瀏覽器傳送給服務端的指令。

讓我們試一試！啟動程式並再次在瀏覽器中發起請求。注意瀏覽器中仍然會出現錯誤頁面，不過終端中程式的輸出現在看起來像這樣：

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

根據不同的瀏覽器，可能會出現稍微不同的輸出。現在我們打印出了請求資料，可以通過觀察第一行 `GET` 之後的路徑就可以看出為何會從瀏覽器得到多個連線。如果重複的連線都是請求 */*，就知道了瀏覽器嘗試重複獲取 */* 因為它沒有從程式得到響應。

讓我們拆開請求資料來理解瀏覽器向程式請求了什麼。

### 更仔細地觀察 HTTP 請求

HTTP 是一個基於文本的協議，請求的格式如下：

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

第一行叫做 **請求行**（*request line*），它存放了客戶端請求了什麼的資訊。請求行的第一部分是所使用的 *method*，比如 `GET` 或 `POST`，這描述了客戶端如何進行請求。這裡客戶端使用了 `GET` 請求，表明它在請求資訊。

請求行接下來的部分是 */*，它代表客戶端請求的 **統一資源識別符號**（*Uniform Resource Identifier*，*URI*）：URI 大體上但也不完全類似於 URL（**統一資源定位符**，*Uniform Resource Locators*）。URI 和 URL 之間的區別對於本章的目的來說並不重要，不過 HTTP 規範使用術語 URI，所以這裡可以簡單的將 URL 理解為 URI。

最後一部分是客戶端使用的 HTTP 版本，然後請求行以 **CRLF 序列** （CRLF 代表回車和換行，*carriage return line feed*，這是打字機時代的術語！）結束。CRLF 序列也可以寫成`\r\n`，其中`\r`是回車符，`\n`是換行符。CRLF 序列將請求行與其餘請求資料分開。請注意，列印 CRLF 時，我們會看到一個新行開始，而不是`\r\n`。

觀察目前為止執行程式所接收到的請求行資料，可以看到 `GET` 是 method，*/* 是請求 URI，而 `HTTP/1.1` 是版本。

從 `Host:` 開始的其餘的行是 headers；`GET` 請求沒有 body。

如果你希望的話，可以嘗試用不同的瀏覽器傳送請求，或請求不同的地址，比如 `127.0.0.1:7878/test`，來觀察請求資料如何變化。

現在我們知道了瀏覽器請求了什麼。讓我們返回一些資料！

### 編寫響應

我們將實現在客戶端請求的響應中傳送資料的功能。響應具有如下格式：

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

第一行叫做 **狀態行**（*status line*），它包含響應的 HTTP 版本、一個數字狀態碼（status code）用以總結請求的結果和一個描述之前狀態碼的文本原因短語（reason phrase）。CRLF 序列之後是任意 header，另一個 CRLF 序列，和響應的 body。

這裡是一個使用 HTTP 1.1 版本的響應例子，其狀態碼為 200，原因短語為 OK，沒有 header，也沒有 body：

```text
HTTP/1.1 200 OK\r\n\r\n
```

狀態碼 200 是一個標準的成功響應。這些文本是一個微型的成功 HTTP 響應。讓我們將這些文本寫入流作為成功請求的響應！在 `handle_connection` 函式中，我們需要去掉列印請求資料的 `println!`，並替換為示例 21-3 中的程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

<span class="caption">示例 21-3: 將一個微型成功 HTTP 響應寫入流</span>

新程式碼中的第一行定義了變數 `response` 來存放成功訊息的資料。接著，我們在 `response` 上呼叫 `as_bytes` 將字串資料轉換為位元組陣列。因為 `stream` 的 `write_all` 方法獲取一個 `&[u8]` 並直接將這些位元組傳送給連線。因為 `write_all` 操作可能會失敗，所以像之前那樣對任何錯誤結果使用 `unwrap`。同理，在真實世界的應用中這裡需要新增錯誤處理。

有了這些修改，執行我們的程式碼並進行請求。由於不再向終端列印任何資料，所以不會再看到除了 Cargo 以外的任何輸出。不過當在瀏覽器中載入 *127.0.0.1:7878* 時，會得到一個空頁面而不是錯誤。我們剛剛手寫了接收 HTTP 請求併發送響應！

### 返回真正的 HTML

讓我們實現不只是返回空頁面的功能。在專案根目錄建立一個新檔案，*hello.html*，不是在 *src* 目錄。在此可以放入任何你期望的 HTML 內容；示例 21-4 展示了一個可能的文本：

<span class="filename">檔名：hello.html</span>

```html
{{#include ../listings/ch21-web-server/listing-21-05/hello.html}}
```

<span class="caption">示例 21-4: 一個示例 HTML 檔案作為響應返回</span>

這是一個極簡 HTML5 文件包含一個標題和一小段文本。為了在服務端接收請求時返回它，需要如示例 21-5 所示修改 `handle_connection` 來讀取 HTML 檔案，將其加入到響應的 body 中併發送：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

<span class="caption">示例 21-5: 將 *hello.html* 的內容作為響應 body 傳送</span>

我們在開頭 `use` 語句將標準庫的檔案系統模組 `fs` 引入作用域。開啟和讀取檔案的程式碼應該看起來很熟悉；在第十二章 `I/O` 專案中的 Listing 12-4 就用到了類似的程式碼。

接下來，使用 `format!` 將檔案內容加入到將要寫入流的成功響應的 body 中。為了確保構造出有效的 HTTP 響應，我們添加了 `Content-Length` header，其值設為響應 `body` 的大小，在這裡也就是 `hello.html` 檔案的大小。

使用 `cargo run` 執行程式，在瀏覽器載入 *127.0.0.1:7878*，你應該會看到渲染後的 HTML！

目前忽略了 `http_request` 中的請求資料並無條件的傳送了 HTML 檔案的內容。這意味著如果嘗試在瀏覽器中請求 *127.0.0.1:7878/something-else* 也會得到同樣的 HTML 響應。目前我們的 server 的作用是非常有限的，也不是大部分 server 所做的那樣；讓我們檢查請求並只對格式良好（well-formed）的請求 `/` 傳送 HTML 檔案。

### 驗證請求並有選擇的進行響應

目前我們的 web server 不管客戶端請求什麼都會返回相同的 HTML 檔案。讓我們增加在返回 HTML 檔案前檢查瀏覽器是否請求 */*，並在其請求任何其他內容時返回錯誤的功能。為此需要如示例 21-6 那樣修改 `handle_connection`。新程式碼接收到的請求的內容與已知的 */* 請求做比較，並增加了 `if` 和 `else` 塊來區別處理請求：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

<span class="caption">示例 21-6: 以不同於其它請求的方式處理 */* 請求</span>

我們只看 HTTP 請求的第一行，所以不同於將整個請求讀取進 vector 中，這裡呼叫 `next` 從迭代器中獲取第一項。第一個 `unwrap` 負責處理 `Option` 並在迭代器沒有項時停止程式。第二個 `unwrap` 處理 `Result` 並與示例 21-2 中增加的 `map` 中的 `unwrap` 有著相同的效果。

接下來檢查 `request_line` 是否等於一個 */* 路徑的 GET 請求。如果是，`if` 程式碼塊返回 HTML 檔案的內容。

如果 `request_line` **不** 等於一個 */* 路徑的 GET 請求，就說明接收的是其它請求。我們之後會在 `else` 塊中增加程式碼來響應所有其他請求。

現在如果執行程式碼並請求 *127.0.0.1:7878*，就會得到 *hello.html* 中的 HTML。如果進行任何其他請求，比如 *127.0.0.1:7878/something-else*，則會得到像執行示例 21-1 和 21-2 中程式碼那樣的連線錯誤。

現在向示例 21-7 的 `else` 塊增加程式碼來返回一個帶有 404 狀態碼的響應，這代表了所請求的內容沒有找到。接著也會返回一個 HTML 向瀏覽器終端使用者渲染該響應。

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

<span class="caption">示例 21-7: 對於任何不是 */* 的請求返回 404 狀態碼的響應和錯誤頁面</span>

這裡，響應的狀態行有狀態碼 404 和原因短語 `NOT FOUND`。仍然沒有返回任何 header，而其 body 將是 *404.html* 檔案中的 HTML。需要在 *hello.html* 同級目錄建立 *404.html* 檔案作為錯誤頁面；這一次也可以隨意使用任何 HTML 或使用示例 21-8 中的示例 HTML：

<span class="filename">檔名：404.html</span>

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

<span class="caption">示例 21-8: 任何 404 響應所返回錯誤頁面內容樣例</span>

有了這些修改，再次執行服務端。請求 *127.0.0.1:7878* 應該會返回 *hello.html* 的內容，而對於任何其他請求，比如 *127.0.0.1:7878/foo*，應該會返回 *404.html* 中的錯誤 HTML。

### 重構

目前 `if` 和 `else` 塊中的程式碼存在大量重複：他們都讀取檔案並將其內容寫入流。唯一的區別是狀態行和檔名。為使程式碼更簡潔，將這些區別分別提取到各自的 `if` 和 `else` 中，對狀態行和檔名變數賦值；然後在讀取檔案和寫入響應的程式碼中無條件地使用這些變數。重構後取代了大段 `if` 和 `else` 塊程式碼後的結果如示例 21-9 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

<span class="caption">示例 21-9: 重構使得 `if` 和 `else` 塊中只包含兩個情況所不同的程式碼</span>

現在 `if` 和 `else` 塊所做的唯一的事就是在一個元組中返回合適的狀態行和檔名的值；接著使用第十九章講到的使用模式的 `let` 語句通過解構元組的兩部分給 `filename` 和 `header` 賦值。

之前讀取檔案和寫入響應的冗餘程式碼現在位於 `if` 和 `else` 塊之外，並會使用變數 `status_line` 和 `filename`。這樣更易於觀察這兩種情況真正有何不同，還意味著如果需要改變如何讀取檔案或寫入響應時只需要更新一處的程式碼。示例 21-9 中程式碼的行為與示例 21-8 完全相同。

好極了！我們現在有了一個 40 行左右 Rust 程式碼的小而簡單的服務端，它對一個請求返回頁面內容而對所有其他請求返回 404 響應。

目前服務端運行於單執行緒中，這意味著它一次只能處理一個請求。讓我們模擬一些慢請求來看看這為何會成為一個問題。然後我們將修復它使得服務端能夠同時處理多個請求。
