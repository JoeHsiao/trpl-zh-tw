## 可反駁性：模式是否可能匹配失敗

[ch19-02-refutability.md](https://github.com/rust-lang/book/blob/f78ab89d7545ac17780e6a367055cc089f4cd2ec/src/ch19-02-refutability.md)

模式分為兩種形式：可反駁（*refutable*）和不可反駁（*irrefutable*）。對傳入的任意可能值都能匹配的模式，稱為**不可反駁**模式。比如語句 `let x = 5;` 中的 `x` 就是一個例子，因為 `x` 可以匹配任何值，因此不可能匹配失敗。對某些可能值會匹配失敗的模式，稱為**可反駁**模式。比如表示式 `if let Some(x) = a_value` 中的 `Some(x)`；如果變數 `a_value` 中的值是 `None` 而不是 `Some`，那麼模式 `Some(x)` 就不會匹配。

函式引數、`let` 語句和 `for` 迴圈只能接受不可反駁模式，因為當值不匹配時，程式無法做出有意義的事情。`if let` 和 `while let` 表示式，以及 `let...else` 語句既接受可反駁模式，也接受不可反駁模式；不過編譯器會對其中的不可反駁模式發出警告，因為根據定義，這些結構本來就是為處理可能失敗的情況而設計的：條件判斷的意義就在於它可以根據成功或失敗執行不同的邏輯。

一般來說，你不必時刻擔心可反駁模式和不可反駁模式的區別；不過你確實需要熟悉“可反駁性”這個概念，這樣當你在錯誤資訊裡看到它時，就知道該如何應對。遇到這類情況時，你需要根據程式碼想要表達的行為，修改模式本身，或者修改與之搭配使用的語法結構。

讓我們來看一個例子：當我們試圖在 Rust 要求使用不可反駁模式的地方使用可反駁模式，以及反過來時，會發生什麼。示例 19-8 展示了一個 `let` 語句，不過我們給它寫了模式 `Some(x)`，這是一個可反駁模式。正如你可能已經猜到的，這段程式碼不會透過編譯。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

<span class="caption">示例 19-8: 嘗試在 `let` 中使用可反駁模式</span>

如果 `some_option_value` 的值是 `None`，它就無法匹配模式 `Some(x)`，這說明該模式是可反駁的。然而，`let` 語句只能接受不可反駁模式，因為當值是 `None` 時，這段程式碼並沒有什麼合法的後續操作可做。在編譯期，Rust 會抱怨我們試圖在要求不可反駁模式的地方使用可反駁模式：

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

因為我們沒有覆蓋模式 `Some(x)` 所對應的所有合法值（而且也不可能覆蓋全部），所以 Rust 理所當然地給出了編譯錯誤。

如果在某個需要不可反駁模式的地方卻有一個可反駁模式，我們可以透過修改使用該模式的程式碼來修復這個問題：不用 `let`，而改用 `let...else`。這樣一來，如果模式不匹配，大括號中的程式碼就會處理該值。示例 19-9 展示瞭如何修復示例 19-8 中的程式碼。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

<span class="caption">示例 19-9: 用 `let...else` 和一個帶可反駁模式的程式碼塊來代替 `let`</span>

我們給這段程式碼留出了一條退路！現在這段程式碼是完全合法的。不過這也意味著，我們不能在這裡使用不可反駁模式而不收到警告。如果我們給 `let...else` 一個總能匹配的模式，比如示例 19-10 中的 `x`，編譯器就會發出警告。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

<span class="caption">示例 19-10: 嘗試在 `let...else` 中使用不可反駁模式</span>

Rust 會抱怨說，在 `let...else` 中使用不可反駁模式沒有意義：

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

基於這個原因，`match` 分支必須使用可反駁模式，只有最後一個分支例外，它應該用一個不可反駁模式來匹配所有剩餘值。Rust 允許我們在一個只有單個分支的 `match` 中使用不可反駁模式，不過這種寫法並沒有太大用處，而且完全可以被更簡單的 `let` 語句替代。

既然現在你已經知道模式該用在什麼地方，以及可反駁模式和不可反駁模式之間的區別，接下來我們就來看看所有可以用來建立模式的語法。
