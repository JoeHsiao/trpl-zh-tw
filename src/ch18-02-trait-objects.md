## 使用 trait object 來抽象出共享行為

[ch18-02-trait-objects.md](https://github.com/rust-lang/book/blob/eb54c18184c0555acd9f636f5ef7875236b0ff53/src/ch18-02-trait-objects.md)

在第八章中，我們談到了 vector 只能儲存同種型別元素的侷限性。示例 8-9 中提供了一個替代方案，透過定義 `SpreadsheetCell` 列舉，來儲存整型、浮點型或文字型別的變體。這意味著，我們可以在每個單元中儲存不同型別的資料，並仍能擁有一個代表一排單元的 vector。只要我們需儲存的值由一組固定的型別組成，並且在程式碼編譯時就知道具體會有哪些型別，那麼這種使用列舉的辦法是完全可行的。

然而有時我們希望庫使用者在特定情況下能夠擴充套件有效的型別集合。為了展示如何實現這一點，這裡將建立一個圖形使用者介面（Graphical User Interface，GUI）工具的例子，它透過遍歷列表並呼叫每一個專案的 `draw` 方法來將其繪製到螢幕上 —— 此乃一個 GUI 工具的常見技術。我們將要建立一個叫做 `gui` 的庫 crate，它含一個 GUI 庫的結構。這個 GUI 庫包含一些可供開發者使用的型別，比如 `Button` 或 `TextField`。在此之上，`gui` 的使用者希望建立自定義的可以繪製於螢幕上的型別：比如，一個程式設計師可能會增加 `Image`，另一個可能會增加 `SelectBox`。

這個例子中並不會實現一個功能完善的 GUI 庫，不過會展示其中各個部分是如何結合在一起的。編寫庫的時候，我們不可能知曉並定義所有其他程式設計師希望建立的型別。我們所知曉的是 `gui` 需要記錄一系列不同型別的值，並需要能夠對其中每一個值呼叫 `draw` 方法。這裡無需知道呼叫 `draw` 方法時具體會發生什麼，只要該值會有那個方法可供我們呼叫即可。

在擁有繼承的語言中，可以定義一個名為 `Component` 的類，該類上有一個 `draw` 方法。其他的類比如 `Button`、`Image` 和 `SelectBox` 會從 `Component` 派生並因此繼承 `draw` 方法。它們各自都可以重寫 `draw` 方法來定義自己的行為，但是框架會把所有這些型別當作是 `Component` 的例項，並在其上呼叫 `draw`。不過 Rust 並沒有繼承，我們需要尋找另一種方式來設計 `gui` 庫，以便使用者能夠使用新型別進行擴充套件。

### 定義通用行為的 trait

為了實現 `gui` 所期望的行為，讓我們定義一個 `Draw` trait，其中包含名為 `draw` 的方法。接著可以定義一個存放**trait 物件**（*trait object*）的 vector。trait 物件指向一個實現了我們指定 trait 的型別的例項，以及一個用於在執行時查詢該型別的 trait 方法的表。我們透過指定某種指標來建立 trait 物件，例如 `&` 引用或 `Box<T>` 智慧指標，還有 `dyn` 關鍵字，以及指定相關的 trait（第二十章 [“動態大小型別和 `Sized` trait”][dynamically-sized] 部分會介紹 trait 物件必須使用指標的原因）。我們可以使用 trait 物件代替泛型或具體型別。任何使用 trait 物件的位置，Rust 的型別系統會在編譯時確保任何在此上下文中使用的值會實現其 trait 物件的 trait。如此便無需在編譯時就知曉所有可能的型別。

之前提到過，在 Rust 中，我們刻意不把結構體和列舉稱為“物件”，以便把它們和其他語言裡的物件區分開來。在結構體或列舉裡，欄位中的資料和 `impl` 塊裡的行為是分開的，而在其他語言中，資料和行為往往會被合併進一個被稱為物件的概念裡。trait object 和其他語言中的物件也不完全相同，因為我們不能向 trait object 中新增資料。trait object 沒有其他語言中的物件那麼通用；它的特定用途是為共享行為提供抽象。

示例 18-3 展示瞭如何定義一個帶有 `draw` 方法的 trait `Draw`：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

<span class="caption">示例 18-3：`Draw` trait 的定義</span>

因為第十章已經討論過如何定義 trait，其語法看起來應該比較眼熟。接下來就是一些新語法：示例 18-4 定義了一個存放了名叫 `components` 的 vector 的結構體 `Screen`。這個 vector 的型別是 `Box<dyn Draw>`，此為一個 trait 物件：它是 `Box` 中任何實現了 `Draw` trait 的型別的替身。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

<span class="caption">示例 18-4: 一個 `Screen` 結構體的定義，它帶有一個欄位 `components`，其包含實現了 `Draw` trait 的 trait 物件的 vector</span>

在 `Screen` 結構體上，我們將定義一個 `run` 方法，該方法會對其 `components` 上的每一個元件呼叫 `draw` 方法，如示例 18-5 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

<span class="caption">示例 18-5：在 `Screen` 上實現一個 `run` 方法，該方法在每個 component 上呼叫 `draw` 方法</span>

這與定義使用了帶有 trait 約束的泛型型別引數的結構體不同。泛型型別引數一次只能替代一個具體型別，而 trait 物件則允許在執行時替代多種具體型別。例如，可以定義 `Screen` 結構體來使用泛型和 trait 約束，如示例 18-6 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

<span class="caption">示例 18-6: 一種 `Screen` 結構體的替代實現，其 `run` 方法使用泛型和 trait 約束</span>

這限制了 `Screen` 例項必須擁有一個全是 `Button` 型別或者全是 `TextField` 型別的元件列表。如果只需要同質（相同型別，homogeneous）集合，則傾向於使用泛型和 trait 約束，因為其定義會在編譯時採用具體型別進行單態化（monomorphized）。

另一方面，透過使用 trait 物件的方法，一個 `Screen` 例項可以存放一個既能包含 `Box<Button>`，也能包含 `Box<TextField>` 的 `Vec<T>`。讓我們看看它是如何工作的，接著會講到其執行時效能影響。

### 實現 trait

現在來增加一些實現了 `Draw` trait 的型別。我們將提供 `Button` 型別。再一次重申，真正實現 GUI 庫超出了本書的範疇，所以 `draw` 方法體中不會有任何有意義的實現。為了想象一下這個實現看起來像什麼，一個 `Button` 結構體可能會擁有 `width`、`height` 和 `label` 欄位，如示例 18-7 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

<span class="caption">示例 18-7: 一個實現了 `Draw` trait 的 `Button` 結構體</span>

在 `Button` 上的 `width`、`height` 和 `label` 欄位會和其他元件不同；比如 `TextField` 可能有 `width`、`height`、`label` 以及 `placeholder` 欄位。每一個我們希望能在螢幕上繪製的型別都會使用不同的程式碼來實現 `Draw` trait 的 `draw` 方法來定義如何繪製特定的型別，像這裡的 `Button` 型別（如上提到的並不包含任何實際的 GUI 程式碼）。除了實現 `Draw` trait 之外，比如 `Button` 還可能有另一個包含按鈕點選如何響應的方法的 `impl` 塊。這類方法並不適用於像 `TextField` 這樣的型別。

如果一些庫的使用者決定實現一個包含 `width`、`height` 和 `options` 欄位的結構體 `SelectBox`，並且也為其實現了 `Draw` trait，如示例 18-8 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

<span class="caption">示例 18-8: 另一個使用 `gui` 的 crate，其在 `SelectBox` 結構體上實現 `Draw` trait</span>

庫使用者現在可以在他們的 `main` 函式中建立一個 `Screen` 例項。至此可以透過將 `SelectBox` 和 `Button` 放入 `Box<T>` 轉變為 trait 物件再放入 `Screen` 例項中。接著可以呼叫 `Screen` 的 `run` 方法，它會呼叫每個元件的 `draw` 方法。示例 18-9 展示了這個實現：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

<span class="caption">示例 18-9: 使用 trait 物件來儲存實現了相同 trait 的不同型別的值</span>

當編寫庫的時候，我們不知道何人會在何時增加 `SelectBox` 型別，不過 `Screen` 的實現能夠操作並繪製這個新型別，因為 `SelectBox` 實現了 `Draw` trait，這意味著它實現了 `draw` 方法。

這個概念 —— 只關心值所反映的資訊而不是其具體型別 —— 類似於動態型別語言中稱為**鴨子型別**（*duck typing*）的概念：如果它走起來像一隻鴨子，叫起來像一隻鴨子，那麼它就是一隻鴨子！在示例 18-5 中 `Screen` 上的 `run` 實現中，`run` 並不需要知道各個元件的具體型別是什麼。它並不檢查元件是 `Button` 或者 `SelectBox` 的例項，而是直接呼叫元件的 `draw` 方法。透過指定 `Box<dyn Draw>` 作為 `components` vector 中值的型別，我們就定義了 `Screen` 為需要可以在其上呼叫 `draw` 方法的值。

使用 trait 物件和 Rust 型別系統來進行類似鴨子型別操作的優勢是無需在執行時檢查一個值是否實現了特定方法或者擔心在呼叫時因為值沒有實現方法而產生錯誤。如果值沒有實現 trait 物件所需的 trait 則 Rust 不會編譯這些程式碼。

例如，示例 18-10 展示了當建立一個使用 `String` 做為其元件的 `Screen` 時發生的情況：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

<span class="caption">示例 18-10: 嘗試使用一種沒有實現 trait 物件的 trait 的型別</span>

我們會遇到這個錯誤因為 `String` 沒有實現 `Draw` trait：

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

這個錯誤告訴我們：要麼是我們把一個本來就不該傳給 `Screen` 的型別傳進來了，因此應該換成別的型別；要麼就該為 `String` 實現 `Draw`，讓 `Screen` 能夠對它呼叫 `draw`。

### trait 物件執行動態分發

回憶一下第十章 [“泛型程式碼的效能”][performance-of-code-using-generics] 部分討論過的，當對泛型使用 trait 約束時編譯器所執行的單態化處理：編譯器為每一個被泛型型別引數代替的具體型別生成了函式和方法的非泛型實現。單態化產生的程式碼在執行**靜態分發**（*static dispatch*），也就是說編譯器在編譯時就知曉要呼叫什麼方法。這與**動態分發** （*dynamic dispatch*）相對，這時編譯器在編譯時無法知曉要呼叫哪個方法。在動態分發的場景下，編譯器會生成負責在執行時確定該呼叫什麼方法的程式碼。

當使用 trait 物件時，Rust 必須使用動態分發。編譯器無法知曉所有可能用於 trait 物件程式碼的型別，所以它也不知道應該呼叫哪個型別的哪個方法實現。為此，Rust 在執行時使用 trait 物件中的指標來知曉需要呼叫哪個方法。這種查詢會帶來在靜態分發中不會產生的執行時開銷。動態分發也阻止編譯器有選擇地內聯方法程式碼，這會相應地停用一些最佳化，Rust 還定義了一些規則，稱為**dyn 相容性**（_dyn compatibility_），用於規定可以和不可以在哪些地方使用動態分發。這些規則超出了本討論範圍，但你可以在[參考資料][dyn-compatibility]中詳細瞭解。儘管在編寫示例 18-5 和可以支援示例 18-9 中的程式碼的過程中確實獲得了額外的靈活性，但仍然需要權衡取捨。

[performance-of-code-using-generics]: ch10-01-syntax.html#泛型程式碼的效能
[dynamically-sized]: ch20-03-advanced-types.html#動態大小型別和-sized-trait
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
