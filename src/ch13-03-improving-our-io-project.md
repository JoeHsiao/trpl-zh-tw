## 改進 I/O 專案

[ch13-03-improving-our-io-project.md](https://github.com/rust-lang/book/blob/4b71f17f7daba738a1363862dacb818d9b12fb81/src/ch13-03-improving-our-io-project.md)

掌握了這些關於迭代器的新知識後，我們可以使用迭代器來改進第十二章中 I/O 專案的實現來使得程式碼更簡潔明瞭。接下來，讓我們看看迭代器如何改進 `Config::build` 函式和 `search` 函式的實現。

### 使用迭代器消除 `clone`

在示例 12-6 中，我們增加了一些程式碼獲取一個 `String` 型別的 slice 並建立一個 `Config` 結構體的例項，它們索引 slice 中的值並克隆這些值以便 `Config` 結構體可以擁有這些值。在示例 13-17 中重現了第十二章結尾示例 12-23 中 `Config::build` 函式的實現：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

<span class="caption">示例 13-17：重現示例 12-23 的 `Config::build` 函式</span>

當時我們說過不必擔心低效的 `clone` 呼叫，因為我們以後會將其移除。好吧，就是現在！

起初這裡需要 `clone` 的原因是引數 `args` 中有一個 `String` 元素的 slice，而 `build` 函式並不擁有 `args`。為了能夠返回 `Config` 例項的所有權，我們不得不克隆 `Config` 中欄位 `query` 和 `file_path` 的值，這樣 `Config` 例項就能擁有這些值。

在學習了迭代器之後，我們可以將 `build` 函式改為獲取一個有所有權的迭代器作為引數，而不是借用 slice。我們將使用迭代器功能代替之前檢查 slice 長度和索引特定位置的程式碼。這樣可以更清晰地表達 `Config::build` 函式的操作，因為迭代器會負責訪問這些值。

一旦 `Config::build` 獲取了迭代器的所有權並不再使用借用的索引操作，就可以將迭代器中的 `String` 值移動到 `Config` 中，而不是呼叫 `clone` 分配新的空間。

#### 直接使用返回的迭代器

開啟 I/O 專案的 *src/main.rs* 檔案，它看起來應該像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

首先我們修改第十二章結尾示例 12-24 中的 `main` 函式的開頭為示例 13-18 中的程式碼。在更新 `Config::build` 之前這些程式碼還不能編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<span class="caption">示例 13-18：將 `env::args` 的返回值傳遞給 `Config::build`</span>

`env::args` 函式返回一個迭代器！不同於將迭代器的值收集到一個 vector 中接著傳遞一個 slice 給 `Config::build`，現在我們直接將 `env::args` 返回的迭代器的所有權傳遞給 `Config::build`。

接下來需要更新 `Config::build` 的定義。在 I/O 專案的 *src/lib.rs* 中，將 `Config::build` 的簽名改為如示例 13-19 所示。這仍然不能編譯因為我們還需更新函式體。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

<span class="caption">示例 13-19：以迭代器作為引數更新 `Config::build` 的簽名</span>

`env::args` 函式的標準庫文件顯示，它返回的迭代器的型別為 `std::env::Args`，並且這個型別實現了 `Iterator` trait 並返回 `String` 值。

我們已經更新了 `Config::build` 函式的簽名，因此引數 `args` 有一個帶有 trait 約束 `impl Iterator<Item = String>` 的泛型型別，而不是 `&[String]`。這裡用到了第十章[“trait 作為引數”][impl-trait]部分討論過的 `impl Trait` 語法，這意味著 `args` 可以是任何實現了 `Iterator` trait 並返回 `String` 項（item）的型別。

由於我們獲取了 `args` 的所有權，並且將通過迭代來修改 `args`，因此我們可以在 `args` 引數的宣告中新增 `mut` 關鍵字，使其可變。

<a id="using-iterator-trait-methods-instead-of-indexing"></a>

#### 使用 `Iterator` trait 方法

接下來，我們將修改 `Config::build` 的函式體。因為 `args` 實現了 `Iterator` trait，因此我們知道可以對其呼叫 `next` 方法！示例 13-20 更新了示例 12-23 中的程式碼，以使用 `next` 方法：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

<span class="caption">示例 13-20：修改 `Config::build` 的函式體來使用迭代器方法</span>

請記住 `env::args` 返回值的第一個值是程式的名稱。我們希望忽略它並獲取下一個值，所以首先呼叫 `next` 且不對其返回值做任何操作。然後，我們再次呼叫 `next` 來獲取要放入 `Config` 結構體的 `query` 欄位的值。如果 `next` 返回 `Some`，使用 `match` 來提取其值。如果它返回 `None`，則意味著沒有提供足夠的引數並通過 `Err` 值提早返回。我們對 `file_path` 的值也進行同樣的操作。

<a id="making-code-clearer-with-iterator-adapters"></a>

### 使用迭代器介面卡簡化程式碼

I/O 專案中其他可以利用迭代器的地方是 `search` 函式，示例 13-21 中重現了第十二章結尾示例 12-19 中此函式的定義：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

<span class="caption">示例 13-21：示例 12-19 中 `search` 函式的定義</span>

可以通過使用迭代器介面卡方法來編寫更簡明的程式碼。這樣做還可以避免使用一個可變的中間 `results` vector。函數語言程式設計風格傾向於最小化可變狀態的數量來使程式碼更清晰。去除可變狀態可能會使未來的並行搜尋最佳化變得更容易，因為我們不必管理對 `results` vector 的併發訪問。示例 13-22 展示了這一變化：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<span class="caption">示例 13-22：在 `search` 函式實現中使用迭代器介面卡</span>

回憶一下，`search` 函式的目的是返回所有 `contents` 中包含 `query` 的行。類似於示例 13-16 中的 `filter` 例子，這段程式碼使用 `filter` 介面卡來保留 `line.contains(query)` 返回 `true` 的行。接著使用 `collect` 將匹配行收集到另一個 vector 中。這樣就容易多了！嘗試對 `search_case_insensitive` 函式做出同樣的使用迭代器方法的修改吧。

<a id="choosing-between-loops-or-iterators"></a>

### 在迴圈和迭代器之間選擇

接下來的邏輯問題就是在程式碼中應該選擇哪種風格，以及原因：是使用示例 13-21 中的原始實現還是使用示例 13-22 中使用迭代器的版本？大部分 Rust 程式設計師傾向於使用迭代器風格。開始這有點難以掌握，不過一旦你對不同迭代器的工作方式有了感覺之後，迭代器反而更容易理解。相比擺弄不同的迴圈並建立新 vector，（迭代器）程式碼則更關注迴圈的高層次目的。這抽象掉那些老生常談的程式碼，這樣就更容易看清程式碼所特有的概念，比如迭代器中每個元素必須滿足的過濾條件。

不過這兩種實現真的完全等價嗎？直覺上的假設是更底層的迴圈會更快一些。讓我們聊聊效能吧。

[impl-trait]: ch10-02-traits.html#使用-trait-作為引數
