<a id="writing-error-messages-to-standard-error-instead-of-standard-output"></a>

## 將錯誤重定向到標準錯誤

[ch12-06-writing-to-stderr-instead-of-stdout.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-06-writing-to-stderr-instead-of-stdout.md)

目前為止，我們將所有的輸出都透過 `println!` 寫到了終端。大部分終端都提供了兩種輸出：**標準輸出**（*standard output*，`stdout`）對應一般資訊，**標準錯誤**（*standard error*，`stderr`）則用於錯誤資訊。這種區別允許使用者選擇將程式正常輸出定向到一個檔案中並仍將錯誤資訊列印到螢幕上。

但是 `println!` 宏只能夠列印到標準輸出，所以我們必須使用其他方法來列印到標準錯誤。

### 檢查錯誤寫入何處

首先，讓我們觀察一下目前 `minigrep` 列印的所有內容是如何被寫入標準輸出的，包括那些應該被寫入標準錯誤的錯誤資訊。可以透過將標準輸出流重定向到一個檔案同時有意產生一個錯誤來做到這一點。我們沒有重定向標準錯誤流，所以任何傳送到標準錯誤的內容將會繼續顯示在螢幕上。

命令列程式被期望將錯誤資訊傳送到標準錯誤流，這樣即便選擇將標準輸出流重定向到檔案中時仍然能看到錯誤資訊。目前我們的程式並不符合期望；相反我們將看到它將錯誤資訊輸出儲存到了檔案中！

我們透過 `>` 和檔案路徑 *output.txt* 來執行程式，我們期望重定向標準輸出流到該檔案中。在這裡，我們沒有傳遞任何引數，所以會產生一個錯誤：

```console
$ cargo run > output.txt
```

`>` 語法告訴 shell 將標準輸出的內容寫入到 *output.txt* 檔案中而不是螢幕上。我們並沒有看到期望的錯誤資訊列印到螢幕上，所以這意味著它一定被寫入了檔案中。如下是 *output.txt* 所包含的：

```text
Problem parsing arguments: not enough arguments
```

是的，錯誤資訊被列印到了標準輸出中。像這樣的錯誤資訊被列印到標準錯誤中將會有用得多，這將使得只有成功執行所產生的輸出才會寫入檔案。我們接下來就修改。

### 將錯誤列印到標準錯誤

讓我們如示例 12-24 所示的程式碼改變錯誤資訊是如何被列印的。得益於本章早些時候的重構，所有列印錯誤資訊的程式碼都位於 `main` 一個函式中。標準庫提供了 `eprintln!` 宏來列印到標準錯誤流，所以將兩個呼叫 `println!` 列印錯誤資訊的位置替換為 `eprintln!`：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

<span class="caption">示例 12-24：使用 `eprintln!` 將錯誤資訊寫入標準錯誤而不是標準輸出</span>

現在我們再次嘗試用同樣的方式執行程式，不使用任何引數並透過 `>` 重定向標準輸出：

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

現在我們看到了螢幕上的錯誤資訊，同時 *output.txt* 裡什麼也沒有，這正是命令列程式所期望的行為。

如果使用不會造成錯誤的引數再次執行程式，不過仍然將標準輸出重定向到一個檔案，像這樣：

```console
$ cargo run -- to poem.txt > output.txt
```

我們並不會在終端看到任何輸出，同時 `output.txt` 將會包含其結果：

<span class="filename">檔名：output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

這一部分展示了現在我們適當地使用了成功時產生的標準輸出和錯誤時產生的標準錯誤。

## 總結

這一章回顧了迄今為止你學到的一些主要概念，並介紹瞭如何在 Rust 中執行常見的 I/O 操作。透過使用命令列引數、檔案、環境變數和用於列印錯誤的 `eprintln!` 宏，你現在已經準備好編寫命令列程式了。結合前幾章的知識，你的程式碼將會組織良好，能將資料有效地儲存在合適的資料結構中，能妥善處理錯誤，並且經過良好測試。

接下來，讓我們探索一些 Rust 中受函數語言程式設計語言影響的功能：閉包和迭代器。
