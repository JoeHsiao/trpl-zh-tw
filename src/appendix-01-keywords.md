## 附錄 A：關鍵字

[appendix-01-keywords.md](https://github.com/rust-lang/book/blob/c0f0135ed8056650d0b4b8ac3cffdb277c31f06a/src/appendix-01-keywords.md)

下面的列表包含 Rust 語言當前使用中或為將來使用而保留的關鍵字。因此，它們不能被用作識別符號（原始識別符號除外，我們會在 [“原始識別符號”][raw-identifiers] 一節中介紹）。_識別符號_ 指的是函式、變數、引數、結構體欄位、模組、crate、常量、宏、靜態值、屬性、型別、trait 或生命週期的名字。

[raw-identifiers]: #原始識別符號

### 目前正在使用的關鍵字

如下為目前正在使用的關鍵字及其功能描述的列表。

- `as` - 原始型別轉換，消除特定包含項的 trait 的歧義，或者對 `use` 語句中的項重新命名
- `async` - 返回一個 `Future` 而不是阻塞當前執行緒
- `await` - 暫停執行直到 `Future` 的結果就緒
- `break` - 立刻退出迴圈
- `const` - 定義常量或常量裸指標（constant raw pointer）
- `continue` - 繼續進入下一次迴圈迭代
- `crate` - 在模組路徑中，代指 crate root
- `dyn` - 動態分發 trait 物件
- `else` - 作為 `if` 和 `if let` 控制流結構的 fallback
- `enum` - 定義一個列舉
- `extern` - 連結一個外部函式或變數
- `false` - 布林字面值 `false`
- `fn` - 定義一個函式或 **函式指標型別** (*function pointer type*)
- `for` - 遍歷一個迭代器或實現一個 trait 或者指定一個更高階的生命週期
- `if` - 基於條件表示式的結果分支
- `impl` - 實現自有或 trait 功能
- `in` - `for` 迴圈語法的一部分
- `let` - 繫結一個變數
- `loop` - 無條件迴圈
- `match` - 模式匹配
- `mod` - 定義一個模組
- `move` - 使閉包獲取其所捕獲項的所有權
- `mut` - 表示引用、裸指標或模式繫結的可變性
- `pub` - 表示結構體欄位、`impl` 塊或模組的公有可見性
- `ref` - 通過引用繫結
- `return` - 從函式中返回
- `Self` - 定義或實現 trait 的型別的類型別名
- `self` - 表示方法本身或當前模組
- `static` - 表示全域性變數或在整個程式執行期間保持其生命週期
- `struct` - 定義一個結構體
- `super` - 表示當前模組的父模組
- `trait` - 定義一個 trait
- `true` - 布林字面值 `true`
- `type` - 定義一個類型別名或關聯型別
- `union` - 定義一個 [union]；僅在 union 宣告中作為關鍵字
- `unsafe` - 表示不安全的程式碼、函式、trait 或實現
- `use` - 將符號引入作用域；為泛型和生命週期約束指定精確捕獲
- `where` - 表示一個約束型別的從句
- `while` - 根據表示式的結果進行條件迴圈

[union]: https://doc.rust-lang.org/reference/items/unions.html

### 為將來使用保留的關鍵字

以下關鍵字目前尚無任何功能，但已被 Rust 保留以備將來使用。

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### 原始識別符號

**原始識別符號**（_Raw identifiers_）是一種允許你使用通常不能使用的關鍵字的語法。通過在關鍵字前加上 `r#` 字首來使用原始識別符號。

例如，`match` 是關鍵字。如果嘗試編譯如下使用 `match` 作為名字的函式：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

會得到這個錯誤：

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

該錯誤表示你不能將關鍵字 `match` 用作函式識別符號。要將 `match` 用作函式名稱，需要使用原始識別符號語法，如下所示：

<span class="filename">檔名：src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

此程式碼編譯沒有任何錯誤。注意 `r#` 字首需同時用於函式名定義和 `main` 函式中的呼叫。

原始識別符號允許你把任意單詞用作識別符號，即使這個單詞恰好是保留關鍵字。這讓我們在選擇識別符號名稱時擁有更大的自由，也讓我們能夠與那些把這些詞當作普通名稱、而不是關鍵字的其他語言程式進行整合。此外，原始識別符號還允許你使用採用不同 Rust edition 編寫的庫。例如，`try` 在 2015 edition 中不是關鍵字，但在 2018、2021 和 2024 edition 中是。如果你依賴的某個庫使用 2015 edition 編寫，並且其中有一個名為 `try` 的函式，那麼在較新的 edition 中呼叫它時，就需要使用原始識別符號語法，在這裡就是 `r#try`。有關 edition 的更多資訊，請參見[附錄 E][appendix-e]。

[appendix-e]: appendix-05-editions.html
