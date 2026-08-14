## 重構改進模組性和錯誤處理

[ch12-03-improving-error-handling-and-modularity.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-03-improving-error-handling-and-modularity.md)

為了改善我們的程式這裡有四個問題需要修復，而且它們都與程式的組織方式和如何處理潛在錯誤有關。第一，`main` 現在進行了兩個任務：它解析了引數並打開了檔案。然而隨著 `main` 中的功能持續增加，`main` 函式處理的獨立任務也會增加。當函式承擔了更多責任，它就更難以推導，更難以測試，並且更難以在不破壞其他部分的情況下做出修改。最好能分離出功能以便每個函式各司其職。

這同時也關係到第二個問題：`query` 和 `file_path` 是程式中的配置變數，而像 `contents` 則用來執行程式邏輯。隨著 `main` 函式的增長，就需要引入更多的變數到作用域中，而當作用域中有更多的變數時，將更難以追蹤每個變數的目的。最好能將配置變數組織進一個結構，這樣就能使它們的目的更明確了。

第三個問題是如果開啟檔案失敗時我們使用 `expect` 來打印出錯誤資訊，不過這個錯誤資訊只是說 `Should have been able to read the file`。讀取檔案失敗的原因有多種：例如檔案可能不存在，或者沒有開啟此檔案的許可權。目前，無論哪種情況，都會顯示相同的錯誤資訊，無法為使用者提供任何有用的線索！

第四，我們也使用 `expect` 來處理引數錯誤，如果使用者沒有指定足夠的引數來執行程式，他們會從 Rust 得到 `index out of bounds` 錯誤，而這並不能明確地解釋問題。如果所有的錯誤處理都位於一處，這樣將來的維護者在需要修改錯誤處理邏輯時就只需要考慮這一處程式碼。將所有的錯誤處理都放在一處也有助於確保我們列印的錯誤資訊對終端使用者來說是有意義的。

讓我們通過重構專案來解決這四個問題。

<a id="separation-of-concerns-for-binary-projects"></a>

### 二進位制專案的關注分離

`main` 函式負責多個任務的組織問題在許多二進位制專案中很常見。所以 Rust 社群開發出一類在 `main` 函式開始變得龐大時進行二進位制程式的關注分離的指南。這些過程包括如下步驟：

- 將程式拆分成 *main.rs* 和 *lib.rs* 並將程式的邏輯放入 *lib.rs* 中。
* 當命令列解析邏輯比較小時，可以保留在 *main.rs* 中。
* 當命令列解析開始變得複雜時，也同樣將其從 *main.rs* 提取到 *lib.rs* 中。

經過這些過程之後保留在 `main` 函式中的責任應該被限制為：

* 使用引數值呼叫命令列解析邏輯
* 設定任何其他的配置
* 呼叫 *lib.rs* 中的 `run` 函式
* 如果 `run` 返回錯誤，則進行錯誤處理

這個模式的一切就是為了關注分離：*main.rs* 處理程序執行，而 *lib.rs* 處理所有的真正的任務邏輯。因為不能直接測試 `main` 函式，這個結構通過將所有的程式邏輯移動到 *lib.rs* 的函式中使得我們可以測試它們。僅僅保留在 *main.rs* 中的程式碼將足夠小以便閱讀就可以驗證其正確性。讓我們遵循這些步驟來重構程式。

#### 提取引數解析器

首先，我們將解析引數的功能提取到一個 `main` 將會呼叫的函式中，為將命令列解析邏輯移動到 *src/lib.rs* 中做準備。示例 12-5 中展示了新 `main` 函式的開頭，它呼叫了新函式 `parse_config`。目前它仍將定義在 *src/main.rs* 中：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<span class="caption">示例 12-5：從 `main` 中提取出 `parse_config` 函式</span>

我們仍然將命令列引數收集進一個 vector，不過不同於在 `main` 函式中將索引 1 的引數值賦值給變數 `query` 和將索引 2 的值賦值給變數 `file_path`，我們將整個 vector 傳遞給 `parse_config` 函式。接著 `parse_config` 函式將包含決定哪個引數該放入哪個變數的邏輯，並將這些值返回到 `main`。我們仍然在 `main` 中建立變數 `query` 和 `file_path`，不過 `main` 不再負責處理命令列引數與變數如何對應。

這對重構我們這小程式可能有點大材小用，不過我們將採用小的、增量的步驟進行重構。在做出這些改變之後，再次執行程式並驗證引數解析是否仍然正常。經常驗證你的進展是一個好習慣，這樣在遇到問題時能幫助你定位問題的成因。

### 組合配置值

我們可以採取另一個小的步驟來進一步改善 `parse_config` 函式。現在函式返回一個元組，不過立刻又將元組拆成了獨立的部分。這是一個我們可能沒有進行正確抽象的訊號。

另一個表明還有改進空間的跡象是 `parse_config` 名稱的 `config` 部分，它暗示了我們返回的兩個值是相關的並都是一個配置值的一部分。目前除了將這兩個值組合進元組之外並沒有表達這個資料結構的意義；相反我們可以將這兩個值放入一個結構體並給每個欄位一個有意義的名字。這會讓未來的維護者更容易理解不同的值如何相互關聯以及它們的目的。

示例 12-6 展示了 `parse_config` 函式的改進。

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<span class="caption">示例 12-6：重構 `parse_config` 返回一個 `Config` 結構體例項</span>

我們添加了一個名為 `Config` 的結構體，其中包含 `query` 和 `file_path` 欄位。`parse_config` 的簽名表明它現在返回一個 `Config` 值。在之前的 `parse_config` 函式體中，我們返回了引用 `args` 中 `String` 值的字串 slice，現在我們定義 `Config` 來包含擁有所有權的 `String` 值。`main` 中的 `args` 變數是引數值的所有者並只允許 `parse_config` 函數借用它們，這意味著如果 `Config` 嘗試獲取 `args` 中值的所有權將違反 Rust 的借用規則。

還有許多不同的方式可以處理 `String` 的資料，而最簡單但有些不太高效的方式是呼叫這些值的 `clone` 方法。這會生成 `Config` 例項可以擁有的資料的完整複製，不過會比儲存字串資料的引用消耗更多的時間和記憶體。不過複製資料使得程式碼顯得更加直白因為無需管理引用的生命週期，所以在這種情況下犧牲一小部分效能來換取簡潔性的取捨是值得的。

> #### 使用 `clone` 的權衡取捨
>
> 由於其執行時消耗，許多 Rustacean 之間有一個趨勢是傾向於避免使用 `clone` 來解決所有權問題。在關於迭代器的第十三章中，我們將會學習如何更有效率的處理這種情況，不過現在，複製一些字串來取得進展是沒有問題的，因為只會進行一次這樣的複製，而且檔案路徑和要搜尋的字串都比較短。在第一輪編寫時擁有一個可以工作但有點低效的程式要比嘗試過度最佳化程式碼更好一些。隨著你對 Rust 更加熟練，將能更輕鬆的直奔合適的方法，不過現在呼叫 `clone` 是完全可以接受的。

我們更新 `main` 將 `parse_config` 返回的 `Config` 例項放入變數 `config` 中，並將之前分別使用 `query` 和 `file_path` 變數的程式碼更新為現在的使用 `Config` 結構體的欄位的程式碼。

現在程式碼更明確的表現了我們的意圖，`query` 和 `file_path` 是相關聯的並且它們的目的是配置程式如何工作。任何使用這些值的程式碼就知道在 `config` 例項中對應目的的欄位名中尋找它們。

### 建立 `Config` 的建構函式

目前為止，我們將負責解析命令列引數的邏輯從 `main` 提取到了 `parse_config` 函式中，這有助於我們看清值 `query` 和 `file_path` 是相互關聯的並應該在程式碼中表現這種關係。接著我們增加了 `Config` 結構體來描述 `query` 和 `file_path` 的相關性，並能夠從 `parse_config` 函式中將這些值的名稱作為結構體欄位名稱返回。

所以現在 `parse_config` 函式的目的是建立一個 `Config` 例項，我們可以將 `parse_config` 從一個普通函式變為一個叫做 `new` 的與結構體關聯的函式。做出這個改變使得程式碼更符合習慣：可以像標準庫中的 `String` 呼叫 `String::new` 來建立一個該型別的例項那樣，將 `parse_config` 變為一個與 `Config` 關聯的 `new` 函式。示例 12-7 展示了需要做出的修改：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<span class="caption">示例 12-7：將 `parse_config` 變為 `Config::new`</span>

這裡將 `main` 中呼叫 `parse_config` 的地方更新為呼叫 `Config::new`。我們將 `parse_config` 的名字改為 `new` 並將其移動到 `impl` 塊中，這使得 `new` 函式與 `Config` 相關聯。再次嘗試編譯並確保它可以工作。

### 修復錯誤處理

現在我們開始修復錯誤處理。回憶一下之前提到過如果 `args` vector 包含少於 3 個項並嘗試訪問 vector 中索引 `1` 或索引 `2` 的值會造成程式 panic。嘗試不帶任何引數執行程式；這將看起來像這樣：

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` 是一個面向程式設計師的錯誤資訊，然而這並不能真正幫助終端使用者理解發生了什麼和他們應該做什麼。現在就讓我們修復它吧。

#### 改善錯誤資訊

在示例 12-8 中，在 `new` 函式中增加了一個檢查在訪問索引 `1` 和 `2` 之前檢查 slice 是否足夠長。如果 slice 不夠長，程式會列印一個更好的錯誤資訊並 panic：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

<span class="caption">示例 12-8：增加一個引數數量檢查</span>

這類似於[示例 9-13 中的 `Guess::new` 函式][ch9-custom-types]，那裡如果 `value` 引數超出了有效值的範圍就呼叫 `panic!`。不同於檢查值的範圍，這裡檢查 `args` 的長度至少是 `3`，而函式的剩餘部分則可以在假設這個條件成立的基礎上執行。如果 `args` 少於 3 個項，則這個條件將為真，並呼叫 `panic!` 立即終止程式。

有了 `new` 中這幾行額外的程式碼，再次不帶任何引數執行程式並看看現在錯誤看起來像什麼：

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

這個輸出就更好了：現在有了一個合理的錯誤資訊。然而，還是有一堆額外的資訊我們不希望提供給使用者。所以在這裡使用示例 9-13 中的技術可能不是最好的；正如[第九章][ch9-error-guidelines]所講到的一樣，`panic!` 的呼叫更趨向於程式上的問題而不是使用上的問題。相反我們可以使用第九章學習的另一個技術 —— 返回一個可以表明成功或錯誤的 [`Result`][ch9-result]。

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### 返回 `Result` 而不是呼叫 `panic!`

我們可以選擇返回一個 `Result` 值，它在成功時會包含一個 `Config` 的例項，而在錯誤時會描述問題。我們還將把函式名從 `new` 改為 `build`，因為許多程式設計師希望 `new` 函式永遠不會失敗。當 `Config::build` 與 `main` 交流時，可以使用 `Result` 型別來表明這裡存在問題。接著修改 `main` 將 `Err` 成員轉換為對使用者更友好的錯誤，而不是 `panic!` 呼叫產生的關於 `thread 'main'` 和 `RUST_BACKTRACE` 的文本。

示例 12-9 展示了為了返回 `Result` 在 `Config::build` 的返回值和函式體中所需的改變。注意這還不能編譯，直到下一個示例更新了 `main` 之後。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<span class="caption">示例 12-9：從 `Config::build` 中返回 `Result`</span>

現在 `build` 函式返回一個 `Result`，在成功時帶有一個 `Config` 例項而在出現錯誤時總是一個有著 `'static` 生命週期的字串字面值。

`build` 函式體中有兩處修改：當沒有足夠引數時不再呼叫 `panic!`，而是返回 `Err` 值。同時我們將 `Config` 返回值包裝進 `Ok` 成員中。這些修改使得函式符合其新的型別簽名。

通過讓 `Config::build` 返回一個 `Err` 值，這就允許 `main` 函式處理 `build` 函式返回的 `Result` 值並在出現錯誤的情況下更明確地結束程序。

<a id="calling-confignew-and-handling-errors"></a>

#### 呼叫 `Config::build` 並處理錯誤

為了處理錯誤情況並列印一個對使用者友好的資訊，我們需要像示例 12-10 那樣更新 `main` 函式來處理現在 `Config::build` 返回的 `Result`。另外還需要手動實現原先由 `panic!` 負責的工作，即以非零錯誤碼退出命令列工具的工作。非零的退出狀態是一個慣例訊號，用來告訴呼叫程式的程序：該程式以錯誤狀態退出了。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<span class="caption">示例 12-10：如果新建 `Config` 失敗則使用錯誤碼退出</span>

在上面的示例中，使用了一個之前沒有詳細說明的方法：`unwrap_or_else`，它定義於標準庫的 `Result<T, E>` 上。使用 `unwrap_or_else` 可以進行一些自定義的非 `panic!` 的錯誤處理。當 `Result` 是 `Ok` 時，這個方法的行為類似於 `unwrap`：它返回 `Ok` 內部封裝的值。然而，當其值是 `Err` 時，該方法會呼叫一個**閉包**（*closure*），也就是一個我們定義的作為引數傳遞給 `unwrap_or_else` 的匿名函式。[第十三章][ch13]會更詳細地介紹閉包。現在你需要理解的是 `unwrap_or_else` 會將 `Err` 的內部值，也就是示例 12-9 中增加的 `not enough arguments` 靜態字串的情況，傳遞給閉包中位於兩道豎線間的引數 `err`。閉包中的程式碼在其執行時可以使用這個 `err` 值。

我們新增了一個 `use` 行來從標準庫中匯入 `process`。在錯誤的情況閉包中將被執行的程式碼只有兩行：我們打印出了 `err` 值，接著呼叫了 `std::process::exit`。`process::exit` 會立即停止程式並將傳遞給它的數字作為退出狀態碼。這類似於示例 12-8 中使用的基於 `panic!` 的錯誤處理，除了不會再得到所有的額外輸出了。讓我們試試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

非常好！現在輸出對於使用者來說就友好多了。

<a id="extracting-logic-from-the-main-function"></a>

### 從 `main` 提取邏輯

現在我們完成了配置解析的重構，讓我們轉向程式的邏輯。正如[“二進位制專案的關注分離”](#二進位制專案的關注分離)部分所展開的討論，我們將提取一個叫做 `run` 的函式來存放目前 `main` 函式中不屬於設定配置或處理錯誤的所有邏輯。一旦完成這些，`main` 函式將簡明得足以通過觀察來驗證，而我們將能夠為所有其他邏輯編寫測試。

示例 12-11 展示了提取出來的 `run` 函式。目前我們只進行小的增量式的提取函式的改進。我們仍將在 *src/main.rs* 中定義這個函式：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<span class="caption">示例 12-11：提取 `run` 函式來包含剩餘的程式邏輯</span>

現在 `run` 函式包含了 `main` 中從讀取檔案開始的剩餘的所有邏輯。`run` 函式獲取一個 `Config` 例項作為引數。

<a id="returning-errors-from-the-run-function"></a>

#### 從 `run` 函式返回錯誤

通過將剩餘的邏輯分離進 `run` 函式中，就可以像示例 12-9 中的 `Config::build` 那樣改進錯誤處理。不再通過 `expect` 允許程式 panic，`run` 函式將會在出錯時返回一個 `Result<T, E>`。這讓我們進一步以一種對使用者友好的方式將處理錯誤的邏輯統一到 `main` 中。示例 12-12 展示了 `run` 簽名和函式體中的改變：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<span class="caption">示例 12-12：修改 `run` 函式返回 `Result`</span>

這裡我們做出了三個明顯的修改。首先，將 `run` 函式的返回型別變為 `Result<(), Box<dyn Error>>`。之前這個函式返回 unit 型別 `()`，現在它仍然保持作為 `Ok` 時的返回值。

對於錯誤型別，使用 **trait 物件** `Box<dyn Error>`（在開頭使用了 `use` 語句將 `std::error::Error` 引入作用域）。[第十八章][ch18]會涉及 trait 物件。目前只需知道 `Box<dyn Error>` 意味著函式會返回實現了 `Error` trait 的型別，不過無需指定具體將會返回的值的型別。這提供了在不同的錯誤場景可能有不同型別的錯誤返回值的靈活性。這也就是 `dyn`，它是“動態的”（“dynamic”）的縮寫。

第二個改變是去掉了 `expect` 呼叫並替換為[第九章][ch9-question-mark]講到的 `?` 運算子。不同於遇到錯誤就 `panic!`，`?` 會從函式中返回錯誤值並讓呼叫者來處理它。

第三個修改是現在成功時這個函式會返回一個 `Ok` 值。因為 `run` 函式簽名中宣告成功型別返回值是 `()`，這意味著需要將 unit 型別值包裝進 `Ok` 值中。`Ok(())` 一開始看起來有點奇怪，不過這樣使用 `()` 是慣用的做法，表明呼叫 `run` 函式只是為了它的副作用；函式並沒有返回什麼有意義的值。

執行上述程式碼時，它能夠編譯通過，但會顯示一條警告：

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust 提示我們的程式碼忽略了 `Result` 值，它可能表明這裡存在一個錯誤。但我們卻沒有檢查這裡是否有一個錯誤，而編譯器提醒我們這裡應該有一些錯誤處理程式碼！現在就讓我們修正這個問題。

#### 在 `main` 中處理 `run` 返回的錯誤

我們將檢查錯誤並使用類似示例 12-10 中 `Config::build` 處理錯誤的技術來處理它們，不過有一些細微的不同：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

我們使用 `if let` 來檢查 `run` 是否返回一個 `Err` 值，不同於 `unwrap_or_else`，並在出錯時呼叫 `process::exit(1)`。`run` 並不返回像 `Config::build` 返回的 `Config` 例項那樣需要 `unwrap` 的值。因為 `run` 在成功時返回 `()`，而我們只關心檢測錯誤，所以並不需要 `unwrap_or_else` 來返回未封裝的值，因為它只會是 `()`。

不過兩個例子中 `if let` 和 `unwrap_or_else` 的函式體都一樣：打印出錯誤並退出。

### 將程式碼拆分到庫 crate

現在我們的 `minigrep` 專案看起來好多了！現在我們將要拆分 *src/main.rs* 並將一些程式碼放入 *src/lib.rs*，這樣就能測試它們並擁有一個含有更少功能的 `main` 函式。

讓我們在 *src/lib.rs* 而不是 *src/main.rs* 中定義具有搜尋功能的程式碼，這樣我們（或任何其他使用我們 `minigrep` 庫的人）就可以在比我們 `minigrep` 二進位制 crate 有更多上下文的程式碼中呼叫這個搜尋函式。

首先，在示例 12-13 中我們定義了 `search` 函式簽名，並使用 `unimplemented!` 宏作為函式主體。我們將在實現函式細節時更詳細地解釋函式簽名。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

<span class="caption">示例 12-13：在 *src/lib.rs* 中定義 `search` 函式</span>

我們在函式定義中使用 `pub` 關鍵字來指定 `search` 函式作為我們庫的公共 API 之一。現在我們可以測試或者在二進位制 crate 中使用這個庫 crate 了！

現在需要在 *src/main.rs* 中將定義在 *src/lib.rs* 的程式碼引入二進位制 crate 的作用域中並呼叫，如示例 12-14 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<span class="caption">示例 12-14：將 `minigrep` crate 引入 *src/main.rs* 的作用域中</span>

我們新增一行 `use minigrep::search` 來把 `search` 函式從庫 crate 引入到二進位制 crate 作用域中，然後在 `run` 函式中，我們呼叫 `search` 函式替代列印檔案資料，並將 `config.query` 和 `contents` 作為函式輸入引數。然後，`run` 函式使用 `for` 迴圈列印所有匹配 `query` 的行。這也是移除 `main` 函式中 `println!` 顯示 `query` 和 `file_path` 的好時機，這樣我們的程式只會列印搜尋結果（如果沒有錯誤的話）。

注意，搜尋功能會在列印前將所有結果收集到 vector 中。這種實現在搜尋大型檔案時顯示結果可能會很慢，因為結果不會像找到的那樣列印；我們將在 13 章討論使用迭代器作為解決這個問題的可能方法。

呼！我們做了大量的工作，不過我們為將來的成功打下了基礎。現在處理錯誤將更容易，同時代碼也更加模組化。從現在開始幾乎所有的工作都將在 *src/lib.rs* 中進行。

讓我們利用這些新建立的模組的優勢來進行一些在舊程式碼中難以展開的工作，這些工作在新程式碼中非常容易實現：那就是編寫測試！

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#為驗證建立自定義型別
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#錯誤處理指導原則
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#-運算子快捷方式
