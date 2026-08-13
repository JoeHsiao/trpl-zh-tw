# 一個 I/O 專案：構建一個命令列程式

[ch12-00-an-io-project.md](https://github.com/rust-lang/book/blob/d7c0e477a22bcb37fdb290c6046058565d6738c2/src/ch12-00-an-io-project.md)

本章既是對目前所學諸多技能的一次回顧，也會進一步探索一些標準庫功能。我們將構建一個與檔案和命令列輸入/輸出互動的命令列工具，藉此練習一些你現在已經掌握的 Rust 概念。

Rust 的執行速度、安全性、單二進位制檔案輸出和跨平臺支援使其成為建立命令列程式的理想語言，所以我們的專案將建立一個我們自己版本的經典命令列搜尋工具：`grep`。grep 是 “**G**lobally search a **R**egular **E**xpression and **P**rint.” 的首字母縮寫。`grep` 最簡單的使用場景是在特定檔案中搜索指定字串。為此，`grep` 獲取一個檔案路徑和一個字串作為引數，接著讀取檔案並找到其中包含字串引數的行，然後打印出這些行。

在這個過程中，我們還會展示如何讓命令列工具使用許多其他命令列工具都會用到的終端特性。我們會讀取環境變數的值，讓使用者能夠配置工具的行為；也會將錯誤資訊列印到標準錯誤流（`stderr`）而不是標準輸出（`stdout`），這樣一來，例如使用者就可以把成功輸出重定向到檔案，同時仍然在螢幕上看到錯誤資訊。

一位 Rust 社群的成員，Andrew Gallant，已經建立了一個功能完整且非常快速的 `grep` 版本，名為 `ripgrep`。相比之下，我們的版本將非常簡單，本章將教會你一些幫助理解像 `ripgrep` 這樣真實專案的背景知識。

我們的 `grep` 專案將會結合之前所學的一些概念：

- 程式碼組織（[第七章][ch7]）
- vector 和字串（[第八章][ch8]）
- 錯誤處理（[第九章][ch9]）
- 合理地使用 trait 和生命週期（[第十章][ch10]）
- 編寫測試（[第十一章][ch11]）

另外還會簡要地講到閉包、迭代器和 trait 物件，它們分別會在[第十三章][ch13]和[第十八章][ch18]中詳細介紹。

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
