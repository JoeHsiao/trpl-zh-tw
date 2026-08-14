## 面嚮物件語言的特徵

[ch18-01-what-is-oo.md](https://github.com/rust-lang/book/blob/eb54c18184c0555acd9f636f5ef7875236b0ff53/src/ch18-01-what-is-oo.md)

關於一門語言必須具備哪些特徵才能被視為面向物件，目前在程式設計社群中並沒有共識。Rust 受到了許多程式設計正規化的影響，包括面向物件程式設計（OOP）；例如，在第 13 章中，我們探討了來自函數語言程式設計的特性。可以說，面向物件的語言共有一些共同的特徵，即物件、封裝和繼承。我們將會討論這些特徵分別是什麼，以及 Rust 是否支援它們。

### 物件包含資料和行為

由 Erich Gamma、Richard Helm、Ralph Johnson 和 John Vlissides（Addison-Wesley, 1994）編寫的書 *Design Patterns: Elements of Reusable Object-Oriented Software*，通稱 *The Gang of Four*，是一本面向物件設計模式的目錄。它這樣定義面向物件程式設計：

> Object-oriented programs are made up of objects. An *object* packages both
> data and the procedures that operate on that data. The procedures are
> typically called *methods* or *operations*.
>
> 面向物件的程式由物件組成。一個**物件**同時封裝了資料以及操作這些資料的過程。這些過程通常被稱為**方法**或**操作**。

在這個定義下，Rust 是面向物件的：結構體和列舉包含資料而 `impl` 塊提供了在結構體和列舉之上的方法。雖然帶有方法的結構體和列舉並不被**稱為**物件，但是參考 *The Gang of Four* 中物件的定義，它們提供了與物件相同的功能。

### 封裝隱藏了實現細節

另一個通常與面向物件程式設計關聯的概念是 **封裝**（*encapsulation*）：一個物件的實現細節對使用該物件的程式碼不可見。因此，物件互動的唯一方式是透過其公有 API；使用物件的程式碼不應能直接觸及物件的內部並改變資料或行為。這使得程式設計師能夠更改和重構一個物件的內部實現，而無需改變使用該物件的程式碼。

我們在第七章討論瞭如何控制封裝：我們可以使用 `pub` 關鍵字來決定程式碼中的哪些模組、型別、函式和方法是公有的，而預設情況下其他所有內容都是私有的。例如，我們可以定義一個 `AveragedCollection` 結構體，其中有一個存有 `Vec<i32>` 的欄位。該結構體還可以有一個欄位儲存向量中值的平均值，從而無需在每次需要時重新計算。換句話說，`AveragedCollection` 會為我們快取已計算的平均值。示例 18-1 給出了 `AveragedCollection` 結構體的定義：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

<span class="caption">示例 18-1: `AveragedCollection` 結構體維護了一個整型列表及其所有元素的平均值。</span>

該結構體被標記為 `pub`，這樣其他程式碼就可以使用它，但結構體內的欄位仍保持私有。這在這種情況下很重要，因為我們想確保每當列表中新增或刪除值時，平均值也會更新。我們透過實現結構體上的 `add`、`remove` 和 `average` 方法來做到這一點，如示例 18-2 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

<span class="caption">示例 18-2: 在 `AveragedCollection` 結構體上實現了 `add`、`remove` 和 `average` 公有方法</span>

公有方法 `add`、`remove` 和 `average` 是訪問或修改 `AveragedCollection` 例項中資料的唯一途徑。當使用 `add` 方法把一個元素加入到 `list` 或者使用 `remove` 方法來刪除時，這些方法的實現同時會呼叫私有的 `update_average` 方法來更新 `average` 欄位。

`list` 和 `average` 是私有的，所以沒有其他方式來使得外部的程式碼直接向 `list` 增加或者刪除元素，否則 `list` 改變時可能會導致 `average` 欄位不同步。`average` 方法返回 `average` 欄位的值，這使得外部的程式碼只能讀取 `average` 而不能修改它。

因為我們已經封裝了 `AveragedCollection` 的實現細節，改動資料結構等內部實現非常簡單。例如，可以使用 `HashSet<i32>` 代替 `Vec<i32>` 作為 `list` 欄位的型別。只要 `add`、`remove` 和 `average` 這些公有方法的簽名保持不變，使用 `AveragedCollection` 的程式碼就無需改變。如果我們將 `list` 設為公有，情況就未必如此：`HashSet<i32>` 和 `Vec<i32>` 使用不同的方法增加或移除項，所以直接修改 `list` 的外部程式碼很可能需要相應改動。

如果封裝被認為是面嚮物件語言所必要的特徵，那麼 Rust 滿足這個要求。在程式碼中不同的部分控制 `pub` 的使用來封裝實現細節。

### 作為型別系統與程式碼共享的繼承

**繼承**（*Inheritance*）是一種機制：一個物件可以從另一個物件的定義中繼承元素，從而獲得父物件的資料和行為，無需再次定義。

如果一種語言必須擁有繼承才能算作面向物件，那麼 Rust 就不是這樣的語言。Rust 沒有辦法在不借助宏的情況下，定義一個結構體去繼承父結構體的欄位和方法實現。

不過，如果你已經習慣了把繼承作為程式設計工具箱的一部分，那麼 Rust 也會根據你最初想借助繼承解決的問題，提供其他方案。

選擇繼承通常有兩個主要原因。其一是複用程式碼：你可以先為某種型別實現特定行為，然後藉助繼承把這份實現複用到另一種型別上。在 Rust 中，可以透過 trait 方法的預設實現，在一定程度上做到這一點。你在示例 10-14 中已經見過：我們為 `Summary` trait 的 `summarize` 方法提供了預設實現。這樣，任何實現 `Summary` trait 的型別，都會自動擁有 `summarize` 方法，而無需額外編寫程式碼。這很像父類已經實現了某個方法，而繼承它的子類也隨之擁有這份實現。同樣地，在實現 `Summary` trait 時，我們也可以覆蓋 `summarize` 的預設實現，這又類似於子類去重寫從父類繼承而來的方法實現。

另一個使用繼承的原因和型別系統有關：它可以讓子型別出現在父型別能出現的地方。這也被稱為**多型**（*polymorphism*），意思是如果多個物件共享某些共同特徵，那麼在執行時就可以把它們彼此替換使用。

> ### 多型（Polymorphism）
>
> 對很多人來說，多型性與繼承同義。但它實際上是一個更廣義的概念，指的是可以處理多種型別資料的程式碼。對繼承而言，這些型別通常是子類。
>
> Rust 使用泛型來抽象不同可能的型別，並透過 trait 約束來約束這些型別所必須提供的內容。這有時被稱為 *bounded parametric polymorphism*。

Rust 透過不提供繼承，選擇了另一組不同的權衡。繼承常常有“共享了超出需要的程式碼”的風險。子類並不總是應該繼承父類的全部特徵，但使用繼承時卻往往會這樣發生。這會讓程式設計變得不夠靈活。它還會引入這樣一種可能：在子類上呼叫一些其實並不適用的方法，結果這些方法要麼根本說不通，要麼會導致錯誤。另外，一些語言只支援**單繼承**（即一個子類只能繼承一個父類），這也進一步限制了程式設計的靈活性。

基於這些原因，Rust 採取了不同的做法：它使用 trait object，而不是繼承，來實現執行時多型。接下來我們就來看看 trait object 是如何工作的。
