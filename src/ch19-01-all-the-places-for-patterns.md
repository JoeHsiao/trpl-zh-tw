## 所有可以使用模式的位置

[ch19-01-all-the-places-for-patterns.md](https://github.com/rust-lang/book/blob/f8f5adae0982518b8b37a6b8cbfced399d12fc3a/src/ch19-01-all-the-places-for-patterns.md)

模式會出現在 Rust 的很多地方，而你可能已經在不知不覺中用了很多次！本節會討論所有可以合法使用模式的位置。

### `match` 分支

正如第六章討論過的，我們會在 `match` 表示式的分支中使用模式。從形式上看，`match` 表示式由 `match` 關鍵字、要匹配的值，以及一個或多個 `match` 分支組成；這些分支包含一個模式，以及當值匹配該分支模式時要執行的表示式，就像這樣：

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

例如，下面是示例 6-5 中那個匹配變數 `x` 內 `Option<i32>` 值的 `match` 表示式：

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

這個 `match` 表示式中的模式，就是每個箭頭左邊的 `None` 和 `Some(i)`。

`match` 表示式有一個要求，那就是它必須是**窮盡的**（*exhaustive*）：`match` 表示式中值的所有可能情況都必須被覆蓋到。確保覆蓋所有可能性的一種方式，是讓最後一個分支使用“捕獲所有”的模式；例如，一個可以匹配任意值的變數名永遠不會失敗，因此它能夠覆蓋所有剩餘情況。

特定的模式 `_` 可以匹配任何東西，但它永遠不會繫結到變數上，因此常被用於最後一個 `match` 分支。當你想忽略某個未指定的值時，`_` 模式會非常有用。稍後在本章的 [“忽略模式中的值”][ignoring-values-in-a-pattern] 一節中，我們會更詳細地討論 `_` 模式。

### `let` 語句

在本章之前，我們只明確討論過在 `match` 和 `if let` 中使用模式，但實際上，我們也在其他地方使用過模式，包括 `let` 語句。例如，來看這個簡單直接的變數賦值：

```rust
let x = 5;
```

每次你寫出像這樣的 `let` 語句時，其實都在使用模式，只是你可能沒有意識到！更正式地說，`let` 語句看起來像這樣：

```text
let PATTERN = EXPRESSION;
```

在像 `let x = 5;` 這樣的語句中，位於 `PATTERN` 位置的變數名只是模式的一種特別簡單的形式。Rust 會拿表示式與模式進行比較，並將它找到的任何名字賦值。所以，在 `let x = 5;` 這個例子中，`x` 是一個模式，表示“把這裡匹配到的內容繫結到變數 `x`”。因為名字 `x` 本身就是整個模式，所以這個模式實際上等於“無論值是什麼，都把它繫結到變數 `x`”。

為了更清楚地看出 `let` 的模式匹配這一面，來看示例 19-1，它在 `let` 中使用模式來解構一個元組。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

<span class="caption">示例 19-1: 使用模式解構元組，並一次建立三個變數</span>

這裡，我們讓一個元組去匹配一個模式。Rust 會比較值 `(1, 2, 3)` 和模式 `(x, y, z)`，並發現該值和模式匹配，也就是說，兩邊的元素個數相同；於是 Rust 將 `1` 繫結到 `x`，將 `2` 繫結到 `y`，將 `3` 繫結到 `z`。你可以把這個元組模式看作其中嵌套了三個獨立的變數模式。

如果模式中的元素數量與元組中的元素數量不一致，那麼整體型別就不會匹配，編譯器也會報錯。例如，示例 19-2 展示了試圖用兩個變數去解構一個三個元素的元組，這樣是行不通的。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

<span class="caption">示例 19-2: 錯誤地構造了一個模式，其變數數量與元組中的元素數量不匹配</span>

嘗試編譯這段程式碼會得到如下型別錯誤：

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

要修復這個錯誤，可以像本章後面 [“忽略模式中的值”][ignoring-values-in-a-pattern] 一節中會看到的那樣，使用 `_` 或 `..` 來忽略元組中的一個或多個值。如果問題在於模式裡的變數太多，那麼解決辦法就是刪掉一些變數，使變數數量與元組中的元素數量相等。

### 條件 `if let` 表示式

在第六章中，我們討論過如何使用 `if let` 表示式，它主要是用來簡寫只匹配一種情況的 `match`。此外，`if let` 還可以有一個對應的 `else`，在 `if let` 中的模式不匹配時執行其中的程式碼。

示例 19-3 表明，我們也可以混合使用 `if let`、`else if`、`else if let` 和 `else`。與 `match` 表示式相比，這樣做給了我們更多靈活性；在 `match` 中，我們只能表達“把一個值與若干模式比較”。另外，Rust 也不要求一連串 `if let`、`else if` 和 `else if let` 分支中的條件彼此相關。

示例 19-3 中的程式碼會根據一系列條件檢查來決定背景色應該是什麼。為了舉例，我們建立了幾個帶硬編碼值的變數；而在真實程式裡，這些值可能來自使用者輸入。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

<span class="caption">示例 19-3: 混合使用 `if let`、`else if`、`else if let` 和 `else`</span>

如果使用者指定了喜歡的顏色，就用該顏色作為背景色。如果沒有指定喜歡的顏色，而且今天是星期二，那麼背景色就是綠色。否則，如果使用者把年齡作為字串提供出來，並且我們能夠成功將其解析為數字，那麼背景色就會根據該數字的值變成紫色或橙色。如果這些條件都不滿足，背景色就是藍色。

這種條件結構讓我們能夠支援複雜的需求。使用這裡的硬編碼值，這個例子會列印 `Using purple as the background color`。

你還可以看到，`if let` 也能像 `match` 分支一樣引入新變數，並遮蔽已有變數：`if let Ok(age) = age` 這一行引入了一個新的 `age` 變數，它儲存 `Ok` 變體中的值，並遮蔽了原來的 `age` 變數。這意味著我們必須把 `if age > 30` 這個條件放在該程式碼塊內部：不能把這兩個條件合併成 `if let Ok(age) = age && age > 30`。因為我們想拿來和 `30` 比較的那個新 `age`，在由大括號開啟的新作用域開始之前是無效的。

使用 `if let` 表示式的缺點是，編譯器不會像檢查 `match` 那樣檢查它的窮盡性。如果我們省略最後一個 `else` 塊，從而漏掉對某些情況的處理，編譯器也不會提醒我們這裡可能存在邏輯錯誤。

### `while let` 條件迴圈

與 `if let` 在結構上類似的是 `while let` 條件迴圈，它允許 `while` 迴圈在模式持續匹配期間一直執行。示例 19-4 展示了一個 `while let` 迴圈，它等待執行緒之間傳送的訊息，不過這裡檢查的是 `Result`，而不是 `Option`。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

<span class="caption">示例 19-4: 使用 `while let` 迴圈，只要 `rx.recv()` 返回 `Ok` 就列印值</span>

這個例子會列印 `1`、`2`，然後是 `3`。`recv` 方法會從通道的接收端取出第一條訊息，並返回 `Ok(value)`。在第十六章第一次見到 `recv` 時，我們是直接對錯誤呼叫 `unwrap`，或者把它當作迭代器配合 `for` 迴圈使用。不過正如示例 19-4 所示，我們也可以使用 `while let`，因為只要傳送端還存在，且不斷有訊息到達，`recv` 每次都會返回 `Ok`；當傳送端斷開連線後，它就會返回 `Err`。

### `for` 迴圈

在 `for` 迴圈中，緊跟在 `for` 關鍵字後面的值就是一個模式。例如，在 `for x in y` 中，`x` 就是那個模式。示例 19-5 展示瞭如何在 `for` 迴圈中使用模式來解構一個元組。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

<span class="caption">示例 19-5: 在 `for` 迴圈中使用模式來解構一個元組</span>

示例 19-5 中的程式碼會打印出如下內容：

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

我們使用 `enumerate` 方法適配了一個迭代器，使它產生由“值及其索引”組成的元組。它產生的第一個值是元組 `(0, 'a')`。當該值與模式 `(index, value)` 匹配時，`index` 會是 `0`，`value` 會是 `'a'`，於是打印出輸出的第一行。

### 函式引數

函式引數也可以是模式。示例 19-6 中的程式碼聲明瞭一個名為 `foo` 的函式，它接收一個名為 `x`、型別為 `i32` 的引數；到現在為止，這種寫法應該已經很熟悉了。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

<span class="caption">示例 19-6: 在引數中使用模式的函式簽名</span>

`x` 這一部分就是一個模式！就像我們在 `let` 中做的那樣，也可以在函式引數中用模式匹配一個元組。示例 19-7 展示了在向函式傳參時如何拆開元組中的值。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

<span class="caption">示例 19-7: 一個在引數中解構元組的函式</span>

這段程式碼會列印 `Current location: (3, 5)`。值 `&(3, 5)` 會匹配模式 `&(x, y)`，因此 `x` 的值是 `3`，`y` 的值是 `5`。

由於閉包與函式類似，正如第十三章所討論的那樣，我們也可以像在函式引數列表中那樣，在閉包引數列表中使用模式。

現在我們已經看過很多使用模式的方式了，不過模式在各處並不總是以同樣的方式工作；在某些位置，模式必須是不可反駁的（*irrefutable*），這意味著它們必須匹配所提供的任何值；而在另一些位置，它們則可以是可反駁的（*refutable*）。接下來讓我們討論這兩個概念。

[ignoring-values-in-a-pattern]:
ch19-03-pattern-syntax.html#忽略模式中的值
