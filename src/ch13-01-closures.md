<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
<a id="closures-anonymous-functions-that-capture-their-environment"></a>

## 閉包

[ch13-01-closures.md](https://github.com/rust-lang/book/blob/4b71f17f7daba738a1363862dacb818d9b12fb81/src/ch13-01-closures.md)

Rust 的 **閉包**（*closures*）是可以儲存在變數中或作為引數傳遞給其他函式的匿名函式。你可以在一個地方建立閉包，然後在不同的上下文中執行閉包運算。不同於函式，閉包允許捕獲其被定義時所在作用域中的值。我們將展示這些閉包特性如何支援程式碼複用和行為定製。

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>

### 捕獲環境

我們首先了解如何通過閉包捕獲定義它的環境中的值以便之後使用。考慮如下場景：我們的 T 恤公司偶爾會向郵件列表中的某位成員贈送一件限量版的獨家 T 恤作為促銷。郵件列表中的成員可以選擇將他們的喜愛的顏色新增到個人資訊中。如果被選中的成員設定了喜愛的顏色，他們將獲得那個顏色的 T 恤。如果他沒有設定喜愛的顏色，他們會獲贈公司當前庫存最多的顏色的款式。

有很多種方式來實現這一點。例如，使用有 `Red` 和 `Blue` 兩個變體的 `ShirtColor` 列舉（出於簡單考慮限定為兩種顏色）。我們使用 `Inventory` 結構體來代表公司的庫存，它有一個型別為 `Vec<ShirtColor>` 的 `shirts` 欄位表示庫存中的襯衫的顏色。`Inventory` 上定義的 `giveaway` 方法獲取免費襯衫得主所喜愛的顏色（如有），並返回其獲得的襯衫的顏色。初始程式碼如示例 13-1 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">示例 13-1：襯衫公司贈送場景</span>

`main` 函式中定義的 `store` 還剩下兩件藍襯衫和一件紅襯衫，可以在限量版促銷活動中贈送。我們通過呼叫 `giveaway` 方法，為一個期望紅襯衫的使用者和一個沒有特定偏好的使用者進行贈送。

再次強調，這段程式碼有多種實現方式。這裡為了專注於閉包，我們繼續使用已經學習過的概念，除了 `giveaway` 方法體中使用了閉包。在 `giveaway` 方法中，我們將使用者偏好作為 `Option<ShirtColor>` 型別的引數獲取，並在 `user_preference` 上呼叫 `unwrap_or_else` 方法。[`Option<T>` 上的 `unwrap_or_else` 方法][unwrap-or-else]由標準庫定義。它接受一個無參閉包作為引數，該閉包返回一個 `T` 型別的值（與 `Option<T>` 的 `Some` 變體中儲存的值型別相同，這裡是 `ShirtColor`）。如果 `Option<T>` 是 `Some` 變體，則 `unwrap_or_else`  返回 `Some` 中的值。如果 `Option<T>` 是 `None` 變體，則 `unwrap_or_else` 呼叫閉包並返回閉包的返回值。

我們將閉包表示式 `|| self.most_stocked()` 作為 `unwrap_or_else` 的引數。這是一個本身不獲取引數的閉包（如果閉包有引數，它們會出現在兩道豎槓之間）。閉包體呼叫了 `self.most_stocked()`。我們在這裡定義了閉包，而 `unwrap_or_else` 的實現會在之後需要其結果的時候執行閉包。

執行程式碼會打印出：

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

這裡有一個有趣的地方是，我們傳遞了一個閉包，該閉包會在當前的 `Inventory` 例項上呼叫 `self.most_stocked()` 方法。標準庫不需要了解我們定義的 `Inventory` 或 `ShirtColor` 型別，也不需要了解我們在這個場景中要使用的邏輯。閉包捕獲了對 `self`（即 `Inventory` 例項）的不可變引用，並將其與我們指定的程式碼一起傳遞給 `unwrap_or_else` 方法。相比之下，函式無法以這種方式捕獲其環境。

<a id="closure-type-inference-and-annotation"></a>

### 推斷和註解閉包型別

函式與閉包還有更多區別。閉包通常不要求像 `fn` 函式那樣對引數和返回值進行型別註解。函式需要型別註解是因為這些型別是暴露給使用者的顯式介面的一部分。嚴格定義這些介面對於確保所有人對函式使用和返回值的型別達成一致理解非常重要。與此相比，閉包並不用於這樣暴露在外的介面：它們儲存在變數中並被使用，不用命名它們或暴露給庫的使用者呼叫。

閉包通常較短，並且只與特定的上下文相關，而不是適用於任意情境。在這些有限的上下文中，編譯器可以推斷引數和返回值的型別，類似於它推斷大多數變數型別的方式（儘管在某些罕見的情況下，編譯器也需要閉包的型別註解）。

類似於變數，如果我們希望增加程式碼的明確性和清晰度，可以新增型別註解，但代價是會使程式碼變得比嚴格必要的更冗長。為示例 13-1 中定義的閉包標註型別看起來如示例 13-2 中的定義一樣。這個例子中，我們定義了一個閉包並將它儲存在變數中，而不是像示例 13-1 那樣在傳參的地方定義它。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">示例 13-2：為閉包的引數和返回值增加可選的型別註解</span>

有了型別註解，閉包的語法看起來就更像函式的語法了。如下是一個對其引數加一的函式的定義與擁有相同行為閉包語法的縱向對比。這裡增加了一些空格來對齊相應部分。這展示了除了使用管道符（|，pipes）以及一些可選語法外，閉包語法與函式語法有多麼地相似：

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行展示了一個函式定義，第二行展示了一個完整標註的閉包定義。第三行閉包定義中省略了型別註解，而第四行去掉了可選的大括號，因為閉包體只有一個表示式，所以大括號是可選的。這些都是有效的閉包定義，並在呼叫時產生相同的行為。呼叫閉包是 `add_one_v3` 和 `add_one_v4` 能夠編譯的必要條件，因為型別將從其用法中推斷出來。這類似於 `let v = Vec::new();`，Rust 需要型別註解或是某種型別的值被插入到 `Vec` 中，才能推斷其型別。

對於閉包定義，編譯器會為每個引數和返回值推斷出一個具體型別。例如，示例 13-3 展示了一個簡短的閉包定義，該閉包僅僅返回作為引數接收到的值。除了作為示例用途外，這個閉包並不是很實用。注意這個定義沒有增加任何型別註解。因為沒有型別註解，我們可以使用任意型別來呼叫這個閉包，我們在這裡第一次呼叫時使用了 `String` 型別。但是如果我們接著嘗試使用整數來呼叫 `example_closure`，就會得到一個錯誤。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">示例 13-3：嘗試呼叫一個被推斷為兩個不同型別的閉包</span>

編譯器給出如下錯誤：

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

第一次使用 `String` 值呼叫 `example_closure` 時，編譯器推斷出 `x` 的型別以及閉包的返回型別為 `String`。接著這些型別被鎖定進閉包 `example_closure` 中，如果嘗試對同一閉包使用不同型別則就會得到型別錯誤。

### 捕獲引用或移動所有權

閉包可以通過三種方式捕獲其環境中的值，它們直接對應到函式獲取引數的三種方式：不可變借用、可變借用和獲取所有權。閉包將根據函式體中對捕獲值的操作來決定使用哪種方式。

在示例 13-4 中定義了一個捕獲名為 `list` 的 vector 的不可變引用的閉包，因為只需不可變引用就能列印其值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">示例 13-4：定義並呼叫一個捕獲不可變引用的閉包</span>

這個示例也展示了變數可以繫結一個閉包定義，並且我們可以像使用函式名一樣，使用變數名和括號來呼叫該閉包。

因為同時可以有多個 `list` 的不可變引用，所以在閉包定義之前，閉包定義之後呼叫之前，閉包呼叫之後程式碼仍然可以訪問 `list`。該程式碼可以編譯、執行並輸出：

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

接下來在示例 13-5 中，我們修改閉包體讓它向 `list` vector 增加一個元素。閉包現在捕獲一個可變引用：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">示例 13-5：定義並呼叫一個捕獲可變引用的閉包</span>

程式碼可以編譯、執行並列印：

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

注意在 `borrows_mutably` 閉包的定義和呼叫之間不再有 `println!`，這是因為當 `borrows_mutably` 被定義時，它捕獲了對 `list` 的可變引用。閉包在被呼叫後就不再被使用，這時可變借用結束。因為當可變借用存在時不允許有其它的借用，所以在閉包定義和呼叫之間不能有不可變引用來進行列印。可以嘗試在這裡新增 `println!` 看看你會得到什麼報錯資訊！

即使閉包體不嚴格需要所有權，如果希望強制閉包獲取它在環境中所使用的值的所有權，可以在引數列表前使用 `move` 關鍵字。

當將閉包傳遞到一個新的執行緒時，這個技巧特別有用，因為它將資料的所有權移動到新執行緒中。我們將在第十六章討論併發時詳細討論執行緒以及為什麼你可能需要使用它們。不過現在，我們先簡要探索一下如何使用需要 `move` 關鍵字的閉包來生成一個新執行緒。示例 13-6 展示瞭如何修改示例 13-4，以便在一個新執行緒中而不是在主執行緒中列印 vector：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<span class="caption">示例 13-6：使用 `move` 來強制閉包為執行緒獲取 `list` 的所有權</span>

我們生成了一個新的執行緒，並給這個執行緒傳遞一個閉包作為引數來執行，閉包體打印出列表。在示例 13-4 中，閉包僅通過不可變引用捕獲了 `list`，因為這是列印列表所需的最小訪問許可權。這個例子中，儘管閉包體依然只需要不可變引用，我們還是在閉包定義前寫上 `move` 關鍵字，以確保 `list` 被移動到閉包中。新執行緒可能在主執行緒剩餘部分執行完前執行完，也可能在主執行緒執行完之後執行完。如果主執行緒維護了 `list` 的所有權但卻在新執行緒之前結束並且丟棄了 `list`，則線上程中的不可變引用將失效。因此，編譯器要求 `list` 被移動到在新執行緒中執行的閉包中，這樣引用就是有效的。試著移除 `move` 關鍵字，或者在閉包定義後在主執行緒中使用 `list`，看看你會得到什麼編譯器報錯！

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures-and-the-fn-traits"></a>

### 將捕獲的值移出閉包

一旦閉包捕獲了定義它的環境中的某個值的引用或所有權（也就影響了什麼會被移**進**閉包，如有），閉包體中的程式碼則決定了在稍後執行閉包時，這些引用或值將如何處理（也就影響了什麼會被移**出**閉包，如有）。閉包體可以執行以下任一操作：將一個捕獲的值移出閉包，修改捕獲的值，既不移動也不修改值，或者一開始就不從環境中捕獲任何值。

閉包捕獲和處理環境中的值的方式會影響閉包實現哪些 trait，而 trait 是函式和結構體指定它們可以使用哪些型別閉包的方式。根據閉包體如何處理這些值，閉包會自動、漸進地實現一個、兩個或全部三個 `Fn` trait。

* `FnOnce` 適用於至少可以被呼叫一次的閉包。因為任何閉包都應當可以被呼叫，所以所有閉包都必須實現這個 trait。一個會將捕獲的值移出閉包自身的閉包只能實現 `FnOnce` 而無法實現另外兩個 `Fn` trait，因為這種閉包最多隻能被呼叫一次。
* `FnMut` 適用於不會將捕獲的值移出閉包自身但可能會修改捕獲的值的閉包。這類閉包可以被呼叫多次。
* `Fn` 適用於既不會將捕獲的值移出閉包自身也不會修改捕獲的值的閉包，以及不從環境中捕獲任何值的閉包。這類閉包可以被呼叫多次而不改變環境，在併發多次呼叫閉包等場景中十分重要。

讓我們來看示例 13-1 中使用的在 `Option<T>` 上的 `unwrap_or_else` 方法的定義：

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

回憶一下，`T` 是表示 `Option` 中 `Some` 變體中的值的型別的泛型。型別 `T` 也是 `unwrap_or_else` 函式的返回值型別：舉例來說，在 `Option<String>` 上呼叫 `unwrap_or_else` 會得到一個 `String`。

接著注意到 `unwrap_or_else` 函式有額外的泛型引數 `F`。`F` 是引數 `f` 的型別，`f` 是呼叫 `unwrap_or_else` 時提供的閉包。

泛型 `F` 的 trait 約束是 `FnOnce() -> T`，這意味著 `F` 必須能夠被呼叫一次，沒有引數並返回一個 `T`。在 trait 約束中使用 `FnOnce` 表示 `unwrap_or_else` 最多隻會呼叫 `f` 一次。在 `unwrap_or_else` 的函式體中可以看到，如果 `Option` 是 `Some`，`f` 不會被呼叫。如果 `Option` 是 `None`，`f` 將會被呼叫一次。由於所有的閉包都實現了 `FnOnce`，`unwrap_or_else` 接受所有三種類型的閉包，靈活性達到極致。

> 注意：如果我們要做的事情不需要從環境中捕獲值，則可以在需要某種實現了 `Fn` trait 的東西時使用函式而不是閉包。舉個例子，可以在 `Option<Vec<T>>` 的值上呼叫 `unwrap_or_else(Vec::new)`，以便在值為 `None` 時獲取一個新的空的 vector。編譯器會自動為函式定義實現適用的 `Fn` trait。

現在讓我們來看定義在 slice 上的標準庫方法 `sort_by_key`，看看它與 `unwrap_or_else` 的區別，以及為什麼 `sort_by_key` 使用 `FnMut` 而不是 `FnOnce` 作為 trait 約束。這個閉包以一個 slice 中當前被考慮的元素的引用作為引數，並返回一個可以排序的 `K` 型別的值。當你想按照 slice 中每個元素的某個屬性進行排序時，這個函式非常有用。在示例 13-7 中，我們有一個 `Rectangle` 例項的列表，並使用 `sort_by_key` 按 `Rectangle` 的 `width` 屬性對它們從低到高排序：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">示例 13-7：使用  `sort_by_key` 對長方形按寬度排序</span>

程式碼輸出：

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key` 被定義為接收一個 `FnMut` 閉包的原因是它會多次呼叫這個閉包：對 slice 中的每個元素呼叫一次。閉包 `|r| r.width` 不捕獲、修改或將任何東西移出它的環境，所以它滿足 trait 約束的要求。

相比之下，示例 13-8 展示了一個只實現了 `FnOnce` trait 的閉包的例子，因為它從環境中移出了一個值。編譯器不允許我們在 `sort_by_key` 中使用這個閉包：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">示例 13-8：嘗試在 `sort_by_key` 上使用一個 `FnOnce` 閉包</span>

這是一個刻意構造的、複雜且無效的方式，試圖統計在對 `list` 進行排序時 `sort_by_key` 呼叫閉包的次數。該程式碼試圖通過將閉包環境中的 `value`（一個 `String`）插入 `sort_operations` vector 來實現計數。閉包捕獲了 `value`，然後通過將 `value` 的所有權轉移給 `sort_operations` vector 的方式將其移出閉包。這個閉包只能被呼叫一次；嘗試第二次呼叫它將無法工作，因為這時 `value` 已經不在閉包的環境中，無法被再次插入 `sort_operations` 中！因而，這個閉包只實現了 `FnOnce`。當我們嘗試編譯此程式碼時，會出現錯誤提示：`value` 不能從閉包中移出，因為閉包必須實現 `FnMut`：

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

報錯指向了閉包體中將 `value` 移出環境的那一行。要修復此問題，我們需要修改閉包體，使其不會將值移出環境。在環境中維護一個計數器，並在閉包體中遞增其值，是計算閉包被呼叫次數的一個更直觀的方法。示例 13-9 中的閉包可以在 `sort_by_key` 中使用，因為它只捕獲了 `num_sort_operations` 計數器的可變引用，因此可以被多次呼叫：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">示例 13-9：允許在 `sort_by_key` 上使用一個  `FnMut`  閉包</span>

當定義或使用涉及閉包的函式或型別時，`Fn` trait 十分重要。在下個小節中，我們將討論迭代器。許多迭代器方法都接收閉包引數，因此在繼續前，請記住這些閉包的細節！

[unwrap-or-else]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
