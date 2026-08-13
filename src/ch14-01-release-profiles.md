## 採用釋出配置自定義構建

[ch14-01-release-profiles.md](https://github.com/rust-lang/book/blob/43b9ad334aaf7353e5708dba49f84f941b50ec4b/src/ch14-01-release-profiles.md)

在 Rust 中，**釋出配置**（*release profiles*）是預定義且可定製的配置檔案集，它們包含不同的配置，允許程式設計師更靈活地控制程式碼編譯的多種選項。每一種配置都獨立於其他配置。

Cargo 有兩個主要的配置：執行 `cargo build` 時採用的 `dev` 配置和執行 `cargo build --release` 的 `release` 配置。`dev` 配置為開發定義了良好的預設配置，`release` 配置則為釋出構建定義了良好的預設配置。

這些配置名稱可能很眼熟，因為它們出現在構建的輸出中：

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

構建輸出中的 `dev` 和 `release` 表明編譯器在使用不同的配置。

當專案的 *Cargo.toml* 檔案中沒有顯式增加任何 `[profile.*]` 部分的時候，Cargo 會對每一個配置都採用預設設定。透過增加任何希望定製的配置對應的 `[profile.*]` 部分，我們可以選擇覆蓋任意預設設定的子集。例如，如下是 `dev` 和 `release` 配置的 `opt-level` 設定的預設值：

<span class="filename">檔名：Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` 設定控制 Rust 會對程式碼進行何種程度的最佳化。這個配置的值從 0 到 3。越高的最佳化級別需要更多的時間編譯，所以如果你在進行開發並經常編譯，可能會希望在犧牲一些程式碼效能的情況下減少最佳化以便編譯得快一些。因此 `dev` 的 `opt-level` 預設為 `0`。當你準備釋出時，花費更多時間在編譯上則更好。只需要在釋出模式編譯一次，而編譯出來的程式則會執行很多次，所以釋出模式用更長的編譯時間換取執行更快的程式碼。這正是為什麼 `release` 配置的 `opt-level` 預設為 `3`。

我們可以選擇透過在 *Cargo.toml* 增加不同的值來覆蓋任何預設設定。比如，如果我們想要在開發配置中使用級別 1 的最佳化，則可以在 *Cargo.toml* 中增加這兩行：

<span class="filename">檔名：Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

這會覆蓋預設的設定 `0`。現在執行 `cargo build` 時，Cargo 將會使用 `dev` 的預設配置加上定製的 `opt-level`。因為 `opt-level` 設定為 `1`，Cargo 會比預設進行更多的最佳化，但是沒有釋出構建那麼多。

對於每個配置的設定和其預設值的完整列表，請參閱[Cargo 的文件](https://doc.rust-lang.org/cargo/reference/profiles.html)。
