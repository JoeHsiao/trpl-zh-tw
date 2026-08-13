## `if let` 和 `let else` 簡潔控制流

[ch06-03-if-let.md](https://github.com/rust-lang/book/blob/f78ab89d7545ac17780e6a367055cc089f4cd2ec/src/ch06-03-if-let.md)

`if let` 語法讓我們以一種不那麼冗長的方式結合 `if` 和 `let`，來處理只匹配一個模式的值而忽略其他模式的情況。考慮示例 6-6 中的程式，它匹配一個 `config_max` 變數中的 `Option<u8>` 值並只希望當值為 `Some` 變體時執行程式碼：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">示例 6-6：`match` 只關心當值為 `Some` 時執行程式碼</span>

如果值是 `Some`，我們希望打印出 `Some` 變體中的值，這個值被繫結到模式中的 `max` 變數裡。對於 `None` 值我們不希望做任何操作。為了滿足 `match` 表示式（窮盡性）的要求，必須在處理完這唯一的變體後加上 `_ => ()`，這樣也要增加很多繁瑣的樣板程式碼。

不過我們可以使用 `if let` 這種簡潔的方式編寫。如下程式碼與示例 6-6 中的 `match` 行為一致：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` 語法獲取透過等號分隔的一個模式和一個表示式。它的工作方式與 `match` 相同，這裡的表示式對應 `match` 而模式則對應第一個分支。在這個例子中，模式是 `Some(max)`，`max` 繫結為 `Some` 中的值。接著可以在 `if let` 程式碼塊中使用 `max` 了，就跟在對應的 `match` 分支中一樣。只有當值匹配該模式時，`if let` 塊中的程式碼才會執行。

使用 `if let` 意味著更少的輸入、更少的縮排，也更少的樣板程式碼。然而，這樣也會失去 `match` 所強制的窮盡性檢查，也就無法確保你沒有遺漏某些情況。`match` 和 `if let` 之間的選擇依賴特定的環境以及增加簡潔度和失去窮盡性檢查的權衡取捨。

換句話說，可以認為 `if let` 是 `match` 的一個語法糖，它當值匹配某一模式時執行程式碼而忽略所有其他值。

可以在 `if let` 中包含一個 `else`。`else` 塊中的程式碼與 `match` 表示式中的 `_` 分支塊中的程式碼相同，這樣的 `match` 表示式就等同於 `if let` 和 `else`。回憶一下示例 6-4 中 `Coin` 列舉的定義，其 `Quarter` 變體也包含一個 `UsState` 值。如果想要計數所有不是 25 美分的硬幣的同時也報告 25 美分硬幣所屬的州，可以使用這樣一個 `match` 表示式：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

或者可以使用這樣的 `if let` 和 `else` 表示式：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## 使用 `let...else` 來保持在 “愉快路徑”（“Happy Path”）

在實際程式設計中，一個常見的場景是：如果某個值存在，就對它做一些操作；如果不存在，就返回一個預設值。還是繼續用處理 `UsState` 的硬幣作為例子。假設我們要寫點有趣的邏輯，它依賴於硬幣所代表的州成立了多久。我們就可以在 `UsState` 上定義一個方法，用來檢查州的“年齡”：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

接著我們可能使用 `if let` 來匹配硬幣的型別，在條件程式碼中引入一個 `state`，如示例 6-7 所示。

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

<figcaption>示例 6-7：使用巢狀在 `if let` 中的條件來檢查一個州在 1900 年是否存在</figcaption>

</figure>

這樣固然可以完成任務，不過這將工作推進了 `if let` 語句中，如果需要完成的工作更為複雜，則可能難以追蹤頂層分支是如何關聯的。我們也可以利用這個表示式要麼從 `if let` 中生成一個 `state` 要麼提前返回的優勢，如示例 6-8 所示。（使用 `match` 也可以實現類似效果。）

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

<figcaption>示例 6-8：使用 `if let` 來產生一個值或提前返回</figcaption>

</figure>

不過這樣寫在某種程度上會讓人覺得有些繁瑣！`if let` 的一個分支產生一個值，而另一個分支則直接從函式中返回。

為了使這個通用模式更容易表達，Rust 提供了 `let...else`。`let...else` 語法左側是一個模式，右側是一個表示式，非常類似於 `if let`，不過它沒有 `if` 分支，只有 `else` 分支。如果模式匹配，它會將匹配到的值繫結到外層作用域。如果模式**不**匹配，程式流會指向 `else` 分支，它必須從函式返回。

在示例 6-9 中，可以看到當在示例 6-8 中的 `if let` 替換為 `let...else` 時看起來如何。

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

<figcaption>示例 6-9：使用 `let...else` 來明確函式的流向</figcaption>

</figure>

注意，這種寫法能讓函式主體沿著“愉快路徑”（“Happy Path”）繼續向下，而不必像 `if let` 那樣讓兩個分支具有明顯不同的控制流。

如果你的程式遇到一個使用 `match` 表達起來過於冗長的邏輯，記住 `if let` 和 `let...else` 也在你的 Rust 工具箱中。

## 總結

現在我們涉及到了如何使用列舉來建立有一系列可列舉值的自定義型別。我們也展示了標準庫的 `Option<T>` 型別是如何幫助你利用型別系統來避免出錯的。當列舉值包含資料時，你可以根據需要處理多少情況來選擇使用 `match` 或 `if let` 來獲取並使用這些值。

你的 Rust 程式現在能夠使用結構體和列舉在自己的作用域內表現其內容了。在你的 API 中使用自定義型別保證了型別安全：編譯器會確保你的函式只會得到它期望的型別的值。

為了向你的使用者提供一個組織良好的 API，它使用起來很直觀並且只向用戶暴露他們確實需要的部分，那麼現在就讓我們轉向 Rust 的模組系統吧。
