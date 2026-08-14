# Rust 程式設計語言

[Rust 程式設計語言](title-page.md)
[前言](foreword.md)
[簡介](ch00-00-introduction.md)

## 入門指南

- [入門指南](ch01-00-getting-started.md)
    - [安裝](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [寫個猜數字遊戲](ch02-00-guessing-game-tutorial.md)

- [常見程式設計概念](ch03-00-common-programming-concepts.md)
    - [變數與可變性](ch03-01-variables-and-mutability.md)
    - [資料型別](ch03-02-data-types.md)
    - [函式](ch03-03-how-functions-work.md)
    - [註釋](ch03-04-comments.md)
    - [控制流](ch03-05-control-flow.md)

- [認識所有權](ch04-00-understanding-ownership.md)
    - [什麼是所有權？](ch04-01-what-is-ownership.md)
    - [引用與借用](ch04-02-references-and-borrowing.md)
    - [Slice 型別](ch04-03-slices.md)

- [使用結構體組織相關聯的資料](ch05-00-structs.md)
    - [結構體的定義和例項化](ch05-01-defining-structs.md)
    - [結構體示例程式](ch05-02-example-structs.md)
    - [方法語法](ch05-03-method-syntax.md)

- [列舉和模式匹配](ch06-00-enums.md)
    - [列舉的定義](ch06-01-defining-an-enum.md)
    - [`match` 控制流結構](ch06-02-match.md)
    - [`if let` 簡潔控制流](ch06-03-if-let.md)

## 基本 Rust 技能

- [使用包、Crate 和模組管理不斷增長的專案](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [包和 Crate](ch07-01-packages-and-crates.md)
    - [定義模組來控制作用域與私有性](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [引用模組專案的路徑](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [使用 `use` 關鍵字將路徑引入作用域](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [將模組拆分成多個檔案](ch07-05-separating-modules-into-different-files.md)

- [常見集合](ch08-00-common-collections.md)
    - [使用 Vector 儲存列表](ch08-01-vectors.md)
    - [使用字串儲存 UTF-8 編碼的文本](ch08-02-strings.md)
    - [使用 Hash Map 儲存鍵值對](ch08-03-hash-maps.md)

- [錯誤處理](ch09-00-error-handling.md)
    - [用 `panic!` 處理不可恢復的錯誤](ch09-01-unrecoverable-errors-with-panic.md)
    - [用 `Result` 處理可恢復的錯誤](ch09-02-recoverable-errors-with-result.md)
    - [要不要 `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [泛型、Trait 和生命週期](ch10-00-generics.md)
    - [泛型資料型別](ch10-01-syntax.md)
    - [Trait：定義共同行為](ch10-02-traits.md)
    - [生命週期確保引用有效](ch10-03-lifetime-syntax.md)

- [編寫自動化測試](ch11-00-testing.md)
    - [如何編寫測試](ch11-01-writing-tests.md)
    - [控制測試如何執行](ch11-02-running-tests.md)
    - [測試的組織結構](ch11-03-test-organization.md)

- [一個 I/O 專案：構建命令列程式](ch12-00-an-io-project.md)
    - [接受命令列引數](ch12-01-accepting-command-line-arguments.md)
    - [讀取檔案](ch12-02-reading-a-file.md)
    - [重構以改進模組化與錯誤處理](ch12-03-improving-error-handling-and-modularity.md)
    - [採用測試驅動開發完善庫的功能](ch12-04-testing-the-librarys-functionality.md)
    - [處理環境變數](ch12-05-working-with-environment-variables.md)
    - [將錯誤資訊輸出到標準錯誤而不是標準輸出](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Rust 程式設計思想

- [Rust 中的函式式語言功能：迭代器與閉包](ch13-00-functional-features.md)
    - [閉包：可以捕獲其環境的匿名函式](ch13-01-closures.md)
    - [使用迭代器處理元素序列](ch13-02-iterators.md)
    - [改進之前的 I/O 專案](ch13-03-improving-our-io-project.md)
    - [效能比較：迴圈對迭代器](ch13-04-performance.md)

- [更多關於 Cargo 和 Crates.io 的內容](ch14-00-more-about-cargo.md)
    - [採用釋出配置自定義構建](ch14-01-release-profiles.md)
    - [將 crate 釋出到 Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo 工作空間](ch14-03-cargo-workspaces.md)
    - [使用 `cargo install` 從 Crates.io 安裝二進位制檔案](ch14-04-installing-binaries.md)
    - [Cargo 自定義擴充套件命令](ch14-05-extending-cargo.md)

- [智慧指標](ch15-00-smart-pointers.md)
    - [使用 `Box<T>` 指向堆上資料](ch15-01-box.md)
    - [使用 `Deref` Trait 將智慧指標當作常規引用處理](ch15-02-deref.md)
    - [使用 `Drop` Trait 執行清理程式碼](ch15-03-drop.md)
    - [`Rc<T>` 引用計數智慧指標](ch15-04-rc.md)
    - [`RefCell<T>` 與內部可變性模式](ch15-05-interior-mutability.md)
    - [引用迴圈會導致記憶體洩漏](ch15-06-reference-cycles.md)

- [無畏併發](ch16-00-concurrency.md)
    - [使用執行緒同時地執行程式碼](ch16-01-threads.md)
    - [使用訊息傳遞線上程間通訊](ch16-02-message-passing.md)
    - [共享狀態併發](ch16-03-shared-state.md)
    - [使用 `Sync` 與 `Send` Traits 的可擴充套件併發：](ch16-04-extensible-concurrency-sync-and-send.md)

- [Rust 的物件導向程式設計特性](ch17-00-oop.md)
    - [物件導向語言的特徵](ch17-01-what-is-oo.md)
    - [顧及不同型別值的 trait 物件](ch17-02-trait-objects.md)
    - [物件導向設計模式的實現](ch17-03-oo-design-patterns.md)

## 高階主題

- [模式與模式匹配](ch18-00-patterns.md)
    - [所有可能會用到模式的位置](ch18-01-all-the-places-for-patterns.md)
    - [Refutability（可反駁性）: 模式是否會匹配失效](ch18-02-refutability.md)
    - [模式語法](ch18-03-pattern-syntax.md)

- [高階特徵](ch19-00-advanced-features.md)
    - [不安全 Rust](ch19-01-unsafe-rust.md)
    - [高階 trait](ch19-03-advanced-traits.md)
    - [高階型別](ch19-04-advanced-types.md)
    - [高階函式與閉包](ch19-05-advanced-functions-and-closures.md)
    - [宏](ch19-06-macros.md)

- [最後的專案：構建多執行緒 web server](ch20-00-final-project-a-web-server.md)
    - [建立單執行緒 web server](ch20-01-single-threaded.md)
    - [將單執行緒 server 變為多執行緒 server](ch20-02-multithreaded.md)
    - [優雅停機與清理](ch20-03-graceful-shutdown-and-cleanup.md)

- [附錄](appendix-00.md)
    - [A - 關鍵字](appendix-01-keywords.md)
    - [B - 運算子與符號](appendix-02-operators.md)
    - [C - 可派生的 trait](appendix-03-derivable-traits.md)
    - [D - 實用開發工具](appendix-04-useful-development-tools.md)
    - [E - 版本](appendix-05-editions.md)
    - [F - 本書譯本](appendix-06-translation.md)
    - [G - Rust 是如何開發的與 “Nightly Rust”](appendix-07-nightly-rust.md)
