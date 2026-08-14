## 使用自定義命令擴充套件 Cargo

[ch14-05-extending-cargo.md](https://github.com/rust-lang/book/blob/43b9ad334aaf7353e5708dba49f84f941b50ec4b/src/ch14-05-extending-cargo.md)

Cargo 的設計允許你用新的子命令來擴充套件它，而不必修改 Cargo 本身。如果你的 `$PATH` 中有一個名為 `cargo-something` 的二進位制檔案，那麼你就可以像執行 Cargo 子命令一樣，透過 `cargo something` 來執行它。這類自定義命令也會在你執行 `cargo --list` 時顯示出來。Cargo 這種設計帶來了一個非常方便的好處：你可以用 `cargo install` 安裝擴充套件，然後像使用 Cargo 內建工具一樣執行它們。

## 總結

透過 Cargo 和 [crates.io](https://crates.io/)<!-- ignore --> 分享程式碼，是 Rust 生態系統之所以能適用於眾多不同任務的重要原因之一。Rust 的標準庫小而穩定，但 crate 很容易被分享、使用和改進，而且它們的演進節奏也可以不同於語言本身。不要猶豫，把那些對你有用的程式碼分享到 [crates.io](https://crates.io/)<!-- ignore --> 上；它很可能也會對別人有用！
