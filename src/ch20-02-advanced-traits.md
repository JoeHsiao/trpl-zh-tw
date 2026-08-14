## 高階 trait

[ch20-02-advanced-traits.md](https://github.com/rust-lang/book/blob/57ff62db22b006e6b319e2e35c9364d932a8b4e5/src/ch20-02-advanced-traits.md)

在第十章 [“使用 trait 定義共享行為”][traits] 部分，我們第一次接觸 trait，不過當時還沒有討論更高階的細節。現在你對 Rust 已經瞭解更多了，我們可以來深入探究這些細枝末節。

### 關聯型別

**關聯型別**（*associated types*）將一個型別佔位符與 trait 相關聯，使得該 trait 的方法定義可以在簽名中使用這些佔位符型別。該 trait 的實現者會為每個具體實現指定要使用的具體型別來替代佔位符型別。這樣，我們就能在定義 trait 時使用佔位符型別，而無需預先知道這些型別的具體內容，直到實現該 trait 時再進行指定。

我們之前提到，本章所討論的大多數高階特性都很少需要。關聯型別則比較適中：它們的使用頻率低於本書其他部分講解的特性，但又高於本章中許多其他特性。

一個帶有關聯型別的 trait 的例子是標準庫提供的 `Iterator` trait。它有一個叫做 `Item` 的關聯型別來替代遍歷的值的型別。`Iterator` trait 的定義如示例 20-13 所示：

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

<span class="caption">示例 20-13: `Iterator` trait 的定義中帶有關聯型別 `Item`</span>

`Item` 是一個佔位符型別，同時 `next` 方法的定義表明它返回 `Option<Self::Item>` 型別的值。`Iterator` trait 的實現者會指定 `Item` 的具體型別，於是 `next` 方法就會返回一個包含該具體型別值的 `Option`。

關聯型別可能看起來與泛型類似，後者允許我們在定義函式時不必指定它可以處理的型別。為了體現這兩者的區別，我們來看一個名為 `Counter` 的型別上的 `Iterator` trait 實現，其中指定 `Item` 的型別為 `u32`：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

這種語法看起來與泛型類似。那麼為什麼不直接像示例 20-14 那樣，用泛型來定義 `Iterator` trait 呢？

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

<span class="caption">示例 20-14: 一個使用泛型的 `Iterator` trait 假想定義</span>

區別在於當如示例 20-14 那樣使用泛型時，則不得不在每一個實現中標註型別；這是因為我們也可以實現為 `Iterator<String> for Counter`，或任何其他型別，這樣就可以有多個針對 `Counter` 的 `Iterator` 的實現。換句話說，當 trait 有泛型引數時，可以多次實現這個 trait，每次都使用不同的具體泛型引數型別。當我們在 `Counter` 上呼叫 `next` 方法時，就必須透過型別註解來指明要使用哪一個 `Iterator` 的實現。

使用關聯型別後，則無需標註型別，因為不能對同一個型別多次實現該 trait。在示例 20-13 中使用關聯型別的定義裡，我們只能為 `Item` 選擇一次具體型別，因為只能有一個 `impl Iterator for Counter`。當呼叫 `Counter` 的 `next` 時不必每次指定我們需要 `u32` 值的迭代器。

關聯型別也會成為 trait 契約的一部分：trait 的實現必須提供一個型別來替代關聯型別佔位符。關聯型別通常以它的用途來命名，在 API 文件中對關聯型別進行說明也是一種良好實踐。

### 使用預設泛型型別引數和運算子過載

當使用泛型型別引數時，可以為泛型指定一個預設的具體型別。如果預設型別就足夠的話，這消除了為具體型別實現 trait 的需要。為泛型型別指定預設型別的語法是在宣告泛型型別時使用 `<PlaceholderType=ConcreteType>`。

這種技術的一個很好的示例是 **運算子過載** (*operator overloading*)，即在特定情況下自定義運算子（比如 `+`）行為的操作。

Rust 並不允許建立自定義運算子或過載任意運算子，但可以透過實現 `std::ops` 中列出的運算子相關 trait 來過載它們。例如，在示例 20-15 中我們過載 `+` 運算子來將兩個 `Point` 例項相加。我們透過在 `Point` 結構體上實現 `Add` trait 來實現這一點。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

<span class="caption">示例 20-15: 實現 `Add` trait 過載 `Point` 例項的 `+` 運算子</span>

`add` 方法將兩個 `Point` 例項的 `x` 值和 `y` 值分別相加來建立一個新的 `Point`。`Add` trait 有一個叫做 `Output` 的關聯型別，它用來決定 `add` 方法的返回值型別。

這裡預設泛型型別位於 `Add` trait 中。這裡是其定義：

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

這些程式碼看來應該很熟悉：一個帶有一個方法和一個關聯型別的 trait。新增的部分是 `Rhs=Self`：這個語法叫做 **預設型別引數**（*default type parameters*）。`Rhs` 是一個泛型型別引數（“right-hand side” 的縮寫），它用於定義 `add` 方法中的 `rhs` 引數。如果實現 `Add` trait 時不指定 `Rhs` 的具體型別，`Rhs` 的型別將預設為 `Self`，即正在實現 `Add` 的型別。

當為 `Point` 實現 `Add` 時，使用了預設的 `Rhs`，因為我們希望將兩個 `Point` 例項相加。讓我們看看一個實現 `Add` trait 時希望自定義 `Rhs` 型別而不是使用預設型別的例子。

這裡有兩個存放不同單元值的結構體，`Millimeters` 和 `Meters`。這種將現有型別簡單封裝進另一個結構體的方式被稱為 **newtype 模式**（*newtype pattern*），之後的[“使用 newtype 模式在外部型別上實現外部 trait”][newtype]部分會做詳細介紹。我們希望能夠將毫米值與米值相加，並讓 `Add` 的實現正確處理單位轉換。可以為 `Millimeters` 實現 `Add` 並以 `Meters` 作為 `Rhs`，如示例 20-16 所示。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

<span class="caption">示例 20-16: 在 `Millimeters` 上實現 `Add`，以便能夠將 `Millimeters` 與 `Meters` 相加</span>

為了使 `Millimeters` 和 `Meters` 能夠相加，我們指定 `impl Add<Meters>` 來設定 `Rhs` 型別引數的值而不是使用預設的 `Self`。

預設引數型別主要用於如下兩個方面：

* 擴充套件型別而不破壞現有程式碼。
* 在大部分使用者都不需要的特定情況進行自定義。

標準庫的 `Add` trait 就是第二個目的的一個例子：大部分時候你會將兩個相似的型別相加，但 `Add` trait 也提供了自定義額外行為的能力。在 `Add` trait 定義中使用預設型別引數意味著大部分時候無需指定額外的引數。換句話說，一小部分實現的樣板程式碼是不必要的，這樣使用 trait 就更容易了。

第一個目的與第二個相似但方向相反：如果需要為現有 trait 增加型別引數，為其提供一個預設型別將允許我們在不破壞現有實現程式碼的基礎上擴充套件 trait 的功能。

### 在同名方法之間消歧義

Rust 既不能避免一個 trait 與另一個 trait 擁有相同名稱的方法，也不能阻止為同一型別同時實現這兩個 trait。同時還可以直接在型別上實現一個與 trait 方法同名的方法。

當呼叫這些同名方法時，需要告訴 Rust 我們想要使用哪一個。考慮一下示例 20-17 中的程式碼，這裡我們定義了兩個 trait，`Pilot` 和 `Wizard`，它們都擁有名為 `fly` 的方法。接著在一個本身已經實現了名為 `fly` 方法的型別 `Human` 上實現這兩個 trait。每一個 `fly` 方法都進行了不同的操作：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

<span class="caption">示例 20-17: 兩個 trait 定義為擁有 `fly` 方法，並在直接定義有 `fly` 方法的 `Human` 型別上實現這兩個 trait</span>

當呼叫 `Human` 例項的 `fly` 時，編譯器預設呼叫直接實現在該型別上的方法，如示例 20-18 所示。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

<span class="caption">示例 20-18: 呼叫一個 `Human` 例項的 `fly`</span>

執行這段程式碼會打印出 `*waving arms furiously*`，這表明 Rust 呼叫了直接實現在 `Human` 上的 `fly` 方法。

為了能夠呼叫 `Pilot` trait 或 `Wizard` trait 的 `fly` 方法，需要使用更明確的語法來指定具體要呼叫的 `fly` 方法。示例 20-19 演示了這種語法。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

<span class="caption">示例 20-19: 指定我們希望呼叫哪一個 trait 的 `fly` 方法</span>

在方法名前指定 trait 名稱可讓 Rust 明確我們想呼叫哪個 `fly` 實現。也可以選擇寫成 `Human::fly(&person)`，這等同於示例 20-19 中的 `person.fly()`，不過如果無需消歧義的話這麼寫就有點冗長了。

執行這段程式碼會打印出如下內容：

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

因為 `fly` 方法獲取一個 `self` 引數，如果有兩個**型別**都實現了同一 **trait**，Rust 可以根據 `self` 的型別計算出應該使用哪一個 trait 實現。

然而，關聯函式中非方法的函式不帶有 `self` 引數。當存在多個型別或者 trait 定義了相同函式名的非方法函式時，Rust 就不總是能計算出我們期望的是哪一個型別，除非使用 **完全限定語法**（*fully qualified syntax*）。例如示例 20-20 中的建立了一個希望將所有小狗叫做 *Spot* 的動物收容所的 trait。`Animal` trait 有一個關聯非方法函式 `baby_name`。結構體 `Dog` 實現了 `Animal`，同時又直接提供了關聯非方法函式 `baby_name`。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

<span class="caption">示例 20-20: 一個帶有關聯函式的 trait 和一個帶有同名關聯函式並實現了此 trait 的型別</span>

在 `Dog` 型別上定義的關聯函式 `baby_name` 中，我們實現了將所有小狗命名為 Spot 的功能。`Dog` 型別還實現了 `Animal` trait，它描述了所有動物所共有的特徵。小狗被稱為 puppy，這表現為 `Dog` 的 `Animal` trait 實現中與 `Animal` trait 相關聯的函式 `baby_name`。

在 `main` 呼叫了 `Dog::baby_name` 函式，它直接呼叫了定義於 `Dog` 之上的關聯函式。這段程式碼會打印出：

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

這不是我們想要的輸出。我們希望呼叫的是 `Dog` 上 `Animal` trait 實現那部分的 `baby_name` 函式，這樣能夠打印出 `A baby dog is called a puppy`。我們在示例 20-19 中使用的指定 trait 名稱的技巧在這裡不起作用；如果將 `main` 改為示例 20-21 中的程式碼，就會得到編譯錯誤：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

<span class="caption">示例 20-21: 嘗試呼叫 `Animal` trait 的 `baby_name` 函式，不過 Rust 並不知道該使用哪一個實現</span>

因為 `Animal::baby_name` 沒有 `self` 引數，而且可能有其他型別實現了 `Animal` trait，Rust 無法確定我們想呼叫哪一個 `Animal::baby_name` 的實現。此時會得到如下編譯錯誤：

```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

為了消歧義並告訴 Rust 我們希望使用的是 `Dog` 的 `Animal` 實現而不是其它型別的 `Animal` 實現，需要使用**完全限定語法**。示例 20-22 演示瞭如何使用完全限定語法：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

<span class="caption">示例 20-22: 使用完全限定語法來指定我們希望呼叫的是 `Dog` 上 `Animal` trait 實現中的 `baby_name` 函式</span>

我們在尖括號中向 Rust 提供了型別註解，這表明我們希望在此次函式呼叫中將 `Dog` 型別視為 `Animal`，從而呼叫在 `Dog` 上實現的 `Animal` trait 中的 `baby_name` 方法。現在這段程式碼將打印出我們期望的結果：

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

通常，完全限定語法定義為如下：

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

對於不是方法的關聯函式，並沒有一個 `receiver`：故只會有其他引數的列表。可以選擇在任何函式或方法呼叫處使用完全限定語法。然而，允許省略任何 Rust 能夠從程式中的其他資訊中計算出的部分。只有當存在多個同名實現而 Rust 需要幫助以便知道我們希望呼叫哪個實現時，才需要使用這個較為冗長的語法。

### 使用超 trait

有時我們可能會需要編寫一個依賴另一個 trait 的 trait 定義：對於一個實現了第一個 trait 的型別，你希望要求這個型別也實現了第二個 trait。如此就可使 trait 定義使用第二個 trait 的關聯項。這個所需的 trait 是我們實現的 trait 的 **超（父）trait**（*supertrait*）。

例如我們希望建立一個帶有 `outline_print` 方法的 trait `OutlinePrint`，它會將給定的值格式化為帶有星號框。也就是說，給定一個實現了標準庫 `Display` trait 的並返回 `(x, y)` 的 `Point`，當我們對一個 `x` 為 `1`、`y` 為 `3` 的 `Point` 例項呼叫 `outline_print` 時，它應該打印出如下內容：

```text
**********
*        *
* (1, 3) *
*        *
**********
```

在 `outline_print` 的實現中，我們希望使用 `Display` trait 的功能。因此，需要說明 `OutlinePrint` trait 僅適用於那些同時實現了 `Display` 並提供 `OutlinePrint` 所需功能的型別。可以透過在 trait 定義中指定 `OutlinePrint: Display` 來做到這一點。這種技術類似於為 trait 增加 trait 約束。示例 20-23 展示了一個 `OutlinePrint` trait 的實現：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

<span class="caption">示例 20-23: 實現 `OutlinePrint` trait，它要求來自 `Display` 的功能</span>

因為我們已經指定 `OutlinePrint` 需要 `Display` trait，因而可以使用自動為任何實現了 `Display` 的型別提供的 `to_string` 方法。如果我們在沒有在 trait 名稱後新增冒號並指定 `Display` trait 的情況下嘗試使用 `to_string`，就會出現錯誤，提示在當前作用域中未為型別 `&Self` 找到名為 `to_string` 的方法。

讓我們看看如果嘗試在一個沒有實現 `Display` 的型別上實現 `OutlinePrint` 會發生什麼，比如 `Point` 結構體：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

這樣會得到一個錯誤說 `Display` 是必須的而未被實現：

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

為了修復這個問題，我們在 `Point` 上實現 `Display` 並滿足 `OutlinePrint` 要求的限制，比如這樣：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

那麼在 `Point` 上實現 `OutlinePrint` trait 就能成功編譯，並可以在 `Point` 例項上呼叫 `outline_print` 將其顯示在由星號組成的邊框內。

### 使用 newtype 模式在外部型別上實現外部 trait

在第十章的 [“為型別實現 trait”][implementing-a-trait-on-a-type] 部分，我們提到了孤兒規則（orphan rule），它規定只有當 trait 或型別至少有一方或兩者都對於當前 crate 是本地時，才能在該型別上實現該 trait。一個繞開這個限制的方法是使用 **newtype 模式**（*newtype pattern*），它涉及到在一個元組結構體（第五章 [“使用元組結構體建立不同的型別”][tuple-structs] 部分介紹了元組結構體）中建立一個新型別。這個元組結構體帶有一個欄位作為希望實現 trait 的型別的簡單封裝。由於這個封裝型別對於 crate 是本地的，這樣就可以在這個封裝上實現 trait。*Newtype* 是一個源自 Haskell 程式語言的概念。使用這個模式沒有執行時效能懲罰，這個封裝型別在編譯時就被省略了。

例如，如果想要在 `Vec<T>` 上實現 `Display`，而孤兒規則阻止我們直接這麼做，因為 `Display` trait 和 `Vec<T>` 都定義於我們的 crate 之外。可以建立一個包含 `Vec<T>` 例項的 `Wrapper` 結構體，接著可以如示例 20-24 那樣在 `Wrapper` 上實現 `Display` 並使用 `Vec<T>` 的值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

<span class="caption">示例 20-24: 建立 `Wrapper` 型別封裝 `Vec<String>` 以便能夠實現 `Display`</span>

`Display` 的實現使用 `self.0` 來訪問其內部的 `Vec<T>`，因為 `Wrapper` 是元組結構體而 `Vec<T>` 是結構體總位於索引 0 的項。接著就可以使用 `Wrapper` 中 `Display` 的功能了。

這種做法的缺點在於因為 `Wrapper` 是一個新型別，它並不具備其所封裝值的方法。必須直接在 `Wrapper` 上實現 `Vec<T>` 的所有方法，這樣就可以代理到`self.0` 上，這就允許我們完全像 `Vec<T>` 那樣對待 `Wrapper`。如果希望新型別擁有其內部型別的每一個方法，為封裝型別實現 `Deref` trait（第十五章 [“將智慧指標視作常規引用”][smart-pointer-deref] 部分討論過）並返回其內部型別是一種解決方案。如果不希望封裝型別擁有所有內部型別的方法 —— 比如為了限制封裝型別的行為 —— 則只需自行實現所需的方法即可。

甚至當不涉及 trait 時 newtype 模式也很有用。現在讓我們將關注點轉移到一些與 Rust 型別系統互動的高階方式上來吧。

[newtype]: ch20-02-advanced-traits.html#使用-newtype-模式在外部型別上實現外部-trait
[implementing-a-trait-on-a-type]: ch10-02-traits.html#為型別實現-trait
[traits]: ch10-02-traits.html
[smart-pointer-deref]: ch15-02-deref.html#將智慧指標視作常規引用
[tuple-structs]: ch05-01-defining-structs.html#使用元組結構體建立不同的型別
