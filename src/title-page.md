# Rust 程式設計語言

[title-page.md](https://github.com/rust-lang/book/blob/f660f341887c8bbcd6c24fbfdf5d2a262f523965/src/title-page.md)

**本書的英文原版作者為 Steve Klabnik 和 Carol Nichols，並由 Rust 社群補充完善。本簡體中文譯本由 Rust 中文社群翻譯。**

本書假設你使用的是 Rust 1.90.0（2025-09-18 釋出）或更高版本，並在所有專案的 Cargo.toml 檔案中透過 `edition = "2024"`將其配置為使用 Rust 2024 Edition 慣用法。請檢視[第一章的 “安裝” 部分][install]瞭解如何安裝和升級 Rust，並檢視[附錄 E][appendix-e]瞭解版本相關的資訊。

本書的英文原版 HTML 格式可以在 [https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/) 線上閱讀；使用 `rustup` 安裝的 Rust 也包含一份英文離線版，執行 `rustup docs --book` 即可開啟。

本書還有一些社群 [翻譯版本][translations]。（譯者注：簡體中文譯本可以在 [https://kaisery.github.io/trpl-zh-cn/](https://kaisery.github.io/trpl-zh-cn/) 線上閱讀，PDF 版本請下載 [Rust 程式設計語言 簡體中文版.pdf](https://kaisery.github.io/trpl-zh-cn/Rust%20%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80%20%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87%E7%89%88.pdf)）

本書也有[由 No Starch Press 出版的紙質版和電子版][nsprust]。

[install]: ch01-01-installation.html
[appendix-e]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[translations]: appendix-06-translation.html

> **🚨 想要具有互動性的學習體驗嗎？試試 Rust Book 的另一個版本，其中包括測驗、高亮、視覺化等功能**：<https://rust-book.cs.brown.edu>
