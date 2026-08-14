## 安裝

[ch01-01-installation.md](https://github.com/rust-lang/book/blob/369386fefd1138cbdf50ae628bae1ffc4ffce669/src/ch01-01-installation.md)

第一步是安裝 Rust。我們會通過 `rustup` 下載 Rust，這是一個管理 Rust 版本和相關工具的命令列工具。下載時需要聯網。

> 注意：如果你出於某些理由傾向於不使用 `rustup`，請到 [Rust 的其他安裝方法頁面][otherinstall] 檢視其它安裝選項。

接下來的步驟會安裝最新穩定版 Rust 編譯器。Rust 的穩定性保證意味著，本書中所有能夠編譯的示例，在更新的 Rust 版本中也應繼續能夠編譯。不同版本之間的輸出可能略有差異，因為 Rust 經常會改進錯誤資訊和警告。也就是說，按照這些步驟安裝的任何較新的穩定版 Rust，都應該能正常配合本書內容使用。

> ### 命令列標記
>
> 本章和全書中，我們會展示一些在終端中使用的命令。所有需要輸入到終端的行都以 `$` 開頭。你不需要輸入 `$` 字元；這裡顯示的 `$` 字元表示命令列提示符，僅用於提示每行命令的起點。不以 `$` 起始的行通常展示前一個命令的輸出。另外，PowerShell 專用的示例會採用 `>` 而不是 `$`。

### 在 Linux 或 macOS 上安裝 `rustup`

如果你使用 Linux 或 macOS，開啟終端並輸入如下命令：

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

這條命令會下載一個指令碼並啟動 `rustup` 工具的安裝，它會安裝最新穩定版 Rust。過程中可能會提示你輸入密碼。如果安裝成功，將會出現如下內容：

```text
Rust is installed now. Great!
```

另外，你還需要一個 *連結器（linker）*，它是 Rust 用來將編譯輸出連線成單個檔案的程式。你很可能已經安裝了它。如果遇到連結器錯誤，你應該安裝一個 C 編譯器，其中通常會包含連結器。C 編譯器本身也很有用，因為一些常見的 Rust 包依賴 C 程式碼，因此需要 C 編譯器。

在 macOS 上，你可以通過執行以下命令獲得 C 語言編譯器：

```console
$ xcode-select --install
```

Linux 使用者通常應根據自己發行版（distribution）的文件安裝 GCC 或 Clang。比如，如果你使用 Ubuntu，可以安裝 `build-essential` 包。

### 在 Windows 上安裝 `rustup`

在 Windows 上，前往 [https://www.rust-lang.org/tools/install][install] 並按照說明安裝 Rust。在安裝過程中的某一步，你會被提示安裝 Visual Studio。它提供了連結器以及編譯程式所需的本地庫。如果你在這一步需要更多幫助，請訪問 [https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]。

本書的餘下部分會使用能同時運行於 *cmd.exe* 和 PowerShell 的命令。如果存在特定差異，我們會解釋使用哪一個。

### 故障排除（Troubleshooting）

要檢查 Rust 是否安裝正確，開啟 shell 並輸入：

```console
$ rustc --version
```

你應該會看到已釋出的最新穩定版的版本號、commit hash 和 commit 日期，格式如下：

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

如果看到了這些資訊，就說明 Rust 已經安裝成功了！如果沒有看到，請按下面的方法檢查 Rust 是否在你的 `%PATH%` 系統變數中。

在 Windows CMD 中，請使用命令：

```console
> echo %PATH%
```

在 PowerShell 中，請使用命令：

```powershell
> echo $env:Path
```

在 Linux 和 macOS 中，請使用命令：

```console
$ echo $PATH
```

如果這些都沒問題但 Rust 仍然無法使用，還有很多地方可以求助。你可以在[社群頁面][community]檢視如何聯絡其他 Rustaceans（Rust 使用者對自己的一個戲稱）。

## 更新與解除安裝

通過 `rustup` 安裝 Rust 之後，更新到新發布的版本很簡單。只需要在 shell 中執行下面的更新指令碼：

```console
$ rustup update
```

若要解除安裝 Rust 和 `rustup`，請在 shell 中執行下面的解除安裝指令碼：

```console
$ rustup self uninstall
```

<a id="local-documentation"></a>

### 本地文件

安裝 Rust 時也會附帶一份文件的本地副本，供你離線閱讀。執行 `rustup doc` 即可在瀏覽器中開啟本地文件。

任何時候，如果標準庫提供了某個型別或函式，而你不確定它是做什麼的或該如何使用，請查閱應用程式介面（application programming interface，API）文件！

<a id="text-editors-and-integrated-development-environments"></a>

### 文本編輯器和整合開發環境（Integrated Development Environments, IDE）

本書不假設你使用什麼工具來編寫 Rust 程式碼。幾乎任何文本編輯器都能勝任！不過，許多文本編輯器和整合開發環境（IDE）都內建了對 Rust 的支援。你總是可以在 Rust 官網的[工具頁面][tools]找到一個相對較新的常見編輯器和 IDE 列表。

### 離線使用本書

在一些示例中，我們會使用標準庫之外的 Rust 包。要執行這些示例，你需要保持網路連線，或者提前下載好這些依賴。要預先下載依賴，可以執行以下命令。（稍後我們會詳細解釋 `cargo` 是什麼，以及這些命令分別有什麼作用。）

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

這會把這些包下載並快取起來，因此之後你就不需要再次下載它們了。執行完這些命令後，你無需保留 `get-dependencies` 資料夾。只要你執行過這些命令，在本書後續所有的 `cargo` 命令中，都可以使用 `--offline` 引數來使用這些已快取的版本，而不必嘗試聯網。

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
