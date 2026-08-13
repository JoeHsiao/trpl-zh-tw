## 接受命令列引數

[ch12-01-accepting-command-line-arguments.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-01-accepting-command-line-arguments.md)

一如既往使用 `cargo new` 新建一個專案，我們稱之為 `minigrep` 以便與可能已經安裝在系統上的 `grep` 工具相區分。

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

第一個任務是讓 `minigrep` 能夠接受兩個命令列引數：檔案路徑和要搜尋的字串。也就是說我們希望能夠使用 `cargo run`，兩個連字元來表明後面的引數是要傳遞給程式而不是 `cargo`，要搜尋的字串和被搜尋的檔案的路徑來執行程式，像這樣：

```console
$ cargo run -- searchstring example-filename.txt
```

現在 `cargo new` 生成的程式還不能處理我們傳給它的引數。[crates.io](https://crates.io/) 上有一些現成的庫可以幫助編寫接受命令列引數的程式，不過既然你現在正在學習這個概念，我們就自己來實現這個功能。

### 讀取引數值

為了確保 `minigrep` 能夠獲取傳遞給它的命令列引數的值，我們需要一個 Rust 標準庫提供的 `std::env::args` 函式。這個函式返回一個傳遞給程式的命令列引數的**迭代器**（*iterator*）。我們會在[第十三章][ch13]全面介紹迭代器。但是現在只需理解迭代器的兩個細節：迭代器生成一系列的值，可以在迭代器上呼叫 `collect` 方法將其轉換為一個集合，比如包含所有迭代器產生元素的 vector。

示例 12-1 中的程式碼允許 `minigrep` 程式讀取任何傳遞給它的命令列引數並將其收集到一個 vector 中。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

<span class="caption">示例 12-1：將命令列引數收集到一個 vector 中並打印出來</span>

首先使用 `use` 語句來將 `std::env` 模組引入作用域以便可以使用它的 `args` 函式。注意 `std::env::args` 函式被巢狀進了兩層模組中。正如[第七章][ch7-idiomatic-use]講到的，當所需函式嵌套了多於一層模組時，通常將父模組引入作用域而不是其自身。這便於我們利用 `std::env` 中的其他函式。這比增加了 `use std::env::args;` 後僅僅使用 `args` 呼叫函式要更明確一些，因為 `args` 容易被錯認成一個定義於當前模組的函式。

> ### `args` 函式和無效的 Unicode
>
> 注意 `std::env::args` 在其任何引數包含無效 Unicode 字元時會 panic。如果你需要接受包含無效 Unicode 字元的引數，使用 `std::env::args_os` 代替。這個函式返回 `OsString` 值而不是 `String` 值。這裡出於簡單考慮使用了 `std::env::args`，因為 `OsString` 值每個平臺都不一樣而且比 `String` 值處理起來更為複雜。

在 `main` 函式的第一行，我們呼叫了 `env::args`，並立即使用 `collect` 來建立了一個包含迭代器所有值的 vector。`collect` 可以被用來建立很多型別的集合，所以這裡顯式註明 `args` 的型別來指定我們需要一個字串 vector。雖然在 Rust 中我們很少會需要註明型別，然而 `collect` 是一個經常需要註明型別的函式，因為 Rust 不能推斷出你想要什麼型別的集合。

最後，我們使用除錯宏打印出 vector。讓我們先在不傳遞任何引數的情況下執行一次程式碼，然後再傳入兩個引數執行一次：

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

注意 vector 的第一個值是 `"target/debug/minigrep"`，它是我們二進位制檔案的名稱。這與 C 中的引數列表的行為相匹配，讓程式使用在執行時呼叫它們的名稱。如果要在訊息中列印它或者根據用於呼叫程式的命令列別名更改程式的行為，通常可以方便地訪問程式名稱，不過考慮到本章的目的，我們將忽略它並只儲存所需的兩個引數。

### 將引數值儲存進變數

目前程式可以訪問指定為命令列引數的值。現在需要將這兩個引數的值儲存進變數這樣就可以在程式的餘下部分使用這些值了。讓我們如示例 12-2 這樣做：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

<span class="caption">示例 12-2：建立變數來存放查詢引數和檔案路徑引數</span>

正如之前打印出 vector 時所看到的，程式的名稱佔據了 vector 的第一個值 `args[0]`，所以我們從索引為 `1` 的引數開始。`minigrep` 獲取的第一個引數是需要搜尋的字串，所以將第一個引數的引用存放在變數 `query` 中。第二個引數將是檔案路徑，所以將第二個引數的引用放入變數 `file_path` 中。

我們將臨時打印出這些變數的值來證明程式碼如我們期望的那樣工作。使用引數 `test` 和 `sample.txt` 再次執行這個程式：

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

太好了，程式正常工作！我們將所需的引數值儲存進了對應的變數中。之後會增加一些錯誤處理來應對類似使用者沒有提供引數的情況。不過現在我們將忽略它們並開始增加讀取檔案功能。

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#建立慣用的-use-路徑
