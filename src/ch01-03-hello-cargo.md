## Hello, Cargo!

[ch01-03-hello-cargo.md](https://github.com/rust-lang/book/blob/369386fefd1138cbdf50ae628bae1ffc4ffce669/src/ch01-03-hello-cargo.md)

Cargo 是 Rust 的構建系統和包管理器。大多數 Rustaceans 都使用 Cargo 來管理他們的 Rust 專案，因為 Cargo 會替你處理許多工，比如構建程式碼、下載程式碼依賴的庫並編譯這些庫。（我們把程式碼所需要的庫稱為 **依賴**（*dependencies*）。）

最簡單的 Rust 程式，比如我們剛剛編寫的那個，沒有任何依賴。如果使用 Cargo 來構建 “Hello, world!” 專案，那麼只會用到 Cargo 中負責構建程式碼的那部分功能。隨著你編寫更復雜的 Rust 程式，就會加入依賴項；如果專案一開始就是用 Cargo 建立的，那麼新增依賴項會容易得多。

由於絕大多數 Rust 專案都使用 Cargo，本書接下來的部分也假設你會使用 Cargo。如果你使用的是 [“安裝”][installation] 部分介紹的官方安裝方式，那麼 Cargo 會隨 Rust 一起安裝。如果你透過其他方式安裝 Rust，可以在終端輸入如下命令檢查是否安裝了 Cargo：

```console
$ cargo --version
```

如果你看到了版本號，說明 Cargo 已經安裝好了！如果看到類似 `command not found` 的錯誤，你應該查閱相應安裝方式的文件，確定如何單獨安裝 Cargo。

### 使用 Cargo 建立專案

我們使用 Cargo 建立一個新專案，然後看看與上面的 “Hello, world!” 專案有什麼不同。回到 *projects* 目錄（或者你存放程式碼的目錄）。接著，可在任何作業系統下執行以下命令：

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

第一條命令會新建一個名為 *hello_cargo* 的目錄和專案。我們將專案命名為 *hello_cargo*，Cargo 也會在同名目錄中建立專案檔案。

進入 *hello_cargo* 目錄並列出文件。你會看到 Cargo 為我們生成了兩個檔案和一個目錄：一個 *Cargo.toml* 檔案、一個 *src* 目錄，以及位於 *src* 目錄中的 *main.rs* 檔案。

它還會在 *hello_cargo* 目錄中初始化一個 Git 倉庫，並生成一個 *.gitignore* 檔案。如果在一個已經存在的 Git 倉庫中執行 `cargo new`，這些 Git 相關檔案就不會再生成；你可以透過執行 `cargo new --vcs=git` 來覆蓋這個行為。

> 注意：Git 是一種常見的版本控制系統（version control system，VCS）。你可以透過 `--vcs` 引數讓 `cargo new` 使用其他版本控制系統，或者不使用 VCS。執行 `cargo new --help` 可以檢視可用選項。

請自行選用文字編輯器開啟 *Cargo.toml* 檔案。它應該看起來與示例 1-2 中程式碼類似：

<figure class="listing">

<span class="file-name">檔名：Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

<figcaption>示例 1-2：`cargo new` 命令生成的 *Cargo.toml* 的內容</figcaption>

</figure>

這個檔案使用 [*TOML*][toml]<!-- ignore -->（*Tom's Obvious, Minimal Language*）格式，這是 Cargo 配置檔案所使用的格式。

第一行 `[package]` 是一個 section 標題，表明下面的語句是在配置一個 package。隨著我們在這個檔案中加入更多資訊，還會增加其他 section。

接下來的三行設定了 Cargo 編譯程式所需的配置資訊：專案名稱、專案版本，以及要使用的 Rust edition。[附錄 E][appendix-e] 會介紹 `edition` 這個鍵。

最後一行 `[dependencies]` 是列出專案依賴的 section 的開始。在 Rust 中，程式碼包被稱為 *crates*。這個專案不需要其他 crate，不過在第二章的第一個專案中我們就會需要依賴，到時候會用到這個 section。

現在開啟 *src/main.rs* 看看：

<span class="filename">檔名：src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 為你生成了一個 “Hello, world!” 程式，正如我們之前寫的示例 1-1！到目前為止，我們的專案與 Cargo 生成的專案之間的區別在於：Cargo 將程式碼放在 *src* 目錄中，並在專案根目錄放置了一個 *Cargo.toml* 配置檔案。

Cargo 期望你的原始檔位於 *src* 目錄中。專案根目錄則只用來放 README、license 資訊、配置檔案，以及其他與程式碼無關的內容。使用 Cargo 有助於讓專案保持整潔：各類檔案各歸其位。

如果你一開始沒有使用 Cargo 建立專案，比如我們之前建立的 “Hello, world!” 專案，你也可以把它轉換成一個使用 Cargo 的專案。只要把專案程式碼移到 *src* 目錄中，並建立一個合適的 *Cargo.toml* 檔案即可。一個簡單的辦法是執行 `cargo init`，它會自動為你建立這個檔案。

### 構建並執行 Cargo 專案

現在讓我們來看看，使用 Cargo 構建和執行 “Hello, world!” 程式有什麼不同！在 *hello_cargo* 目錄下，輸入下面的命令來構建專案：

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

這條命令會在 *target/debug/hello_cargo* 生成一個可執行檔案（Windows 上是 *target\debug\hello_cargo.exe*），而不是放在當前目錄下。因為預設構建方式是除錯構建（debug build），Cargo 會把可執行檔案放在名為 *debug* 的目錄中。你可以用下面的命令執行這個可執行檔案：

```console
$ ./target/debug/hello_cargo # 或者在 Windows 下為 .\target\debug\hello_cargo.exe
Hello, world!
```

如果一切順利，終端上應該會打印出 `Hello, world!`。第一次執行 `cargo build` 時，Cargo 還會在專案根目錄建立一個新檔案：*Cargo.lock*。這個檔案會記錄專案依賴的精確版本。由於這個專案沒有依賴，所以檔案內容比較少。你永遠都不需要手動修改這個檔案；Cargo 會替你管理它的內容。

我們剛剛使用 `cargo build` 構建了專案，並使用 `./target/debug/hello_cargo` 運行了程式；也可以使用 `cargo run`，在一條命令中完成編譯並執行生成的可執行檔案：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

比起必須先執行 `cargo build` 再用可執行檔案的完整路徑來執行程式，使用 `cargo run` 更方便，所以大多數開發者會選擇 `cargo run`。

注意，這一次並沒有出現表明 Cargo 正在編譯 `hello_cargo` 的輸出。Cargo 發現檔案沒有發生變化，所以它沒有重新構建，而是直接運行了二進位制檔案。如果你修改了原始檔，Cargo 就會在執行之前重新構建專案，並會出現像這樣的輸出：

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo 還提供了一個叫 `cargo check` 的命令。該命令快速檢查程式碼確保其可以編譯，但並不產生可執行檔案：

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

為什麼你會不需要可執行檔案呢？通常 `cargo check` 比 `cargo build` 快得多，因為它省略了生成可執行檔案這一步。如果你在編寫程式碼時持續檢查，`cargo check` 可以讓你更快知道當前程式碼是否還能正常編譯！因此，很多 Rustaceans 都會在編寫程式時定期執行 `cargo check` 來確保程式碼可以編譯；等到準備好使用可執行檔案時，再執行 `cargo build`。

我們回顧下已學習的 Cargo 內容：

- 可以使用 `cargo new` 建立專案。
- 可以使用 `cargo build` 構建專案。
- 可以使用 `cargo run` 一步構建並執行專案。
- 可以使用 `cargo check` 在不生成二進位制檔案的情況下構建專案來檢查錯誤。
- 有別於將構建結果放在與原始碼相同的目錄，Cargo 會將其放到 *target/debug* 目錄。

使用 Cargo 的另一個優點是，不論你使用什麼作業系統，這些命令都是相同的。所以從現在開始，本書將不再分別為 Linux、macOS 和 Windows 提供單獨的命令。

### 釋出（release）構建

當專案最終準備好釋出時，可以使用 `cargo build --release` 以啟用最佳化方式編譯專案。這會在 *target/release* 而不是 *target/debug* 下生成可執行檔案。這些最佳化會讓 Rust 程式碼執行得更快，不過開啟最佳化也會延長編譯時間。這就是為什麼會有兩種不同的 profile：一種用於開發，你會希望它能快速且頻繁地重新構建；另一種用於構建最終交付給使用者的程式，這種程式不會頻繁重新構建，但會希望它執行得儘可能快。如果你在做程式碼執行時間的基準測試，請務必執行 `cargo build --release`，並使用 *target/release* 下的可執行檔案進行測試。

<a id="cargo-as-convention"></a>

### 把 Cargo 當作習慣

對於簡單專案，Cargo 相比直接使用 `rustc` 並不會帶來太多額外價值，但隨著程式變得更復雜，它的價值就會逐漸顯現。一旦程式增長到由多個檔案組成，或者需要其他依賴，讓 Cargo 來協調構建過程就會容易得多。

儘管 `hello_cargo` 專案很簡單，但它已經用上了許多你在後續 Rust 開發中會經常使用的真實工具。實際上，當你在任何已有專案上工作時，都可以使用如下命令，透過 Git 檢出程式碼、進入專案目錄並構建它：

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

關於更多 Cargo 的資訊，請查閱 [其文件][cargo]。

## 總結

你已經為繼續 Rust 之旅做好準備了！在本章中，你學習瞭如何：

- 使用 `rustup` 安裝最新穩定版 Rust
- 更新到較新的 Rust 版本
- 開啟本地安裝的文件
- 直接透過 `rustc` 編寫並執行 Hello, world! 程式
- 使用 Cargo 建立並執行新專案

現在正是透過構建一個更實在的程式來熟悉 Rust 程式碼讀寫的好時機。因此，在第二章中我們會構建一個猜數字遊戲程式。如果你更想先學習 Rust 中常見的程式設計概念，請先閱讀第三章，然後再回到第二章。

[installation]: ch01-01-installation.html#安裝
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
