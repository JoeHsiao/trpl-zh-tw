## 附錄 D：實用開發工具

[appendix-04-useful-development-tools.md](https://github.com/rust-lang/book/blob/7b2b4804c0bd9442805441d21a945e4dd37092b2/src/appendix-04-useful-development-tools.md)

在本附錄中，我們將討論 Rust 專案提供的一些有助於開發 Rust 程式碼的工具。我們將介紹自動格式化、快速應用警告修復、linter 以及與 IDE 的整合。

### 通過 `rustfmt` 自動格式化

`rustfmt` 工具根據社群程式碼風格格式化程式碼。很多專案使用 `rustfmt` 來避免編寫 Rust 程式碼風格的爭論：所有人都用這個工具格式化程式碼！

Rust 安裝預設已包含 `rustfmt`，因此你的系統上應該已經有 `rustfmt` 和 `cargo-fmt` 這兩個程式了。它們類似於 `rustc` 和 `cargo` 的關係：`rustfmt` 提供更細粒度的控制，而 `cargo-fmt` 則理解使用 Cargo 的專案約定。要格式化任意 Cargo 專案，請執行以下命令：

```console
$ cargo fmt
```

執行此命令會格式化當前 crate 中所有的 Rust 程式碼。這應該只會改變程式碼風格，而不是程式碼語義。有關 `rustfmt` 的更多資訊，請參閱 [該文件][rustfmt]。

[rustfmt]: https://github.com/rust-lang/rustfmt

### 通過 `rustfix` 修復程式碼

`rustfix` 工具已隨 Rust 安裝一併提供，可以自動修復那些具有明確修復方式的編譯器警告，這通常正是你所需要的。你可能已經見過類似的編譯器警告。例如，考慮如下程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

這裡定義變數 `x` 為可變，但我們實際上從未修改它。Rust 會對此發出警告：

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

警告中建議移除 `mut` 關鍵字。我們可以通過執行 `cargo fix` 命令使用 `rustfix` 工具來自動採用該建議：

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

如果再次檢視 _src/main.rs_，會發現 `cargo fix` 修改了程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

變數 `x` 現在是不可變的了，警告也不再出現。

你也可以用 `cargo fix` 在不同 Rust edition 之間遷移程式碼。edition 在[附錄 E][editions]中介紹。

### 使用 Clippy 獲取更多 lint

Clippy 工具是一組 lints 的集合，用於分析你的程式碼，幫助你捕捉常見錯誤並改進 Rust 程式碼。Clippy 已包含在 Rust 的標準安裝中。

要對任何 Cargo 專案執行 Clippy 的 lint，請輸入以下命令：

```console
$ cargo clippy
```

例如，假設你寫了一個程式，像下面這樣使用某個數學常量的近似值，例如 pi：

<span class="filename">檔名：src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

在此專案上執行 `cargo clippy` 會導致這個錯誤：

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

該錯誤提示你 Rust 已經定義了一個更精確的 `PI` 常量，如果使用該常量，你的程式會更為正確。你可以將程式碼改為使用 `PI` 常量。如下程式碼就不會引發 Clippy 的任何錯誤或警告：

<span class="filename">檔名：src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

有關 Clippy 的更多資訊，請參閱 [其文件][clippy]。

[clippy]: https://github.com/rust-lang/rust-clippy

### 使用 `rust-analyzer` 的 IDE 整合

為了幫助 IDE 整合，Rust 社群建議使用 [`rust-analyzer`][rust-analyzer]。這個工具是一組以編譯器為中心的實用程式，它實現了 [Language Server Protocol][lsp]，這是 IDE 與程式語言之間通訊的一項規範。不同的客戶端都可以使用 `rust-analyzer`，例如 [Visual Studio Code 的 Rust analyzer 外掛][vscode]。

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

訪問 `rust-analyzer` 專案的[主頁][rust-analyzer]獲取安裝說明，然後為你所使用的 IDE 安裝相應的 language server 支援。這樣一來，你的 IDE 就會獲得自動補全、跳轉到定義以及內聯錯誤等能力。

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.html
