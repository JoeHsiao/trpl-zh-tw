## 將模組拆分成多個檔案

[ch07-05-separating-modules-into-different-files.md](https://github.com/rust-lang/book/blob/79b9d15410a7b15a65cb86bcb40cbea99198a9e4/src/ch07-05-separating-modules-into-different-files.md)

到目前為止，本章的所有示例都在一個檔案中定義了多個模組。當模組變大時，你可能想把它們的定義移到單獨的檔案中，以便讓程式碼更容易瀏覽。

例如，我們從示例 7-17 中包含多個餐廳模組的程式碼開始。我們會將模組提取到各自的檔案中，而不是將所有模組都定義到 crate 根檔案中。在這裡，crate 根檔案是 *src/lib.rs*，不過這個過程也適用於 crate 根檔案是 *src/main.rs* 的二進位制 crate。

首先把 `front_of_house` 模組提取到它自己的檔案中。刪除 `front_of_house` 模組花括號內的程式碼，只保留 `mod front_of_house;` 宣告，這樣 *src/lib.rs* 就會只剩下示例 7-21 所示的程式碼。注意，在創建出示例 7-22 中的 *src/front_of_house.rs* 檔案之前，這段程式碼都無法編譯。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">示例 7-21: 宣告 `front_of_house` 模組，其內容將位於 *src/front_of_house.rs*</span>

接下來，把剛才花括號中的程式碼放進一個名為 *src/front_of_house.rs* 的新檔案中，如示例 7-22 所示。編譯器之所以知道要去這個檔案裡查詢，是因為它在 crate 根中看到了名為 `front_of_house` 的模組宣告。

<span class="filename">檔名：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">示例 7-22: 在 *src/front_of_house.rs* 中定義 `front_of_house` 模組</span>

注意，在模組樹中，你只需要用 `mod` 宣告載入某個檔案一次。一旦編譯器知道這個檔案是專案的一部分了，並且因為 `mod` 語句所在的位置而知道這段程式碼位於模組樹的什麼位置，那麼專案中的其他檔案就應該用它宣告位置對應的路徑來引用這段程式碼，這一點會在[“引用模組樹中項的路徑”][paths]部分講到。換句話說，`mod` **不是**某些其他程式語言裡那種 “include” 操作。

接下來，我們也把 `hosting` 模組提取到它自己的檔案中。這個過程稍有不同，因為 `hosting` 是 `front_of_house` 的子模組，而不是根模組。我們會把 `hosting` 對應的檔案放進一個以其在模組樹中的祖先模組命名的新目錄中，這裡就是 *src/front_of_house*。

要開始移動 `hosting`，先把 *src/front_of_house.rs* 改成只包含 `hosting` 模組的宣告：

<span class="filename">檔名：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

然後，建立一個 *src/front_of_house* 目錄和一個 *hosting.rs* 檔案，用來放置 `hosting` 模組中的定義：

<span class="filename">檔名：src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

如果我們反而把 *hosting.rs* 放在 *src* 目錄中，編譯器就會認為 *hosting.rs* 裡的程式碼屬於在 crate 根中宣告的 `hosting` 模組，而不是 `front_of_house` 的子模組。編譯器關於“要去哪些檔案中查詢哪些模組程式碼”的規則，使得目錄和檔案結構會更貼近模組樹本身。

> ### 替代檔案路徑
>
> 到目前為止，我們介紹的是 Rust 編譯器最慣用的檔案路徑，不過 Rust 也支援一種較舊的檔案路徑風格。
>
> 對於在 crate 根中宣告的 `front_of_house` 模組，編譯器會在以下位置查詢模組程式碼：
>
> - *src/front_of_house.rs*（前面介紹的方式）
> - *src/front_of_house/mod.rs*（較舊的風格，但仍然受支援）
>
> 對於 `front_of_house` 的子模組 `hosting`，編譯器會在以下位置查詢模組程式碼：
>
> - *src/front_of_house/hosting.rs*（前面介紹的方式）
> - *src/front_of_house/hosting/mod.rs*（較舊的風格，但仍然受支援）
>
> 如果你對同一個模組同時使用這兩種風格，編譯器就會報錯。在同一個專案裡為不同模組混用兩種風格是允許的，不過這可能會讓瀏覽專案的人感到困惑。
>
> 使用 *mod.rs* 這種檔名風格的主要缺點是，專案裡最後可能會有很多都叫 *mod.rs* 的檔案；當你在編輯器裡同時開啟它們時，就會變得很混亂。

現在，我們已經把每個模組的程式碼都移動到了獨立檔案中，而模組樹保持不變。`eat_at_restaurant` 中的函式呼叫也完全不需要修改，即使這些定義現在位於不同的檔案中也一樣能工作。這個技巧讓你可以在模組逐漸變大時，再把它們遷移到新檔案裡。

注意，*src/lib.rs* 中的 `pub use crate::front_of_house::hosting` 語句也完全沒有變化，而且 `use` 也不會影響 crate 會編譯哪些檔案。`mod` 關鍵字用來宣告模組，而 Rust 會在與模組同名的檔案中查詢應放進該模組的程式碼。


## 總結

Rust 允許你把一個包拆分成多個 crate，再把一個 crate 拆分成多個模組，這樣你就能在一個模組中引用另一個模組裡定義的項。你既可以使用絕對路徑，也可以使用相對路徑。還可以用 `use` 語句把路徑引入作用域，這樣在同一作用域內多次使用該項時就能寫更短的路徑。模組程式碼預設是私有的，不過你可以加上 `pub` 關鍵字，把定義公開出去。

下一章，我們將看看標準庫中的一些集合資料結構，你可以在組織良好的程式碼中使用它們。

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
