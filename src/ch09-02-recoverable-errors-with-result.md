## 用 `Result` 處理可恢復的錯誤

[ch09-02-recoverable-errors-with-result.md](https://github.com/rust-lang/book/blob/13e27c4a35705c4bd473bd90a3d3a8f87ef9ae58/src/ch09-02-recoverable-errors-with-result.md)

大部分錯誤並沒有嚴重到需要程式完全停止執行。有時函式失敗的原因很容易理解並加以處理。例如，如果因為開啟一個並不存在的檔案而失敗，此時我們可能想要建立這個檔案，而不是終止程序。

回憶一下第二章 [“使用 `Result` 型別來處理潛在的錯誤”][handle_failure] 部分中的那個 `Result` 列舉，它定義有如下兩個變體，`Ok` 和 `Err`：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` 和 `E` 是泛型型別引數；第十章會詳細介紹泛型。現在你需要知道的就是 `T` 代表成功時返回的 `Ok` 變體中的資料的型別，而 `E` 代表失敗時返回的 `Err` 變體中的錯誤的型別。因為 `Result` 有這些泛型型別引數，我們可以將 `Result` 型別和標準庫中為其定義的函式用於很多不同的場景，這些情況中需要返回的成功值和失敗值可能會各不相同。

讓我們呼叫一個返回 `Result` 的函式，因為它可能會失敗：如示例 9-3 所示嘗試開啟一個檔案：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">示例 9-3：開啟檔案</span>

`File::open` 的返回值是 `Result<T, E>`。泛型引數 `T` 會被 `File::open` 的實現放入成功返回值的型別 `std::fs::File`，這是一個檔案控制代碼。錯誤返回值使用的 `E` 的型別是 `std::io::Error`。這些返回型別意味著 `File::open` 呼叫可能成功並返回一個可以讀寫的檔案控制代碼。這個函式呼叫也可能會失敗：例如，也許檔案不存在，或者可能沒有許可權訪問這個檔案。`File::open` 函式需要一個方法在告訴我們成功與否的同時返回檔案控制代碼或者錯誤資訊。這些資訊正好是 `Result` 列舉所代表的。

當 `File::open` 成功時，`greeting_file_result` 變數將會是一個包含檔案控制代碼的 `Ok` 例項。當失敗時，`greeting_file_result` 變數將會是一個包含了更多關於發生了何種錯誤的資訊的 `Err` 例項。

我們需要在示例 9-3 的程式碼中增加根據 `File::open` 返回值進行不同處理的邏輯。示例 9-4 展示了一個使用基本工具處理 `Result` 的例子，第六章學習過的 `match` 表示式。

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">示例 9-4：使用 `match` 表示式處理可能會返回的 `Result` 變體</span>

注意與 `Option` 列舉一樣，`Result` 列舉和其變體也被匯入到了 prelude 中，所以就不需要在 `match` 分支中的 `Ok` 和 `Err` 之前指定 `Result::`。

這裡我們告訴 Rust 當結果是 `Ok` 時，返回 `Ok` 變體中的 `file` 值，然後將這個檔案控制代碼賦值給變數 `greeting_file`。`match` 之後，我們可以利用這個檔案控制代碼來進行讀寫。

`match` 的另一個分支處理從 `File::open` 得到 `Err` 值的情況。在這種情況下，我們選擇呼叫 `panic!` 宏。如果當前目錄沒有一個叫做 *hello.txt* 的檔案，當執行這段程式碼時會看到如下來自 `panic!` 宏的輸出：

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

一如既往，此輸出準確地告訴了我們到底出了什麼錯。

### 匹配不同的錯誤

示例 9-4 中的程式碼不管 `File::open` 是因為什麼原因失敗都會 `panic!`。我們真正希望的是對不同的錯誤原因採取不同的行為：如果 `File::open `因為檔案不存在而失敗，我們希望建立這個檔案並返回新檔案的控制代碼。如果 `File::open` 因為任何其他原因失敗 -- 例如沒有開啟檔案的許可權 -- 我們仍然希望像示例 9-4 那樣 `panic!`。為此，我們在示例 9-5 中添加了一個內部 `match` 表示式，如下所示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">示例 9-5：使用不同的方式處理不同型別的錯誤</span>

`File::open` 返回的 `Err` 變體中的值型別 `io::Error`，它是一個標準庫中提供的結構體。這個結構體有一個返回 `io::ErrorKind` 值的 `kind` 方法可供呼叫。`io::ErrorKind` 是一個標準庫提供的列舉，它的變體對應 `io` 操作可能導致的不同錯誤型別。我們感興趣的變體是 `ErrorKind::NotFound`，它代表嘗試開啟的檔案並不存在。這樣，`match` 就匹配完 `greeting_file_result` 了，不過對於 `error.kind()` 還有一個內層 `match`。

我們希望在內層 `match` 中檢查的條件是 `error.kind()` 的返回值是否為 `ErrorKind`的 `NotFound` 變體。如果是，則透過 `File::create` 嘗試建立該檔案。然而因為 `File::create` 也可能會失敗，還需要在內層 `match` 表示式中增加了第二個分支。當檔案不能被建立，會打印出一個不同的錯誤資訊。外層 `match` 的最後一個分支保持不變，這樣對任何除了檔案不存在的錯誤會使程式 panic。

> #### 使用 `match` 處理 `Result<T, E>` 的替代方案
>
> 這裡有好多 `match`！`match` 確實很強大，不過也非常的原始。第十三章我們會介紹閉包（closure），它會和定義在 `Result<T, E>` 中的很多方法一起使用。在處理程式碼中的 `Result<T, E>` 值時，使用這些方法往往比直接寫 `match` 更簡潔。
>
> 例如，這是另一個編寫與示例 9-5 邏輯相同但是使用閉包和 `unwrap_or_else` 方法的例子：
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {error:?}");
>             })
>         } else {
>             panic!("Problem opening the file: {error:?}");
>         }
>     });
> }
> ```
>
> 雖然這段程式碼有著如示例 9-5 一樣的行為，但並沒有包含任何 `match` 表示式且更容易閱讀。在閱讀完第十三章後再回到這個例子，並檢視標準庫文件 `unwrap_or_else` 方法都做了什麼操作。在處理錯誤時，還有很多這類方法可以消除大量巢狀的 `match` 表示式。

<a id="shortcuts-for-panic-on-error-unwrap-and-expect"></a>

#### 失敗時 panic 的快捷方式

`match` 能夠勝任它的工作，不過它可能有點冗長並且不總是能很好的表明其意圖。`Result<T, E>` 型別定義了很多輔助方法來處理各種更為特定的任務。`unwrap` 方法是一個快捷方式，其內部實現與我們在 Listing 9-4 中編寫的 `match` 表示式相同。如果 `Result` 值是變體 `Ok`，`unwrap` 會返回 `Ok` 中的值。如果 `Result` 是變體 `Err`，`unwrap` 會為我們呼叫 `panic!`。這裡是一個實踐 `unwrap` 的例子：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

如果呼叫這段程式碼時不存在 *hello.txt* 檔案，我們將會看到一個 `unwrap` 呼叫 `panic!` 時提供的錯誤資訊：

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

同樣，`expect` 方法也允許我們自定義 `panic!` 的錯誤資訊。使用 `expect` 而不是 `unwrap` 並提供一個好的錯誤資訊可以表明你的意圖並更易於追蹤 panic 的根源。`expect` 的語法看起來像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

`expect` 與 `unwrap` 的使用方式一樣：返回檔案控制代碼或呼叫 `panic!` 宏。`expect` 在呼叫 `panic!` 時使用的錯誤資訊將是我們傳遞給 `expect` 的引數，而不像 `unwrap` 那樣使用預設的 `panic!` 資訊。它看起來像這樣：

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

在生產級別的程式碼中，大部分 Rustaceans 選擇 `expect` 而不是 `unwrap` 並提供更多關於為何操作期望是一直成功的上下文。如此如果該假設真的被證明是錯的，你也有更多的資訊來用於除錯。

### 傳播錯誤

當函式的實現中呼叫了可能會失敗的操作時，除了在這個函式中處理錯誤外，還可以選擇讓呼叫者知道這個錯誤並決定該如何處理。這被稱為**傳播**（*propagating*）錯誤，這樣能更好的控制程式碼呼叫，因為比起你程式碼所擁有的上下文，呼叫者可能擁有更多資訊或邏輯來決定應該如何處理錯誤。

例如，示例 9-6 展示了一個從檔案中讀取使用者名稱的函式。如果檔案不存在或不能讀取，這個函式會將這些錯誤返回給呼叫它的程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">示例 9-6：一個函式使用 `match` 將錯誤返回給程式碼呼叫者</span>

這個函式可以編寫成更加簡短的形式，不過我們以大量手動處理開始以便探索錯誤處理；在最後我們會展示更簡潔的形式。讓我們看看函式的返回值：`Result<String, io::Error>`。這意味著函式返回一個 `Result<T, E>` 型別的值，其中泛型引數 `T` 的具體型別是 `String`，而 `E` 的具體型別是 `io::Error`。

如果這個函式沒有出任何錯誤成功返回，函式的呼叫者會收到一個包含 `String` 的 `Ok` 值 —— 函式從檔案中讀取到的使用者名稱。如果函式遇到任何錯誤，函式的呼叫者會收到一個 `Err` 值，它儲存了一個包含更多這個問題相關資訊的 `io::Error` 例項。這裡選擇 `io::Error` 作為函式的返回值是因為它正好是函式體中那兩個可能會失敗的操作的錯誤返回值：`File::open` 函式和 `read_to_string` 方法。

函式體以呼叫 `File::open` 函式開始。接著使用 `match` 處理返回值 `Result`，類似示例 9-4，如果 `File::open` 成功了，模式變數 `file` 中的檔案控制代碼就變成了可變變數 `username_file` 中的值，接著函式繼續執行。在 `Err` 的情況下，我們沒有呼叫 `panic!`，而是使用 `return` 關鍵字提前結束整個函式，並將來自 `File::open` 的錯誤值（現在在模式變數 `e` 中）作為函式的錯誤值傳回給呼叫者。

所以，如果在 `username_file` 中有一個檔案控制代碼，該函式隨後會在變數 `username` 中建立一個新的 `String` 並呼叫檔案控制代碼 `username_file` 上的 `read_to_string` 方法，以將檔案的內容讀入 `username`。`read_to_string` 方法也返回一個 `Result`，因為它可能會失敗，哪怕是 `File::open` 已經成功了。因此，我們需要另一個 `match` 來處理這個 `Result`：如果 `read_to_string` 執行成功，那麼這個函式也就成功了，我們將從檔案中讀取的使用者名稱返回，此時使用者名稱位於被封裝進 `Ok` 的 `username` 中。如果 `read_to_string` 執行失敗，則像之前處理 `File::open` 的返回值的 `match` 那樣返回錯誤值。然而，我們無需顯式寫出 `return`，因為這是函式的最後一個表示式。

呼叫這個函式的程式碼最終會得到一個包含使用者名稱的 `Ok` 值，或者一個包含 `io::Error` 的 `Err` 值。我們無從得知呼叫者會如何處理這些值。例如，如果他們得到了一個 `Err` 值，他們可能會選擇 `panic!` 並使程式崩潰、使用一個預設的使用者名稱或者從檔案之外的地方尋找使用者名稱。我們沒有足夠的資訊知曉呼叫者具體會如何嘗試，所以將所有的成功或失敗資訊向上傳播，讓他們選擇合適的處理方法。

這種傳播錯誤的模式在 Rust 中太常見了，因此 Rust 提供了問號運算子 `?` 來簡化這一過程。

<a id="a-shortcut-for-propagating-errors-the--operator"></a>

#### `?` 運算子快捷方式

示例 9-7 展示了一個 `read_username_from_file` 的實現，它實現了與示例 9-6 中的程式碼相同的功能，不過這個實現使用了 `?` 運算子：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">示例 9-7：一個使用 `?` 運算子向呼叫者返回錯誤的函式</span>

放在 `Result` 值後面的 `?`，其定義的工作方式與我們在示例 9-6 中編寫的處理 `Result` 值的 `match` 表示式幾乎完全相同。如果 `Result` 的值是 `Ok`，這個表示式就會返回 `Ok` 中的值，程式繼續執行。如果值是 `Err`，`Err` 就會像使用了 `return` 關鍵字一樣，作為整個函式的返回值提前返回，這樣錯誤值就被傳播給了呼叫者。

示例 9-6 中的 `match` 表示式和 `?` 運算子還有一點不同：被 `?` 作用的錯誤值會經過 `from` 函式。這個函式定義在標準庫的 `From` trait 中，用於把一種型別的值轉換成另一種型別。當 `?` 運算子呼叫 `from` 函式時，接收到的錯誤型別會被轉換成當前函式返回型別裡定義的錯誤型別。當一個函式用單一錯誤型別來表示它所有可能的失敗方式時，這會非常有用，即使函式內部的不同部分可能會因為很多不同的原因而失敗。

例如，我們可以將示例 9-7 中的 `read_username_from_file` 函式修改為返回一個自定義的 `OurError` 錯誤型別。如果我們也定義了 `impl From<io::Error> for OurError` 來從 `io::Error` 構造一個 `OurError` 例項，那麼 `read_username_from_file` 函式體中的 `?` 運算子呼叫會呼叫 `from` 並轉換錯誤而無需在函式中增加任何額外的程式碼。

在示例 9-7 的上下文中，`File::open` 呼叫結尾的 `?` 會將 `Ok` 中的值返回給變數 `username_file`。如果發生了錯誤，`?` 運算子會使整個函式提前返回並將任何 `Err` 值返回給呼叫程式碼。同理也適用於 `read_to_string` 呼叫結尾的 `?`。

`?` 運算子消除了大量樣板程式碼並使得函式的實現更簡單。我們甚至可以在 `?` 之後直接使用鏈式方法呼叫來進一步簡化程式碼，如示例 9-8 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">示例 9-8：問號運算子之後的鏈式方法呼叫</span>

在 `username` 中建立新的 `String` 被放到了函式開頭；這一部分沒有變化。我們對 `File::open("hello.txt")?` 的結果直接鏈式呼叫了 `read_to_string`，而不再建立變數 `username_file`。仍然需要 `read_to_string` 呼叫結尾的 `?`，而且當 `File::open` 和 `read_to_string` 都成功沒有失敗時返回包含使用者名稱 `username` 的 `Ok` 值，而不是返回錯誤。其功能再一次與示例 9-6 和示例 9-7 保持一致，不過這是一個與眾不同且更符合工程學（ergonomic）的寫法。

示例 9-9 展示了一個使用 `fs::read_to_string` 的更為簡短的寫法：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">示例 9-9: 使用 `fs::read_to_string` 而不是開啟後讀取檔案</span>

將檔案讀取到一個字串是相當常見的操作，所以標準庫提供了名為 `fs::read_to_string` 的函式，它會開啟檔案、新建一個 `String`、讀取檔案的內容，並將內容放入 `String`，接著返回它。當然，這樣做就沒有展示所有這些錯誤處理的機會了，所以我們最初就選擇了艱苦的道路。

<a id="where-the--operator-can-be-used"></a>

### 哪裡可以使用 `?` 運算子

`?` 運算子只能被用於返回值與 `?` 作用的值相相容的函式。因為 `?` 運算子被定義為從函式中提早返回一個值，這與示例 9-6 中的 `match` 表示式有著完全相同的工作方式。示例 9-6 中 `match` 作用於一個 `Result` 值，提早返回的分支返回了一個 `Err(e)` 值。函式的返回值必須是 `Result` 才能與這個 `return` 相相容。

在示例 9-10 中，讓我們看看在返回值不相容的 `main` 函式中使用 `?` 運算子會得到什麼錯誤：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">示例 9-10: 嘗試在返回 `()` 的 `main` 函式中使用 `?` 的程式碼不能編譯</span>

這段程式碼開啟一個檔案，這可能會失敗。`?` 運算子作用於 `File::open` 返回的 `Result` 值，不過 `main` 函式的返回型別是 `()` 而不是 `Result`。當編譯這些程式碼，會得到如下錯誤資訊：

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

這個錯誤指出只能在返回 `Result`、`Option` 或者其它實現了 `FromResidual` 的型別的函式中使用 `?` 運算子。

為了修復這個錯誤，有兩個選擇。一個是，如果沒有限制的話將函式的返回值改為與你在 `?` 運算子所作用的值相容的型別。另一個是使用 `match` 或者 `Result<T, E>` 型別的方法，以適當的方式處理 `Result<T, E>`。

錯誤資訊也提到 `?` 也可用於 `Option<T>` 值。如同對 `Result` 使用 `?` 一樣，只能在返回 `Option` 的函式中對 `Option` 使用 `?`。在 `Option<T>` 上呼叫 `?` 運算子的行為與 `Result<T, E>` 類似：如果值是 `None`，此時 `None` 會從函式中提前返回。如果值是 `Some`，`Some` 中的值作為表示式的返回值同時函式繼續。示例 9-11 中有一個從給定文字中返回第一行最後一個字元的函式的例子：

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">示例 9-11: 在 `Option<T>` 值上使用 `?` 運算子</span>

這個函式返回 `Option<char>` 因為它可能會在這個位置找到一個字元，也可能沒有字元。這段程式碼獲取 `text` 字串 slice 作為引數並呼叫其 `lines` 方法，這會返回一個字串中每一行的迭代器。因為函式希望檢查第一行，所以呼叫了迭代器 `next` 來獲取迭代器中第一個值。如果 `text` 是空字串，`next` 呼叫會返回 `None`，此時我們可以使用 `?` 來停止並從 `last_char_of_first_line` 返回 `None`。如果 `text` 不是空字串，`next` 會返回一個包含 `text` 中第一行的字串 slice 的 `Some` 值。

`?` 會提取這個字串 slice，然後可以在字串 slice 上呼叫 `chars` 來獲取字元的迭代器。我們感興趣的是第一行的最後一個字元，所以可以呼叫 `last` 來返回迭代器的最後一項。這是一個 `Option`，因為有可能第一行是一個空字串；例如 `text` 以一個空行開頭而後面的行有文字，像是 `"\nhi"`。不過，如果第一行有最後一個字元，它會返回在一個 `Some` 變體中。`?` 運算子作用於其中給了我們一個簡潔的表達這種邏輯的方式。如果我們不能在 `Option` 上使用 `?` 運算子，則不得不使用更多的方法呼叫或者 `match` 表示式來實現這些邏輯。

注意你可以在返回 `Result` 的函式中對 `Result` 使用 `?` 運算子，可以在返回 `Option` 的函式中對 `Option` 使用 `?` 運算子，但是不可以混合搭配。`?` 運算子不會自動將 `Result` 轉化為 `Option`，反之亦然；在這些情況下，可以使用類似 `Result` 的 `ok` 方法或者 `Option` 的 `ok_or` 方法來顯式轉換。

目前為止，我們所使用的所有 `main` 函式都返回 `()`。`main` 函式是特殊的因為它是可執行程式的入口點和退出點，為了使程式能正常工作，其可以返回的型別是有限制的。

幸運的是 `main` 函式也可以返回 `Result<(), E>`，示例 9-12 中的程式碼來自示例 9-10 不過修改了 `main` 的返回值為 `Result<(), Box<dyn Error>>` 並在結尾增加了一個 `Ok(())` 作為返回值。這段程式碼就可以編譯了。

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">示例 9-12: 修改 `main` 返回 `Result<(), E>` 允許對 `Result` 值使用 `?` 運算子</span>

`Box<dyn Error>` 型別是一個**trait 物件**（*trait object*），第十八章的[“使用 trait object 來抽象出共享行為”][trait-objects]部分會介紹它。現在可以把 `Box<dyn Error>` 理解為“任何型別的錯誤”。在返回錯誤型別 `Box<dyn Error>` 的 `main` 函式中，對 `Result` 使用 `?` 是被允許的，因為它允許任何 `Err` 值提前返回。即便 `main` 函式體現在只會返回 `std::io::Error` 錯誤型別，透過指定 `Box<dyn Error>`，這個簽名仍然是正確的；即使以後在 `main` 函式體中加入返回其他錯誤型別的程式碼，這個函式簽名依然保持正確。

當 `main` 函式返回 `Result<(), E>`，如果 `main` 返回 `Ok(())` 可執行程式會以 `0` 值退出，而如果 `main` 返回 `Err` 值則會以非零值退出；成功退出的程式會返回整數 `0`，執行錯誤的程式會返回非 `0` 的整數。Rust 也會從二進位制程式中返回與這個慣例相相容的整數。

`main` 函式也可以返回任何實現了 [`std::process::Termination` trait][termination] 的型別，它包含了一個返回 `ExitCode` 的 `report` 函式。請查閱標準庫文件瞭解更多為自定義型別實現 `Termination` trait 的細節。

現在我們討論過了呼叫 `panic!` 或返回 `Result` 的細節，讓我們回到在不同場景下如何決定使用哪種方式的問題。

[handle_failure]: ch02-00-guessing-game-tutorial.html#使用-result-型別來處理潛在的錯誤
[trait-objects]: ch18-02-trait-objects.html#使用-trait-object-來抽象出共享行為
[termination]: https://doc.rust-lang.org/std/process/trait.Termination.html
