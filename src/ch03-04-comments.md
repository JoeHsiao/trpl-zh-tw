## 註釋

[ch03-04-comments.md](https://github.com/rust-lang/book/blob/9cc190796f28505c7a9a9cacea42f50d895ff3bd/src/ch03-04-comments.md)

所有程式設計師都努力讓自己的程式碼易於理解，不過有時仍然需要額外的解釋。在這種情況下，程式設計師會在原始碼中留下 **註釋**（*comments*），編譯器會忽略它們，但閱讀原始碼的人可能會覺得這些註釋很有幫助。

這是一個簡單的註釋：

```rust
// hello, world
```

在 Rust 中，慣用的註釋風格是用兩個斜槓開始一條註釋，並讓註釋持續到該行末尾。對於跨越多行的註釋，你需要在每一行前面都加上 `//`，像這樣：

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

註釋也可以放在包含程式碼的行的末尾：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

不過，你更常見到的用法是把註釋放在它所解釋的程式碼上一行，像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust 還有另一種註釋，叫作文件註釋，我們會在第十四章的[“將 crate 釋出到 Crates.io”][publishing]部分討論它。

[publishing]: ch14-02-publishing-to-crates-io.html
