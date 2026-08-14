## 將智慧指標視作常規引用

[ch15-02-deref.md](https://github.com/rust-lang/book/blob/ecef81cbc6f0c2d1c8a67409329b0641258c04c2/src/ch15-02-deref.md)

實現 `Deref` trait 允許你自定義**解引用運算子**（*dereference operator*）`*` 的行為（不要把它和乘法運算子或萬用字元運算子混淆）。通過以某種方式實現 `Deref`，使智慧指標能夠像常規引用一樣被對待，你就可以編寫操作引用的程式碼，並同樣把它用於智慧指標。

先來看看解引用運算子是如何作用於常規引用的。然後，我們會嘗試定義一個行為類似 `Box<T>` 的自定義型別，並看看為什麼解引用運算子在我們新定義的型別上不能像引用那樣工作。接著，我們會探討實現 `Deref` trait 如何讓智慧指標能夠像引用一樣工作。最後，我們會看看 Rust 的  **Deref 強制轉換**（_deref coercions_）特性，以及它如何讓我們既能處理引用，也能處理智慧指標。


### 追蹤引用的值

常規引用是一個指標型別，一種理解指標的方式是將其看成指向儲存在其他某處值的箭頭。在示例 15-6 中，建立了一個 `i32` 值的引用，接著使用解引用運算子來跟蹤所引用的值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

<span class="caption">示例 15-6：使用解引用運算子來跟蹤 `i32` 值的引用</span>

變數 `x` 存放了一個 `i32` 值 `5`。`y` 等於 `x` 的一個引用。可以斷言 `x` 等於 `5`。然而，如果希望對 `y` 的值做出斷言，必須使用 `*y` 來追蹤引用所指向的值（也就是**解引用**），這樣編譯器就可以比較實際的值了。一旦解引用了 `y`，就可以訪問 `y` 所指向的整型值並可以與 `5` 做比較。

相反如果嘗試編寫 `assert_eq!(5, y);`，則會得到如下編譯錯誤：

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

不允許比較數字的引用與數字，因為它們是不同的型別。必須使用解引用運算子追蹤引用所指向的值。

### 像引用一樣使用 `Box<T>`

可以使用 `Box<T>` 代替引用來重寫示例 15-6 中的程式碼，示例 15-7 中 `Box<T>` 上使用的解引用運算子與示例 15-6 中引用上使用的解引用運算子有著一樣的功能：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

<span class="caption">示例 15-7：在 `Box<i32>` 上使用解引用運算子</span>

示例 15-7 相比示例 15-6 主要不同的地方就是將 `y` 設定為一個指向 `x` 值複製的 `Box<T>` 例項，而不是指向 `x` 值的引用。在最後的斷言中，可以使用解引用運算子以 `y` 為引用時相同的方式追蹤 `Box<T>` 的指標。接下來讓我們通過實現自己的型別來探索 `Box<T>` 能這麼做有何特殊之處。

### 定義我們自己的智慧指標

為了體會預設情況下智慧指標與引用的不同，讓我們建立一個類似於標準庫提供的 `Box<T>` 型別的智慧指標。接著學習如何增加使用解引用運算子的功能。

> 注意：我們即將構建的 `MyBox<T>` 型別與真正的 `Box<T>` 有一個很大的區別：我們的版本不會把資料儲存在堆上。因為這個示例關注的是 `Deref`，所以資料實際儲存在哪裡並不像這種“類似指標的行為”那樣重要。

從根本上說，`Box<T>` 被定義為包含一個元素的元組結構體，所以示例 15-8 以相同的方式定義了 `MyBox<T>` 型別。我們還定義了 `new` 函式來對應定義於 `Box<T>` 的 `new` 函式：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

<span class="caption">示例 15-8：定義 `MyBox<T>` 型別</span>

這裡定義了一個結構體 `MyBox` 並聲明瞭一個泛型引數 `T`，因為我們希望其可以存放任何型別的值。`MyBox` 是一個包含 `T` 型別元素的元組結構體。`MyBox::new` 函式獲取一個 `T` 型別的引數並返回一個存放傳入值的 `MyBox` 例項。

嘗試將示例 15-7 中的程式碼加入示例 15-8 中並修改 `main` 使用我們定義的 `MyBox<T>` 型別代替 `Box<T>`。示例 15-9 中的程式碼不能編譯，因為 Rust 不知道如何解引用 `MyBox`：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

<span class="caption">示例 15-9：嘗試以使用引用和 `Box<T>` 相同的方式使用 `MyBox<T>`</span>

下面是相應的編譯錯誤：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

`MyBox<T>` 型別不能解引用，因為我們尚未在該型別上實現這個功能。為了啟用 `*` 運算子的解引用功能，需要實現 `Deref` trait。

### 實現 `Deref` trait

如第十章[“為型別實現 Trait”][impl-trait]所討論的，為了實現 trait，我們需要為 trait 所要求的方法提供實現。標準庫提供的 `Deref` trait 要求我們實現一個名為 `deref` 的方法，該方法借用 `self` 並返回一個指向內部資料的引用。示例 15-10 展示了要新增到 `MyBox<T>` 定義上的 `Deref` 實現：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

<span class="caption">示例 15-10：`MyBox<T>` 上的 `Deref` 實現</span>

`type Target = T;` 語法定義了用於此 trait 的關聯型別。關聯型別是一個稍有不同的定義泛型引數的方式，現在還無需過多地擔心它；第二十章會詳細介紹。

`deref` 方法體中寫入了 `&self.0`，這樣 `deref` 返回了我希望通過 `*` 運算子訪問的值的引用。回憶一下第五章 [“使用元組結構體建立不同的型別”][tuple-structs] 部分 `.0` 用來訪問元組結構體的第一個元素。示例 15-9 中的 `main` 函式中對 `MyBox<T>` 值的 `*` 呼叫現在可以編譯並能通過斷言了！

沒有 `Deref` trait 的話，編譯器只會解引用 `&` 引用型別。`deref` 方法向編譯器提供了獲取任何實現了 `Deref` trait 的型別的值，並且呼叫這個型別的 `deref` 方法來獲取一個它知道如何解引用的 `&` 引用的能力。

當我們在示例 15-9 中輸入 `*y` 時，Rust 事實上在底層運行了如下程式碼：

```rust,ignore
*(y.deref())
```

Rust 將 `*` 運算子替換為先呼叫 `deref` 方法再進行普通解引用的操作，如此我們便不用擔心是否還需手動呼叫 `deref` 方法了。Rust 的這個特性可以讓我們寫出行為一致的程式碼，無論面對的是常規引用還是實現了 `Deref` 的型別。

`deref` 方法返回值的引用，以及 `*(y.deref())` 括號外邊的普通解引用仍為必須的原因在於所有權。如果 `deref` 方法直接返回值而不是值的引用，其值將被移出 `self`。在這裡以及大部分使用解引用運算子的情況下我們並不希望獲取 `MyBox<T>` 內部值的所有權。

注意，每次當我們在程式碼中使用 `*` 時， `*` 運算子都被替換成了先呼叫 `deref` 方法再接著使用 `*` 解引用的操作，且只會發生一次，不會對 `*` 運算子無限遞迴替換，解引用出上面 `i32` 型別的值就停止了，這個值與示例 15-9 中 `assert_eq!` 的 `5` 相匹配。

### 在函式和方法中使用 Deref 強制轉換

**Deref 強制轉換**（_deref coercions_）將實現了 `Deref` trait 的型別的引用轉換為另一種型別的引用。例如，Deref 強制轉換可以將 `&String` 轉換為 `&str`，因為 `String` 實現了 `Deref` trait 因此可以返回 `&str`。Deref 強制轉換是 Rust 在函式或方法傳參上的一種便利操作，並且只能作用於實現了 `Deref` trait 的型別。當這種特定型別的引用作為實參傳遞給和形參型別不同的函式或方法時將自動進行。這時會有一系列的 `deref` 方法被呼叫，把我們提供的型別轉換成了引數所需的型別。

Deref 強制轉換的加入使得 Rust 程式設計師編寫函式和方法呼叫時無需增加過多顯式使用 `&` 和 `*` 的引用和解引用。這個功能也使得我們可以編寫更多同時作用於引用或智慧指標的程式碼。

作為展示 Deref 強制轉換的例項，讓我們使用示例 15-8 中定義的 `MyBox<T>`，以及示例 15-10 中增加的 `Deref` 實現。示例 15-11 展示了一個有著字串 slice 引數的函式定義：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

<span class="caption">示例 15-11：`hello` 函式有著 `&str` 型別的引數 `name`</span>

可以使用字串 slice 作為引數呼叫 `hello` 函式，比如 `hello("Rust");`。Deref 強制轉換使得用 `MyBox<String>` 型別值的引用呼叫 `hello` 成為可能，如示例 15-12 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

<span class="caption">示例 15-12：因為 Deref 強制轉換，使用 `MyBox<String>` 的引用呼叫 `hello` 是可行的</span>

這裡使用 `&m` 呼叫 `hello` 函式，其為 `MyBox<String>` 值的引用。因為示例 15-10 中在 `MyBox<T>` 上實現了 `Deref` trait，Rust 可以通過 `deref` 呼叫將 `&MyBox<String>` 變為 `&String`。標準庫中提供了 `String` 上的 `Deref` 實現，其會返回字串 slice，這可以在 `Deref` 的 API 文件中看到。Rust 再次呼叫 `deref` 將 `&String` 變為 `&str`，這就符合 `hello` 函式的定義了。

如果 Rust 沒有實現 Deref 強制轉換，為了使用 `&MyBox<String>` 型別的值呼叫 `hello`，則不得不編寫示例 15-13 中的程式碼來代替示例 15-12：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

<span class="caption">示例 15-13：如果 Rust 沒有 Deref 強制轉換則必須編寫的程式碼</span>

`(*m)` 將 `MyBox<String>` 解引用為 `String`。接著 `&` 和 `[..]` 獲取了整個 `String` 的字串 slice 來匹配 `hello` 的簽名。沒有 Deref 強制轉換所有這些符號混在一起將更難以讀寫和理解。Deref 強制轉換使得 Rust 自動的幫我們處理這些轉換。

當所涉及到的型別定義了 `Deref` trait，Rust 會分析這些型別並使用任意多次 `Deref::deref` 呼叫以獲得匹配引數的型別。這些解析都發生在編譯時，所以利用 Deref 強制轉換並沒有執行時開銷！

### 處理可變引用的 Deref 強制轉換

類似於如何使用 `Deref` trait 過載不可變引用的 `*` 運算子，Rust 提供了 `DerefMut` trait 用於過載可變引用的 `*` 運算子。

Rust 在發現型別和 trait 實現滿足三種情況時會進行 Deref 強制轉換：

1. 當 `T: Deref<Target=U>` 時從 `&T` 到 `&U`。
2. 當 `T: DerefMut<Target=U>` 時從 `&mut T` 到 `&mut U`。
3. 當 `T: Deref<Target=U>` 時從 `&mut T` 到 `&U`。

頭兩個情況除了第二種實現了可變性之外是相同的：第一種情況表明如果有一個 `&T`，而 `T` 實現了返回 `U` 型別的 `Deref`，則可以透明地得到 `&U`。第二種情況表明對於可變引用也有著相同的行為。

第三個情況有些微妙：Rust 也會將可變引用強轉為不可變引用。但反之是**不可能** 的：不可變引用永遠也不能強轉為可變引用。因為根據借用規則，如果有一個可變引用，其必須是這些資料的唯一引用（否則程式將無法編譯）。將一個可變引用轉換為不可變引用永遠也不會打破借用規則。將不可變引用轉換為可變引用則需要初始的不可變引用是資料唯一的不可變引用，而借用規則無法保證這一點。因此，Rust 無法假設將不可變引用轉換為可變引用是可能的。

[impl-trait]: ch10-02-traits.html#為型別實現-trait
[tuple-structs]: ch05-01-defining-structs.html#使用元組結構體建立不同的型別
