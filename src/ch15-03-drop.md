## 使用 `Drop` Trait 執行清理程式碼

[ch15-03-drop.md](https://github.com/rust-lang/book/blob/ecef81cbc6f0c2d1c8a67409329b0641258c04c2/src/ch15-03-drop.md)

對智慧指標模式來說，第二個重要的 trait 是 `Drop`，它允許你自定義一個值即將離開作用域時要發生的事情。你可以為任何型別提供 `Drop` trait 的實現，而其中的程式碼可以用來釋放諸如檔案或網路連線之類的資源。

我們在智慧指標的上下文中介紹 `Drop`，是因為實現智慧指標時幾乎總會用到 `Drop` trait。例如，當一個 `Box<T>` 被丟棄時，它會釋放 box 所指向的堆空間。

在某些語言裡，對於某些型別，程式設計師每次在使用完這些型別的例項後，都必須呼叫程式碼去釋放記憶體或其他資源。常見例子包括檔案控制代碼、套接字和鎖。如果程式設計師忘了這麼做，系統就可能因為負擔過重而崩潰。在 Rust 中，你可以指定某段程式碼在值離開作用域時執行，而編譯器會自動插入這段程式碼。這樣一來，你就不必小心翼翼地在程式各處都放置清理程式碼來處理某個型別例項結束使用時的情況，同時也不會洩漏資源！

指定在值離開作用域時應該執行的程式碼的方式是實現 `Drop` trait。`Drop` trait 要求實現一個叫做 `drop` 的方法，它獲取一個 `self` 的可變引用。為了能夠看出 Rust 何時呼叫 `drop`，讓我們暫時使用 `println!` 語句實現 `drop`。

示例 15-14 展示了唯一定製功能就是當其例項離開作用域時，打印出 `Dropping CustomSmartPointer!` 的結構體 `CustomSmartPointer`，這會演示 Rust 何時執行 `drop` 方法：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<span class="caption">示例 15-14：結構體 `CustomSmartPointer`，其實現了放置清理程式碼的 `Drop` trait</span>

`Drop` trait 包含在 prelude 中，因此無需將其引入作用域。我們在 `CustomSmartPointer` 上實現了 `Drop` trait，並提供了一個呼叫 `println!` 的 `drop` 方法實現。`drop` 函式體是放置任何當型別例項離開作用域時期望執行的邏輯的地方。這裡選擇列印一些文本以視覺化地展示 Rust 何時呼叫 `drop`。

在 `main` 中，我們新建了兩個 `CustomSmartPointer` 例項並打印出了 `CustomSmartPointer created.`。在 `main` 的結尾，`CustomSmartPointer` 的例項會離開作用域，而 Rust 會呼叫放置於 `drop` 方法中的程式碼，打印出最後的資訊。注意無需顯式呼叫 `drop` 方法：

當執行這個程式，會出現如下輸出：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

當例項離開作用域時，Rust 會自動替我們呼叫 `drop`，並執行我們指定的程式碼。變數會按照建立順序的逆序被丟棄，所以 `d` 會先於 `c` 被丟棄。這個例子的目的，是讓你直觀地看到 `drop` 方法是如何工作的；而通常在真實程式碼裡，你會寫的是型別所需的清理邏輯，而不是一條列印訊息。

不幸的是，停用自動 `drop` 功能並不是一件容易的事。通常也不需要停用 `drop` ；整個 `Drop` trait 存在的意義在於其是自動處理的。然而，有時你可能需要提早清理某個值。一個例子是當使用智慧指標管理鎖時；你可能希望強制執行 `drop` 方法來釋放鎖以便作用域中的其他程式碼可以獲取鎖。Rust 並不允許我們主動呼叫 `Drop` trait 的 `drop` 方法；當我們希望在作用域結束之前就強制釋放變數的話，我們應該使用的是由標準庫提供的 `std::mem::drop` 函式。

如果我們像是示例 15-14 那樣嘗試呼叫 `Drop` trait 的 `drop` 方法，就會得到像示例 15-15 那樣的編譯錯誤：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<span class="caption">示例 15-15：嘗試手動呼叫 `Drop` trait 的 `drop` 方法提早清理</span>

如果嘗試編譯程式碼會得到如下錯誤：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

錯誤資訊表明，我們不被允許顯式呼叫 `drop`。錯誤資訊中使用了術語**解構函式**（*destructor*），這是程式設計中對“清理某個例項的函式”的通用稱呼。解構函式與**建構函式**（*constructor*）相對應，後者用於建立例項。Rust 中的 `drop` 函式就是一種特定的解構函式。

Rust 不允許我們顯式呼叫 `drop`，因為 Rust 仍然會在 `main` 結束時自動對該值呼叫 `drop`。這會導致二次釋放（*double free*）錯誤，因為 Rust 會嘗試清理同一個值兩次。

因為不能停用當值離開作用域時自動插入的 `drop`，並且不能顯式呼叫 `drop` 方法。如果我們需要強制提早清理值，可以使用 `std::mem::drop` 函式。

`std::mem::drop` 函式與 `Drop` trait 中的 `drop` 方法不同。我們通過把想要強制提前丟棄的值作為引數傳給它來呼叫。這個函式位於 prelude 中，因此我們可以修改示例 15-15 裡的 `main`，改為呼叫 `drop` 函式，如示例 15-16 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<span class="caption">示例 15-16：在值離開作用域之前呼叫 `std::mem::drop` 顯式清理</span>

執行這段程式碼將打印出如下內容：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

文本 ``Dropping CustomSmartPointer with data `some data`!`` 會出現在 `CustomSmartPointer created.` 和 `CustomSmartPointer dropped before the end of main.` 之間，這表明 `drop` 方法的程式碼在那個時刻被呼叫，以丟棄 `c`。

你可以以多種方式利用 `Drop` trait 實現裡指定的程式碼，讓清理既方便又安全。例如，你可以用它來建立自己的記憶體分配器！有了 `Drop` trait 和 Rust 的所有權系統，你就不必記住何時進行清理，因為 Rust 會自動替你完成這些工作。

你也不必擔心由於不小心清理仍在使用的值而導致的問題：所有權系統確保引用總是有效的，也會確保 `drop` 只會在值不再被使用時被呼叫一次。

現在我們已經瞭解了 `Box<T>` 以及智慧指標的一些特徵，接下來看看標準庫中定義的其他幾種智慧指標。
