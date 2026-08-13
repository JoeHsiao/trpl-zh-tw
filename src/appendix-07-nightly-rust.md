## 附錄 G：Rust 是如何開發的與 “Nightly Rust”

[appendix-07-nightly-rust.md](https://github.com/rust-lang/book/blob/af415fc6c8a6823dfb4595074f27d5a3e9e2fe49/src/appendix-07-nightly-rust.md)

本附錄介紹 Rust 是如何開發的以及這對你作為 Rust 開發者的影響。

### 無停滯穩定

作為一門語言，Rust **十分**注重程式碼的穩定性。我們希望 Rust 成為你可依賴的堅實基礎，假如事務持續地在變化，這個希望就實現不了。但與此同時，如果不能實驗新功能的話，在釋出之前我們又無法發現其中重大的缺陷，而一旦釋出便再也沒有修改的機會了。

對於這個問題我們的解決方案被稱為 “無停滯穩定”（“stability without stagnation”），其指導性原則是：無需擔心升級到最新的穩定版 Rust。每次升級應該是無痛的，並應帶來新功能，更少的 bug 和更快的編譯速度。

### Choo, Choo! 釋出通道和釋出時刻表（Riding the Trains）

Rust 開發運行於一個**釋出時刻表**（_train schedule_）之上。也就是說，所有的開發工作都發生在 Rust 倉庫的主分支上。釋出採用 software release train 模型，它曾被 Cisco IOS 及其他軟體專案採用。Rust 有三個**釋出通道**（_release channel_）：

- Nightly
- Beta
- Stable（穩定版）

大部分 Rust 開發者主要採用穩定版通道，不過希望實驗新功能的開發者可能會使用 nightly 或 beta 版。

如下是一個開發和釋出過程如何運轉的例子：假設 Rust 團隊正在進行 Rust 1.5 的釋出工作。該版本釋出於 2015 年 12 月，不過這裡只是為了提供一個真實的版本號。Rust 新增了一項功能：一個新提交進入了主分支。每天晚上，都會產生一個新的 nightly 版本。每天都是釋出日，而這些釋出由釋出基礎設施自動完成。所以隨著時間推移，釋出軌跡看起來像這樣，版本每晚一發：

```text
nightly: * - - * - - *
```

每六週時間，是準備釋出新版本的時候了！Rust 倉庫的 `beta` 分支會從 nightly 所使用的主分支分出。現在，有了兩個釋出渠道：

```text
nightly: * - - * - - *
                     |
beta:                *
```

大部分 Rust 使用者不會主要使用 beta 版本，不過在 CI 系統中對 beta 版本進行測試能夠幫助 Rust 發現可能的迴歸缺陷（regression）。同時，每晚仍產生 nightly 釋出：

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

比如我們發現了一個迴歸缺陷。好訊息是，在這些迴歸缺陷流入穩定釋出之前，我們還有一些時間來測試 beta 版本！修復會先應用到主分支，因此 nightly 版本先得到修復；然後再把修復回移植到 `beta` 分支，於是新的 beta 釋出就產生了：

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

第一個 beta 版的六週後，是釋出穩定版的時候了！`stable` 分支從 `beta` 分支生成：

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

好的！Rust 1.5 釋出了！然而，我們忘了些東西：因為又過了六週，我們還需釋出**下一個** Rust 的 beta 版，Rust 1.6。所以從 `beta` 生成 `stable` 分支後，新版的 `beta` 分支也再次從 `nightly` 生成：

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

這被稱為 “train model”，因為每六週，一個版本 “離開車站”（“leaves the station”），不過從 beta 通道到達穩定通道還需歷經一段旅程。

Rust 每六週釋出一個版本，如時鐘般準確。如果你知道了某個 Rust 版本的釋出時間，就可以知道下個版本的時間：六週後。每六週釋出版本的一個好的方面是下一班車會來得更快。如果特定版本碰巧缺失某個功能也無需擔心：另一個版本很快就會到來！這有助於減少因臨近發版時間而偷偷釋出未經完善的功能的壓力。

多虧了這個過程，你總是可以切換到下一版本的 Rust 並驗證是否可以輕易的升級：如果 beta 版不能如期工作，你可以向 Rust 團隊報告並在釋出穩定版之前得到修復！beta 版造成的破壞是非常少見的，不過 `rustc` 也不過是一個軟體，難免會有 bug。

### 維護時間

Rust 專案僅對最近的穩定版本提供支援。當釋出新穩定版本時，舊版本即達到生命週期終止（EOL, end of life），這意味著每個版本的支援期為六週。

### 不穩定功能

這個釋出模型中另一個值得注意的地方：不穩定功能（unstable features）。Rust 使用一種叫做 **feature flags** 的技術來決定某個釋出中啟用了哪些功能。如果一個新功能仍在積極開發中，它會進入主分支，因此也會出現在 nightly 版本里，但會被放在某個 **功能標記** 之後。作為使用者，如果你想嘗試這個仍在開發中的功能，可以這麼做，但你必須使用 nightly 版 Rust，並在原始碼中新增相應的標記來顯式啟用它。

如果使用的是 beta 或穩定版 Rust，則不能使用任何功能標記。這是在新功能被宣佈為永久穩定之前讓大家提前實際使用它們的關鍵。這既滿足了希望使用最尖端技術的同學，那些堅持穩定版的同學也知道其程式碼不會被破壞。這就是無停滯穩定。

本書只包含穩定的功能，因為還在開發中的功能仍可能改變，當其進入穩定版時肯定會與編寫本書的時候有所不同。你可以在網上獲取只存在 nightly 版中功能的文件。

### Rustup 和 Rust Nightly 的職責

Rustup 使得在不同 Rust 釋出通道之間切換變得很容易，無論是全域性還是按專案都可以。預設情況下，你安裝的是穩定版 Rust。例如，要安裝 nightly：

```console
$ rustup toolchain install nightly
```

你也可以用 `rustup` 檢視已經安裝的所有**工具鏈**（_toolchains_，也就是 Rust 釋出版本及其相關元件）。下面是一位作者的 Windows 電腦上的例子：

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

如你所見，預設是穩定版。大部分 Rust 使用者在大部分時間使用穩定版。你可能也會這麼做，不過如果你關心最新的功能，可以為特定專案使用 nightly 版。為此，可以在專案目錄使用 `rustup override` 來設定當前目錄 `rustup` 使用 nightly 工具鏈：

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

現在，每次在 _~/projects/needs-nightly_ 中呼叫 `rustc` 或 `cargo`，`rustup` 會確保使用 nightly 版 Rust 而非預設的穩定版。在你有很多 Rust 專案時大有裨益！

### RFC 流程和團隊

那麼你如何瞭解這些新功能呢？Rust 開發模式遵循一個 **Request For Comments (RFC) 流程**。如果你希望改進 Rust，可以編寫一個提案，也就是 RFC。

任何人都可以編寫 RFC 來改進 Rust，同時這些 RFC 會被 Rust 團隊評審和討論，他們由很多不同分工的子團隊組成。這裡是 [Rust 官網上](https://www.rust-lang.org/governance) 所有團隊的總列表，其包含了專案中每個領域的團隊：語言設計、編譯器實現、基礎設施、文件等。各個團隊會閱讀相應的提議和評論，發表自己的意見，並最終達成接受或回絕功能的一致。

如果功能被接受了，Rust 倉庫裡就會開一個 issue，然後就會有人去實現它。最終完成實現的人，很可能並不是最初提出這個功能的人！當實現準備好之後，它會合併到主分支，並被放在一個 feature gate 之後，正如 [“不穩定功能”](#不穩定功能) 一節所討論的那樣。

在稍後的某個時間，一旦使用 nightly 版的 Rust 團隊能夠嘗試這個功能了，團隊成員會討論這個功能，它如何在 nightly 中工作，並決定是否應該進入穩定版。如果決定繼續推進，功能開關會移除，然後這個功能就被認為是穩定的了！乘著“釋出列車”，最終在新的穩定版 Rust 中出現。
