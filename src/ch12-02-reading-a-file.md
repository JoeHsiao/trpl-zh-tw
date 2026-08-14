## 讀取檔案

[ch12-02-reading-a-file.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-02-reading-a-file.md)

現在我們要增加讀取由 `file_path` 命令列引數指定的檔案的功能。首先，需要一個用來測試的示例檔案：我們會用一個擁有多行少量文本且有一些重複單詞的檔案。示例 12-3 是一首艾米莉·狄金森（Emily Dickinson）的詩，它正適合這個工作！在專案根目錄建立一個檔案 *poem.txt*，並輸入詩 "I'm nobody! Who are you?"：

<span class="filename">檔名：poem.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

<span class="caption">示例 12-3：艾米莉·狄金森的詩 “I’m nobody! Who are you?”，一個好的測試用例</span>

有了文本之後，編輯 *src/main.rs* 並新增讀取檔案的程式碼，如示例 12-4 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

<span class="caption">示例 12-4：讀取第二個引數所指定的檔案內容</span>

首先，我們增加了一個 `use` 語句來引入標準庫中的相關部分：我們需要 `std::fs` 來處理檔案。

在 `main` 中新增了一行語句：`fs::read_to_string` 接受 `file_path`，開啟檔案，接著返回包含其內容的 `std::io::Result<String>`。

在這些程式碼之後，我們再次增加了臨時的 `println!` 打印出讀取檔案之後 `contents` 的值，這樣就可以檢查目前為止的程式能否工作。

嘗試執行這些程式碼，隨意指定一個字串作為第一個命令列引數（因為還未實現搜尋功能的部分）而將 *poem.txt* 檔案將作為第二個引數：

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

太好了！這段程式碼讀取並打印出了檔案內容。不過這段程式碼還有幾個缺點。目前 `main` 函式承擔了多個職責，而一般來說，如果每個函式只負責一個概念，程式碼會更清晰，也更容易維護。另一個問題是，我們對錯誤的處理也還不夠完善。雖然程式目前還很小，這些缺點還不算大問題，但隨著程式變大，要乾淨地修復它們就會更困難。在開發程式時，儘早開始重構是一種良好實踐，因為重構較少的程式碼總是更容易一些。接下來我們就這麼做。
