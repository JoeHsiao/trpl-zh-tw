<a id="traits-defining-shared-behavior"></a>

## Trait：定義共同行為

[ch10-02-traits.md](https://github.com/rust-lang/book/blob/3986f214fd82427b4401adf4d7dc0911c917e1e8/src/ch10-02-traits.md)

*trait* 定義了某個特定型別擁有可能與其他型別共享的功能。可以透過 trait 以一種抽象的方式定義共同行為。可以使用 **trait 約束**（*trait bounds*）指定泛型是任何擁有特定行為的型別。

> 注意：*trait* 類似於其他語言中的常被稱為 **介面**（*interfaces*）的功能，雖然有一些不同。

### 定義 trait

一個型別的行為由其可供呼叫的方法構成。如果可以對不同型別呼叫相同的方法的話，這些型別就可以共享相同的行為了。trait 定義是一種將方法簽名組合起來的方法，目的是定義一個實現某些目的所必需的行為的集合。

例如，這裡有多個存放了不同型別和屬性文字的結構體：結構體 `NewsArticle` 用於存放發生於世界各地的新聞故事，而結構體 `SocialPost` 最多隻能存放 280 個字元的內容，以及指示該帖子是新發布的、轉發的還是對另一條帖子的回覆的元資料。

我們想要建立一個名為 `aggregator` 的多媒體聚合庫用來顯示可能儲存在 `NewsArticle` 或 `SocialPost` 例項中的資料摘要。為了實現功能，每個結構體都要能夠獲取摘要，這樣的話就可以呼叫例項的 `summarize` 方法來請求摘要。示例 10-12 中展示了一個表現這個概念的公有 `Summary` trait 的定義：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">示例 10-12：`Summary` trait 定義，它包含由 `summarize` 方法提供的行為</span>

這裡使用 `trait` 關鍵字來宣告一個 trait，後面是 trait 的名字，在這個例子中是 `Summary`。我們也宣告 `trait` 為 `pub` 以便依賴這個 crate 的其它 crate 也可以使用這個 trait，正如我們見過的一些示例一樣。在大括號中宣告描述實現這個 trait 的型別所需要的行為的方法簽名，在這個例子中是 `fn summarize(&self) -> String`。

在方法簽名後跟分號，而不是在大括號中提供其實現。接著每一個實現這個 trait 的型別都需要提供其自定義行為的方法體，編譯器也會確保任何實現 `Summary` trait 的型別都擁有與這個簽名的定義完全一致的 `summarize` 方法。

trait 體中可以有多個方法：一行一個方法簽名且都以分號結尾。

### 為型別實現 trait

現在我們定義了 `Summary` trait 的簽名，接著就可以在多媒體聚合庫中實現這個型別了。示例 10-13 中展示了 `NewsArticle` 結構體上 `Summary` trait 的一個實現，它使用標題、作者和建立的位置作為 `summarize` 的返回值。對於 `SocialPost` 結構體，我們選擇將 `summarize` 定義為使用者名稱後跟帖子全文作為返回值，並假設帖子內容已經被限制為 280 字元以內。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">示例 10-13：在 `NewsArticle` 和 `SocialPost` 型別上實現 `Summary` trait</span>

在型別上實現 trait 類似於實現常規方法。區別在於 `impl` 關鍵字之後，我們提供需要實現 trait 的名稱，接著是 `for` 和需要實現 trait 的型別的名稱。在 `impl` 塊中，使用 trait 定義中的方法簽名，不過不再後跟分號，而是需要在大括號中編寫函式體來為特定型別實現 trait 方法所擁有的行為。

現在庫在 `NewsArticle` 和 `SocialPost` 上實現了`Summary` trait，crate 的使用者可以像呼叫常規方法一樣呼叫 `NewsArticle` 和 `SocialPost` 例項的 trait 方法了。唯一的區別是 trait 必須和型別一起引入作用域以便使用額外的 trait 方法。這是一個二進位制 crate 如何利用 `aggregator` 庫 crate 的例子：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

這會打印出 `1 new post: horse_ebooks: of course, as you probably already know, people`。

其他依賴 `aggregator` crate 的 crate 也可以將 `Summary` 引入作用域以便為其自己的型別實現該 trait。需要注意的限制是，只有在 trait 或型別至少有一個屬於當前 crate 時，我們才能對型別實現該 trait。例如，可以為 `aggregator` crate 的自定義型別 `SocialPost` 實現如標準庫中的 `Display` trait，這是因為 `SocialPost` 型別位於 `aggregator` crate 本地的作用域中。類似地，也可以在 `aggregator` crate 中為 `Vec<T>` 實現 `Summary`，這是因為 `Summary` trait 位於 `aggregator` crate 本地作用域中。

但是不能為外部型別實現外部 trait。例如，不能在 `aggregator` crate 中為 `Vec<T>` 實現 `Display` trait。這是因為 `Display` 和 `Vec<T>` 都定義於標準庫中，它們並不位於 `aggregator` crate 本地作用域中。這個限制是被稱為**相干性**（*coherence*）的程式屬性的一部分，或者更具體的說是 **孤兒規則**（*orphan rule*），其得名於不存在父型別。這條規則確保了其他人編寫的程式碼不會破壞你的程式碼，反之亦然。沒有這條規則的話，兩個 crate 可以分別對相同型別實現相同的 trait，而 Rust 將無從得知應該使用哪一個實現。

<a id="default-implementations"></a>

### 使用預設實現

有時為 trait 中的某些或全部方法提供預設的行為，而不是在每個型別的每個實現中都定義自己的行為是很有用的。這樣當為某個特定型別實現 trait 時，可以選擇保留或過載每個方法的預設行為。

示例 10-14 中我們為 `Summary` trait 的 `summarize` 方法指定一個預設的字串值，而不是像示例 10-12 中那樣只是定義方法簽名：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">示例 10-14：`Summary` trait 的定義，帶有一個 `summarize` 方法的預設實現</span>

如果想要對 `NewsArticle` 例項使用這個預設實現，可以透過 `impl Summary for NewsArticle {}` 指定一個空的 `impl` 塊。

雖然我們不再直接為 `NewsArticle` 定義 `summarize` 方法了，但是我們提供了一個預設實現並且指定 `NewsArticle` 實現 `Summary` trait。因此，我們仍然可以對 `NewsArticle` 例項呼叫 `summarize` 方法，如下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

這段程式碼會列印 `New article available! (Read more...)`。

為 `summarize` 建立預設實現並不要求對示例 10-13 中 `SocialPost` 上的 `Summary` 實現做任何改變。其原因是過載一個預設實現的語法與實現沒有預設實現的 trait 方法的語法一樣。

預設實現允許呼叫相同 trait 中的其他方法，哪怕這些方法沒有預設實現。如此，trait 可以提供很多有用的功能而只需要實現指定一小部分內容。例如，我們可以定義 `Summary` trait，使其具有一個需要實現的 `summarize_author` 方法，然後定義一個 `summarize` 方法，此方法的預設實現呼叫 `summarize_author` 方法：

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

為了使用這個版本的 `Summary`，只需在為型別實現 trait 時定義 `summarize_author` 即可：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

一旦定義了 `summarize_author`，我們就可以對 `SocialPost` 結構體的例項呼叫 `summarize` 了，而 `summarize` 的預設實現會呼叫我們提供的 `summarize_author` 定義。因為實現了 `summarize_author`，`Summary` trait 就提供了 `summarize` 方法的功能，且無需編寫更多的程式碼。

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

這會打印出 `1 new post: (Read more from @horse_ebooks...)`。

請注意，無法從一個方法的重寫實現中呼叫與其同名的預設實現。

<a id="traits-as-parameters"></a>

### 使用 trait 作為引數

知道了如何定義 trait 和在型別上實現這些 trait 之後，我們可以探索一下如何使用 trait 來接受多種不同型別的引數。示例 10-13 中為 `NewsArticle` 和 `SocialPost` 型別實現了 `Summary` trait，用其來定義了一個函式 `notify` 來呼叫其引數 `item` 上的 `summarize` 方法，該引數是實現了 `Summary` trait 的某種型別。為此可以使用 `impl Trait` 語法，像這樣：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

對於 `item` 引數，我們指定了 `impl` 關鍵字和 trait 名稱，而不是具體的型別。該引數支援任何實現了指定 trait 的型別。在 `notify` 函式體中，可以呼叫任何來自 `Summary` trait 的方法，比如 `summarize`。我們可以傳遞任何 `NewsArticle` 或 `SocialPost` 的例項來呼叫 `notify`。任何用其它如 `String` 或 `i32` 的型別呼叫該函式的程式碼都不能編譯，因為它們沒有實現 `Summary`。

<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Trait 約束語法

`impl Trait` 語法更直觀，但它實際上是更長形式的 *trait 約束* 語法的語法糖。它看起來像：

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

這種更冗長的寫法與上一節的示例等價，但更為冗長。trait 約束與泛型引數宣告在一起，位於尖括號中的冒號後面。

`impl Trait` 很方便，適用於短小的例子。更長的 trait 約束則適用於更復雜的場景。例如，可以獲取兩個實現了 `Summary` 的引數。使用 `impl Trait` 的語法看起來像這樣：

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

這適用於 `item1` 和 `item2` 可以是不同型別的情況，只要它們都實現了 `Summary`。不過，如果我們希望強制兩個引數必須具有相同型別，就必須使用 trait 約束，如下所示：

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

泛型 `T` 被指定為 `item1` 和 `item2` 的引數限制，如此傳遞給引數 `item1` 和 `item2` 值的具體型別必須一致。

<a id="specifying-multiple-trait-bounds-with-the--syntax"></a>

#### 透過 `+` 語法指定多個 trait 約束

我們也可以指定多個 trait 約束 `notify` 在 `item` 上既能使用格式化顯示，又能使用 `summarize` 方法：在 `notify` 的定義中，指定 `item` 必須同時實現 `Display` 和 `Summary` 兩個 trait。這可以透過 `+` 語法實現：

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

`+` 語法也適用於泛型的 trait 約束：

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

透過指定這兩個 trait 約束，`notify` 的函式體可以呼叫 `summarize` 並使用 `{}` 來格式化 `item`。

#### 透過 `where` 簡化 trait 約束

然而，使用過多的 trait 約束 也有缺點。每個泛型有其自己的 trait 約束，所以有多個泛型引數的函式在名稱和引數列表之間會有很長的 trait 約束 資訊，這使得函式簽名難以閱讀。為此，Rust 有另一個在函式簽名之後的 `where` 從句中指定 trait 約束的語法。所以除了這麼寫：

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

還可以像這樣使用 `where` 從句：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

這個函式簽名就顯得不那麼雜亂，函式名、引數列表和返回值型別都離得很近，看起來跟沒有那麼多 trait 約束的函式很像。

### 返回實現了 trait 的型別

也可以在返回值中使用 `impl Trait` 語法，來返回實現了某個 trait 的型別：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

透過使用 `impl Summary` 作為返回值型別，我們指定了 `returns_summarizable` 函式返回某個實現了 `Summary` trait 的型別，但是不確定其具體的型別。在這個例子中 `returns_summarizable` 返回了一個 `SocialPost`，不過呼叫方並不知情。

返回一個只是指定了需要實現的 trait 的型別的能力在閉包和迭代器場景十分的有用，第十三章會介紹它們。閉包和迭代器建立只有編譯器知道的型別，或者是非常非常長的型別。`impl Trait` 允許你簡單的指定函式返回一個 `Iterator` 而無需寫出實際的冗長的型別。

不過這隻適用於返回單一型別的情況。例如，這段程式碼的返回值型別指定為返回 `impl Summary`，但是返回了 `NewsArticle` 或 `SocialPost` 就行不通：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

這裡嘗試返回 `NewsArticle` 或 `SocialPost` 是不被允許的，原因在於編譯器中 `impl Trait` 語法的實現限制。第十八章的[“使用 trait object 來抽象出共享行為”][using-trait-objects-that-allow-for-values-of-different-types]部分會介紹如何編寫這樣一個函式。

### 使用 trait 約束有條件地實現方法

透過使用帶有 trait 約束的泛型引數的 `impl` 塊，可以有條件地只為那些實現了特定 trait 的型別實現方法。例如，示例 10-15 中的型別 `Pair<T>` 總是實現了 `new` 方法並返回一個 `Pair<T>` 的例項（回憶一下第五章的 [“方法語法”][methods] 部分，`Self` 是一個 `impl` 塊型別的類型別名（type alias），在這裡是 `Pair<T>`）。不過在下一個 `impl` 塊中，只有那些為 `T` 型別實現了 `PartialOrd` trait（來允許比較） **和** `Display` trait（來啟用列印）的 `Pair<T>` 才會實現 `cmp_display` 方法：

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

<span class="caption">示例 10-15：根據 trait 約束在泛型上有條件的實現方法</span>

也可以對任何實現了特定 trait 的型別有條件地實現 trait。對任何滿足特定 trait 約束的型別實現 trait 被稱為 *blanket implementations*，它們被廣泛的用於 Rust 標準庫中。例如，標準庫為任何實現了 `Display` trait 的型別實現了 `ToString` trait。這個 `impl` 塊看起來像這樣：

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

因為標準庫有了這些 blanket implementation，我們可以對任何實現了 `Display` trait 的型別呼叫由 `ToString` 定義的 `to_string` 方法。例如，可以將整型轉換為對應的 `String` 值，因為整型實現了 `Display`：

```rust
let s = 3.to_string();
```

Blanket implementation 會出現在 trait 文件的 “Implementers” 部分。

Trait 和 trait 約束讓我們能夠使用泛型型別引數來減少重複，而且能夠向編譯器明確指定泛型型別需要擁有哪些行為。然後編譯器可以利用 trait 約束資訊檢查程式碼中所用到的具體型別是否提供了正確的行為。在動態型別語言中，如果我們呼叫了一個未定義的方法，會在執行時出現錯誤。Rust 將這些錯誤移動到了編譯時，甚至在程式碼能夠執行之前就強迫我們修復問題。另外，我們也無需編寫執行時檢查行為的程式碼，因為在編譯時就已經檢查過了。這樣既提升了效能又不必放棄泛型的靈活性。

[using-trait-objects-that-allow-for-values-of-different-types]:
ch18-02-trait-objects.html#使用-trait-object-來抽象出共享行為
[methods]: ch05-03-method-syntax.html#方法語法
