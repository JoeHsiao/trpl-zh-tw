## 泛型資料型別

[ch10-01-syntax.md](https://github.com/rust-lang/book/blob/3986f214fd82427b4401adf4d7dc0911c917e1e8/src/ch10-01-syntax.md)

我們使用泛型來為函式簽名或結構體之類的項建立定義，這樣它們就可以配合多種不同的具體資料型別使用。先來看看如何使用泛型定義函式、結構體、列舉和方法。然後再討論泛型會如何影響程式碼效能。

### 在函式定義中使用泛型

當使用泛型定義函式時，本來在函式簽名中指定引數和返回值的型別的地方，會改用泛型來表示。採用這種技術，使得程式碼適應性更強，從而為函式的呼叫者提供更多的功能，同時也避免了程式碼的重複。

回到 `largest` 函式。示例 10-4 展示了兩個都用來尋找切片中最大值的函式。接著，我們會把它們合併成一個使用泛型的函式。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">示例 10-4：兩個函式，不同點只是名稱和簽名型別</span>

`largest_i32` 函式是從示例 10-3 中摘出來的，它用來尋找 slice 中最大的 `i32`。`largest_char` 函式尋找 slice 中最大的 `char`。因為兩者函式體的程式碼是一樣的，我們可以定義一個函式，再引進泛型引數來消除這種重複。

為了給這個新函式中的型別做引數化，我們需要給型別引數命名，就像給函式的值引數命名一樣。任何識別符號都可以作為型別引數名。但這裡我們使用 `T`，因為按照慣例，Rust 中的型別引數名都很短，通常只有一個字母，而 Rust 型別名的命名約定是 UpperCamelCase。`T` 是 _type_ 的縮寫，也是大多數 Rust 程式設計師的預設選擇。

如果你要在函式體中使用某個引數，就必須先在函式簽名中宣告它的名字，讓編譯器知道這個名字表示什麼。同理，當你在函式簽名中使用型別引數名時，也必須先宣告它。為了定義泛型版的 `largest` 函式，我們把型別名宣告放在函式名和引數列表之間的尖括號 `<>` 裡，像這樣：

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

我們可以把這個定義讀作：“函式 `largest` 對某個型別 `T` 是泛型的。”這個函式有一個名為 `list` 的引數，它是由 `T` 型別值組成的切片。`largest` 函式會返回一個指向同樣型別 `T` 值的引用。

示例 10-5 中的 `largest` 函式在它的簽名中使用了泛型，統一了兩個實現。該示例也展示瞭如何呼叫 `largest` 函式，把 `i32` 值的 slice 或 `char` 值的 slice 傳給它。請注意這些程式碼還不能編譯。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">示例 10-5：一個使用泛型引數的 `largest` 函式定義，尚不能編譯</span>

如果現在就編譯這段程式碼，會出現如下錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

幫助資訊裡提到了 `std::cmp::PartialOrd`，這是一個 trait，我們會在下一節討論 trait。現在先知道，這個錯誤說明 `largest` 的函式體並不能適用於 `T` 所有可能的型別。因為我們想在函式體中比較 `T` 型別的值，所以只能使用那些值可以排序的型別。為了支援比較，標準庫提供了 `std::cmp::PartialOrd` trait，你可以為型別實現它（更多內容見附錄 C）。要修復示例 10-5，我們可以按照幫助資訊的建議，把 `T` 限制為只接受實現了 `PartialOrd` 的型別。這樣程式碼就能編譯，因為標準庫已經為 `i32` 和 `char` 實現了 `PartialOrd`。

### 結構體定義中的泛型

我們也可以使用 `<>` 語法來定義結構體，讓一個或多個欄位使用泛型型別引數。示例 10-6 定義了一個 `Point<T>` 結構體，用來儲存任意型別的 `x` 和 `y` 座標值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">示例 10-6：`Point` 結構體存放了兩個 `T` 型別的值 `x` 和 `y`</span>

在結構體定義中使用泛型的語法類似於函式定義中使用泛型。首先，必須在結構體名稱後面的尖括號中宣告泛型引數的名稱。接著在結構體定義中可以指定具體資料型別的位置使用泛型型別。

注意 `Point<T>` 的定義中只使用了一個泛型型別，這個定義表明結構體 `Point<T>` 對於一些型別 `T` 是泛型的，而且欄位 `x` 和 `y` **都是** 相同型別的，無論它具體是何型別。如果嘗試建立一個有不同型別值的 `Point<T>` 的例項，像示例 10-7 中的程式碼就不能編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">示例 10-7：欄位 `x` 和 `y` 的型別必須相同，因為它們都有相同的泛型型別 `T`</span>

在這個例子中，當把整型值 `5` 賦值給 `x` 時，就告訴了編譯器這個 `Point<T>` 例項中的泛型 `T` 全是整型。接著指定 `y` 為浮點值 `4.0`，因為 `y` 被定義為與 `x` 相同型別，所以將會得到一個像這樣的型別不匹配錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

如果想要定義一個 `x` 和 `y` 可以有不同型別且仍然是泛型的 `Point` 結構體，我們可以使用多個泛型型別引數。在示例 10-8 中，我們修改 `Point` 的定義為擁有兩個泛型型別 `T` 和 `U`。其中欄位 `x` 是 `T` 型別的，而欄位 `y` 是 `U` 型別的：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">示例 10-8：使用兩個泛型的 `Point`，這樣 `x` 和 `y` 可能是不同型別</span>

現在，示例裡的所有這些 `Point` 例項都合法了！你可以在定義中使用任意多個泛型型別引數，但如果超過幾個，程式碼就會變得難以閱讀。如果你發現自己在程式碼裡需要很多泛型型別，那可能意味著這段程式碼應該被重構成更小的部分。

### 列舉定義中的泛型

和結構體類似，列舉也可以在成員中存放泛型資料型別。第六章我們曾用過標準庫提供的 `Option<T>` 列舉，這裡再回顧一下：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

現在這個定義應該更容易理解了。如你所見 `Option<T>` 是一個擁有泛型 `T` 的列舉，它有兩個成員：`Some`，它存放了一個型別 `T` 的值，和不存在任何值的`None`。透過 `Option<T>` 列舉可以表達有一個可能的值的抽象概念，同時因為 `Option<T>` 是泛型的，無論這個可能的值是什麼型別都可以使用這個抽象。

列舉也可以擁有多個泛型型別。第九章使用過的 `Result` 列舉定義就是一個這樣的例子：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 列舉有兩個泛型型別，`T` 和 `E`。`Result` 有兩個成員：`Ok`，它存放一個型別 `T` 的值，而 `Err` 則存放一個型別 `E` 的值。這個定義使得 `Result` 列舉能很方便的表達任何可能成功（返回 `T` 型別的值）也可能失敗（返回 `E` 型別的值）的操作。實際上，這就是我們在示例 9-3 用來開啟檔案的方式：當成功開啟檔案的時候，`T` 對應的是 `std::fs::File` 型別；而當開啟檔案出現問題時，`E` 的值則是 `std::io::Error` 型別。

當你意識到程式碼中定義了多個結構體或列舉，它們不一樣的地方只是其中的值的型別的時候，不妨透過泛型型別來避免重複。

### 方法定義中的泛型

我們可以像第五章那樣為結構體和列舉實現方法，並在這些方法定義中使用泛型。示例 10-9 展示了示例 10-6 中定義的 `Point<T>` 結構體，以及在其上實現的一個名為 `x` 的方法。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">示例 10-9：在 `Point<T>` 結構體上實現方法 `x`，它返回 `T` 型別的欄位 `x` 的引用</span>

這裡在 `Point<T>` 上定義了一個叫做 `x` 的方法用於返回欄位 `x` 中資料的引用。

注意必須在 `impl` 後面宣告 `T`，這樣就可以在 `Point<T>` 上實現的方法中使用 `T` 了。透過在 `impl` 之後宣告泛型 `T`，Rust 就知道 `Point` 的尖括號中的型別是泛型而不是具體型別。我們可以為泛型引數選擇一個與結構體定義中宣告的泛型引數所不同的名稱，不過依照慣例使用了相同的名稱。如果你在`impl`中編寫一個宣告泛型型別的方法，那麼該方法將在任何型別的例項上定義，無論最終用什麼具體型別來替換泛型型別。（譯者注：以示例 10-9 為例，`impl` 中聲明瞭泛型型別引數 `T`，`x` 是編寫在 `impl` 中的方法，`x` 方法將會定義在 `Point<T>` 的任何例項上，無論最終替換泛型型別引數 `T` 的是何具體型別）。

定義方法時也可以為泛型指定限制（constraint）。例如，可以選擇為 `Point<f32>` 例項實現方法，而不是為泛型 `Point` 例項。示例 10-10 展示了一個沒有在 `impl` 之後（的尖括號）宣告泛型的例子，這裡使用了一個具體型別，`f32`：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">示例 10-10：構建一個只用於擁有泛型引數 `T` 的結構體的具體型別的 `impl` 塊</span>

這段程式碼意味著 `Point<f32>` 型別會有一個方法 `distance_from_origin`，而其他 `T` 不是 `f32` 型別的 `Point<T>` 例項則沒有定義此方法。這個方法計算點例項與座標 (0.0, 0.0) 之間的距離，並使用了只能用於浮點型的數學運算子。

結構體定義中的泛型型別引數並不總是與結構體方法簽名中使用的泛型是同一型別。示例 10-11 中為 `Point` 結構體使用了泛型型別 `X1` 和 `Y1`，為 `mixup` 方法簽名使用了 `X2` 和 `Y2` 來使得示例更加清楚。這個方法用 `self` 的 `Point` 型別的 `x` 值（型別 `X1`）和引數的 `Point` 型別的 `y` 值（型別 `Y2`）來建立一個新 `Point` 型別的例項：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">示例 10-11：方法使用了與結構體定義中不同型別的泛型</span>

在 `main` 函式中，定義了一個有 `i32` 型別的 `x`（其值為 `5`）和 `f64` 的 `y`（其值為 `10.4`）的 `Point`。`p2` 則是一個有著字串 slice 型別的 `x`（其值為 `"Hello"`）和 `char` 型別的 `y`（其值為`c`）的 `Point`。在 `p1` 上以 `p2` 作為引數呼叫 `mixup` 會返回一個 `p3`，它會有一個 `i32` 型別的 `x`，因為 `x` 來自 `p1`，並擁有一個 `char` 型別的 `y`，因為 `y` 來自 `p2`。`println!` 會打印出 `p3.x = 5, p3.y = c`。

這個例子的目的是展示一些泛型透過 `impl` 宣告而另一些透過方法定義宣告的情況。這裡泛型引數 `X1` 和 `Y1` 聲明於 `impl` 之後，因為它們與結構體定義相對應。而泛型引數 `X2` 和 `Y2` 聲明於 `fn mixup` 之後，因為它們只是相對於方法本身的。

### 泛型程式碼的效能

你可能會好奇，使用泛型型別引數是否會帶來執行時開銷。好訊息是：使用泛型不會讓程式比使用具體型別執行得更慢。

Rust 透過在編譯時對泛型程式碼進行**單態化**（*monomorphization*）來實現這一點。單態化就是把泛型程式碼轉換成具體程式碼的過程，方法是用編譯時實際用到的具體型別去填充泛型程式碼。

在這個過程中，編譯器所做的工作正好與示例 10-5 中我們建立泛型函式的步驟相反。編譯器尋找所有泛型程式碼被呼叫的位置並使用泛型程式碼針對具體型別生成程式碼。

讓我們看看這在標準庫的 `Option<T>` 列舉上是如何工作的：

```rust
let integer = Some(5);
let float = Some(5.0);
```

當 Rust 編譯這些程式碼的時候，它會進行單態化。編譯器會讀取傳遞給 `Option<T>` 的值並發現有兩種 `Option<T>`：一個對應 `i32` 另一個對應 `f64`。為此，它會將泛型定義 `Option<T>` 展開為兩個針對 `i32` 和 `f64` 的定義，接著將泛型定義替換為這兩個具體的定義。

編譯器生成的單態化版本的程式碼看起來像這樣（編譯器會使用不同於如下假想的名字）：

<span class="filename">檔名：src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

泛型 `Option<T>` 被編譯器替換為了具體的定義。因為 Rust 會將每種情況下的泛型程式碼編譯為具體型別，使用泛型沒有執行時開銷。當代碼執行時，它的執行效率就跟好像手寫每個具體定義的重複程式碼一樣。這個單態化過程正是 Rust 泛型在執行時極其高效的原因。
