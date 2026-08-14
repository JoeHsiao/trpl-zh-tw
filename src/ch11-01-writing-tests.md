## 如何編寫測試

[ch11-01-writing-tests.md](https://github.com/rust-lang/book/blob/99071589c5358114de6324d9aa2643caeee305bd/src/ch11-01-writing-tests.md)

Rust 中的測試函式是用來驗證非測試程式碼是否按照期望的方式執行的。測試函式體通常執行如下三種操作：

- 設定任何所需的資料或狀態
- 執行需要測試的程式碼
- 斷言其結果是我們所期望的

讓我們看看 Rust 提供的專門用來編寫測試的功能：`test` 屬性、一些宏和 `should_panic` 屬性。

<a id="the-anatomy-of-a-test-function"></a>

### 精心組織測試函式

作為最簡單例子，Rust 中的測試就是一個帶有 `test` 屬性註解的函式。屬性（attribute）是關於 Rust 程式碼片段的後設資料；第五章中結構體中用到的 `derive` 屬性就是一個例子。為了將一個函式變成測試函式，需要在 `fn` 行之前加上 `#[test]`。當使用 `cargo test` 命令執行測試時，Rust 會構建一個測試執行程式用來呼叫被標註的函式，並報告每一個測試是通過還是失敗。

每次使用 Cargo 新建一個庫專案時，它會自動為我們生成一個測試模組和一個測試函式。這個模組提供了一個編寫測試的模板，為此每次開始新專案時不必去查詢測試函式的具體結構和語法了。當然你也可以額外增加任意多的測試函式以及測試模組！

在實際編寫測試程式碼之前，讓我們先通過嘗試那些自動生成的測試模版來探索測試是如何工作的。接著，我們會寫一些真正的測試，呼叫我們編寫的程式碼並斷言它們的行為的正確性。

讓我們建立一個新的庫專案 `adder`，它會將兩個數字相加：

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

`adder` 庫中 _src/lib.rs_ 的內容應該看起來如示例 11-1 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">示例 11-1：由 `cargo new` 自動生成的測試模組和函式</span>

檔案以一個示例 `add` 函式開頭，這樣我們就有東西可以測試。

現在讓我們只關注 `it_works` 函式本身。注意 `fn` 行之前的 `#[test]`：這個屬性表明這是一個測試函式，這樣測試執行者就知道將其作為測試處理。`tests` 模組中也可以有非測試的函式來幫助我們建立通用場景或進行常見操作，必須每次都標明哪些函式是測試。

示例函式體通過使用 `assert_eq!` 宏來斷言 `result`（其中包含 2 加 2 的結果）等於 4。這個斷言示例展示了典型測試的格式。接下來執行就可以看到測試通過。

`cargo test` 命令會執行專案中所有的測試，如示例 11-2 所示：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">示例 11-2：執行自動生成測試的輸出</span>

Cargo 編譯並運行了測試。可以看到 `running 1 test` 這一行。下一行顯示了生成的測試函式的名稱，`tests::it_works`，以及測試的執行結果，`ok`。接著可以看到全體測試執行結果的摘要：`test result: ok.` 意味著所有測試都通過了。`1 passed; 0 failed` 表示通過或失敗的測試數量。

可以將一個測試標記為忽略以便在特定情況下它就不會執行；本章之後的[“除非特別指定否則忽略測試”][ignoring]部分會介紹它。因為之前我們並沒有將任何測試標記為忽略，所以摘要中會顯示 `0 ignored`。

`0 measured` 統計是針對性能測試的。效能測試（benchmark tests）在編寫本書時，仍只能用於 Rust 開發版（nightly Rust）。請檢視 [效能測試的文件][bench] 瞭解更多。

我們可以將引數傳遞給 `cargo test` 命令，以便只執行名稱與字串匹配的測試；這就是所謂的**過濾**（_filtering_），我們會在 [“通過名稱執行部分測試”][subset] 討論這一點。這裡我們沒有過濾需要執行的測試，所以摘要中會顯示`0 filtered out`。

測試輸出中的以 `Doc-tests adder` 開頭的這一部分是所有文件測試的結果。我們現在並沒有任何文件測試，不過 Rust 會編譯任何在 API 文件中的程式碼示例。這個功能幫助我們使文件和程式碼保持同步！在第十四章的 [“文件註釋作為測試”][doc-comments] 部分會講到如何編寫文件測試。現在我們將忽略 `Doc-tests` 部分的輸出。

讓我們開始自定義測試來滿足我們的需求。首先給 `it_works` 函式起個不同的名字，比如 `exploration`，像這樣：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

並再次執行 `cargo test`。現在輸出中將出現 `exploration` 而不是 `it_works`：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

現在讓我們增加另一個測試，不過這一次是一個會失敗的測試！當測試函式中出現 panic 時測試就失敗了。每一個測試都在一個新執行緒中執行，當主執行緒發現測試執行緒異常了，就將對應測試標記為失敗。第九章講到了最簡單的造成 panic 的方法：呼叫 `panic!` 宏。寫入新測試 `another` 後，_src/lib.rs_ 現在看起來如示例 11-3 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

<span class="caption">示例 11-3：增加第二個因呼叫了 `panic!` 而失敗的測試</span>

再次 `cargo test` 執行測試。輸出應該看起來像示例 11-4，它表明 `exploration` 測試通過了而 `another` 失敗了：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">示例 11-4：一個測試通過和一個測試失敗的測試結果</span>

`test tests::another` 這一行是 `FAILED` 而不是 `ok` 了。在單獨測試結果和摘要之間多了兩個新的部分：第一個部分顯示了測試失敗的詳細原因。在這個例子中，我們看到 `another` 因為在 _src/lib.rs_ 的第 17 行 `panicked at 'Make this test fail'` 而失敗的詳細資訊。下一部分列出了所有失敗的測試，這在有很多測試和很多失敗測試的詳細輸出時很有幫助。我們可以通過使用失敗測試的名稱來只執行這個測試，以便除錯；下一部分 [“控制測試如何執行”][controlling-how-tests-are-run] 會講到更多執行測試的方法。

最後是摘要行：總體上講，測試結果是 `FAILED`。有一個測試通過和一個測試失敗。

現在我們見過不同場景中測試結果是什麼樣子的了，再來看看除 `panic!` 之外的一些在測試中有幫助的宏吧。

<a id="checking-results-with-the-assert-macro"></a>

### 使用 `assert!` 宏來檢查結果

`assert!` 宏由標準庫提供，在希望確保測試中一些條件為 `true` 時非常有用。需要向 `assert!` 宏提供一個求值為布林值的引數。如果值是 `true`，`assert!` 什麼也不做，同時測試會通過。如果值為 `false`，`assert!` 呼叫 `panic!` 宏，這會導致測試失敗。`assert!` 宏幫助我們檢查程式碼是否以期望的方式執行。

回憶一下第五章中，示例 5-15 中有一個 `Rectangle` 結構體和一個 `can_hold` 方法，在示例 11-5 中再次使用它們。將它們放進 _src/lib.rs_ 並使用 `assert!` 宏編寫一些測試。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

<span class="caption">示例 11-5：第五章中 `Rectangle` 結構體和其 `can_hold` 方法</span>

`can_hold` 方法返回一個布林值，這意味著它完美符合 `assert!` 宏的使用場景。在示例 11-6 中，讓我們編寫一個 `can_hold` 方法的測試來作為練習，這裡建立一個寬為 8 高為 7 的 `Rectangle` 例項，並假設它可以放得下另一個寬為 5 高為 1 的 `Rectangle` 例項：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">示例 11-6：一個 `can_hold` 的測試，檢查一個較大的矩形確實能放得下一個較小的矩形</span>

注意在 `tests` 模組中新增加了一行：`use super::*;`。`tests` 是一個普通的模組，它遵循第七章 [“路徑用於引用模組樹中的項”][paths-for-referring-to-an-item-in-the-module-tree] 部分介紹的可見性規則。因為 `tests` 模組是一個內部模組，要測試外部模組中的程式碼，需要將其引入到內部模組的作用域中。這裡選擇使用 glob 全域性匯入，以便在 `tests` 模組中使用所有在外部模組定義的內容。

我們將測試命名為 `larger_can_hold_smaller`，並建立所需的兩個 `Rectangle` 例項。接著呼叫 `assert!` 宏並傳遞 `larger.can_hold(&smaller)` 呼叫的結果作為引數。這個表示式預期會返回 `true`，所以測試應該通過。讓我們拭目以待！

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

它確實通過了！再來增加另一個測試，這一回斷言一個更小的矩形不能放下一個更大的矩形：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

因為這裡 `can_hold` 函式的正確結果是 `false` ，我們需要將這個結果取反後傳遞給 `assert!` 宏。因此 `can_hold` 返回 `false` 時測試就會通過：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

兩個測試通過了！現在讓我們看看如果引入一個 bug 的話測試結果會發生什麼。將 `can_hold` 方法中比較寬度時本應使用大於號的地方改成小於號：

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

現在執行測試會產生以下結果：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

我們的測試捕獲了 bug！因為 `larger.width` 是 8 而 `smaller.width` 是 5，`can_hold` 中的寬度比較現在因為 8 不小於 5 而返回 `false`：8 並不小於 5。

<a id="testing-equality-with-the-assert_eq-and-assert_ne-macros"></a>

### 使用 `assert_eq!` 和 `assert_ne!` 宏測試相等

測試功能的一個常用方法是將需要測試程式碼的值與期望值做比較，並檢查是否相等。可以通過向 `assert!` 宏傳遞一個使用 `==` 運算子的表示式來做到。不過這個操作實在是太常見了，以至於標準庫提供了一對宏來更方便的處理這些操作 —— `assert_eq!` 和 `assert_ne!`。這兩個宏分別比較兩個值是相等還是不相等。當斷言失敗時它們也會打印出這兩個值具體是什麼，以便於觀察測試**為什麼**失敗，而 `assert!` 只會打印出它從 `==` 表示式中得到了 `false` 值，而不是列印導致 `false` 的具體值。

示例 11-7 中，讓我們編寫一個對其引數加二並返回結果的函式 `add_two`。接著使用 `assert_eq!` 宏測試這個函式。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">示例 11-7：使用 `assert_eq!` 宏測試 `add_two` 函式</span>

測試通過了！

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

我們建立一個名為 `result` 的變數，用於儲存呼叫 `add_two(2)` 的結果。然後我們將 `result` 和 `4` 作為引數傳遞給 `assert_eq!`。測試中的這一行 `test tests::it_adds_two ... ok` 中 `ok` 表明測試通過！

在程式碼中引入一個 bug 來看看使用 `assert_eq!` 的測試失敗是什麼樣的。修改 `add_two` 函式的實現使其加 `3`：

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

再次執行測試：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

測試捕獲到了 bug！`it_adds_two` 測試失敗，錯誤資訊告訴我們斷言失敗了，它告訴我們 ``assertion `left == right` failed`` 以及 `left` 和 `right` 的值是什麼。這個錯誤資訊有助於我們開始除錯：它說 `assert_eq!` 的 `left` 引數（也就是 `add_two(2)` 的結果）是 `5`，而 `right` 引數是 `4`。可以想像當有很多測試在執行時這些資訊是多麼的有用。

需要注意的是，在一些語言和測試框架中，斷言兩個值相等的函式的引數被稱為 `expected` 和 `actual`，而且指定引數的順序非常重要。然而在 Rust 中，它們則叫做 `left` 和 `right`，同時指定期望的值和被測試程式碼產生的值的順序並不重要。這個測試中的斷言也可以寫成 `assert_eq!(add_two(2), result)`，這時失敗資訊仍同樣是 `` assertion failed: `(left == right)` ``。

`assert_ne!` 宏在傳遞給它的兩個值不相等時通過，而在相等時失敗。當我們不確定值**會**是什麼，不過能確定值絕對**不會**是什麼的時候，這個宏最有用處。例如，如果一個函式保證會以某種方式改變其輸入，不過這種改變方式是由執行測試時是星期幾來決定的，這時最好的斷言可能就是函式的輸出不等於其輸入。

`assert_eq!` 和 `assert_ne!` 宏在底層分別使用了 `==` 和 `!=`。當斷言失敗時，這些宏會使用除錯格式打印出其引數，這意味著被比較的值必須實現了 `PartialEq` 和 `Debug` trait。所有的基本型別和大部分標準庫型別都實現了這些 trait。對於自定義的結構體和列舉，需要實現 `PartialEq` 才能斷言它們的值是否相等。需要實現 `Debug` 才能在斷言失敗時列印它們的值。因為這兩個 trait 都是派生 trait，如第五章示例 5-12 所提到的，通常可以直接在結構體或列舉上新增 `#[derive(PartialEq, Debug)]` 註解。附錄 C [“可派生 trait”][derivable-traits] 中有更多關於這些和其他派生 trait 的詳細資訊。

### 自定義失敗資訊

你也可以向 `assert!`、`assert_eq!` 和 `assert_ne!` 宏傳遞一個可選的失敗資訊引數，可以在測試失敗時將自定義失敗資訊一同打印出來。任何在 `assert!` 的一個必需引數和 `assert_eq!` 和 `assert_ne!` 的兩個必需引數之後指定的引數都會傳遞給 `format!` 宏（在第八章的 [“使用 `+` 運算子或 `format!` 宏拼接字串”][concatenation-with-the--operator-or-the-format-macro] 部分討論過），所以可以傳遞一個包含 `{}` 佔位符的格式字串和需要放入佔位符的值。自定義資訊有助於記錄斷言的意義；當測試失敗時就能更好的理解程式碼出了什麼問題。

例如，比如說有一個根據人名進行問候的函式，而我們希望測試將傳遞給函式的人名顯示在輸出中：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

這個程式的需求還沒有被確定，因此問候文本開頭的 `Hello` 文本很可能會改變。然而我們並不想在需求改變時不得不更新測試，所以相比檢查 `greeting` 函式返回的確切值，我們將僅僅斷言輸出的文本中包含輸入引數。

讓我們通過將 `greeting` 改為不包含 `name` 在程式碼中引入一個 bug 來測試失敗時是怎樣的：

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

執行測試會產生：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

結果僅僅告訴了我們斷言失敗了和失敗的行號。一個更有用的失敗資訊應該打印出 `greeting` 函式的值。讓我們為測試函式增加一個自定義失敗資訊引數：帶佔位符的格式字串，以及 `greeting` 函式的值：

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

現在如果再次執行測試，將會看到更有價值的資訊：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

可以在測試輸出中看到所取得的確切的值，這會幫助我們除錯真正發生了什麼，而不是期望發生什麼。

### 使用 `should_panic` 檢查 panic

除了檢查返回值之外，檢查程式碼是否按照期望處理錯誤也是很重要的。例如，考慮第九章示例 9-13 建立的 `Guess` 型別。其他使用 `Guess` 的程式碼都是基於 `Guess` 例項僅有的值範圍在 1 到 100 的前提。可以編寫一個測試來確保建立一個超出範圍的值的 `Guess` 例項會 panic。

可以通過對函式增加另一個屬性 `should_panic` 來實現這些。這個屬性在函式中的程式碼 panic 時會通過，而在其中的程式碼沒有 panic 時失敗。

示例 11-8 展示了一個測試，檢查 `Guess::new` 在錯誤條件下是否如我們所料那樣。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">示例 11-8：測試會造成 `panic!` 的條件</span>

`#[should_panic]` 屬性位於 `#[test]` 之後，對應的測試函式之前。讓我們看看測試通過時它是什麼樣子：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

看起來不錯！現在在程式碼中引入 bug，移除 `new` 函式在值大於 100 時會 panic 的條件：

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

如果執行示例 11-8 的測試，它會失敗：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

這回並沒有得到非常有用的資訊，不過一旦我們觀察測試函式，會發現它標註了 `#[should_panic]`。這個錯誤意味著程式碼中測試函式 `Guess::new(200)` 並沒有產生 panic。

然而 `should_panic` 測試結果可能會非常含糊不清。`should_panic` 甚至在一些不是我們期望的原因而導致 panic 時也會通過。為了使 `should_panic` 測試結果更精確，我們可以給 `should_panic` 屬性增加一個可選的 `expected` 引數。測試工具會確保錯誤資訊中包含其提供的文本。例如，考慮示例 11-9 中修改過的 `Guess`，這裡 `new` 函式根據其值是過大還或者過小而提供不同的 panic 資訊：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">示例 11-9：一個錯誤資訊包含特定子字串的 `panic!` 條件的測試</span>

這個測試會通過，因為 `should_panic` 屬性中 `expected` 引數提供的值是 `Guess::new` 函式 panic 資訊的子串。我們可以指定期望的整個 panic 資訊，在這個例子中是 `Guess value must be less than or equal to 100, got 200` 。資訊的選擇取決於 panic 資訊有多獨特或動態，和你希望測試有多準確。在這個例子中，錯誤資訊的子字串足以確保函式在 `else if value > 100` 的情況下執行。

為了觀察帶有 `expected` 資訊的 `should_panic` 測試失敗時會發生什麼，讓我們再次引入一個 bug，將 `if value < 1` 和 `else if value > 100` 的程式碼塊對換：

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

這一次執行 `should_panic` 測試，它會失敗：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

失敗資訊表明測試確實如期望 panic 了，不過 panic 資訊中並沒有包含期望的資訊 `less than or equal to 100`。而我們得到的 panic 資訊是 `Guess value must be greater than or equal to 1, got 200.`。這樣就可以開始尋找 bug 在哪了！

### 在測試中使用 `Result<T, E>`

目前為止，我們編寫的測試在失敗時都會 panic。我們也可以使用 `Result<T, E>` 編寫測試！這是一個延伸自示例 11-1 的測試，使用 `Result<T, E>` 重寫，並在失敗時返回 `Err` 而非 panic：

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

現在 `it_works` 函式的返回值型別為 `Result<(), String>`。在函式體中，不同於呼叫 `assert_eq!` 宏，而是在測試通過時返回 `Ok(())`，在測試失敗時返回帶有 `String` 的 `Err`。

這樣編寫測試來返回 `Result<T, E>` 就可以在函式體中使用問號運算子，如此可以方便的編寫任何會返回 `Err` 變體的操作的測試。

不能對這些使用 `Result<T, E>` 的測試使用 `#[should_panic]` 註解。為了斷言一個操作返回 `Err` 成員，**不要**對 `Result<T, E>` 值使用問號表示式（`?`）。而是使用 `assert!(value.is_err())`。

現在你知道了幾種編寫測試的方法，讓我們看看執行測試時會發生什麼，以及可以用於 `cargo test` 的不同選項。

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#使用--運算子或-format-宏拼接字串
[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#除非特別指定否則忽略測試
[subset]: ch11-02-running-tests.html#通過名稱執行部分測試
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#控制測試如何執行
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#文件註釋作為測試
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
