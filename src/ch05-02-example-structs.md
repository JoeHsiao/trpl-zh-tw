## 結構體示例程式

[ch05-02-example-structs.md](https://github.com/rust-lang/book/blob/8a6130451b0817ead5c2522ce641dcb0f11a8571/src/ch05-02-example-structs.md)

為了理解何時會需要使用結構體，讓我們編寫一個計算長方形面積的程式。我們會從單獨的變數開始，接著重構程式直到使用結構體替代它們為止。

使用 Cargo 新建一個叫做 *rectangles* 的二進位制程式，它獲取以畫素為單位的長方形的寬度和高度，並計算出長方形的面積。示例 5-8 顯示了位於專案的 *src/main.rs* 中的小程式，它剛剛好實現此功能：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">示例 5-8：通過分別指定長方形的寬和高的變數來計算長方形面積</span>

現在使用 `cargo run` 執行程式：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

這個示例程式碼在呼叫 `area` 函式時傳入每個維度，雖然可以正確計算出長方形的面積，但我們仍然可以修改這段程式碼來使它的意義更加明確，並且增加可讀性。

這些程式碼的問題突顯在 `area` 的簽名上：

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

函式 `area` 本應該計算一個長方形的面積，不過函式卻有兩個引數。這兩個引數是相關聯的，不過程式本身卻沒有表現出這一點。將長度和寬度組合在一起將更易懂也更易處理。第三章的 [“元組型別”][the-tuple-type] 部分已經討論過了一種可行的方法：元組。

### 使用元組重構

示例 5-9 展示了使用元組的另一個程式版本。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">示例 5-9：使用元組來指定長方形的寬高</span>

在某種程度上說，這個程式更好一點了。元組幫助我們增加了一些結構性，並且現在只需傳一個引數。不過在另一方面，這個版本卻有一點不明確了：元組並沒有給出元素的名稱，所以計算變得更費解了，因為不得不使用索引來獲取元組的每一部分。

在計算面積時將寬和高弄混倒無關緊要，不過當在螢幕上繪製長方形時就有問題了！我們必須牢記 `width` 的元組索引是 `0`，`height` 的元組索引是 `1`。如果其他人要使用這些程式碼，他們必須要搞清楚這一點，並也要牢記於心。很容易忘記或者混淆這些值而造成錯誤，因為我們沒有在程式碼中傳達資料的意圖。

<a id="refactoring-with-structs-adding-more-meaning"></a>

### 使用結構體重構

我們使用結構體為資料命名來為其賦予意義。我們可以將我們正在使用的元組轉換成一個有整體名稱而且每個部分也有對應名字的結構體，如示例 5-10 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">示例 5-10：定義 `Rectangle` 結構體</span>

這裡我們定義了一個結構體並稱其為 `Rectangle`。在大括號中定義了欄位 `width` 和 `height`，型別都是 `u32`。接著在 `main` 中，我們建立了一個具體的 `Rectangle` 例項，它的寬是 `30`，高是 `50`。

函式 `area` 現在被定義為接收一個名叫 `rectangle` 的引數，其型別是一個結構體 `Rectangle` 例項的不可變借用。第四章講到過，我們希望借用結構體而不是獲取它的所有權，這樣 `main` 函式就可以保持 `rect1` 的所有權並繼續使用它，所以這就是為什麼在函式簽名和呼叫的地方會有 `&`。

`area` 函式訪問 `Rectangle` 例項的 `width` 和 `height` 欄位（注意，訪問對結構體的引用的欄位不會移動欄位的所有權，這就是為什麼你經常看到對結構體的引用）。`area` 的函式簽名現在明確的闡述了我們的意圖：使用 `Rectangle` 的 `width` 和 `height` 欄位，計算 `Rectangle` 的面積。這表明寬高是相互聯絡的，併為這些值提供了描述性的名稱而不是使用元組的索引值 `0` 和 `1` 。這在可讀性上是一個明顯的提升。

<a id="adding-useful-functionality-with-derived-traits"></a>

### 通過派生 trait 增加功能

在除錯程式時打印出 `Rectangle` 例項來檢視其所有欄位的值非常有用。示例 5-11 像前面章節那樣嘗試使用 [`println!` 宏][println]。但這並不行。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">示例 5-11：嘗試打印出 `Rectangle` 例項</span>

當我們執行這個程式碼時，會出現帶有如下核心資訊的錯誤：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` 宏能處理很多型別的格式，不過，`{}` 預設告訴 `println!` 使用被稱為 `Display` 的格式：意在提供給直接終端使用者檢視的輸出。目前為止見過的基本型別都預設實現了 `Display`，因為它就是向用戶展示 `1` 或其他任何基本型別的唯一方式。不過對於結構體，`println!` 應該用來輸出的格式是不明確的，因為這有更多顯示的可能性：是否需要逗號？需要打印出大括號嗎？所有欄位都應該顯示嗎？由於這種不確定性，Rust 不會嘗試猜測我們的意圖，所以結構體並沒有提供一個 `Display` 實現來使用 `println!` 與 `{}` 佔位符。

但是如果我們繼續閱讀錯誤，將會發現這個有幫助的資訊：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:12:13}}
```

讓我們來試試！現在 `println!` 巨集呼叫看起來會像 `println!("rect1 is {rect1:?}");`。在大括號中加入 `:?` 指示符，告訴 `println!` 我們想使用一種叫做 `Debug` 的輸出格式。`Debug` 是一個 trait，它允許我們以一種對開發者有幫助的方式列印結構體，這樣在除錯程式碼時就能看到它的值。

這樣調整後再次執行程式。見鬼了！仍然能看到一個錯誤：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

不過編譯器又一次給出了一個有幫助的資訊：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:11:12}}
```

Rust **確實** 包含了打印出除錯資訊的功能，不過我們必須為結構體顯式選擇這個功能。為此，在結構體定義之前加上外部屬性 `#[derive(Debug)]`，如示例 5-12 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">示例 5-12：增加屬性來派生 `Debug` trait，並使用除錯格式列印 `Rectangle` 例項</span>

現在我們再執行這個程式時，就不會有任何錯誤，並會出現如下輸出：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

好極了！這並不是最漂亮的輸出，不過它顯示這個例項的所有欄位，毫無疑問這對除錯有幫助。當我們有一個更大的結構體時，能有更易讀一點的輸出就好了，為此可以使用 `{:#?}` 替換 `println!` 字串中的 `{:?}`。在這個例子中使用 `{:#?}` 風格將會輸出如下：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

另一種使用 `Debug` 格式列印數值的方法是使用 [`dbg!` 宏][dbg]。`dbg!` 宏接收一個表示式的所有權（與 `println!` 宏相反，後者接收的是引用），打印出程式碼中呼叫 dbg! 宏時所在的檔案和行號，以及該表示式的結果值，並返回該值的所有權。

> 注意：呼叫 `dbg!` 宏會列印到標準錯誤控制台流（`stderr`），與 `println!` 不同，後者會列印到標準輸出控制台流（`stdout`）。我們將在[第十二章 “將錯誤資訊寫入標準錯誤而不是標準輸出” 一節][err]中更多地討論 `stderr` 和 `stdout`。

下面是一個例子，我們對分配給 `width` 欄位的值以及 `rect1` 中整個結構的值感興趣。

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

我們可以把 `dbg!` 放在表示式 `30 * scale` 周圍，因為 `dbg!` 返回表示式的值的所有權，所以 `width` 欄位將獲得相同的值，就像我們在那裡沒有 `dbg!` 呼叫一樣。我們不希望 `dbg!` 擁有 `rect1` 的所有權，所以我們在下一次呼叫 `dbg!` 時傳遞一個引用。下面是這個例子的輸出結果：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

我們可以看到第一點輸出來自 *src/main.rs* 第 10 行，我們正在調試表達式 `30 * scale`，其結果值是 `60`（為整數實現的 `Debug` 格式化是隻列印它們的值）。在 *src/main.rs* 第 14 行 的 `dbg!` 呼叫輸出 `&rect1` 的值，即 `Rectangle` 結構。這個輸出使用了更為易讀的 `Debug` 格式。當你試圖弄清楚你的程式碼在做什麼時，`dbg!` 宏可能真的很有幫助！

除了 `Debug` trait，Rust 還為我們提供了很多可以通過 `derive` 屬性來使用的 trait，它們可以為我們的自定義型別增加實用的行為。[附錄 C][app-c] 中列出了這些 trait 和行為。第十章會介紹如何通過自定義行為來實現這些 trait，同時還有如何建立你自己的 trait。除了 `derive` 之外，還有很多屬性；更多資訊請參見 [Rust Reference][attributes] 的 Attributes 部分。

我們的 `area` 函式用途非常專一：它僅計算長方形的面積。如果這個行為與 `Rectangle` 結構體再結合得更緊密一些就更好了，因為它不能用於其他型別。現在讓我們看看如何繼續重構這些程式碼，來將 `area` 函式協調進 `Rectangle` 型別定義的 `area` **方法** 中。

[the-tuple-type]: ch03-02-data-types.html#元組型別
[app-c]: appendix-03-derivable-traits.html
[println]: https://doc.rust-lang.org/std/macro.println.html
[dbg]: https://doc.rust-lang.org/std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: https://doc.rust-lang.org/stable/reference/attributes.html
