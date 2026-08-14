## Cargo 工作空間

[ch14-03-cargo-workspaces.md](https://github.com/rust-lang/book/blob/43b9ad334aaf7353e5708dba49f84f941b50ec4b/src/ch14-03-cargo-workspaces.md)

第十二章中，我們構建了一個同時包含二進位制 crate 和庫 crate 的包。隨著專案不斷發展，你可能會發現庫 crate 變得越來越大，並希望進一步將這個包拆分為多個庫 crate。Cargo 提供了一項叫作**工作空間**（*workspace*）的功能，可以幫助管理多個彼此相關、並行開發的包。

### 建立工作空間

**工作空間**是一組共享同一個 *Cargo.lock* 和輸出目錄的包。讓我們用工作空間建立一個專案，這裡會使用簡單程式碼，以便把注意力集中在工作空間的結構上。組織工作空間的方式有很多種，因此我們只展示一種常見方式。這個工作空間會包含一個二進位制 crate 和兩個庫。二進位制 crate 提供主要功能，並依賴這兩個庫。一個庫提供 `add_one` 函式，另一個庫提供 `add_two` 函式。這三個 crate 都屬於同一個工作空間。我們先為工作空間建立一個新目錄：

```console
$ mkdir add
$ cd add
```

接下來，在 *add* 目錄中建立 *Cargo.toml* 檔案，用來配置整個工作空間。這個檔案不會有 `[package]` 部分，而是會以 `[workspace]` 部分開頭，這樣我們就能向工作空間新增成員。我們還會把 `resolver` 的值設為 `"3"`，以便在工作空間中使用 Cargo 最新的依賴解析演算法：

<span class="filename">檔名：Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

接下來，在 *add* 目錄執行 `cargo new` 新建 `adder` 二進位制 crate：

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

在工作空間中執行 `cargo new` 時，新建立的包也會被自動加入工作空間 *Cargo.toml* 中 `[workspace]` 定義的 `members` 鍵，像這樣：

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

現在，我們可以執行 `cargo build` 來構建工作空間。你的 *add* 目錄中的檔案應如下所示：

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

工作空間在頂層只有一個 *target* 目錄，用來存放編譯產物；`adder` 包不會有自己的 *target* 目錄。即使我們在 *adder* 目錄中執行 `cargo build`，編譯產物也仍會放到 *add/target*，而不是 *add/adder/target*。Cargo 之所以這樣組織工作空間中的 *target* 目錄，是因為工作空間中的 crate 本來就是要彼此依賴的。如果每個 crate 都有各自的 *target* 目錄，那麼每個 crate 都不得不重新編譯工作空間中的其他 crate，才能把產物放進自己的 *target* 目錄。共享一個 *target* 目錄可以避免不必要的重複構建。

### 在工作空間中建立第二個包

接下來，讓我們在工作空間中建立另一個成員包，並將其命名為 `add_one`。生成一個名為 `add_one` 的庫 crate：

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

現在頂層的 *Cargo.toml* 的 `members` 列表將會包含 *add_one* 路徑：

<span class="filename">檔名：Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

現在 *add* 目錄應該有如下目錄和檔案：

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

在 *add_one/src/lib.rs* 檔案中，增加一個 `add_one` 函式：

<span class="filename">檔名：add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

現在，我們可以讓二進位制包 `adder` 依賴包含庫的 `add_one` 包了。首先，需要在 *adder/Cargo.toml* 中把 `add_one` 新增為一個路徑依賴：

<span class="filename">檔名：adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo 並不會假定工作空間中的 crate 會彼此依賴，因此我們需要顯式宣告這些依賴關係。

接下來，讓我們在 `adder` crate 中使用 `add_one` crate 裡的 `add_one` 函式。開啟 *adder/src/main.rs* 檔案，並將 `main` 函式改為呼叫 `add_one`，如示例 14-7 所示。

<span class="filename">檔名：adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<span class="caption">示例 14-7：在 `adder` crate 中使用 `add_one` 庫 crate</span>

在頂層 *add* 目錄中執行 `cargo build` 來構建工作空間！

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

要從 *add* 目錄執行這個二進位制 crate，可以在 `cargo run` 時透過 `-p` 引數加上包名，指定要執行工作空間中的哪個包：

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

這會執行 *adder/src/main.rs* 中的程式碼，其依賴 `add_one` crate。


#### 依賴外部包

注意，工作空間只在頂層有一個 *Cargo.lock* 檔案，而不是讓每個 crate 目錄裡都各自有一個 *Cargo.lock*。這能確保所有 crate 使用的都是同一個版本的依賴。如果我們把 `rand` 包同時加到 _adder/Cargo.toml_ 和 _add_one/Cargo.toml_ 中，Cargo 會把它們都解析為同一個 `rand` 版本，並把結果記錄到唯一的 _Cargo.lock_ 中。讓工作空間中的所有 crate 使用相同依賴，意味著這些 crate 會始終彼此相容。現在我們先把 `rand` crate 加到 *add_one/Cargo.toml* 的 `[dependencies]` 部分，以便能在 `add_one` crate 中使用它：

<span class="filename">檔名：add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

現在我們可以在 *add_one/src/lib.rs* 中加入 `use rand;`，然後在 *add* 目錄中執行 `cargo build` 來構建整個工作空間，這會引入並編譯 `rand` crate。我們會得到一條警告，因為我們並沒有實際使用引入到作用域中的 `rand`：

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

頂層的 *Cargo.lock* 現在已經包含了 `add_one` 依賴 `rand` 的資訊。不過，即使 `rand` 在工作空間的某處被使用，我們也不能直接在工作空間裡的其他 crate 中使用它，除非也把 `rand` 加到它們各自的 *Cargo.toml* 中。例如，如果我們在 `adder` 包的 *adder/src/main.rs* 中加入 `use rand;`，就會得到一個錯誤：

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

要修復這個錯誤，就編輯 `adder` 包的 *Cargo.toml* 檔案，宣告 `rand` 也是它的依賴。構建 `adder` 包時，會把 `rand` 加到 *Cargo.lock* 中 `adder` 的依賴列表裡，但不會額外下載一份新的 `rand`。Cargo 會確保工作空間中每個使用 `rand` 的 crate 都使用同一個版本，只要它們宣告的是彼此相容的 `rand` 版本，這樣既節省空間，也確保工作空間中的 crate 彼此相容。

如果工作空間中的 crate 為同一個依賴指定了彼此不相容的版本，Cargo 仍然會分別解析它們，但會盡量把版本數量控制得儘可能少。

#### 為工作空間增加測試

作為另一個改進，讓我們為 `add_one` crate 中的 `add_one::add_one` 函式增加一個測試：

<span class="filename">檔名：add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

現在，在頂層 *add* 目錄中執行 `cargo test`。在這種結構的工作空間裡執行 `cargo test`，會執行工作空間中所有 crate 的測試：

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

輸出的第一部分表明 `add_one` crate 中的 `it_works` 測試通過了。下一部分表明在 `adder` crate 中沒有找到測試，最後一部分表明 `add_one` crate 中也沒有文件測試。

你也可以選擇只執行工作空間中某個特定 crate 的測試，只需在根目錄中使用 `-p` 引數並指定想要測試的 crate 名稱：

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

輸出表明，`cargo test` 只運行了 `add_one` crate 的測試，而沒有執行 `adder` crate 的測試。

如果你打算把工作空間中的 crate 釋出到 [crates.io](https://crates.io/) 上，那麼工作空間中的每個 crate 都需要單獨釋出。和 `cargo test` 一樣，你可以透過 `-p` 引數並指定要釋出的 crate 名稱，來發布工作空間中的某個特定 crate。

現在，試著仿照 `add_one` crate 的方式，把 `add_two` crate 也加入工作空間，作為額外練習吧！

隨著專案規模增長，可以考慮使用工作空間：每個較小的元件都比一大塊程式碼更容易理解。如果這些元件經常需要一起修改，那麼把它們保留在同一個工作空間中，會更容易協調彼此的變更。
