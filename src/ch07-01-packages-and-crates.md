## 包和 Crate

[ch07-01-packages-and-crates.md](https://github.com/rust-lang/book/blob/79b9d15410a7b15a65cb86bcb40cbea99198a9e4/src/ch07-01-packages-and-crates.md)

模組系統的第一部分，我們將介紹包和 crate。

crate 是 Rust 編譯器每次處理的最小程式碼單位。即使你用 `rustc` 而不是 `cargo` 來編譯單個原始碼檔案（正如我們在第一章的 [“Rust 程式基礎”][basics] 中做的那樣），編譯器也會把那個檔案視為一個 crate。crate 可以包含模組，而這些模組也可以定義在其他檔案中，並與該 crate 一起編譯；我們會在接下來的小節中看到這一點。

crate 有兩種形式：二進位制 crate 和庫 crate。**二進位制 crate**（*Binary crates*）可以被編譯為可執行程式，比如命令列程式或者服務端。它們必須有一個名為 `main` 函式來定義當程式被執行的時候所需要做的事情。目前我們所建立的 crate 都是二進位制 crate。

**庫 crate**（*Library crates*）並沒有 `main` 函式，它們也不會編譯為可執行程式。相反它們定義了可供多個專案複用的功能模組。比如 [第二章][rand] 的 `rand` crate 就提供了生成隨機數的功能。大多數時間 `Rustaceans` 說的 “crate” 指的都是庫 crate，這與其他程式語言中 “library” 概念一致。

*crate root* 是一個原始檔，Rust 編譯器以它為起始點，並構成你的 crate 的根模組（我們將在 [“定義模組來控制作用域與私有性”][modules] 一節深入解讀）。

*包*（*package*）是提供一系列功能的一個或者多個 crate 的捆綁。一個包會包含一個 *Cargo.toml* 檔案，闡述如何去構建這些 crate。Cargo 實際上就是一個包，它包含了用於構建你程式碼的命令列工具的二進位制 crate。其他專案也依賴 Cargo 庫來實現與 Cargo 命令列程式一樣的邏輯。

包中可以包含至多一個庫 crate(library crate)。包中可以包含任意多個二進位制 crate(binary crate)，但是必須至少包含一個 crate（無論是庫的還是二進位制的）。

讓我們來看看建立包的時候會發生什麼。首先，我們輸入命令 `cargo new my-project`：

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

運行了這條命令後，我們先用 `ls` （譯者注：此命令為 Linux 平臺的指令，Windows 下可用 dir）來看看 Cargo 給我們建立了什麼，Cargo 會給我們的包建立一個 *Cargo.toml* 檔案。檢視 *Cargo.toml* 的內容，會發現並沒有提到 *src/main.rs*，因為 Cargo 遵循的一個約定：*src/main.rs* 就是一個與包同名的二進位制 crate 的 crate 根。同樣的，Cargo 知道如果包目錄中包含 *src/lib.rs*，則包帶有與其同名的庫 crate，且 *src/lib.rs* 是 crate 根。crate 根檔案將由 Cargo 傳遞給 `rustc` 來實際構建庫或者二進位制專案。

在此，我們有了一個只包含 *src/main.rs* 的包，意味著它只含有一個名為 `my-project` 的二進位制 crate。如果一個包同時含有 *src/main.rs* 和 *src/lib.rs*，則它有兩個 crate：一個二進位制的和一個庫的，且名字都與包相同。透過將檔案放在 *src/bin* 目錄下，一個包可以擁有多個二進位制 crate：每個 *src/bin* 下的檔案都會被編譯成一個獨立的二進位制 crate。

[basics]: ch01-02-hello-world.html#rust-程式基礎
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#生成一個隨機數
