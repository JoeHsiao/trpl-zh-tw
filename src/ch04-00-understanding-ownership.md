# 認識所有權

[ch04-00-understanding-ownership.md](https://github.com/rust-lang/book/blob/a5e0c5b2c5f9054be3b961aea2c7edfeea591de8/src/ch04-00-understanding-ownership.md)

所有權是 Rust 最獨特的特性，也是對語言其餘部分影響最深的特性之一。它使 Rust 無需垃圾回收器（garbage collector）也能提供記憶體安全保證，因此理解所有權在 Rust 中是如何工作的非常重要。在本章中，我們將討論所有權，以及幾個相關特性：借用（borrowing）、slice，以及 Rust 如何在記憶體中佈局資料。
