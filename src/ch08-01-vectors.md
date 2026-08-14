## 使用 Vector 儲存列表

[ch08-01-vectors.md](https://github.com/rust-lang/book/blob/2581c23b669eff30c26e036a13475ec5cf70c1b8/src/ch08-01-vectors.md)

我們要討論的第一種集合型別是 `Vec<T>`，也被稱為 *vector*。vector 允許你在單個數據結構中存放多個值，並把這些值在記憶體中彼此相鄰地排列起來。vector 只能儲存相同型別的值。當你有一組專案要處理時，它就很有用，例如檔案中的文本行，或者購物車中商品的價格。

### 新建 vector

要建立一個新的空 vector，可以呼叫 `Vec::new` 函式，如示例 8-1 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">示例 8-1：新建一個空的 vector 來儲存 `i32` 型別的值</span>

注意，這裡我們加了一個型別註解。因為還沒有往這個 vector 裡插入任何值，Rust 並不知道我們打算儲存什麼型別的元素。這一點很重要。vector 是使用泛型實現的；第十章會講到如何在你自己的型別上使用泛型。現在你只需要知道，標準庫提供的 `Vec<T>` 型別可以容納任意型別。當我們建立一個用來存放特定型別的 vector 時，可以在尖括號中指定這個型別。在示例 8-1 中，我們告訴 Rust，`v` 中的 `Vec<T>` 將存放 `i32` 型別的元素。

更常見的情況是，我們會用初始值建立 `Vec<T>`，而 Rust 會推斷出你想儲存的值的型別，所以很少需要寫這種型別註解。Rust 還很貼心地提供了 `vec!` 宏，它會建立一個新的 vector，並把你提供的值放進去。示例 8-2 建立了一個包含 `1`、`2` 和 `3` 的新 `Vec<i32>`。這裡的整數型別之所以是 `i32`，是因為它是預設整數型別，正如我們在第三章的[“資料型別”][data-types]部分討論過的那樣：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">示例 8-2：新建一個包含初值的 vector</span>

因為我們給出了 `i32` 型別的初始值，Rust 可以推斷出 `v` 的型別是 `Vec<i32>`，因此這裡不需要型別註解。接下來看看如何修改 vector。

### 更新 vector

要先建立一個 vector 再向其中新增元素，可以使用 `push` 方法，如示例 8-3 所示：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">示例 8-3：使用 `push` 方法向 vector 增加值</span>

和任何變數一樣，如果想修改它的值，就必須像第三章講過的那樣，使用 `mut` 關鍵字讓它變成可變的。放進去的數字都是 `i32` 型別，Rust 會從資料中推斷出這一點，因此也不需要寫 `Vec<i32>` 註解。

### 讀取 vector 的元素

有兩種方式可以引用 vector 中儲存的值：通過索引，或者使用 `get` 方法。在接下來的示例中，為了更清楚地說明這一點，我們給這些函式返回的值標註了型別。

示例 8-4 展示了訪問 vector 中某個值的兩種方式：索引語法和 `get` 方法。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">示例 8-4：使用索引語法或 `get` 方法來訪問 vector 中的項</span>

這裡有幾個細節需要注意。我們用索引值 `2` 獲取第三個元素，因為 vector 的索引是從 0 開始的。使用 `&` 和 `[]` 會得到索引位置處元素的引用。當我們把索引作為引數傳給 `get` 方法時，會得到一個可以與 `match` 一起使用的 `Option<&T>`。

Rust 提供這兩種引用元素的方式，是為了讓你可以選擇：當嘗試使用超出已有元素範圍的索引值時，程式該如何表現。舉個例子，假設我們有一個包含 5 個元素的 vector，然後嘗試分別用這兩種技術訪問索引 100 處的元素，看看會發生什麼，如示例 8-5 所示：

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">示例 8-5：嘗試訪問一個包含 5 個元素的 vector 的索引 100 處的元素</span>

執行這段程式碼時，第一種 `[]` 方法會讓程式 panic，因為它引用了一個不存在的元素。當你希望程式在有人嘗試訪問 vector 末尾之外的元素時直接崩潰，這種方式就很合適。

當傳給 `get` 方法的索引超出了 vector 的範圍時，它不會 panic，而是返回 `None`。如果在正常情況下，訪問超出 vector 範圍的元素偶爾是可能發生的，那麼你就會使用這種方法。此時你的程式碼可以像第六章討論過的那樣，處理 `Some(&element)` 和 `None` 兩種情況。例如，索引可能來自使用者輸入的數字。如果使用者不小心輸入了一個過大的數字，程式就會得到 `None`，這時你可以告訴使用者當前 vector 中有多少項，並給他們一次重新輸入有效值的機會。這就比因為一個輸入錯誤而讓程式崩潰更友好。

當程式拿到了一個有效引用後，借用檢查器會應用所有權和借用規則（第四章講過），來確保這個對 vector 內容的引用以及其他任何引用都保持有效。回憶一下那條規則：在同一作用域中，不能同時擁有可變引用和不可變引用。這條規則就適用於示例 8-6：我們持有了 vector 第一個元素的不可變引用，然後又嘗試在 vector 末尾新增一個元素。如果還想在函式後面繼續使用那個元素，這個程式就無法通過編譯：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">示例 8-6：嘗試在擁有 vector 中項的引用的同時向其增加一個元素</span>

編譯會給出這個錯誤：

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

示例 8-6 中的程式碼看起來似乎應該能工作：為什麼對第一個元素的引用，會在乎 vector 末尾發生的變化呢？這是由 vector 的工作方式決定的。因為 vector 會把值彼此相鄰地存放在記憶體中，所以如果末尾追加一個新元素，而當前存放位置又沒有足夠空間容納所有元素，程式就可能需要分配一塊新記憶體，並把舊元素複製到新空間裡去。在這種情況下，原來指向第一個元素的引用就會指向已釋放的記憶體。借用規則正是為了防止程式陷入這種情況。

> 注意：如果想了解 `Vec<T>` 型別更多的實現細節，請參閱 [“The Rustonomicon”][nomicon]。

### 遍歷 vector 中的元素

如果想依次訪問 vector 中的每個元素，我們會遍歷所有元素，而不是一次只通過索引訪問一個。示例 8-7 展示瞭如何使用 `for` 迴圈，獲取一個裝有 `i32` 值的 vector 中每個元素的不可變引用，並把它們打印出來：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">示例 8-7：通過 `for` 迴圈遍歷 vector 的元素並列印</span>

我們也可以遍歷可變 vector 中每個元素的可變引用，從而修改所有元素。示例 8-8 中的 `for` 迴圈會給每個元素都加上 `50`：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">示例 8-8：遍歷 vector 中元素的可變引用</span>

要修改可變引用所指向的值，在使用 `+=` 運算子前，必須先使用解引用運算子 `*` 取到 `i` 指向的值。第十五章的[“追蹤引用的值”][deref]部分會更詳細地討論解引用運算子。

由於借用檢查器的規則，不管是可變還是不可變地遍歷 vector，都是安全的。如果我們嘗試在示例 8-7 和示例 8-8 的 `for` 迴圈體內插入或刪除項，就會得到一個和示例 8-6 類似的編譯錯誤。`for` 迴圈持有的那個對 vector 的引用，會阻止對整個 vector 的同時修改。

### 使用列舉來儲存多種型別

vector 只能儲存相同型別的值。這可能會帶來不便；確實有些場景需要存放一組不同型別的值。幸運的是，列舉的各個變體都定義在同一個列舉型別之下，所以當我們需要用一個型別來表示不同種類的元素時，就可以定義並使用列舉！

例如，假設我們想從電子表格的一行中讀取值，而這一行中有些列包含整數，有些包含浮點數，還有些是字串。我們可以定義一個列舉，讓它的各個變體分別持有這些不同型別的值，而所有這些列舉變體都會被視為同一種類型，也就是該列舉本身的型別。然後，我們就可以建立一個存放這種列舉的 vector，從而最終在其中儲存不同型別的值。示例 8-9 展示了這種做法：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">示例 8-9：定義一個列舉，以便能在 vector 中存放不同型別的資料</span>

Rust 必須在編譯時知道 vector 中會有哪些型別，這樣它才能準確知道在堆上儲存每個元素需要多少記憶體。我們還必須明確指出這個 vector 允許哪些型別。如果 Rust 允許 vector 存放任意型別，那麼在對 vector 元素執行操作時，就有可能因為某一種或多種型別而導致錯誤。使用列舉再配合 `match` 表示式，意味著 Rust 會像第六章所說的那樣，在編譯時確保每一種可能的情況都得到了處理。

如果在編寫程式時，你並不知道執行時究竟會有哪些型別需要存進 vector，那麼這種列舉技巧就不適用了。相反，你可以使用 trait 物件，第 18 章會講到它。

現在我們已經討論了一些最常見的 vector 用法，記得去看看標準庫為 `Vec<T>` 定義的許多其他實用方法的 [API 文件][vec-api]。例如，除了 `push` 之外，還有一個 `pop` 方法會移除並返回 vector 的最後一個元素。

### 丟棄 vector 時也會丟棄其所有元素

和其他任何 `struct` 一樣，vector 會在離開作用域時被釋放，如示例 8-10 所標示的那樣：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">示例 8-10：展示 vector 和其元素於何處被丟棄</span>

當 vector 被丟棄時，它包含的所有內容也都會被一併丟棄，這意味著它持有的整數會被清理掉。借用檢查器會確保，對 vector 內容的任何引用都只會在 vector 本身有效時被使用。

讓我們繼續下一個集合型別：`String`！

[data-types]: ch03-02-data-types.html#資料型別
[nomicon]: https://doc.rust-lang.org/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#追蹤引用的值
