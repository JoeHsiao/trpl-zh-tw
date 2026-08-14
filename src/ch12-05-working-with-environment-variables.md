## 處理環境變數

[ch12-05-working-with-environment-variables.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-05-working-with-environment-variables.md)

我們將增加一個額外的功能來改進 `minigrep`：使用者可以通過設定環境變數來設定搜尋是否是大小寫敏感的選項。當然，我們也可以將其設計為一個命令列引數並要求使用者每次需要時都加上它，不過在這裡我們將使用環境變數。這允許使用者設定環境變數一次之後在整個終端會話中所有的搜尋都將是大小寫不敏感的。

<a id="writing-a-failing-test-for-the-case-insensitive-search-function"></a>

### 編寫一個大小寫不敏感 `search` 函式的失敗測試

首先我們希望增加一個新函式 `search_case_insensitive`，並將會在環境變數有值時呼叫它。這裡將繼續遵循 TDD 過程，其第一步是再次編寫一個失敗測試。我們將為新的大小寫不敏感搜尋函式新增一個測試函式，並將老的測試函式從 `one_result` 改名為 `case_sensitive` 來更清楚的表明這兩個測試的區別，如示例 12-20 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

<span class="caption">示例 12-20：為準備新增的大小寫不敏感函式新增失敗測試</span>

注意我們也改變了老測試中 `contents` 的值。還新增了一個含有文本 `"Duct tape."` 的行，它有一個大寫的 D，這在大小寫敏感搜尋時不應該匹配 `"duct"`。我們修改這個測試以確保不會意外破壞已經實現的大小寫敏感搜尋功能；這個測試現在應該能通過並在處理大小寫不敏感搜尋時應該能一直通過。

大小寫**不敏感**搜尋的新測試使用 `"rUsT"` 作為其查詢字串。在我們將要增加的 `search_case_insensitive` 函式中，`"rUsT"` 查詢應該包含帶有一個大寫 R 的 `"Rust:"` 還有 `"Trust me."` 這兩行，即便它們與查詢的大小寫都不同。這個測試現在不能編譯，因為還沒有定義 `search_case_insensitive` 函式。請隨意增加一個總是返回空 vector 的骨架實現，正如示例 12-16 中 `search` 函式為了使測試通過編譯並失敗時所做的那樣。

### 實現 `search_case_insensitive` 函式

`search_case_insensitive` 函式，如示例 12-21 所示，將與 `search` 函式基本相同。唯一的區別是它會將 `query` 變數和每一 `line` 都變為小寫，這樣不管輸入引數是大寫還是小寫，在檢查該行是否包含查詢字串時都會是小寫。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

<span class="caption">示例 12-21：定義 `search_case_insensitive` 函式，它在比較查詢和每一行之前將它們都轉換為小寫</span>

首先我們將 `query` 字串轉換為小寫，並將其覆蓋到同名的變數中，遮蔽原來的 `query`。對查詢字串呼叫 `to_lowercase` 是必需的，這樣不管使用者的查詢是 `"rust"`、`"RUST"`、`"Rust"` 或者 `"rUsT"`，我們都將其當作 `"rust"` 處理並對大小寫不敏感。雖然 `to_lowercase` 可以處理基本的 Unicode，但它不是 100% 準確。如果編寫真實的程式的話，我們還需多做一些工作，不過這一部分是關於環境變數而不是 Unicode 的，所以這樣就夠了。

注意 `query` 現在是一個 `String` 而不是字串 slice，因為呼叫 `to_lowercase` 是在建立新資料，而不是引用現有資料。如果查詢字串是 `"rUsT"`，這個字串 slice 並不包含可供我們使用的小寫的 `u` 或 `t`，所以必需分配一個包含 `"rust"` 的新 `String`。現在當我們將 `query` 作為一個引數傳遞給 `contains` 方法時，需要增加一個 & 因為 `contains` 的簽名被定義為獲取一個字串 slice。

接下來我們對每一 `line` 都呼叫 `to_lowercase` 將其轉為小寫。現在我們將 `line` 和 `query` 都轉換成了小寫，這樣就可以不管查詢的大小寫進行匹配了。

讓我們看看這個實現能否通過測試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

太好了！測試都通過了。現在，讓我們在 `run` 函式中實際呼叫新的 `search_case_insensitive` 函式。首先，我們將在 `Config` 結構體中增加一個配置項來切換大小寫敏感和大小寫不敏感搜尋。增加這個欄位會導致編譯錯誤，因為我們還沒有在任何地方初始化它：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

這裡增加了 `ignore_case` 欄位來存放一個布林值。接著我們需要 `run` 函式檢查 `ignore_case` 欄位的值，並用它來決定是否呼叫 `search` 函式或 `search_case_insensitive` 函式，如示例 12-22 所示。注意這還不能編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

<span class="caption">示例 12-22：根據 `config.ignore_case` 的值呼叫 `search` 或 `search_case_insensitive`</span>

最後需要實際檢查環境變數。處理環境變數的函式位於標準庫的 `env` 模組中，而這個模組已經在 *src/main.rs* 頂部被引入了作用域。接著使用 `env` 模組的 `var` 方法來檢查一個叫做 `IGNORE_CASE` 的環境變數，如示例 12-23 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

<span class="caption">示例 12-23：檢查叫做 `IGNORE_CASE` 的環境變數</span>

這裡建立了一個新變數 `ignore_case`。為了設定它的值，需要呼叫 `env::var` 函式，並把環境變數 `IGNORE_CASE` 的名字傳給它。`env::var` 返回一個 `Result`：如果環境變數被設定成任意值，就返回包含該值的成功 `Ok` 變體；如果環境變數沒有被設定，就返回 `Err` 變體。

我們使用 `Result` 的 `is_ok` 方法來檢查環境變數是否被設定，這也就意味著程式應該進行一個大小寫不敏感的搜尋。如果 `IGNORE_CASE` 環境變數沒有被設定為任何值，`is_ok` 會返回 `false` 並將進行大小寫敏感的搜尋。我們並不關心環境變數所設定的**值**，只關心它是否被設定了，所以檢查 `is_ok` 而不是 `unwrap`、`expect` 或任何我們已經見過的 `Result` 的方法。

我們將變數 `ignore_case` 的值傳遞給 `Config` 例項，這樣 `run` 函式可以讀取其值並決定是否呼叫示例 12-22 中實現的 `search_case_insensitive` 或者 `search`。

讓我們試一試吧！首先不設定環境變數並使用查詢 `to` 執行程式，這應該會匹配任何全小寫的單詞 “to” 的行：

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

看起來程式仍然能夠工作！現在將 `IGNORE_CASE` 設定為 `1` 並仍使用相同的查詢 *to* 來執行程式：

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

如果你使用 PowerShell，則需要用兩個命令來分別設定環境變數並執行程式：

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

而這會讓 `IGNORE_CASE` 的效果在當前 shell 會話中持續生效。可以通過 `Remove-Item` cmdlet 來取消設定：

```console
PS> Remove-Item Env:IGNORE_CASE
```

這回應該得到包含可能有大寫字母的 *to* 的行：

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

好極了，我們也得到了包含 *to* 的行！現在 `minigrep` 程式可以通過環境變數控制進行大小寫不敏感搜尋了。現在你知道了如何管理由命令列引數或環境變數設定的選項了！

一些程式允許對相同配置同時使用引數**和**環境變數。在這種情況下，程式來決定引數和環境變數的優先順序。作為一個留給你的測試，嘗試通過一個命令列引數或一個環境變數來控制大小寫敏感搜尋。並在執行程式時遇到矛盾值時決定命令列引數和環境變數的優先順序。

`std::env` 模組還包含了更多處理環境變數的實用功能；請檢視其文件來了解其可用的功能。
