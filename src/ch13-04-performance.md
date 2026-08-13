<a id="comparing-performance-loops-vs-iterators"></a>

## 效能：迴圈 VS 迭代器

[ch13-04-performance.md](https://github.com/rust-lang/book/blob/4b71f17f7daba738a1363862dacb818d9b12fb81/src/ch13-04-performance.md)

為了決定是否使用迴圈或迭代器，你需要了解哪個實現更快：使用顯式 `for` 迴圈的 `search` 函式版本，還是使用迭代器的版本。

我們進行了一個基準測試，將阿瑟·柯南·道爾的《福爾摩斯探案集》全文載入到一個 `String` 中，並在內容中查詢單詞 *the*。以下是使用 `for` 迴圈版本和使用迭代器版本的 `search` 函式的基準測試結果：

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

兩種實現的效能相近！這裡我們不會解釋效能測試程式碼，因為重點並不是證明這兩個版本完全等價，而是大致感受一下它們在效能上的對比。

對於更全面的基準測試，你應該嘗試把不同大小的文字作為 `contents`、把不同的單詞和不同長度的單詞作為 `query`，再結合其他各種變化一起測試。重點在於：迭代器雖然是高階抽象，但編譯出的程式碼和你手寫底層程式碼時大致相同。迭代器是 Rust 的**零成本抽象**（*zero-cost abstractions*）之一，也就是說使用這種抽象不會帶來額外的執行時開銷。這和 C++ 的最初設計者與實現者 Bjarne Stroustrup 在 2012 年 ETAPS 主題演講《Foundations of C++》中對零開銷的定義類似：

> In general, C++ implementations obey the zero-overhead principle: What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better.
>
> 總的來說，C++ 的實現遵循了零開銷原則：不使用的功能無需為其付出代價；而已經使用的功能，也不可能透過手寫程式碼做得更好。

在很多情況下，使用迭代器的 Rust 程式碼會編譯成與你手寫出來相同的彙編。像迴圈展開、消除陣列訪問邊界檢查這樣的最佳化都會生效，並讓最終生成的程式碼極其高效。現在你已經知道這一點了，就可以放心使用迭代器和閉包：它們讓程式碼看起來更高層，但不會因此帶來執行時效能損失。

## 總結

閉包和迭代器是 Rust 受函數語言程式設計語言理念啟發而來的特性。它們有助於 Rust 以低層效能清晰地表達高層概念。閉包和迭代器的實現不會影響執行時效能，這正是 Rust 致力於提供零成本抽象這一目標的一部分。

現在我們已經改進了 I/O 專案的表達力，接下來看看 `cargo` 的更多功能，它們會幫助我們把專案分享給全世界。
