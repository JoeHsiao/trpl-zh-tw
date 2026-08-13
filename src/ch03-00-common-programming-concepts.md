# 常見程式設計概念

[ch03-00-common-programming-concepts.md](https://github.com/rust-lang/book/blob/9cc190796f28505c7a9a9cacea42f50d895ff3bd/src/ch03-00-common-programming-concepts.md)

本章介紹一些幾乎出現在所有程式語言中的概念，以及它們在 Rust 中是如何工作的。許多程式語言在其核心層面上都有很多共通之處。本章介紹的概念沒有一個是 Rust 獨有的，不過我們會在 Rust 的語境下討論它們，並解釋圍繞這些概念的使用慣例。

具體來說，你將學習變數、基本型別、函式、註釋和控制流。這些基礎內容會出現在每一個 Rust 程式中，儘早掌握它們會讓你在起步時擁有一個堅實的基礎。

> #### 關鍵字
> Rust 語言有一組保留的 **關鍵字**（*keywords*），就像大部分語言一樣，它們只能由語言本身使用。記住，你不能使用這些關鍵字作為變數或函式的名稱。大部分關鍵字有特殊的意義，你將在 Rust 程式中使用它們完成各種任務；一些關鍵字目前沒有相應的功能，是為將來可能新增的功能保留的。可以在[附錄 A][appendix_a]<!-- ignore --> 中找到關鍵字的列表。

[appendix_a]: appendix-01-keywords.html
