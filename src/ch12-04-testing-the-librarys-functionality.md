<a id="developing-the-librarys-functionality-with-test-driven-development"></a>

## 採用測試驅動開發增加功能

[ch12-04-testing-the-librarys-functionality.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-04-testing-the-librarys-functionality.md)

現在我們將邏輯提取到了 *src/lib.rs* 並將所有的引數解析和錯誤處理留在了 *src/main.rs* 中，為程式碼的核心功能編寫測試將更加容易。我們可以直接使用多種引數呼叫函式並檢查返回值而無需從命令列執行二進位制檔案了。

在這一部分，我們將遵循測試驅動開發（Test Driven Development, TDD）的模式來逐步增加 `minigrep` 的搜尋邏輯。它遵循如下步驟：

1. 編寫一個失敗的測試，並執行它以確保它失敗的原因是你所期望的。
2. 編寫或修改足夠的程式碼來使新的測試透過。
3. 重構剛剛增加或修改的程式碼，並確保測試仍然能透過。
4. 從步驟 1 開始重複！

雖然這只是眾多編寫軟體的方法之一，不過 TDD 有助於驅動程式碼的設計。在編寫能使測試透過的程式碼之前編寫測試有助於在開發過程中保持高測試覆蓋率。

我們將以測試驅動來實現實際在檔案內容中搜索查詢字串並返回匹配行的示例功能。我們將在一個叫做 `search` 的函式中增加這些功能。

### 編寫失敗測試

去掉 *src/lib.rs* 和 *src/main.rs* 中用於檢查程式行為的 `println!` 語句，因為不再真正需要它們了。接著我們會像[第十一章][ch11-anatomy]那樣增加一個 `test` 模組和一個測試函式。測試函式指定了 `search` 函式期望擁有的行為：它會獲取一個需要查詢的字串和用來查詢的文字，並只會返回包含請求的文字行。示例 12-15 展示了這個測試，它還不能編譯：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">示例 12-15：建立一個我們期望的 `search` 函式的失敗測試</span>

這裡選擇使用 `"duct"` 作為這個測試中需要搜尋的字串。用來搜尋的文字有三行，其中只有一行包含 `"duct"`。（注意雙引號之後的反斜槓，這告訴 Rust 不要在字串字面值內容的開頭加入換行符）我們斷言 `search` 函式的返回值只包含期望的那一行。

我們還不能執行這個測試並看到它失敗，因為它甚至都還不能編譯：`search` 函式還不存在呢！根據 TDD 的原則，我們將增加足夠的程式碼來使其能夠編譯：一個總是會返回空 vector 的 `search` 函式定義，如示例 12-16 所示。然後這個測試應該能夠編譯並因為空 vector 並不匹配一個包含一行 `"safe, fast, productive."` 的 vector 而失敗。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">示例 12-16：剛好足夠使測試透過編譯的 `search` 函式定義</span>

注意需要在 `search` 的簽名中定義一個顯式生命週期 `'a` 並用於 `contents` 引數和返回值。回憶一下[第十章][ch10-lifetimes]中講到生命週期引數指定哪個引數的生命週期與返回值的生命週期相關聯。在這個例子中，我們表明返回的 vector 中應該包含引用引數 `contents`（而不是引數`query`）slice 的字串 slice。

換句話說，我們告訴 Rust 函式 `search` 返回的資料將與 `search` 函式中的引數 `contents` 的資料存在的一樣久。這是非常重要的！為了使這個引用有效那麼**被** slice 引用的資料也需要保持有效；如果編譯器認為我們是在建立 `query` 而不是 `contents` 的字串 slice，那麼安全檢查將是不正確的。

如果我們忘記新增生命週期註解而嘗試編譯此函式，就會得到如下錯誤：

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust 不可能知道我們需要的是哪一個引數，所以需要顯式地告訴它。因為引數 `contents` 包含了所有的文字而且我們希望返回匹配的那部分文字，所以我們知道 `contents` 是應該要使用生命週期語法來與返回值相關聯的引數。

其他語言中並不需要你在函式簽名中將引數與返回值相關聯。所以這麼做可能仍然感覺有些陌生，隨著時間的推移這將會變得越來越容易。你可能想要將這個例子與第十章中[“生命週期確保引用有效”][validating-references-with-lifetimes]部分做對比。

現在執行測試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

很好，測試失敗了，這正是我們所期望的。修改程式碼來讓測試透過吧！

### 編寫使測試透過的程式碼

目前測試之所以會失敗是因為我們總是返回一個空的 vector。為了修復並實現 `search`，我們的程式需要遵循如下步驟：

1. 遍歷內容的每一行文字。
2. 檢視這一行是否包含要搜尋的字串。
3. 如果有，將這一行加入列表返回值中。
4. 如果沒有，什麼也不做。
5. 返回匹配到的結果列表。

讓我們一步一步的來，從遍歷每行開始。

#### 使用 `lines` 方法逐行遍歷

Rust 有一個有助於一行一行遍歷字串的方法，出於方便它被命名為 `lines`，它如示例 12-17 這樣工作。注意這還不能編譯：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">示例 12-17：遍歷 `contents` 的每一行</span>

`lines` 方法返回一個迭代器。[第十三章][ch13-iterators]會深入瞭解迭代器，不過我們已經在[示例 3-5][ch3-iter] 中見過使用迭代器的方法了，在那裡使用了一個 `for` 迴圈和迭代器在一個集合的每一項上運行了一些程式碼。

#### 用查詢字串搜尋每一行

接下來將會增加檢查當前行是否包含查詢字串的功能。幸運的是，字串型別為此也有一個叫做 `contains` 的實用方法！如示例 12-18 所示在 `search` 函式中加入 `contains` 方法呼叫。注意這仍然不能編譯：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">示例 12-18：增加檢查文字行是否包含 `query` 中字串的功能</span>

目前，我們正在構建功能。為了讓程式碼能夠編譯，需要從函式體返回一個我們在函式簽名中所宣告的值。

#### 儲存匹配的行

為了完成這個函式，我們需要一種方法來儲存要返回的匹配行。為此可以在 `for` 迴圈之前建立一個可變的 vector 並呼叫 `push` 方法在 vector 中存放一個 `line`。在 `for` 迴圈之後，返回這個 vector，如示例 12-19 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">示例 12-19：儲存匹配的行以便可以返回它們</span>

現在 `search` 函式應該返回只包含 `query` 的那些行，而測試應該會透過。讓我們執行測試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

測試通過了，我們知道它可以工作了！

現在正是可以考慮重構的時機，在保證測試透過，保持功能不變的前提下重構 `search` 函式。`search` 函式中的程式碼並不壞，不過並沒有利用迭代器的一些實用功能。第十三章將回到這個例子並深入探索迭代器並看看如何改進程式碼。

#### 在 `run` 函式中使用 `search` 函式

現在 `search` 函式是可以工作並測試通過了的，我們需要實際在 `run` 函式中呼叫 `search`。需要將 `config.query` 值和 `run` 從檔案中讀取的 `contents` 傳遞給 `search` 函式。接著 `run` 會打印出 `search` 返回的每一行：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

這裡仍然使用了 `for` 迴圈獲取了 `search` 返回的每一行並打印出來。

現在整個程式應該可以工作了！讓我們試一試，首先使用一個只會在艾米莉·狄金森的詩中返回一行的單詞 “frog”：

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

不錯！現在試試一個會匹配多行的單詞，比如 “body”：

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

最後，讓我們確保搜尋一個在詩中哪裡都沒有的單詞時不會得到任何行，比如 *monomorphization*：

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```


非常好！我們建立了一個屬於自己的迷你版經典工具，並學習了很多如何組織程式的知識。我們還學習了一些檔案輸入輸出、生命週期、測試和命令列解析的內容。

為了使這個專案更豐滿，我們將簡要的展示如何處理環境變數和列印到標準錯誤，這兩者在編寫命令列程式時都很有用。



[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#生命週期確保引用有效
[ch11-anatomy]: ch11-01-writing-tests.html#精心組織測試函式
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#使用-for-遍歷集合
[ch13-iterators]: ch13-02-iterators.html
