## 高階型別

[ch20-03-advanced-types.md](https://github.com/rust-lang/book/blob/57ff62db22b006e6b319e2e35c9364d932a8b4e5/src/ch20-03-advanced-types.md)

Rust 的型別系統有一些我們曾經提到但尚未討論過的特性。首先我們將從一般意義上討論 newtype 並探討它們作為型別為何有用。接著會轉向類型別名（type aliases），一個類似於 newtype 但有著稍微不同的語義的功能。我們還會討論 `!` 型別和動態大小型別。

### 使用 newtype 模式實現型別安全與抽象

本小節假設你已經閱讀了之前的 [“使用 newtype 模式在外部型別上實現外部 trait”][newtype] 部分。

newtype 模式還可用於我們到目前為止尚未討論的其他任務，包括靜態地確保值不會混淆以及標註值的單位。你在示例 20-16 中已經看到了一個使用 newtype 來表示單位的例子：`Millimeters` 和 `Meters` 結構體都在 newtype 中封裝了 `u32` 值。如果編寫了一個有 `Millimeters` 型別引數的函式，不小心使用 `Meters` 或普通的 `u32` 值來呼叫該函式的程式是不能編譯的。

newtype 模式也可以用於抽象掉某個型別的部分實現細節：新的型別可以暴露與其私有內部型別不同的共有 API。

newtype 模式還可以隱藏內部實現。例如，可以提供一個封裝了 `HashMap<i32, String>` 的 `People` 型別，用來儲存人名以及相應的 ID。使用 `People` 的程式碼只需與我們提供的公有 API 互動即可，比如向 `People` 集合增加名字字串的方法；這樣這些程式碼就無需知道在內部我們將一個 `i32` ID 賦予了這個名字了。newtype 模式是一種實現第十八章 [“封裝隱藏了實現細節”][encapsulation-that-hides-implementation-details] 中討論的隱藏實現細節的輕量級封裝方法。

### 型別同義詞與類型別名

Rust 提供了宣告 **類型別名**（*type alias*）的能力，使用 `type` 關鍵字為現有型別賦予另一個名字。例如，可以像這樣建立 `i32` 的別名 `Kilometers`：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

這意味著 `Kilometers` 是 `i32` 的 **同義詞**（*synonym*）；不同於示例 20-16 中建立的 `Millimeters` 和 `Meters` 型別。`Kilometers` 並不是一個新的、單獨的型別。`Kilometers` 型別的值將被完全當作 `i32` 型別值來對待：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

因為 `Kilometers` 是 `i32` 的別名，它們是同一型別，可以將 `i32` 與 `Kilometers` 相加，也可以將 `Kilometers` 傳遞給獲取 `i32` 引數的函式。但通過這種手段無法獲得上一部分討論的 newtype 模式所提供的型別檢查的好處。換句話說，如果在某處混用 `Kilometers` 和 `i32` 的值，編譯器也不會給出一個錯誤。

類型別名的主要用途是減少重複。例如，可能會有這樣很長的型別：

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

在函式簽名和型別註解中到處書寫這個冗長的型別既乏味又容易出錯。想像一下有一個專案，到處都是像 Listing 20-25 那樣的程式碼。

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

<span class="caption">示例 20-25: 在很多地方使用名稱很長的型別</span>

類型別名通過減少重複使程式碼更易於管理。在示例 20-26 中，我們為這個冗長的型別引入了名為 `Thunk` 的別名，並可以使用更簡潔的 `Thunk` 來替換所有使用該型別的地方。

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

<span class="caption">示例 20-26: 引入類型別名 `Thunk` 來減少重複</span>

這樣閱讀和編寫程式碼都容易多了！為類型別名選擇一個好名字也可以幫助你表達意圖（單詞 *thunk* 表示會在之後被計算的程式碼，所以這是一個存放閉包的合適的名字）。

類型別名也經常與 `Result<T, E>` 結合使用來減少重複。考慮一下標準庫中的 `std::io` 模組。I/O 操作通常會返回一個 `Result<T, E>` 來處理操作失敗的情況。標準庫中的 `std::io::Error` 結構體代表了所有可能的 I/O 錯誤。`std::io` 中的許多函式都會返回 `Result<T, E>`，其中 `E` 是 `std::io::Error`，比如 `Write` trait 中的這些函式：

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

這裡重複出現了很多次 `Result<..., Error>`。為此，`std::io` 有這個類型別名宣告：

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

該宣告位於 `std::io` 模組中，因此我們可以使用完全限定的別名 `std::io::Result<T>`；也就是說，`Result<T, E>` 中 `E` 放入了 `std::io::Error`。`Write` trait 中的函式最終看起來像這樣：

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

類型別名在兩個方面有幫助：易於編寫**並**在整個 `std::io` 中提供了一致的介面。因為這是一個別名，它只是另一個 `Result<T, E>`，這意味著可以在其上使用 `Result<T, E>` 的任何方法，以及像 `?` 這樣的特殊語法。

### 從不返回的 never type

Rust 有一個叫做 `!` 的特殊型別。在型別理論術語中被稱為 *empty type*，因為它沒有值。我們更傾向於稱之為 *never type*。這個名字描述了它的作用：在函式從不返回的時候充當返回值。下面是一個示例：

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

這段程式碼可以讀作 “函式 `bar` 從不返回”，而從不返回的函式被稱為 **發散函式**（*diverging functions*）。不能建立 `!` 型別的值，所以 `bar` 也不可能返回值。

不過一個不能建立值的型別有什麼用呢？回想一下示例 2-5 中猜數字遊戲的程式碼；我們在示例 20-27 中重現了其中的一小部分：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

<span class="caption">示例 20-27: `match` 語句和一個以 `continue` 結束的分支</span>

當時我們略過了這段程式碼中的一些細節。在第六章 [“`match` 控制流結構”][the-match-control-flow-construct] 部分，我們討論過：`match` 的各個分支必須返回相同的型別。所以下面的程式碼是行不通的：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

這裡的 `guess` 必須既是整型 **也是** 字串，而 Rust 要求 `guess` 只能是一個型別。那麼 `continue` 返回了什麼呢？為什麼在示例 20-27 中，一個分支返回 `u32`，而另一個分支卻以 `continue` 結束呢？

正如你可能猜到的，`continue` 的值是 `!`。也就是說，當 Rust 要計算 `guess` 的型別時，它會檢視這兩個分支。前者是 `u32` 值，而後者是 `!` 值。因為 `!` 型別永遠不會有值，Rust 決定 `guess` 的型別是 `u32`。

描述這種行為的正式方式是，型別為 `!` 的表示式可以被強制轉換為任意其他型別。之所以允許 `match` 分支以 `continue` 結束是因為 `continue` 並不真正返回值；相反它把控制權交回上層迴圈，所以在 `Err` 的情況，事實上並未對 `guess` 進行賦值。

never type 在 `panic!` 宏中也很有用。還記得 `Option<T>` 上的 `unwrap` 函式嗎？它產生一個值或 panic。這裡是它的定義：

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

這裡與示例 20-27 中的 `match` 發生了相同的情況：Rust 知道 `val` 是 `T` 型別，`panic!` 是 `!` 型別，所以整個 `match` 表示式的結果是 `T` 型別。這能工作是因為 `panic!` 並不產生一個值；它會終止程式。對於 `None` 的情況，`unwrap` 並不返回一個值，所以這些程式碼是有效的。

最後一個有著 `!` 型別的表示式是 `loop`：

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

這裡，迴圈永遠也不結束，所以此表示式的值是 `!`。但是如果引入 `break` 這就不為真了，因為迴圈在執行到 `break` 後就會終止。

### 動態大小型別和 `Sized` trait

Rust 需要知道有關型別的某些細節，例如為特定型別的值需要分配多少空間。這便是起初留下的一個型別系統中令人迷惑的角落：即 **動態大小型別**（*dynamically sized types*）的概念。這有時被稱為 “DST” 或 “unsized types”，它們讓我們能夠編寫使用那些只有在執行時才能知道大小的值的程式碼。

讓我們深入研究我們在整本書中一直在使用的動態大小型別 `str` 的細節。沒錯，不是 `&str`，而是單獨的 `str` 就是一個 DST。直到執行時我們都不知道字串有多長。我們無法在編譯時知道字串的長度，這意味著我們無法建立 `str` 型別的變數，也不能獲取 `str` 型別的引數。考慮一下這些程式碼，它們不能工作：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust 需要知道應該為特定型別的值分配多少記憶體，同時所有同一型別的值必須使用相同數量的記憶體。如果允許編寫這樣的程式碼，也就意味著這兩個 `str` 需要佔用完全相同大小的空間。不過它們有著不同的長度：`s1` 需要 12 位元組儲存，而 `s2` 需要 15 位元組。這也就是為什麼不可能建立一個存放動態大小型別的變數的原因。

那麼該怎麼辦呢？在這種情況下，你已經知道答案：`s1` 和 `s2` 的型別是 `&str` 而不是 `str`。如果你回想第四章 [“字串 slice”][string-slices] 中提到，slice 資料結構僅僅儲存了開始位置和 slice 的長度。所以雖然 `&T` 是一個儲存了 `T` 所在的記憶體位置的單個值，`&str` 則是**兩個**值：`str` 的地址和其長度。這樣，`&str` 就有了一個在編譯時可以知道的大小：它是 `usize` 長度的兩倍。也就是說，無論所引用的字串多長，我們總是知道 `&str` 的大小。一般來說，這就是 Rust 使用動態大小型別的方式：它們有一些額外的元資訊來儲存動態資訊的大小。這引出了動態大小型別的黃金法則：必須將動態大小型別的值置於某種指標之後。

可以將 `str` 與各種指標型別組合使用：例如 `Box<str>` 或 `Rc<str>`。事實上，你以前已經見過這種做法，不過物件換成了另一種動態大小型別：trait。每個 trait 本身也是一種動態大小型別，我們可以通過 trait 的名字來引用它。在第十八章 [“使用 trait object 來抽象出共享行為”][using-trait-objects-to-abstract-over-shared-behavior] 一節中，我們提到過：為了把 trait 用作 trait 物件，必須把它放在某種指標之後，比如 `&dyn Trait` 或 `Box<dyn Trait>`（`Rc<dyn Trait>` 也可以）。

為了處理 DST，Rust 提供了 `Sized` trait 來決定一個型別的大小是否在編譯時可知。該 trait 會自動為所有在編譯時大小已知的型別實現。此外，Rust 隱式地為每一個泛型函式增加了 `Sized` 約束。也就是說，對於如下泛型函式定義：

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

實際上，這會被當作我們寫了如下內容來處理：

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

預設情況下，泛型函式只能作用於在編譯時大小已知的型別。然而，你可以使用如下特殊語法來放寬這一限制：

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized` 這個 trait 約束表示 “`T` 可以是 `Sized`，也可以不是 `Sized`” 同時這個註解會覆蓋泛型型別必須在編譯時擁有固定大小的預設規則。具有該含義的 `?Trait` 語法僅適用於 `Sized`，而不適用於其他任何 trait。

另外注意我們將 `t` 引數的型別從 `T` 變為了 `&T`：因為其型別可能不是 `Sized` 的，所以需要將其置於某種指標之後。在這個例子中選擇了引用。

接下來，我們將討論函式和閉包！

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#封裝隱藏了實現細節
[string-slices]: ch04-03-slices.html#字串-slice
[the-match-control-flow-construct]: ch06-02-match.html#match-控制流結構
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-objects.html#使用-trait-object-來抽象出共享行為
[newtype]: ch20-02-advanced-traits.html#使用-newtype-模式在外部型別上實現外部-trait
