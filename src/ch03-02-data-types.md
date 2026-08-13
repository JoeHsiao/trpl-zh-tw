## 資料型別

[ch03-02-data-types.md](https://github.com/rust-lang/book/blob/9cc190796f28505c7a9a9cacea42f50d895ff3bd/src/ch03-02-data-types.md)

在 Rust 中，每個值都屬於某種特定的 **資料型別**（*data type*），這會告訴 Rust 當前指定的是什麼種類的資料，從而知道該如何處理這些資料。我們將看到兩類資料型別的子集：標量（scalar）和複合（compound）。

記住，Rust 是 **靜態型別**（*statically typed*）語言，也就是說，它必須在編譯時就知道所有變數的型別。編譯器通常可以根據值以及它的使用方式推斷出我們想要使用的型別。但在存在多種可能型別的情況下，比如我們在第二章[“比較猜測的數字和秘密數字”][comparing-the-guess-to-the-secret-number]一節中使用 `parse` 將 `String` 轉換為數值型別時，就必須加上型別註解，像這樣：

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

如果不按前面的程式碼那樣加上 `: u32` 型別註解，Rust 就會顯示如下錯誤。這說明編譯器需要我們提供更多資訊，才能知道我們想使用哪一種型別：

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

你會看到其它資料型別的各種型別註解。

### 標量型別

**標量**（*scalar*）型別代表一個單獨的值。Rust 有四種基本的標量型別：整型、浮點型、布林型別和字元型別。你可能在其他語言中見過它們。讓我們深入瞭解它們在 Rust 中是如何工作的。

#### 整型

**整型** 是沒有小數部分的數字。我們在第二章已經用過 `u32` 這種整數型別。這個型別宣告表明，它關聯的值應該是一個佔用 32 位空間的無符號整數（有符號整數型別以 `i` 開頭，而不是 `u`）。表格 3-1 展示了 Rust 內建的整數型別。我們可以使用其中任意一種來宣告整數值的型別。

<span class="caption">表格 3-1: Rust 中的整型</span>

| 長度 | 有符號 | 無符號 |
|------|--------|--------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| 架構相關 | `isize` | `usize` |

每一種變體都可以是有符號或無符號的，並且具有明確的大小。**有符號** 和 **無符號** 指的是數字是否可能為負數。換句話說，這個數字是需要帶符號的（有符號），還是它永遠為正，因此無需符號（無符號）。這有點像我們在紙上寫數字：當符號有意義時，數字前面會帶上加號或減號；但如果可以安全地假定它是正數，通常就不會寫加號。有符號數使用[二進位制補碼（two’s complement）][twos-complement]表示。

每一個有符號的變體可以儲存包含從 -(2<sup>n - 1</sup>) 到 2<sup>n - 1</sup> - 1 在內的數字，這裡 *n* 是變體使用的位數。所以 `i8` 可以儲存從 -(2<sup>7</sup>) 到 2<sup>7</sup> - 1 在內的數字，也就是從 -128 到 127。無符號的變體可以儲存從 0 到 2<sup>n</sup> - 1 的數字，所以 `u8` 可以儲存從 0 到 2<sup>8</sup> - 1 的數字，也就是從 0 到 255。

另外，`isize` 和 `usize` 型別依賴執行程式的計算機架構：64 位架構上它們是 64 位的，32 位架構上它們是 32 位的。

你可以使用表格 3-2 中展示的任意一種形式來編寫整數字面值。請注意，那些可能對應多種數值型別的數字字面值可以帶上型別字尾，例如 `57u8`，用來顯式指定型別。數字字面值也可以使用 `_` 作為視覺分隔符，方便閱讀，例如 `1_000`，它和 `1000` 的值完全相同。

<span class="caption">表格 3-2: Rust 中的整型字面值</span>

| 數字字面值 | 例子 |
|------------|------|
| Decimal（十進位制） | `98_222` |
| Hex（十六進位制） | `0xff` |
| Octal（八進位制） | `0o77` |
| Binary（二進位制） | `0b1111_0000` |
| Byte（位元組字面值，僅限 `u8`） | `b'A'` |

那麼該使用哪種整型呢？如果拿不定主意，Rust 的預設型別通常是一個不錯的起點：整型預設是 `i32`。而 `isize` 和 `usize` 主要用在對某種集合進行索引的場景中。

> ##### 整型溢位
>
> 假設你有一個 `u8` 型別的變數，它可以儲存 `0` 到 `255` 之間的值。如果你試圖把它改成超出該範圍的值，比如 `256`，就會發生 **整型溢位**（*integer overflow*），並可能導致兩種行為之一。當你在 debug 模式下編譯時，Rust 會加入整型溢位的檢查，並在發生這種情況時讓程式在執行時 *panic*。Rust 用 *panicking* 這個術語表示程式因錯誤而退出；我們會在第九章[“`panic!` 與不可恢復的錯誤”][unrecoverable-errors-with-panic]一節中更深入地討論 panic。
>
> 當你使用 `--release` flag 在 release 模式下編譯時，Rust **不會**加入會導致 panic 的整型溢位檢查。相反，如果發生溢位，Rust 會執行一種叫做 *two’s complement wrapping* 的行為。簡而言之，超過該型別最大值的數會“迴繞”到該型別所能表示的最小值。對於 `u8` 來說，`256` 會變成 `0`，`257` 會變成 `1`，依此類推。程式不會 panic，但變數得到的值很可能不是你原本期望的值。依賴整型溢位的迴繞行為通常被認為是一種錯誤。
>
> 為了顯式地處理溢位的可能性，可以使用這幾類標準庫提供的原始數字型別方法：
> * 所有模式下都可以使用 `wrapping_*` 方法進行 wrapping，如 `wrapping_add`
> * 如果 `checked_*` 方法發生溢位，則返回 `None` 值
> * 用 `overflowing_*` 方法返回值和一個布林值，表示是否出現溢位
> * 用 `saturating_*` 方法在值的最小值或最大值處進行飽和處理

#### 浮點型

Rust 也有兩個原生的 **浮點數**（*floating-point numbers*）型別，它們是帶小數點的數字。Rust 的浮點數型別是 `f32` 和 `f64`，分別佔 32 位和 64 位。預設型別是 `f64`，因為在現代 CPU 中，它與 `f32` 速度幾乎一樣，不過精度更高。所有的浮點型都是有符號的。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

浮點數按照 IEEE-754 標準表示。

#### 數值運算

Rust 中的所有數字型別都支援基本數學運算：加法、減法、乘法、除法和取餘。整數除法會向零舍入到最接近的整數。下面的程式碼展示瞭如何在 `let` 語句中使用各種數值運算：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

這些語句中的每個表示式使用了一個數學運算子並計算出了一個值，然後繫結給一個變數。[附錄 B][appendix_b]<!-- ignore --> 包含 Rust 提供的所有運算子的列表。

#### 布林型別

正如其他大部分程式語言一樣，Rust 中的布林型別有兩個可能的值：`true` 和 `false`。Rust 中的布林型別使用 `bool` 表示。例如：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

布林值最主要的使用場景是條件表示式，例如 `if` 表示式。我們會在[“控制流”][control-flow]一節介紹 `if` 表示式在 Rust 中是如何工作的。

#### 字元型別

Rust 的 `char` 型別是語言中最原始的字母型別。下面是一些宣告 `char` 值的例子：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

注意，我們使用單引號來表示 `char` 字面值，而字串字面值使用的是雙引號。Rust 的 `char` 型別大小為 4 個位元組，並表示一個 Unicode 標量值（Unicode Scalar Value），這意味著它所能表示的內容遠不止 ASCII。帶重音符號的字母，中文、日文、韓文字元，emoji，以及零寬空格，都是 Rust 中合法的 `char` 值。Unicode 標量值的範圍包括 `U+0000` 到 `U+D7FF`，以及 `U+E000` 到 `U+10FFFF`。不過，“字元”並不是 Unicode 中一個嚴格對應的概念，因此你直覺上認為的“字元”未必和 Rust 中的 `char` 一一對應。我們會在第八章[“使用字串儲存 UTF-8 編碼的文字”][strings]中更詳細地討論這個主題。

### 複合型別

**複合型別**（*compound types*）可以把多個值組合成一個型別。Rust 有兩種原生的複合型別：元組（tuple）和陣列（array）。

#### 元組型別

元組是一種將多個不同型別的值組合成一個複合型別的通用方式。元組長度固定：一旦宣告，它的大小就不能增長或縮小。

我們透過在圓括號中寫一組由逗號分隔的值來建立元組。元組中的每個位置都有一個型別，而且這些不同位置上的值型別不必相同。下面這個例子中加入了可選的型別註解：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

變數 `tup` 繫結到整個元組上，因為元組本身會被視為一個單獨的複合值。為了從元組中取出單個值，我們可以使用模式匹配（pattern matching）來解構（destructure）元組，像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

程式首先建立了一個元組並繫結到 `tup` 變數上。接著使用了 `let` 和一個模式將 `tup` 分成了三個不同的變數，`x`、`y` 和 `z`。這叫做 **解構**（*destructuring*），因為它將一個元組拆成了三個部分。最後，程式打印出了 `y` 的值，也就是 `6.4`。

我們也可以使用點號（`.`）後跟值的索引來直接訪問所需的元組元素。例如：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

這個程式建立了一個元組，`x`，然後使用其各自的索引訪問元組中的每個元素。跟大多數程式語言一樣，元組的第一個索引值是 0。

不帶任何值的元組有一個特殊名字，叫做 **單元（unit）**。這種值以及其對應的型別都寫作 `()`，表示空值或空的返回型別。如果一個表示式沒有返回任何其他值，它就會隱式返回單元值。

#### 陣列型別

另一種包含多個值的方式是 **陣列**（*array*）。和元組不同，陣列中的每個元素都必須具有相同型別。Rust 中的陣列也不同於某些其他語言中的陣列：Rust 的陣列長度是固定的。

我們將陣列的值寫成在方括號內，用逗號分隔的列表：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

當你希望把資料分配在棧（stack）上而不是堆（heap）上時（[第四章][stack-and-heap]會更詳細地討論棧與堆），或者當你想確保始終擁有固定數量的元素時，陣列就非常有用。不過，陣列不如 vector 型別靈活。vector 是標準庫提供的一種類似陣列的集合型別，它 **允許** 長度增長或縮小。如果你不確定該用陣列還是 vector，那麼很可能你應該用 vector。[第八章][vectors]會更詳細地討論 vector。

不過，當你明確知道元素個數不會變化時，陣列就更有用。例如，如果你在程式中使用月份名稱，那麼你大概會選擇陣列而不是 vector，因為你知道它始終只有 12 個元素。

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

可以像這樣編寫陣列的型別：在方括號中包含每個元素的型別，後跟分號，再後跟陣列元素的數量。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

這裡，`i32` 是每個元素的型別。分號之後，數字 `5` 表明該陣列包含五個元素。

你還可以透過在方括號中指定初始值加分號再加元素個數的方式來建立一個每個元素都為相同值的陣列：

```rust
let a = [3; 5];
```

變數名為 `a` 的陣列將包含 `5` 個元素，這些元素的值最初都將被設定為 `3`。這種寫法與 `let a = [3, 3, 3, 3, 3];` 效果相同，但更簡潔。

<a id="accessing-array-elements"></a>

##### 訪問陣列元素

陣列是在棧（stack）上分配的一整塊、大小已知且固定的記憶體。你可以像下面這樣使用索引來訪問陣列中的元素：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

在這個例子中，叫做 `first` 的變數的值是 `1`，因為它是陣列索引 `[0]` 的值。變數 `second` 將會是陣列索引 `[1]` 的值 `2`。

##### 無效的陣列元素訪問

讓我們看看如果嘗試訪問陣列末尾之後的元素會發生什麼。假設你執行下面這段程式碼，它類似於第 2 章中的猜數字遊戲：從使用者那裡讀取一個數組索引。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

這段程式碼能夠成功編譯。如果你用 `cargo run` 執行它，並輸入 `0`、`1`、`2`、`3` 或 `4`，程式就會打印出陣列中對應索引位置的值。相反，如果你輸入一個超出陣列末尾的數字，比如 `10`，你就會看到像下面這樣的輸出：

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

程式在索引操作中使用了無效值，因此產生了一個 **執行時** 錯誤。程式帶著錯誤資訊退出，並且沒有執行最後那條 `println!` 語句。當你嘗試透過索引訪問元素時，Rust 會檢查你指定的索引是否小於陣列長度。如果索引大於或等於陣列長度，Rust 就會 *panic*。這種檢查必須在執行時完成，尤其是在這種場景下，因為編譯器不可能知道使用者之後執行程式碼時會輸入什麼值。

這是 Rust 記憶體安全原則在實踐中的一個例子。在許多底層語言中，不會進行這種檢查，因此如果你提供了錯誤的索引，就可能訪問到無效記憶體。Rust 透過立即退出，而不是允許這次記憶體訪問繼續發生並讓程式往下執行，來保護你免受這類錯誤的影響。第九章會更詳細地討論 Rust 的錯誤處理機制，以及如何編寫既可讀又安全的程式碼，讓程式既不會 panic，也不會發生非法記憶體訪問。

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#比較猜測的數字和秘密數字
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#控制流
[strings]: ch08-02-strings.html#使用字串儲存-utf-8-編碼的文字
[stack-and-heap]: ch04-01-what-is-ownership.html#棧stack與堆heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.html
