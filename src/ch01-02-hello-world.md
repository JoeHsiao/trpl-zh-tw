## Hello, World!

[ch01-02-hello-world.md](https://github.com/rust-lang/book/blob/d46785983db2d2f94ca3d571db2cfbad0f5ad3e6/src/ch01-02-hello-world.md)

既然安裝好了 Rust，是時候來編寫第一個 Rust 程式了。當學習一門新語言的時候，使用該語言在螢幕上列印 `Hello, world!` 是一項傳統，我們將沿用這一傳統！

> 注意：本書假設你熟悉基本的命令列操作。Rust 對於你的編輯器、工具，以及程式碼位於何處並沒有特定的要求，如果你更傾向於使用整合開發環境（IDE），而不是命令列，請儘管使用你喜歡的 IDE。目前很多 IDE 都在一定程度上支援 Rust；檢視 IDE 文件以瞭解更多細節。Rust 團隊一直致力於藉助 `rust-analyzer` 提供強大的 IDE 支援。詳見[附錄 D][devtools]<!-- ignore -->。

<a id="creating-a-project-directory"></a>

### 建立專案目錄

首先建立一個存放 Rust 程式碼的目錄。Rust 並不關心程式碼存放在哪裡，不過對於本書中的練習和專案，我們建議你在 home 目錄中建立一個 *projects* 目錄，並將所有專案都放在那裡。

開啟終端並輸入如下命令建立 *projects* 目錄，並在 *projects* 目錄中為 “Hello, world!” 專案建立一個目錄。

對於 Linux、macOS 和 Windows PowerShell，輸入：

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

對於 Windows CMD，輸入：

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<a id="writing-and-running-a-rust-program"></a>

### Rust 程式基礎

接下來，新建一個原始檔，命名為 *main.rs*。Rust 檔案總是以 *.rs* 副檔名結尾。如果檔名包含多個單詞，那麼按照命名習慣，應當使用下劃線來分隔單詞。例如應命名為 *hello_world.rs*，而不是 *helloworld.rs*。

現在開啟剛建立的 *main.rs* 檔案，輸入示例 1-1 中的程式碼。

<figure class="listing">

<span class="file-name">檔名：main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<figcaption>示例 1-1：一個列印 `Hello, world!` 的程式</figcaption>

</figure>


儲存檔案，並回到當前目錄為 *~/projects/hello_world* 的終端視窗。在 Linux 或 macOS 上，輸入如下命令，編譯並執行檔案：

```console
$ rustc main.rs
$ ./main
Hello, world!
```

在 Windows 上，輸入命令 `.\main`，而不是 `./main`：

```powershell
> rustc main.rs
> .\main
Hello, world!
```

不管使用何種作業系統，終端應該列印字串 `Hello, world!`。如果沒有看到這些輸出，回到安裝部分的 [“故障排除”][troubleshooting] 小節查詢有幫助的方法。

如果 `Hello, world!` 確實打印出來了，恭喜你！你已經正式寫出了一個 Rust 程式。現在你已經是一名 Rust 程式設計師了，歡迎加入！

<a id="anatomy-of-a-rust-program"></a>

### Rust 程式的結構

現在，讓我們回過頭來仔細看看這個 “Hello, world!” 程式。這是第一塊拼圖：

```rust
fn main() {

}
```

這幾行定義了一個名叫 `main` 的函式。`main` 函式很特殊：在每個可執行的 Rust 程式中，它都是最先執行的程式碼。這裡第一行聲明瞭一個名為 `main` 的函式，它沒有引數也沒有返回值。如果有引數，它們會寫在小括號 `()` 中。

函式體被包裹在 `{}` 中。Rust 要求所有函式體都要用花括號包裹起來。一般來說，將左花括號與函式宣告置於同一行並以空格分隔，是良好的程式碼風格。

> 注：如果你希望在 Rust 專案中保持一種標準風格，可以使用名為 `rustfmt` 的自動格式化工具將程式碼格式化為特定的風格（更多內容詳見[附錄 D][devtools] 中的 `rustfmt`<!-- ignore -->）。Rust 團隊已經在標準的 Rust 發行版中包含了這個工具，就像 `rustc` 一樣。所以它應該已經安裝在你的電腦中了！

在 `main` 函式體中有如下程式碼：

```rust
println!("Hello, world!");
```

這行程式碼完成這個簡單程式的所有工作：在螢幕上列印文字。這裡有三個重要的細節需要注意。

首先，`println!` 呼叫了一個 Rust 宏（macro）。如果呼叫的是函式，就應該寫成 `println`（不帶 `!`）。Rust 宏是一種用來編寫可生成程式碼的程式碼，從而擴充套件 Rust 語法的方式；我們將在[第二十章][ch20-macros]詳細討論宏。現在你只需要知道，看到 `!` 就意味著呼叫的是宏而不是普通函式，並且宏並不總是遵循與函式相同的規則。

第二，`"Hello, world!"` 是一個字串。我們把這個字串作為一個引數傳遞給 `println!`，字串將被列印到螢幕上。

第三，該行以分號結尾（`;`），這代表一個表示式的結束和下一個表示式可以開始。大部分 Rust 程式碼行以分號結尾。

<a id="compiling-and-running-are-separate-steps"></a>

### 編譯與執行

你剛剛運行了一個新建立的程式，那麼讓我們檢查此過程中的每一個步驟。

在執行 Rust 程式之前，必須先使用 Rust 編譯器編譯它，即輸入 `rustc` 命令並傳入原始檔名稱，如下：

```console
$ rustc main.rs
```

如果你有 C 或 C++ 背景，就會發現這與 `gcc` 和 `clang` 類似。編譯成功後，Rust 會輸出一個二進位制的可執行檔案。

在 Linux、macOS，以及 Windows 的 PowerShell 上，你可以在 shell 中輸入 `ls` 命令來檢視這個可執行檔案。

```console
$ ls
main  main.rs
```

在 Linux 和 macOS，你會看到兩個檔案。在 Windows PowerShell 中，你會看到同使用 CMD 相同的三個檔案。在 Windows 的 CMD 上，則輸入如下命令：

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

這展示了帶有 *.rs* 副檔名的原始碼檔案、可執行檔案（Windows 上是 *main.exe*，其他平臺上則是 *main*），以及在 Windows 上一個帶有 *.pdb* 副檔名、包含除錯資訊的檔案。接下來，你可以執行 *main* 或 *main.exe*，如下所示：

```console
$ ./main # Windows 是 .\main.exe
```

如果這裡的 *main.rs* 是上文所述的 “Hello, world!” 程式，那麼在終端上就會打印出 `Hello, world!`。

如果你更熟悉 Ruby、Python 或 JavaScript 這樣的動態語言，可能不太習慣把編譯和執行分成兩個獨立步驟。Rust 是一種 **預先編譯**（*ahead-of-time compiled*）語言，這意味著你可以先將程式編譯好，再把可執行檔案交給其他人；即使他們沒有安裝 Rust，也可以直接執行。如果你給別人的是一個 *.rb*、*.py* 或 *.js* 檔案，他們就需要分別安裝 Ruby、Python 或 JavaScript 的實現（執行時環境）。不過在這些語言中，編譯和執行程式通常只需要一條命令。語言設計中的一切都是權衡取捨。

僅僅使用 `rustc` 編譯簡單程式是沒問題的，不過隨著專案的增長，你可能需要管理你專案的方方面面，並讓程式碼易於分享。接下來，我們要介紹一個叫做 Cargo 的工具，它會幫助你編寫真實世界中的 Rust 程式。

[troubleshooting]: ch01-01-installation.html#故障排除troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
