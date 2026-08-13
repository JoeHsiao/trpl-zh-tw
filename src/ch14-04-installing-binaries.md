## 使用 `cargo install` 安裝二進位制檔案

[ch14-04-installing-binaries.md](https://github.com/rust-lang/book/blob/43b9ad334aaf7353e5708dba49f84f941b50ec4b/src/ch14-04-installing-binaries.md)

`cargo install` 命令允許你在本地安裝和使用二進位制 crate。它並不是為了替代系統包管理器，而是為 Rust 開發者提供一種方便的方式，用來安裝他人在 [crates.io](https://crates.io/)<!-- ignore --> 上分享的工具。注意，只有帶有二進位制目標的包才能被安裝。**二進位制目標**是指當 crate 包含 *src/main.rs* 檔案，或將其他檔案指定為二進位制目標時所生成的可執行程式；這與庫目標不同，庫目標本身不能單獨執行，但適合被其他程式引入。通常，crate 的 *README* 檔案會說明它是庫、帶有二進位制目標，還是兩者兼有。

所有透過 `cargo install` 安裝的二進位制檔案，都會放在安裝根目錄下的 *bin* 資料夾中。如果你使用 *rustup.rs* 安裝 Rust，並且沒有做任何自定義配置，那麼這個目錄就是 *$HOME/.cargo/bin*。請確保這個目錄已經加入你的 `$PATH`，這樣你才能執行透過 `cargo install` 安裝的程式。

例如，在第十二章中我們提到過，有一個名為 `ripgrep` 的 `grep` 工具 Rust 實現，可用於搜尋檔案。要安裝 `ripgrep`，可以執行以下命令：

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

輸出的倒數第二行展示了已安裝二進位制檔案的位置和名稱；對於 `ripgrep` 來說，這個可執行檔名是 `rg`。只要安裝目錄已經像前面說的那樣加入了 `$PATH`，你就可以執行 `rg --help`，開始使用這個更快、也更“Rust 風格”的檔案搜尋工具了！
