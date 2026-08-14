## 控制測試如何執行

[ch11-02-running-tests.md](https://github.com/rust-lang/book/blob/99071589c5358114de6324d9aa2643caeee305bd/src/ch11-02-running-tests.md)

就像 `cargo run` 會編譯程式碼並執行生成的二進位制檔案一樣，`cargo test` 在測試模式下編譯程式碼並執行生成的測試二進位制檔案。`cargo test` 產生的二進位制檔案的預設行為是併發執行所有的測試，並截獲測試執行過程中產生的輸出，阻止它們被顯示出來，使得閱讀測試結果相關的內容變得更容易。不過可以指定命令列引數來改變 `cargo test` 的預設行為。

可以將一部分命令列引數傳遞給 `cargo test`，而將另外一部分傳遞給生成的測試二進位制檔案。為了分隔這兩種引數，需要先列出傳遞給 `cargo test` 的引數，接著是分隔符 `--`，再之後是傳遞給測試二進位制檔案的引數。執行 `cargo test --help` 會顯示可用於 `cargo test` 的選項，而執行 `cargo test -- --help` 會顯示可用於分隔符之後的選項。有關這些選項的說明，也可以參閱 [《rustc 手冊》中的 “Tests” 一節][tests]。

[tests]: https://doc.rust-lang.org/rustc/tests/index.html

### 並行或順序執行測試

當執行多個測試時，Rust 預設使用執行緒來並行執行。這意味著測試會更快地執行完畢，所以你可以更快的得到程式碼能否工作的反饋。因為測試是在同時執行的，你應該確保測試不能相互依賴，或依賴任何共享的狀態，包括依賴共享的環境，比如當前工作目錄或者環境變數。

舉個例子，每一個測試都執行一些程式碼，假設這些程式碼都在硬碟上建立一個 *test-output.txt* 檔案並寫入一些資料。接著每一個測試都讀取檔案中的資料並斷言這個檔案包含特定的值，而這個值在每個測試中都是不同的。因為所有測試都是同時執行的，一個測試可能會在另一個測試讀寫檔案過程中修改了檔案。那麼第二個測試就會失敗，並不是因為程式碼不正確，而是因為測試並行執行時相互干擾。一個解決方案是使每一個測試讀寫不同的檔案；另一個解決方案是一次執行一個測試。

如果你不希望測試並行執行，或者想要更加精確的控制執行緒的數量，可以傳遞 `--test-threads` 引數和希望使用執行緒的數量給測試二進位制檔案。例如：

```console
$ cargo test -- --test-threads=1
```

這裡將測試執行緒設定為 `1`，告訴程式不要使用任何並行機制。這也會比並行執行花費更多時間，不過在有共享的狀態時，測試就不會潛在的相互干擾了。

### 顯示函式輸出

預設情況下，當測試透過時，Rust 的測試庫會捕獲列印到標準輸出的所有內容。比如在測試中呼叫了 `println!` 而測試通過了，我們將不會在終端看到 `println!` 的輸出：只會看到說明測試透過的提示行。如果測試失敗了，則會看到所有標準輸出和其他錯誤資訊。

例如，示例 11-10 有一個無意義的函式，它打印出其引數的值並接著返回 10。接著還有一個會透過的測試和一個會失敗的測試：

<span class="filename">檔名：src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

<span class="caption">示例 11-10：一個呼叫了 `println!` 的函式的測試</span>

執行 `cargo test` 將會看到這些測試的輸出：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

注意輸出中不會出現測試透過時列印的內容，即 `I got the value 4`。因為當測試透過時，這些輸出會被截獲。失敗測試的輸出 `I got the value 8` ，則出現在輸出的測試摘要部分，同時也顯示了測試失敗的原因。

如果你希望也能看到透過的測試中列印的值，也可以在結尾加上 `--show-output` 告訴 Rust 顯示成功測試的輸出。

```console
$ cargo test -- --show-output
```

使用 `--show-output` 引數再次執行示例 11-10 中的測試會顯示如下輸出：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### 透過名稱執行部分測試

有時執行整個測試集會耗費很長時間。如果你負責特定位置的程式碼，你可能會希望只執行與這些程式碼相關的測試。你可以向 `cargo test` 傳遞所希望執行的測試名稱的引數來選擇執行哪些測試。

為了展示如何執行部分測試，示例 11-11 為 `add_two` 函式建立了三個測試，我們可以選擇具體執行哪一個：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<span class="caption">示例 11-11：不同名稱的三個測試</span>

如果沒有傳遞任何引數就執行測試，如你所見，所有測試都會並行執行：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### 執行單個測試

可以向 `cargo test` 傳遞任意測試的名稱來只執行這個測試：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

只有名稱為 `one_hundred` 的測試被運行了；因為其餘兩個測試並不匹配這個名稱。測試輸出在末尾顯示了 `2 filtered out`，表明還有 2 個測試被過濾，未被執行。

不能像這樣指定多個測試名稱；只有傳遞給 `cargo test` 的第一個值才會被使用。不過有個執行多個測試的方法。

#### 過濾執行多個測試

我們可以指定部分測試的名稱，任何名稱匹配這個名稱的測試會被執行。例如，因為頭兩個測試的名稱包含 `add`，可以透過 `cargo test add` 來執行這兩個測試：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

這運行了所有名字中帶有 `add` 的測試，也過濾掉了名為 `one_hundred` 的測試。同時注意測試所在的模組也是測試名稱的一部分，所以可以透過過濾模組名來執行一個模組中的所有測試。

<a id="ignoring-some-tests-unless-specifically-requested"></a>

### 除非特別指定否則忽略測試

有時一些特定的測試執行起來是非常耗費時間的，所以在大多數執行 `cargo test` 的時候希望能排除它們。雖然可以透過引數列舉出所有希望執行的測試來做到，也可以使用 `ignore` 屬性來標記耗時的測試並排除它們，如下所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

對於想要排除的測試，我們在 `#[test]` 之後增加了 `#[ignore]` 行。現在如果執行測試，就會發現 `it_works` 運行了，而 `expensive_test` 沒有執行：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`expensive_test` 被列為 `ignored`，如果我們只希望執行被忽略的測試，可以使用 `cargo test -- --ignored`：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

透過控制執行哪些測試，你可以確保 `cargo test` 的結果能夠快速返回。當你需要執行 `ignored` 的測試時，可以執行 `cargo test -- --ignored`。如果你希望不管是否忽略都要執行全部測試，可以執行 `cargo test -- --include-ignored`。
