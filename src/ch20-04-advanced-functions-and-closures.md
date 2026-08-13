## 高階函式與閉包

[ch20-04-advanced-functions-and-closures.md](https://github.com/rust-lang/book/blob/57ff62db22b006e6b319e2e35c9364d932a8b4e5/src/ch20-04-advanced-functions-and-closures.md)

本部分將探索一些有關函式和閉包的高階特性，這包括函式指標以及返回閉包。

### 函式指標

我們討論過了如何向函式傳遞閉包；也可以將普通函式傳遞給函式！這個技術在我們希望傳遞已經定義的函式而不是重新定義閉包作為引數時很有用。函式會被強制轉換為 `fn` 型別（小寫的 f），不要與閉包 trait 的 `Fn` 相混淆。`fn` 被稱為 **函式指標**（*function pointer*）。透過函式指標允許我們使用函式作為其它函式的引數。

指定引數為函式指標的語法類似於閉包，如示例 20-28 所示，這裡定義了一個 `add_one` 函式用於將其引數加一。`do_twice` 函式獲取兩個引數：一個指向任何獲取一個 `i32` 引數並返回一個 `i32` 的函式指標，和一個 `i32` 值。`do_twice` 函式傳入 `arg` 引數呼叫 `f` 函式兩次，接著將兩次函式呼叫的結果相加。`main` 函式使用 `add_one` 和 `5` 作為引數呼叫 `do_twice`。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

<span class="caption">示例 20-28: 使用 `fn` 型別接受函式指標作為引數</span>

這段程式碼會打印出 `The answer is: 12`。`do_twice` 中的 `f` 被指定為一個接受一個 `i32` 引數並返回 `i32` 的 `fn`。接著就可以在 `do_twice` 函式體中呼叫 `f`。在 `main` 中，可以將函式名 `add_one` 作為第一個引數傳遞給 `do_twice`。

不同於閉包，`fn` 是一個型別而不是一個 trait，所以直接指定 `fn` 作為引數而不是宣告一個帶有 `Fn` 作為 trait 約束的泛型引數。

函式指標實現了所有三個閉包 trait（`Fn`、`FnMut` 和 `FnOnce`），所以總是可以在呼叫期望閉包的函式時傳遞函式指標作為引數。傾向於編寫使用泛型和閉包 trait 的函式，這樣它就能接受函式或閉包作為引數。

儘管如此，一個只期望接受 `fn` 而不接受閉包的情況的例子是與不存在閉包的外部程式碼互動時：C 語言的函式可以接受函式作為引數，但 C 語言沒有閉包。

作為一個既可以使用內聯定義的閉包又可以使用命名函式的例子，讓我們看看一個標準庫中 `Iterator` trait 提供的 `map` 方法的應用。使用 `map` 函式將一個數字 vector 轉換為一個字串 vector，就可以使用閉包，如示例 20-29 所示：

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

<figcaption>示例 20-29：使用閉包和 `map` 方法將數字轉換為字串</figcaption>

</figure>

或者，也可以把一個函式作為 `map` 的引數來代替閉包。示例 20-30 展示了這種寫法。

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

<figcaption>示例 20-30：使用 `String::to_string` 函式配合 `map` 方法將數字轉換為字串</figcaption>

</figure>

注意這裡必須使用 [“高階 trait”][advanced-traits] 部分講到的完全限定語法，因為存在多個叫做 `to_string` 的函式。

這裡使用了定義於 `ToString` trait 的 `to_string` 函式，標準庫為所有實現了 `Display` 的型別實現了這個 trait。

回憶一下第六章 [“列舉值”][enum-values] 部分中定義的每一個列舉成員也變成了一個建構函式。我們可以使用這些建構函式作為實現了閉包 trait 的函式指標，這意味著可以指定建構函式作為接受閉包的方法的引數，如示例 20-31 所示：

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

<figcaption>示例 20-31：使用列舉建構函式和 `map` 方法從數字建立 `Status` 例項</figcaption>

</figure>


這裡，我們透過 `Status::Value` 的初始化函式，對 `map` 所作用的範圍內每個 `u32` 值建立 `Status::Value` 例項。一些人傾向於函式式風格，一些人喜歡閉包。它們會編譯成相同的程式碼，因此請選擇對你來說更清晰的那一種。

### 返回閉包

閉包表現為 trait，這意味著不能直接返回閉包。對於大部分需要返回 trait 的場景中，可以使用實現了期望返回的 trait 的具體型別來替代函式的返回值。但是這不能用於閉包，因為它們沒有一個可返回的具體型別；例如，當閉包從其作用域捕獲任何值時，就不允許使用函式指標 `fn` 作為返回型別。

相反，可以正常地使用第十章所學的 `impl Trait` 語法。可以使用 `Fn`、`FnOnce` 和 `FnMut` 返回任何函式型別。例如，示例 20-32 中的程式碼就可以正常工作。

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

<figcaption>示例 20-32：使用 `impl Trait` 語法從函式返回閉包</figcaption>

</figure>

然而，正如我們在第十三章 [“推斷和註解閉包型別”][closure-types] 一節中提到的，每個閉包也都有自己獨特的型別。如果你需要處理多個簽名相同但實現不同的函式，就需要為它們使用 trait 物件。來看一下，如果寫出類似示例 20-33 的程式碼，會發生什麼。

<figure class="listing">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

示例 20-33：建立一個由返回 `impl Fn` 的函式定義的閉包的 `Vec<T>`

</figure>

這裡有兩個函式，`returns_closure` 和 `returns_initialized_closure`，它們都返回 `impl Fn(i32) -> i32`。注意它們返回的閉包是不同的，即使它們實現了相同的型別。如果嘗試編譯這段程式碼，Rust 會告訴我們這不可行：

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

錯誤資訊告訴我們每當返回一個 `impl Trait` Rust 會建立一個獨特的**不透明型別**（*opaque type*），這是一個無法看清 Rust 為我們構建了什麼細節的型別。所以即使這些函式都返回了實現了相同 trait（ `Fn(i32) -> i32`）的閉包，Rust 為我們生成的不透明型別也是不同的。這類似於 Rust 如何為不同的非同步程式碼塊生成不同的具體型別，即使它們有著相同的輸出型別，如第十七章 [“`Pin` 型別與 `Unpin` trait”][future-types] 所示。我們已經多次看到這個問題的解決方案：我們可以使用 trait 物件，如示例 20-34 所示。

<figure class="listing">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

示例 20-34：建立一個由返回 `Box<dyn Fn>` 的函式定義的閉包的 `Vec<T>` 以便它們有相同的型別

</figure>

這段程式碼可以順利編譯。關於 trait 物件的更多內容，請回顧第十八章 [“使用 trait 物件抽象出共享行為”][trait-objects] 一節。

接下來讓我們學習宏！

[advanced-traits]: ch20-02-advanced-traits.html#高階-trait
[enum-values]: ch06-01-defining-an-enum.html#列舉值
[closure-types]: ch13-01-closures.html#推斷和註解閉包型別
[future-types]: ch17-05-traits-for-async.html#pin-型別與-unpin-trait
[trait-objects]: ch18-02-trait-objects.html#使用-trait-object-來抽象出共享行為
