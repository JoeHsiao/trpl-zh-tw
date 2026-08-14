## 測試的組織結構

[ch11-03-test-organization.md](https://github.com/rust-lang/book/blob/99071589c5358114de6324d9aa2643caeee305bd/src/ch11-03-test-organization.md)

本章一開始就提到，測試是一個複雜的概念，而且不同的開發者也採用不同的術語和組織。Rust 社群傾向於根據測試的兩個主要分類來考慮問題：**單元測試**（*unit tests*）與**整合測試**（*integration tests*）。單元測試傾向於更小而更集中，在隔離的環境中一次測試一個模組，並且可以測試私有介面。而**整合測試**對於你的庫來說則完全是外部的。它們與其他外部程式碼一樣，透過相同的方式使用你的程式碼，只測試公有介面而且每個測試都有可能會測試多個模組。

為了保證你的庫能夠按照你的預期執行，從獨立和整體的角度編寫這兩類測試都是非常重要的。

### 單元測試

單元測試的目的是在與其他部分隔離的環境中測試每一個單元的程式碼，以便於快速而準確地驗證某個單元的程式碼功能是否符合預期。單元測試與它們要測試的程式碼共同存放在位於 *src* 目錄下相同的檔案中。規範是在每個檔案中建立包含測試函式的 `tests` 模組，並使用 `cfg(test)` 標註模組。

#### 測試模組和 `#[cfg(test)]`

測試模組的 `#[cfg(test)]` 註解告訴 Rust 只在執行 `cargo test` 時才編譯和執行測試程式碼，而在執行 `cargo build` 時不這麼做。這在只希望構建庫的時候可以節省編譯時間，並且因為它們並沒有包含測試，所以能減少編譯產生的檔案的大小。與之對應的整合測試因為位於另一個資料夾，所以它們並不需要 `#[cfg(test)]` 註解。然而單元測試位於與原始碼相同的檔案中，所以你需要使用 `#[cfg(test)]` 來指定它們不應該被包含進編譯結果中。

回憶本章第一部分新建的 `adder` 專案，Cargo 為我們生成了如下程式碼：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

上述程式碼就是自動生成的測試模組。`cfg` 屬性代表**配置**（*configuration*），它告訴 Rust 接下來的項只有在給定特定配置選項時，才會被包含。在這種情況下，配置選項是 `test`，即 Rust 所提供的用於編譯和執行測試的配置選項。透過使用 `cfg` 屬性，Cargo 只會在我們主動使用 `cargo test` 執行測試時才編譯測試程式碼。這包括測試模組中可能存在的輔助函式，以及標註為 `#[test]` 的函式。

<a id="testing-private-functions"></a>

#### 私有函式測試

測試社群中一直存在關於是否應該對私有函式直接進行測試的論戰，而在其他語言中想要測試私有函式是一件困難的，甚至是不可能的事。不過無論你堅持哪種測試意識形態，Rust 的私有性規則確實允許你測試私有函式。考慮示例 11-12 中帶有私有函式 `internal_adder` 的程式碼。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<span class="caption">示例 11-12：測試私有函式</span>

注意 `internal_adder` 函式並沒有標記為 `pub`。測試也只是 Rust 程式碼，而 `tests` 模組也只是另一個模組。正如[“路徑用於引用模組樹中的項”][paths] 部分所說，子模組中的項可以使用其祖先模組中的項。在這個測試中，我們透過 `use super::*` 將 `tests` 模組父模組中的所有項引入作用域，然後測試就可以呼叫 `internal_adder`。如果你並不認為應該測試私有函式，Rust 也不會強迫你這麼做。

### 整合測試

在 Rust 中，整合測試對於你需要測試的庫來說完全是外部的。同其他使用庫的程式碼一樣使用庫檔案，也就是說它們只能呼叫一部分庫中的公有 API。整合測試的目的是測試庫的多個部分能否一起正常工作。一些單獨能正確執行的程式碼單元整合在一起也可能會出現問題，所以整合測試的覆蓋率也是很重要的。為了建立整合測試，你需要先建立一個 *tests* 目錄。

#### *tests* 目錄

為了編寫整合測試，需要在專案根目錄建立一個 *tests* 目錄，與 *src* 同級。Cargo 知道如何去尋找這個目錄中的整合測試檔案。接著可以隨意在這個目錄中建立任意多的測試檔案，Cargo 會將每一個檔案當作單獨的 crate 來編譯。

讓我們來建立一個整合測試。保留示例 11-12 中 *src/lib.rs* 的程式碼。建立一個 *tests* 目錄，新建一個檔案 *tests/integration_test.rs*。目錄結構應該看起來像這樣：

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

將示例 11-13 中的程式碼輸入到 *tests/integration_test.rs* 檔案中。

<span class="filename">檔名：tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<span class="caption">示例 11-13：一個 `adder` crate 中函式的整合測試</span>

因為每一個 *tests* 目錄中的測試檔案都是完全獨立的 crate，所以需要將庫引入到每個測試 crate 的作用域中。為此與單元測試不同，我們需要在檔案頂部新增 `use adder::add_two;`，這在單元測試中是不需要的。

並不需要將 *tests/integration_test.rs* 中的任何程式碼標註為 `#[cfg(test)]`。 `tests` 資料夾在 Cargo 中是一個特殊的資料夾，Cargo 只會在執行 `cargo test` 時編譯這個目錄中的檔案。現在就執行 `cargo test` 試試：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

現在有了三個部分的輸出：單元測試、整合測試和文件測試。注意如果一個部分的任何測試失敗，之後的部分都不會執行。例如如果一個單元測試失敗，則不會有任何整合測試和文件測試的輸出，因為這些測試只會在所有單元測試都通過後才會執行。

第一部分單元測試與我們之前見過的一樣：每個單元測試一行（示例 11-12 中有一個叫做 `internal` 的測試），接著是一個單元測試的摘要行。

整合測試部分以行 `Running tests/integration_test.rs`開頭。接下來每一行是一個整合測試中的測試函式，以及一個位於 `Doc-tests adder` 部分之前的整合測試的摘要行。

每一個整合測試檔案有對應的測試結果部分，所以如果在 *tests* 目錄中增加更多檔案，測試結果中就會有更多整合測試結果部分。

我們仍然可以透過指定測試函式的名稱作為 `cargo test` 的引數來執行特定整合測試。也可以使用 `cargo test` 的 `--test` 後跟檔案的名稱來執行某個特定整合測試檔案中的所有測試：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

這個命令只執行 *tests/integration_test.rs* 檔案中的測試。

#### 整合測試中的子模組

隨著整合測試的增加，你可能希望在 `tests` 目錄建立更多檔案以便更好地組織它們；例如根據測試的功能來將測試分組。如前所述，*tests* 目錄中的每一個檔案都被編譯成一個單獨的 crate，這有助於建立獨立的作用域，以便更接近於終端使用者使用你的 crate 的方式。但這意味著，*tests* 目錄中的檔案的行為，和你在第七章中學習如何將程式碼分為模組和檔案時，學到的 *src* 中的檔案的行為不一樣。

當你有一些在多個整合測試檔案都會用到的幫助函式，而你嘗試按照第七章[“將模組移動到其他檔案”][separating-modules-into-files]部分的步驟將它們提取到一個通用的模組中時，*tests* 目錄中檔案行為的不同就會凸顯出來。例如，如果我們在 *tests/common.rs* 中建立一個名為 `setup` 的函式，並希望在多個測試檔案的測試函式中呼叫它，就可以在 `setup` 中新增想要複用的程式碼：

<span class="filename">檔名：tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

如果再次執行測試，將會在測試結果中看到一個新的對應 *common.rs* 檔案的測試結果部分，即便這個檔案並沒有包含任何測試函式，也沒有任何地方呼叫了 `setup` 函式：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

我們並不想要 `common` 出現在測試結果中顯示 `running 0 tests`。我們只是想與其他整合測試檔案共享一些程式碼而已。為了不讓 `common` 出現在測試輸出中，我們將建立 *tests/common/mod.rs*，而不是建立 *tests/common.rs* 。現在專案目錄結構看起來像這樣：

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

這是一種老的命名規範，正如第七章[“替代檔案路徑”][alt-paths]中提到的 Rust 仍然理解它們。這樣命名告訴 Rust 不要將 `common` 看作一個整合測試檔案。將 `setup` 函式程式碼移動到 *tests/common/mod.rs* 並刪除 *tests/common.rs* 檔案之後，測試輸出中將不會出現這一部分。*tests* 目錄中的子目錄不會被作為單獨的 crate 編譯或作為一個測試結果部分出現在測試輸出中。

一旦建立了 *tests/common/mod.rs*，就可以將其作為模組以便在任何整合測試檔案中使用。這裡是一個 *tests/integration_test.rs* 中呼叫 `setup` 函式的 `it_adds_two` 測試的示例：

<span class="filename">檔名：tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

注意 `mod common;` 宣告與示例 7-21 中展示的模組宣告相同。接著在測試函式中就可以呼叫 `common::setup()` 了。

#### 二進位制 crate 的整合測試

如果專案是二進位制 crate 並且只包含 *src/main.rs* 而沒有 *src/lib.rs*，這樣就不可能在 *tests* 目錄建立整合測試並也無法透過 `use` 語句將 *src/main.rs* 中定義的函式引入作用域。只有庫 crate 才會向其他 crate 暴露了可供呼叫和使用的函式；二進位制 crate 只意在單獨執行。

這就是許多 Rust 二進位制專案使用一個簡單的 *src/main.rs* 呼叫 *src/lib.rs* 中的邏輯的原因之一。因為透過這種結構，整合測試**就可以**透過 `use` 來測試庫 crate 中的重要功能了。而如果這些重要的功能沒有問題的話，*src/main.rs* 中的少量程式碼也就會正常工作且不需要測試。

## 總結

Rust 的測試功能提供了一個確保即使你改變了函式的實現方式，也能繼續以期望的方式執行的途徑。單元測試獨立地驗證庫的不同部分，也能夠測試私有函式實現細節。整合測試則檢查多個部分是否能結合起來正確地工作，並像其他外部程式碼那樣測試庫的公有 API。即使 Rust 的型別系統和所有權規則可以幫助避免某些型別的 bug，不過測試對於減少程式碼中不符合期望行為的邏輯 bug 仍然是很重要的。

讓我們將本章和前面各章所學的知識組合起來，在下一章一起編寫一個專案！

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#替代檔案路徑
