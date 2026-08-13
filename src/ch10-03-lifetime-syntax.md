## 生命週期確保引用有效

[ch10-03-lifetime-syntax.md](https://github.com/rust-lang/book/blob/3986f214fd82427b4401adf4d7dc0911c917e1e8/src/ch10-03-lifetime-syntax.md)

生命週期是另一類我們已經使用過的泛型。不同於確保型別有期望的行為，生命週期用於保證引用在我們需要的整個期間內都是有效的。

當在第四章討論 [“引用和借用”][references-and-borrowing] 部分時，我們遺漏了一個重要的細節：Rust 中的每一個引用都有其**生命週期**（*lifetime*），也就是引用保持有效的作用域。大部分時候生命週期是隱含並可以推斷的，正如大部分時候型別也是可以推斷的一樣。類似於當因為有多種可能型別的時候必須註明型別，也會出現引用的生命週期以一些不同方式相關聯的情況，所以 Rust 需要我們使用泛型生命週期引數來註明它們的關係，這樣就能確保執行時實際使用的引用絕對是有效的。

生命週期註解甚至不是一個大部分語言都有的概念，所以這可能感覺起來有些陌生。雖然本章不可能涉及到它全部的內容，我們會講到一些通常你可能會遇到的生命週期語法以便你熟悉這個概念。

<a id="preventing-dangling-references-with-lifetimes"></a>

### 懸垂引用

生命週期的主要目標是避免**懸垂引用**（*dangling references*），後者會導致程式引用了非預期引用的資料。考慮一下示例 10-16 中的程式，它有一個外部作用域和一個內部作用域。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

<span class="caption">示例 10-16：嘗試使用離開作用域的值的引用</span>

> 注意：示例 10-16、10-17 和 10-23 中聲明瞭沒有初始值的變數，所以這些變數存在於外部作用域。這乍看之下好像和 Rust 不允許存在空值相沖突。然而如果嘗試在給它一個值之前使用這個變數，會出現一個編譯時錯誤，這就說明了 Rust 確實不允許空值。

外部作用域聲明瞭一個沒有初值的變數 `r`，而內部作用域聲明瞭一個初值為 `5` 的變數`x`。在內部作用域中，我們嘗試將 `r` 的值設定為一個 `x` 的引用。接著在內部作用域結束後，嘗試打印出 `r` 的值。這段程式碼不能編譯因為 `r` 引用的值在嘗試使用之前就離開了作用域。如下是錯誤資訊：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

變數 `x` 並沒有 “存在的足夠久”。其原因是 `x` 在到達第 7 行內部作用域結束時就離開了作用域。不過 `r` 在外部作用域仍是有效的；作用域越大我們就說它 “存在的越久”。如果 Rust 允許這段程式碼工作，`r` 將會引用在 `x` 離開作用域時被釋放的記憶體，這時嘗試對 `r` 做任何操作都不能正常工作。那麼 Rust 是如何決定這段程式碼是不被允許的呢？這得益於借用檢查器。

#### 借用檢查器

Rust 編譯器有一個**借用檢查器**（*borrow checker*），它比較作用域來確保所有的借用都是有效的。示例 10-17 展示了與示例 10-16 相同的例子不過帶有變數生命週期的註釋：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

<span class="caption">示例 10-17：`r` 和 `x` 的生命週期註解，分別叫做 `'a` 和 `'b`</span>

這裡將 `r` 的生命週期標記為 `'a` 並將 `x` 的生命週期標記為 `'b`。如你所見，內部的 `'b` 塊要比外部的生命週期 `'a` 小得多。在編譯時，Rust 比較這兩個生命週期的大小，並發現 `r` 擁有生命週期 `'a`，不過它引用了一個擁有生命週期 `'b` 的物件。程式被拒絕編譯，因為生命週期 `'b` 比生命週期 `'a` 要小：被引用的物件比它的引用者存在的時間更短。

讓我們看看示例 10-18 中這個並沒有產生懸垂引用且可以正確編譯的例子：

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

<span class="caption">示例 10-18：一個有效的引用，因為資料比引用有著更長的生命週期</span>

這裡 `x` 擁有生命週期 `'b`，比 `'a` 要大。這就意味著 `r` 可以引用 `x`：Rust 知道 `r` 中的引用在 `x` 有效的時候也總是有效的。

現在我們已經在一個具體的例子中展示了引用的生命週期位於何處，並討論了 Rust 如何分析生命週期來保證引用總是有效的，接下來讓我們聊聊在函式的上下文中引數和返回值的泛型生命週期。

### 函式中的泛型生命週期

讓我們來編寫一個返回兩個字串 slice 中較長者的函式。這個函式獲取兩個字串 slice 並返回一個字串 slice。一旦我們實現了 `longest` 函式，示例 10-19 中的程式碼應該會打印出 `The longest string is abcd`：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

<span class="caption">示例 10-19：`main` 函式呼叫 `longest` 函式來尋找兩個字串 slice 中較長的一個</span>

注意這個函式獲取作為引用的字串 slice，而不是字串，因為我們不希望 `longest` 函式獲取引數的所有權。參考之前第四章中的[“字串 slice 作為引數”][string-slices-as-parameters]部分中更多關於為什麼示例 10-19 的引數正符合我們期望的討論。

如果嘗試像示例 10-20 中那樣實現 `longest` 函式，它並不能編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

<span class="caption">示例 10-20：一個 `longest` 函式的實現，它返回兩個字串 slice 中較長者，現在還不能編譯</span>

相應地會出現如下有關生命週期的錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

提示文字揭示了返回值需要一個泛型生命週期引數，因為 Rust 並不知道將要返回的引用是指向 `x` 或 `y`。事實上我們也不知道，因為函式體中 `if` 塊返回一個 `x` 的引用而 `else` 塊返回一個 `y` 的引用！

當我們定義這個函式的時候，並不知道傳遞給函式的具體值，所以也不知道到底是 `if` 還是 `else` 會被執行。我們也不知道傳入的引用的具體生命週期，所以也就不能像示例 10-17 和 10-18 那樣透過觀察作用域來確定返回的引用是否總是有效。借用檢查器自身同樣也無法確定，因為它不知道 `x` 和 `y` 的生命週期是如何與返回值的生命週期相關聯的。為了修復這個錯誤，我們將增加泛型生命週期引數來定義引用間的關係以便借用檢查器可以進行分析。

### 生命週期註解語法

生命週期註解並不改變任何引用的生命週期的長短。相反它們描述了多個引用生命週期相互的關係，而不影響其生命週期。與當函式簽名中指定了泛型型別引數後就可以接受任何型別一樣，當指定了泛型生命週期後函式也能接受任何生命週期的引用。

生命週期註解有著一個不太常見的語法：生命週期引數名稱必須以撇號（`'`）開頭，其名稱通常全是小寫，類似於泛型其名稱非常短。大多數人使用 `'a` 作為第一個生命週期註解。生命週期引數註解位於引用的 `&` 之後，並有一個空格來將引用型別與生命週期註解分隔開。

這裡有一些例子：我們有一個沒有生命週期引數的 `i32` 的引用，一個有叫做 `'a` 的生命週期引數的 `i32` 的引用，和一個生命週期也是 `'a` 的 `i32` 的可變引用：

```rust,ignore
&i32        // 引用
&'a i32     // 帶有顯式生命週期的引用
&'a mut i32 // 帶有顯式生命週期的可變引用
```

單個的生命週期註解本身沒有多少意義，因為生命週期註解告訴 Rust 多個引用的泛型生命週期引數如何相互聯絡的。讓我們在 `longest` 函式的上下文中理解生命週期註解如何相互聯絡。

<a id="lifetime-annotations-in-function-signatures"></a>

### 在函式簽名中

為了在函式簽名中使用生命週期註解，需要在函式名和引數列表間的尖括號中宣告泛型生命週期（*lifetime*）引數，就像泛型型別（*type*）引數一樣。

我們希望這個簽名表達如下約束：返回的引用只要兩個引數都有效，就會一直有效。這就是引數生命週期和返回值生命週期之間的關係。我們將這個生命週期命名為 `'a`，然後像示例 10-21 那樣將它加到每個引用上。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<span class="caption">示例 10-21：`longest` 函式定義指定了簽名中所有的引用必須有相同的生命週期 `'a`</span>

這段程式碼能夠編譯並會產生我們希望得到的示例 10-19 中的 `main` 函式的結果。

現在函式簽名表明對於某些生命週期 `'a`，函式會獲取兩個引數，它們都是與生命週期 `'a` 存在的至少一樣長的字串 slice。函式會返回一個同樣也與生命週期 `'a` 存在的至少一樣長的字串 slice。它的實際含義是 `longest` 函式返回的引用的生命週期與函式引數所引用的值的生命週期的較小者一致。這些關係就是我們希望 Rust 分析程式碼時所使用的。

記住透過在函式簽名中指定生命週期引數時，我們並沒有改變任何傳入值或返回值的生命週期，而是指出任何不滿足這個約束條件的值都將被借用檢查器拒絕。注意 `longest` 函式並不需要知道 `x` 和 `y` 具體會存在多久，而只需要知道有某個可以被 `'a` 替代的作用域將會滿足這個簽名。

當在函式中使用生命週期註解時，這些註解出現在函式簽名中，而不存在於函式體中的任何程式碼中。生命週期註解成為了函式約定的一部分，非常像簽名中的型別。讓函式簽名包含生命週期約定意味著 Rust 編譯器的工作變得更簡單了。如果函式註解有誤或者呼叫方法不對，編譯器錯誤可以更準確地指出程式碼和限制的部分。如果不這麼做的話，Rust 編譯會對我們期望的生命週期關係做更多的推斷，這樣編譯器可能只能指出離出問題地方很多步之外的程式碼。

當具體的引用被傳遞給 `longest` 時，被 `'a` 所替代的具體生命週期是 `x` 的作用域與 `y` 的作用域相重疊的那一部分。換一種說法就是泛型生命週期 `'a` 的具體生命週期等同於 `x` 和 `y` 的生命週期中較小的那一個。因為我們用相同的生命週期引數 `'a` 標註了返回的引用值，所以返回的引用值就能保證在 `x` 和 `y` 中較短的那個生命週期結束之前保持有效。

讓我們看看如何透過傳遞擁有不同具體生命週期的引用來限制 `longest` 函式的使用。示例 10-22 是一個很直觀的例子。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<span class="caption">示例 10-22：透過擁有不同的具體生命週期的 `String` 值呼叫 `longest` 函式</span>

在這個例子中，`string1` 直到外部作用域結束都是有效的，`string2` 則在內部作用域中是有效的，而 `result` 則引用了一些直到內部作用域結束都是有效的值。借用檢查器認可這些程式碼；它能夠編譯和執行，並打印出 `The longest string is long string is long`。

接下來，讓我們嘗試另外一個例子，該例子揭示了 `result` 的引用的生命週期必須是兩個引數中較短的那個。以下程式碼將 `result` 變數的宣告移動出內部作用域，但是將 `result` 和 `string2` 變數的賦值語句一同留在內部作用域中。接著，使用了變數 `result` 的 `println!` 也被移動到內部作用域之外。注意示例 10-23 中的程式碼不能透過編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<span class="caption">示例 10-23：嘗試在 `string2` 離開作用域之後使用 `result` </span>

如果嘗試編譯這段程式碼會出現如下錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

錯誤表明為了保證 `println!` 中的 `result` 是有效的，`string2` 需要直到外部作用域結束都是有效的。Rust 知道這些是因為（`longest`）函式的引數和返回值都使用了相同的生命週期引數 `'a`。

作為人類，我們可以直觀地發現 `string1` 比 `string2` 更長，因此 `result` 會包含指向 `string1` 的引用。因為 `string1` 尚未離開作用域，對於 `println!` 來說 `string1` 的引用仍然是有效的。然而，編譯器並不能識別出這種情況。我們透過生命週期引數告訴 Rust 的是： `longest` 函式返回的引用的生命週期應該與傳入引數的生命週期中較短那個保持一致。因此，借用檢查器不允許示例 10-23 中的程式碼，因為它可能會存在無效的引用。

請嘗試更多采用不同的值和不同生命週期的引用作為 `longest` 函式的引數和返回值的實驗。並在開始編譯前猜想你的實驗能否透過借用檢查器，接著編譯一下看看你的理解是否正確！

<a id="thinking-in-terms-of-lifetimes"></a>

### 關係

指定生命週期引數的正確方式依賴函式實現的具體功能。例如，如果將 `longest` 函式的實現修改為總是返回第一個引數而不是最長的字串 slice，就不需要為引數 `y` 指定一個生命週期。如下程式碼將能夠編譯：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

我們為引數 `x` 和返回值指定了生命週期引數 `'a`，不過沒有為引數 `y` 指定，因為 `y` 的生命週期與引數 `x` 和返回值的生命週期沒有任何關係。

當從函式返回一個引用，返回值的生命週期引數需要與一個引數的生命週期引數相匹配。如果返回的引用**沒有**指向任何一個引數，那麼唯一的可能就是它指向一個函式內部建立的值。然而它將會是一個懸垂引用，因為它將會在函式結束時離開作用域。嘗試考慮這個並不能編譯的 `longest` 函式實現：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

即便我們為返回值指定了生命週期引數 `'a`，這個實現卻編譯失敗了，因為返回值的生命週期與引數完全沒有關聯。這裡是會出現的錯誤資訊：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

出現的問題是 `result` 在 `longest` 函式的結尾將離開作用域並被清理，而我們嘗試從函式返回一個 `result` 的引用。無法指定生命週期引數來改變懸垂引用，而且 Rust 也不允許我們建立一個懸垂引用。在這種情況，最好的解決方案是返回一個有所有權的資料型別而不是一個引用，這樣函式呼叫者就需要負責清理這個值了。

綜上，生命週期語法是用於將函式的多個引數與其返回值的生命週期進行關聯的。一旦它們形成了某種關聯，Rust 就有了足夠的資訊來允許記憶體安全的操作並阻止會產生懸垂指標亦或是違反記憶體安全的行為。

<a id="lifetime-annotations-in-struct-definitions"></a>

### 在結構體定義中

目前為止，我們定義的結構體全都包含擁有所有權的型別。也可以定義包含引用的結構體，不過這需要為結構體定義中的每一個引用新增生命週期註解。示例 10-24 中有一個存放了一個字串 slice 的結構體 `ImportantExcerpt`。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

<span class="caption">示例 10-24：一個存放引用的結構體，所以其定義需要生命週期註解</span>

這個結構體只有一個欄位 `part`，它存放了一個字串 slice，這是一個引用。類似於泛型引數型別，必須在結構體名稱後面的尖括號中宣告泛型生命週期引數，以便在結構體定義中使用生命週期引數。這個註解意味著 `ImportantExcerpt` 的例項不能比其 `part` 欄位中的引用存在的更久。

這裡的 `main` 函式建立了一個 `ImportantExcerpt` 的例項，它存放了變數 `novel` 所擁有的 `String` 的第一個句子的引用。`novel` 的資料在 `ImportantExcerpt` 例項建立之前就存在。另外，直到 `ImportantExcerpt` 離開作用域之後 `novel` 都不會離開作用域，所以 `ImportantExcerpt` 例項中的引用是有效的。

### 生命週期省略（Lifetime Elision）

現在我們已經知道了每一個引用都有一個生命週期，而且我們需要為那些使用了引用的函式或結構體指定生命週期。然而，第四章的示例 4-9 中有一個函式，再次展示為如示例 10-25 所示，它沒有生命週期註解卻能編譯成功：

<span class="filename">檔名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

<span class="caption">示例 10-25：示例 4-9 定義了一個沒有使用生命週期註解的函式，即便其引數和返回值都是引用</span>

這個函式沒有生命週期註解卻能編譯是由於一些歷史原因：在早期版本（pre-1.0）的 Rust 中，這的確是不能編譯的。每一個引用都必須有明確的生命週期。那時的函式簽名將會寫成這樣：

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

在編寫了很多 Rust 程式碼後，Rust 團隊發現在特定情況下 Rust 程式設計師們總是重複地編寫一模一樣的生命週期註解。這些場景是可預測的並且遵循幾個明確的模式。接著 Rust 團隊就把這些模式編碼進了 Rust 編譯器中，如此借用檢查器在這些情況下就能推斷出生命週期而不再強制程式設計師顯式的增加註解。

這裡我們提到一些 Rust 的歷史是因為更多的明確的模式被合併和新增到編譯器中是完全可能的。未來可能只需要更少的生命週期註解。

被編碼進 Rust 引用分析的模式被稱為 **生命週期省略規則**（*lifetime elision rules*）。這並不是需要程式設計師遵守的規則；這些規則是一系列特定的場景，此時編譯器會考慮，如果程式碼符合這些場景，就無需明確指定生命週期。

省略規則並不提供完整的推斷。如果 Rust 在明確遵守這些規則的前提下變數的生命週期仍然是模稜兩可的話，它不會猜測剩餘引用的生命週期應該是什麼。編譯器會在可以透過增加生命週期註解來解決錯誤問題的地方給出一個錯誤提示，而不是進行推斷或猜測。

函式或方法的引數的生命週期被稱為 **輸入生命週期**（*input lifetimes*），而返回值的生命週期被稱為 **輸出生命週期**（*output lifetimes*）。

編譯器採用三條規則來判斷引用何時不需要明確的註解。第一條規則適用於輸入生命週期，後兩條規則適用於輸出生命週期。如果編譯器檢查完這三條規則後仍然存在沒有計算出生命週期的引用，編譯器將會停止並生成錯誤。這些規則適用於 `fn` 定義，以及 `impl` 塊。

第一條規則是編譯器為每一個引用引數都分配一個生命週期引數。換句話說就是，函式有一個引用引數的就有一個生命週期引數：`fn foo<'a>(x: &'a i32)`，有兩個引用引數的函式就有兩個不同的生命週期引數，`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，依此類推。

第二條規則是如果只有一個輸入生命週期引數，那麼將它賦予給所有輸出生命週期引數：`fn foo<'a>(x: &'a i32) -> &'a i32`。

第三條規則是如果方法有多個輸入生命週期引數並且其中一個引數是 `&self` 或 `&mut self`，說明這是個方法，那麼所有輸出生命週期引數被賦予 `self` 的生命週期。第三條規則使得方法更容易讀寫，因為只需更少的符號。

假設我們自己就是編譯器。並應用這些規則來計算示例 10-25 中 `first_word` 函式簽名中的引用的生命週期。開始時簽名中的引用並沒有關聯任何生命週期：

```rust,ignore
fn first_word(s: &str) -> &str {
```

接著編譯器應用第一條規則，也就是每個引用引數都有其自己的生命週期。我們像往常一樣稱之為 `'a`，所以現在簽名看起來像這樣：

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

對於第二條規則，因為這裡正好只有一個輸入生命週期引數所以是適用的。第二條規則表明輸入引數的生命週期將被賦予輸出生命週期引數，所以現在簽名看起來像這樣：

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

現在這個函式簽名中的所有引用都有了生命週期，如此編譯器可以繼續它的分析而無須程式設計師標記這個函式簽名中的生命週期。

讓我們再看看另一個例子，這次我們從示例 10-20 中沒有生命週期引數的 `longest` 函式開始：

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

再次假設我們自己就是編譯器並應用第一條規則：每個引用引數都有其自己的生命週期。這次有兩個引數，所以就有兩個（不同的）生命週期：

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

再來應用第二條規則，因為函式存在多個輸入生命週期，它並不適用於這種情況。再來看第三條規則，它同樣也不適用，這是因為沒有 `self` 引數。應用了三個規則之後編譯器還沒有計算出返回值型別的生命週期。這就是在編譯示例 10-20 的程式碼時會出現錯誤的原因：編譯器使用所有已知的生命週期省略規則，仍不能計算出簽名中所有引用的生命週期。

因為第三條規則真正能夠適用的就只有方法簽名，現在就讓我們看看那種情況中的生命週期，並看看為什麼這條規則意味著我們經常不需要在方法簽名中標註生命週期。

<a id="lifetime-annotations-in-method-definitions"></a>

### 在方法定義中

當為帶有生命週期的結構體實現方法時，其語法依然類似示例 10-11 中展示的泛型型別引數的語法。我們在哪裡宣告和使用生命週期引數，取決於它們是與結構體欄位相關還是與方法引數和返回值相關。

（實現方法時）結構體欄位的生命週期必須總是在 `impl` 關鍵字之後宣告並在結構體名稱之後被使用，因為這些生命週期是結構體型別的一部分。

`impl` 塊裡的方法簽名中，引用可能與結構體欄位中的引用相關聯，也可能是獨立的。另外，生命週期省略規則也經常讓我們無需在方法簽名中使用生命週期註解。讓我們看看一些使用示例 10-24 中定義的結構體 `ImportantExcerpt` 的例子。

首先，這裡有一個方法 `level`。其唯一的引數是 `self` 的引用，而且返回值只是一個 `i32`，並不引用任何值：

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

`impl` 之後和型別名稱之後的生命週期引數是必要的，不過因為第一條生命週期規則我們並不必須標註 `self` 引用的生命週期。

這裡是一個適用於第三條生命週期省略規則的例子：

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

這裡有兩個輸入生命週期，所以 Rust 應用第一條生命週期省略規則並給予 `&self` 和 `announcement` 它們各自的生命週期。接著，因為其中一個引數是 `&self`，返回值型別被賦予了 `&self` 的生命週期，這樣所有的生命週期都被計算出來了。

### 靜態生命週期

這裡有一種特殊的生命週期值得討論：`'static`，其生命週期**能夠**存活於整個程式期間。所有的字串字面值都擁有 `'static` 生命週期，我們也可以選擇像下面這樣標註出來：

```rust
let s: &'static str = "I have a static lifetime.";
```

這個字串的文字被直接儲存在程式的二進位制檔案中而這個檔案總是可用的。因此所有的字串字面值都是 `'static` 的。

你可能在錯誤資訊的幫助文字中見過使用 `'static` 生命週期的建議，不過將引用指定為 `'static` 之前，思考一下這個引用是否真的在整個程式的生命週期裡都有效，以及你是否希望它存在得這麼久。大部分情況中，推薦 `'static` 生命週期的錯誤資訊都是嘗試建立一個懸垂引用或者可用的生命週期不匹配的結果。在這種情況下的解決方案是修復這些問題而不是指定一個 `'static` 的生命週期。

<a id="generic-type-parameters-trait-bounds-and-lifetimes-together"></a>

### 泛型型別引數、trait 約束和生命週期

讓我們簡要的看一下在同一函式中指定泛型型別引數、trait 約束和生命週期的語法！

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

這個是示例 10-21 中那個返回兩個字串 slice 中較長者的 `longest` 函式，不過帶有一個額外的引數 `ann`。`ann` 的型別是泛型 `T`，它可以被放入任何實現了 `where` 從句中指定的 `Display` trait 的型別。這個額外的引數會使用 `{}` 列印，這也就是為什麼 `Display` trait 約束是必須的。因為生命週期也是泛型，所以生命週期引數 `'a` 和泛型型別引數 `T` 都位於函式名後的同一尖括號列表中。

## 總結

這一章介紹了很多的內容！現在你知道了泛型型別引數、trait 和 trait 約束以及泛型生命週期型別，你已經準備好編寫既不重複又能適用於多種場景的程式碼了。泛型型別引數意味著程式碼可以適用於不同的型別。trait 和 trait 約束保證了即使型別是泛型的，這些型別也會擁有所需要的行為。由生命週期註解所指定的引用生命週期之間的關係保證了這些靈活多變的程式碼不會出現懸垂引用。而所有的這一切發生在編譯時所以不會影響執行時效率！

你可能不會相信，這個話題還有更多需要學習的內容：第十八章會討論 trait 物件，這是另一種使用 trait 的方式。還有更多更復雜的涉及生命週期註解的場景，只有在非常高階的情況下才會需要它們；對於這些內容，請閱讀 [Rust Reference][reference]。不過接下來，讓我們聊聊如何在 Rust 中編寫測試，來確保程式碼的所有功能能像我們希望的那樣工作！

[references-and-borrowing]:
ch04-02-references-and-borrowing.html#引用與借用
[string-slices-as-parameters]:
ch04-03-slices.html#字串-slice-作為引數
[reference]: https://doc.rust-lang.org/reference/index.html
