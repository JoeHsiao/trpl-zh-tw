# 面向物件程式設計特性

[ch18-00-oop.md](https://github.com/rust-lang/book/blob/d46785983db2d2f94ca3d571db2cfbad0f5ad3e6/src/ch18-00-oop.md)

面向物件程式設計（Object-Oriented Programming，OOP）是一種對程式進行建模的方式。物件（object）作為程式設計概念，最早出現在 20 世紀 60 年代的 Simula 程式語言中。這些物件影響了 Alan Kay 的程式設計架構，在那套架構裡，物件會互相傳遞訊息。1967 年，他創造了 **面向物件程式設計**（*object-oriented programming*）這一術語。對於 OOP 到底是什麼，存在許多彼此競爭的定義；按其中一些定義，Rust 是面向物件的，而按另一些定義，它又不是。在本章中，我們會考察一些通常被認為具有“面向物件”特徵的能力，並看看這些特徵在符合 Rust 習慣的寫法中是如何體現的。隨後，我們還會展示如何在 Rust 中實現一種面向物件的設計模式，並討論：與改用 Rust 自身的某些優勢來實現同類方案相比，這樣做的取捨是什麼。
