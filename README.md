> 本專案為 [trpl-zh-cn](https://github.com/KaiserY/trpl-zh-cn) 的繁體中文自動化翻譯版本,
> 原文版權歸屬 Rust 中文社區(MIT License)。翻譯內容由 OpenCC 自動轉換產生,可能有未校對之處。

---

# Rust 程式設計語言（2024 edition）繁體中文版

![Build Status](https://github.com/KaiserY/trpl-zh-cn/workflows/CI/badge.svg)

## 狀態

- 2024 edtion 施工完畢。

PS:

- 對照原始碼位置：[https://github.com/rust-lang/book/tree/main/src][source]
- 注意原始碼 `src` 目錄一般會比 [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/) 要新，如果遇到衝突建議對照 `src` 目錄。
- 每章翻譯開頭都帶有官方連結和 commit hash 的註釋，若發現與官方不一致，歡迎 Issue 或 PR

[source]: https://github.com/rust-lang/book/tree/main/src

## 校對

部分章節採用 Codex 輔助校對。提示詞可參考 [proofreading_prompt.md](proofreading_prompt.md)

## 靜態頁面構建與文件撰寫

### 構建

你可以將本 mdbook 構建成一系列靜態 html 頁面。這裡我們採用 [mdbook](https://rust-lang.github.io/mdBook/index.html) 打包出靜態網頁。在這之前，你需要安裝 [Rust](https://www.rust-lang.org/zh-CN/)。

全域性安裝 mdbook

```bash
cargo install mdbook
```

cd 到專案目錄，然後開始構建。構建好的靜態文件會出現在 "./book/html" 中

```bash
mdbook build
```

在本地部署 HTTP 伺服器以閱讀文件：

```bash
mdbook serve
```

## 社群資源

- Rust 語言中文社群：<https://rustcc.cn/>
- Rust 程式語言社群 1 群，群號：303838735（已滿，只能內部邀請）
- Rust 程式語言社群 2 群，群號：813448660

## PDF

[Rust 程式設計語言 簡體中文版.pdf](https://kaisery.github.io/trpl-zh-cn/Rust%20%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80%20%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87%E7%89%88.pdf)

- 由 [mdbook-typst-pdf](https://github.com/KaiserY/mdbook-typst-pdf) 生成，有任何問題歡迎 issue 或 PR

## EPUB

你可以使用專案中提供的 Rust 構建工具將本書編譯為 EPUB 電子書。

在此之前，請確保您的系統已安裝了 [Pandoc](https://pandoc.org/)。

在專案根目錄下，執行以下命令開始編譯：

```bash
cargo run --release --manifest-path epub-builder/Cargo.toml
```

編譯成功後，會在根目錄下生成 `rust_programming_language.epub`。

## GitBook

本翻譯主要採用 [mdBook](https://github.com/rust-lang-nursery/mdBook) 格式。同時支援 [GitBook](https://github.com/GitbookIO/gitbook)，但會缺失部分功能，如一些程式碼沒有語法高亮。

本翻譯加速檢視站點有：

- 深圳站點：<http://120.78.128.153/rustbook>

[GitBook.com](https://www.gitbook.com/) 地址：<https://kaisery.github.io/trpl-zh-cn/>
