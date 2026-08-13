## 引用模組樹中項的路徑

[ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md](https://github.com/rust-lang/book/blob/79b9d15410a7b15a65cb86bcb40cbea99198a9e4/src/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)

為了向 Rust 指示在模組樹中從何處查詢某個項，我們使用路徑，就像在檔案系統中使用路徑一樣。為了呼叫一個函式，我們需要知道它的路徑。

路徑有兩種形式：

- **絕對路徑**（*absolute path*）是以 crate 根（root）開頭的完整路徑；對於外部 crate 的程式碼，是以 crate 名開頭的絕對路徑，對於當前 crate 的程式碼，則以字面值 `crate` 開頭。
- **相對路徑**（*relative path*）從當前模組開始，以 `self`、`super` 或當前模組中的某個識別符號開頭。

絕對路徑和相對路徑都後跟一個或多個由雙冒號（`::`）分割的識別符號。

回到示例 7-1，假設我們希望呼叫 `add_to_waitlist` 函式。這相當於在問：`add_to_waitlist` 函式的路徑是什麼？在示例 7-3 中刪除了示例 7-1 的一些模組和函式。

我們在 crate 根定義了一個新函式 `eat_at_restaurant`，並在其中展示呼叫 `add_to_waitlist` 函式的兩種方法。這些路徑都是正確的，不過因為存在另一個問題導致示例無法照原樣編譯。稍後我們會解釋為什麼。

`eat_at_restaurant` 函式是我們 crate 庫的一個公共 API，所以我們使用 `pub` 關鍵字來標記它。在 [“使用 `pub` 關鍵字暴露路徑”][pub] 一節，我們將詳細介紹 `pub`。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">示例 7-3: 使用絕對路徑和相對路徑來呼叫 `add_to_waitlist` 函式</span>

第一次在 `eat_at_restaurant` 中呼叫 `add_to_waitlist` 函式時，使用的是絕對路徑。`add_to_waitlist` 函式與 `eat_at_restaurant` 被定義在同一 crate 中，這意味著我們可以使用 `crate` 關鍵字為起始的絕對路徑。接著我們依次包含各級模組，直到我們找到 `add_to_waitlist`。你可以想象出一個相同結構的檔案系統：我們透過指定路徑 `/front_of_house/hosting/add_to_waitlist` 來執行 `add_to_waitlist` 程式。我們使用 `crate` 從 crate 根開始就類似於在 shell 中使用 `/` 從檔案系統根開始。

第二次在 `eat_at_restaurant` 中呼叫 `add_to_waitlist` 時，使用的是相對路徑。這個路徑以 `front_of_house` 為起始，這個模組在模組樹中與 `eat_at_restaurant` 定義在同一層級。與之等價的檔案系統路徑就是 `front_of_house/hosting/add_to_waitlist`。以模組名開頭意味著該路徑是相對路徑。

選擇使用相對路徑還是絕對路徑要取決於你的專案，也取決於你是更傾向於將項的定義程式碼與使用該項的程式碼分開來移動，還是一起移動。例如，如果我們要將 `front_of_house` 模組和 `eat_at_restaurant` 函式一起移動到一個名為 `customer_experience` 的模組中，我們需要更新 `add_to_waitlist` 的絕對路徑，但是相對路徑還是可用的。相反，如果我們要將 `eat_at_restaurant` 函式單獨移到一個名為 `dining` 的模組中，還是可以使用原本的絕對路徑來呼叫 `add_to_waitlist`，但是相對路徑必須要更新。我們更傾向於使用絕對路徑，因為把程式碼定義和項呼叫各自獨立地移動是更常見的。

讓我們試著編譯一下示例 7-3，並查明其為何不能編譯！示例 7-4 展示了這個錯誤。

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">示例 7-4: 構建示例 7-3 出現的編譯器錯誤</span>

錯誤資訊說 `hosting` 模組是私有的。換句話說，我們擁有 `hosting` 模組和 `add_to_waitlist` 函式的正確路徑，但是 Rust 不讓我們使用，因為它不能訪問私有片段。在 Rust 中，所有項（函式、方法、結構體、列舉、模組和常量）預設對父模組都是私有的。如果希望建立一個如函式或結構體的私有項，可以將其放入一個模組。

父模組中的項不能使用子模組中的私有項，但是子模組中的項可以使用它們父模組中的項。這是因為子模組封裝並隱藏了它們的實現詳情，但是子模組可以看到定義它們的上下文。繼續我們的比喻，把私有性規則想象成餐館的後臺辦公室：後臺的事務對餐廳顧客來說是不可知的，但辦公室經理可以洞悉其經營的餐廳並在其中做任何事情。

Rust 選擇以這種方式來實現模組系統功能，因此預設隱藏內部實現細節。這樣一來，你就知道可以更改內部程式碼的哪些部分而不會破壞外部程式碼。不過 Rust 也確實提供了透過使用 `pub` 關鍵字來建立公共項，使子模組的內部部分暴露給上級模組。

### 使用 `pub` 關鍵字暴露路徑

讓我們回頭看一下示例 7-4 的錯誤，它告訴我們 `hosting` 模組是私有的。我們想讓父模組中的 `eat_at_restaurant` 函式可以訪問子模組中的 `add_to_waitlist` 函式，因此我們使用 `pub` 關鍵字來標記 `hosting` 模組，如示例 7-5 所示。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

<span class="caption">示例 7-5: 使用 `pub` 關鍵字宣告 `hosting` 模組使其可在 `eat_at_restaurant` 使用</span>

不幸的是，示例 7-5 的程式碼編譯仍然有錯誤，如示例 7-6 所示。

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">示例 7-6: 構建示例 7-5 出現的編譯器錯誤</span>

發生了什麼？在 `mod hosting` 前添加了 `pub` 關鍵字，使其變成公有的。伴隨著這種變化，如果我們可以訪問 `front_of_house`，那我們也可以訪問 `hosting`。但是 `hosting` 的**內容**（_contents_）仍然是私有的；這表明使模組公有並不使其內容也是公有的。模組上的 `pub` 關鍵字只允許其父模組引用它，而不允許訪問內部程式碼。因為模組是一個容器，只是將模組變為公有能做的其實並不太多；同時需要更深入地選擇將一個或多個項變為公有。

示例 7-6 中的錯誤說，`add_to_waitlist` 函式是私有的。私有性規則不但應用於模組，還應用於結構體、列舉、函式和方法。

讓我們繼續將 `pub` 關鍵字放置在 `add_to_waitlist` 函式的定義之前，使其變成公有。如示例 7-7 所示。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

<span class="caption">示例 7-7: 為 `mod hosting` 和 `fn add_to_waitlist` 新增 `pub` 關鍵字使它們可以在 `eat_at_restaurant` 函式中被呼叫</span>

現在程式碼可以編譯通過了！為了瞭解為何增加 `pub` 關鍵字使得我們可以在 `eat_at_restaurant` 中呼叫這些路徑與私有性規則有關，讓我們看看絕對路徑和相對路徑。

在絕對路徑，我們從 `crate` 也就是 crate 根開始。crate 根中定義了 `front_of_house` 模組。雖然 `front_of_house` 模組不是公有的，不過因為 `eat_at_restaurant` 函式與 `front_of_house` 定義於同一級模組中（即，`eat_at_restaurant` 和 `front_of_house` 是兄弟），我們可以從 `eat_at_restaurant` 中引用 `front_of_house`。接下來是使用 `pub` 標記的 `hosting` 模組。我們可以訪問 `hosting` 的父模組，所以可以訪問 `hosting`。最後，`add_to_waitlist` 函式被標記為 `pub` ，我們可以訪問其父模組，所以這個函式呼叫是有效的！

在相對路徑，其邏輯與絕對路徑相同，除了第一步：不同於從 crate 根開始，路徑從 `front_of_house` 開始。`front_of_house` 模組與 `eat_at_restaurant` 定義於同一級模組，所以從 `eat_at_restaurant` 中開始定義的該模組相對路徑是有效的。接下來因為 `hosting` 和 `add_to_waitlist` 被標記為 `pub`，路徑其餘的部分也是有效的，因此函式呼叫也是有效的！

如果你計劃共享你的庫 crate 以便其它專案可以使用你的程式碼，公有 API 將是決定 crate 使用者如何與你程式碼互動的契約。關於管理公有 API 的修改以便被人更容易依賴你的庫有著很多考量。這些考量超出了本書的範疇；如果你對這些話題感興趣，請查閱 [The Rust API Guidelines][api-guidelines]。

> ### 二進位制和庫 crate 包的最佳實踐
>
> 我們提到過包（package）可以同時包含一個 *src/main.rs* 二進位制 crate 根和一個 *src/lib.rs* 庫 crate 根，並且這兩個 crate 預設以包名來命名。通常，這種包含二進位制 crate 和庫 crate 的模式的包，在二進位制 crate 中只保留足以生成一個可執行檔案的程式碼，並由可執行檔案呼叫庫 crate 的程式碼。又因為庫 crate 可以共享，這使得其它專案從包提供的大部分功能中受益。
>
> 模組樹應該定義在 *src/lib.rs* 中。這樣透過以包名開頭的路徑，公有項就可以在二進位制 crate 中使用。二進位制 crate 就變得像一個完全外部的 crate 來使用庫 crate 的使用者一樣：它只能使用 public API。你不僅僅是作者，也是使用者！
>
> 在[第十二章][ch12]我們會透過一個同時包含二進位制 crate 和庫 crate 的命令列程式來展示這些組織上的實踐。

### `super` 開始的相對路徑

我們可以透過在路徑的開頭使用 `super` ，從父模組開始構建相對路徑，而不是從當前模組或者 crate 根開始。這類似以 `..` 語法開始一個檔案系統路徑。使用 `super` 允許我們引用父模組中的已知項，這使得當模組與父模組關聯的很緊密，但某天父模組可能要移動到模組樹的其它位置時重新組織模組樹變得更容易。

考慮一下示例 7-8 中的程式碼，它模擬了廚師更正了一個錯誤訂單並親自將其提供給客戶的情況。`back_of_house` 模組中的定義的 `fix_incorrect_order` 函式透過指定的 `super` 起始的 `deliver_order` 路徑來呼叫父模組中的 `deliver_order` 函式。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">示例 7-8: 使用以 `super` 開頭的相對路徑呼叫函式</span>

`fix_incorrect_order` 函式在 `back_of_house` 模組中，所以我們可以使用 `super` 進入 `back_of_house` 父模組，也就是本例中的 `crate` 根。在這裡，我們可以找到 `deliver_order`。成功！我們認為 `back_of_house` 模組和 `deliver_order` 函式之間可能保持某種關聯關係並且如果我們要重新組織這個 crate 的模組樹時，需要一起移動它們。因此，我們使用 `super`，這樣一來，如果這些程式碼被移動到了其他模組，只需要更新很少的程式碼。

### 建立公有的結構體和列舉

我們還可以使用 `pub` 來設計公有的結構體和列舉，不過關於在結構體和列舉上使用 `pub` 還有一些額外的細節需要注意。如果我們在一個結構體定義的前面使用了 `pub`，這個結構體會變成公有的，但是這個結構體的欄位仍然是私有的。我們可以根據情況決定每個欄位是否公有。在示例 7-9 中，我們定義了一個公有結構體 `back_of_house::Breakfast`，其中有一個公有欄位 `toast` 和私有欄位 `seasonal_fruit`。這個例子模擬的情況是，在一家餐館中，顧客可以選擇隨餐麵包的型別，但是廚師會根據季節和庫存情況來決定隨餐搭配的水果。餐館可用的水果變化是很快的，所以顧客不能選擇水果，甚至無法看到他們將會得到什麼水果。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">示例 7-9: 帶有公有和私有欄位的結構體</span>

因為 `back_of_house::Breakfast` 結構體的 `toast` 欄位是公有的，所以我們可以在 `eat_at_restaurant` 中使用點號來讀寫 `toast` 欄位。注意，我們不能在 `eat_at_restaurant` 中使用 `seasonal_fruit` 欄位，因為 `seasonal_fruit` 是私有的。嘗試去除那一行修改 `seasonal_fruit` 欄位值的程式碼的註釋，看看你會得到什麼錯誤！

還請注意一點，因為 `back_of_house::Breakfast` 具有私有欄位，所以這個結構體需要提供一個公共的關聯函式來構造 `Breakfast` 的例項 (這裡我們命名為 `summer`)。如果 `Breakfast` 沒有這樣的函式，我們將無法在 `eat_at_restaurant` 中建立 `Breakfast` 例項，因為我們不能在 `eat_at_restaurant` 中設定私有欄位 `seasonal_fruit` 的值。

與之相反，如果我們將列舉設為公有，則它的所有變體都將變為公有。我們只需要在 `enum` 關鍵字前面加上 `pub`，就像示例 7-10 展示的那樣。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">示例 7-10: 設計公有列舉，使其所有成員公有</span>

因為我們將 `Appetizer` 列舉宣告為公有，所以可以在 `eat_at_restaurant` 中使用 `Soup` 和 `Salad` 變體。

如果列舉變體不是公有的，那麼列舉會顯得用處不大；給列舉的所有變體挨個新增 `pub` 是很令人惱火的，因此列舉變體預設就是公有的。結構體在許多情況下即使欄位不可公有也能正常使用，所以結構體欄位遵循預設私有的通用規則，除非使用 `pub` 關鍵字。

還有一個我們尚未介紹的與 `pub` 相關的情形，那就是模組系統的最後一個特性：`use` 關鍵字。我們將先單獨介紹 `use`，然後展示如何結合使用 `pub` 和 `use`。

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#使用-pub-關鍵字暴露路徑
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
