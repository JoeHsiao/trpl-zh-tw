## 函式

[ch03-03-how-functions-work.md](https://github.com/rust-lang/book/blob/9cc190796f28505c7a9a9cacea42f50d895ff3bd/src/ch03-03-how-functions-work.md)

函式在 Rust 程式碼中非常普遍。你已經見過語言中最重要的函式之一：`main` 函式，它是很多程式的入口點。你也見過 `fn` 關鍵字，它用來宣告新函式。

Rust 程式碼中的函式名和變數名通常使用 *snake case* 風格。在 snake case 中，所有字母都使用小寫，並用下劃線分隔單詞。下面是一個包含函式定義示例的程式：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

我們在 Rust 中通過輸入 `fn` 後面跟著函式名和一對圓括號來定義函式。大括號告訴編譯器哪裡是函式體的開始和結尾。

可以使用函式名後跟圓括號來呼叫我們定義過的任意函式。因為程式中已定義 `another_function` 函式，所以可以在 `main` 函式中呼叫它。注意，原始碼中 `another_function` 定義在 `main` 函式 **之後**；也可以定義在之前。Rust 不關心函式定義所在的位置，只要函式被呼叫時出現在呼叫之處可見的作用域內就行。

讓我們新建一個叫做 *functions* 的二進位制專案，來進一步探索函式。把上面的 `another_function` 示例放到 *src/main.rs* 中並執行。你應該會看到如下輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

這些程式碼行會按照它們在 `main` 函式中出現的順序執行。首先列印 “Hello, world!”，然後呼叫 `another_function` 並列印它的訊息。

### 引數

我們可以定義帶有 **引數**（*parameters*）的函式，引數是特殊變數，是函式簽名的一部分。當函式帶有引數時，你就可以為這些引數提供具體的值。從嚴格意義上說，這些具體值叫做 *arguments*，不過在日常交流中，人們通常會把 *parameter* 和 *argument* 混用，用來指函式定義中的變數，或呼叫函式時傳入的具體值。

在這個版本的 `another_function` 中，我們增加了一個引數：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

嘗試執行程式，將會輸出如下內容：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`another_function` 的宣告中有一個命名為 `x` 的引數。`x` 的型別被指定為 `i32`。當我們將 `5` 傳給 `another_function` 時，`println!` 宏會把 `5` 放在格式字串中包含 `x` 的那對花括號的位置。

在函式簽名中，**必須** 宣告每個引數的型別。這是 Rust 設計中經過深思熟慮的一個決定：要求在函式定義裡提供型別註解，意味著編譯器幾乎不再需要你在程式碼的其他地方額外標明型別來表達意圖。而且，如果編譯器知道函式期望什麼型別，就能給出更有幫助的錯誤資訊。

當定義多個引數時，使用逗號分隔，像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

這個例子建立了一個名為 `print_labeled_measurement` 的函式，它有兩個引數。第一個引數名為 `value`，型別是 `i32`。第二個引數是 `unit_label` ，型別是 `char`。然後，該函式列印包含 `value` 和 `unit_label` 的文本。

嘗試執行程式碼。使用上面的例子替換當前 *functions* 專案的 *src/main.rs* 檔案，並用 `cargo run` 執行它：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

因為我們使用 `5` 作為 `value` 的值，`h` 作為 `unit_label` 的值來呼叫函式，所以程式輸出包含這些值。

### 語句和表示式

函式體由一系列語句組成，並且可以選擇以一個表示式結束。到目前為止，我們講過的函式還沒有包含結尾表示式，不過你已經見過出現在語句中的表示式了。由於 Rust 是一門基於表示式（expression-based）的語言，理解這一點非常重要。其他語言通常沒有這種區分，所以讓我們看看語句和表示式分別是什麼，以及它們的差異會如何影響函式體。

- **語句**（*Statements*）是執行一些操作但不返回值的指令。
- **表示式**（*Expressions*）計算併產生一個值。

讓我們看一些例子。

實際上，我們已經使用過語句和表示式。使用 `let` 關鍵字建立變數並繫結一個值是一個語句。在示例 3-1 中，`let y = 6;` 是一個語句。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">示例 3-1：包含一個語句的 `main` 函式定義</span>

函式定義本身也是語句，因此前面的整個例子本身也是一條語句。（不過，正如我們稍後會看到的，**呼叫**函式並不是語句。）

語句不返回值。因此，不能把 `let` 語句賦值給另一個變數，比如下面的例子嘗試做的，會產生一個錯誤：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

當執行這個程式時，會得到如下錯誤：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` 這條語句不會返回值，因此沒有什麼東西可以繫結到 `x` 上。這和一些其他語言不同，比如 C 和 Ruby，在那些語言裡，賦值語句會返回被賦的值。因此，在那些語言中你可以寫 `x = y = 6`，讓 `x` 和 `y` 都得到值 `6`；但 Rust 不是這樣。

表示式會計算出一個值，並且你將編寫的大部分 Rust 程式碼是由表示式組成的。考慮一個數學運算，比如 `5 + 6`，這是一個表示式並計算出值 `11`。表示式可以是語句的一部分：在示例 3-1 中，語句 `let y = 6;` 中的 `6` 是一個表示式，它計算出的值是 `6`。函式呼叫是一個表示式。巨集呼叫是一個表示式。用大括號建立的一個新的塊作用域也是一個表示式，例如：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

這個表示式：

```rust,ignore
{
    let x = 3;
    x + 1
}
```

是一個程式碼塊，它的值是 `4`。這個值作為 `let` 語句的一部分被繫結到 `y` 上。注意 `x + 1` 這一行在結尾沒有分號，與你見過的大部分程式碼行不同。表示式的結尾沒有分號。如果在表示式的結尾加上分號，它就變成了語句，而語句不會返回值。在接下來探索具有返回值的函式和表示式時要謹記這一點。

### 具有返回值的函式

函式可以把值返回給呼叫它的程式碼。我們不會給返回值命名，但必須在箭頭（`->`）後面宣告它的型別。在 Rust 中，函式的返回值等同於函式體中最後一個表示式的值。你也可以使用 `return` 關鍵字並指定一個值，從函式中提前返回；不過大多數函式都會隱式返回最後一個表示式的值。下面是一個帶有返回值的函式示例：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

在 `five` 函式中，沒有函式呼叫、沒有宏，甚至連 `let` 語句都沒有，只有單獨一個數字 `5`。這在 Rust 中是一個完全合法的函式。注意，我們也指定了函式返回值的型別，即 `-> i32`。試著執行這段程式碼；輸出應該如下所示：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`five` 函式的返回值是 `5`，所以返回型別是 `i32`。讓我們更仔細地看看這段程式碼。有兩個重要的點：首先，`let x = five();` 這一行表明我們用函式的返回值來初始化一個變數。因為 `five` 返回的是 `5`，所以這一行與下面的程式碼等價：

```rust
let x = 5;
```

其次，`five` 函式沒有引數，但定義了返回值型別，而且函式體裡只有單獨一個沒有分號的 `5`，因為這是一個表示式，而我們想返回它的值。

讓我們看看另一個例子：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

執行程式碼會打印出 `The value of x is: 6`。但如果在包含 `x + 1` 的行尾加上一個分號，把它從表示式變成語句，我們將看到一個錯誤。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

執行程式碼會產生一個錯誤，如下：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

主要的錯誤資訊 `mismatched types`（型別不匹配）揭示了這段程式碼的核心問題。`plus_one` 函式的定義表明它要返回一個 `i32`，但語句不會求值為某個值，語句對應的是單元型別 `()`。因此，函式實際上沒有返回值，這就與函式定義相矛盾，從而產生了錯誤。在這段輸出中，Rust 還提供了一條可能有助於修復該問題的提示：它建議刪除這個分號，而這麼做確實能修復錯誤。
