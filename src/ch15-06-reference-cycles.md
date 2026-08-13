## 引用迴圈與記憶體洩漏

[ch15-06-reference-cycles.md](https://github.com/rust-lang/book/blob/ecef81cbc6f0c2d1c8a67409329b0641258c04c2/src/ch15-06-reference-cycles.md)

Rust 的記憶體安全性保證使其難以意外地製造永遠也不會被清理的記憶體（被稱為 **記憶體洩漏**，_memory leak_），但並非不可能。Rust 並不保證完全防止記憶體洩漏，這意味著記憶體洩漏在 Rust 中被認為是記憶體安全的。這一點可以透過 `Rc<T>` 和 `RefCell<T>` 看出 Rust 允許出現記憶體洩漏：建立引用迴圈的可能性是存在的。這會造成記憶體洩漏，因為每一項的引用計數永遠也到不了 0，持有的資料也就永遠不會被釋放。

### 製造引用迴圈

讓我們看看引用迴圈可能是如何發生的，以及如何避免它。先從示例 15-25 中 `List` 列舉和 `tail` 方法的定義開始：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs:here}}
```

<span class="caption">示例 15-25：一個持有 `RefCell<T>` 的 cons list 定義，這樣我們就能修改 `Cons` 變體所引用的內容</span>

這裡採用了示例 15-5 中 `List` 定義的另一種變體。現在 `Cons` 變體的第二個元素是 `RefCell<Rc<List>>`，這意味著不同於像示例 15-24 那樣能夠修改 `i32` 的值，我們希望能夠修改 `Cons` 變體所指向的 `List`。這裡還增加了一個 `tail` 方法來方便我們在有 `Cons` 變體的時候訪問其第二項。

在示例 15-26 中，我們添加了一個 `main` 函式，它使用了示例 15-25 中的定義。這段程式碼會先在 `a` 中建立一個列表，再建立一個指向 `a` 中列表的 `b` 列表。然後，它會修改 `a` 中的列表，使其指向 `b`，從而建立一個引用迴圈。沿途加入的 `println!` 語句會展示這一過程中不同位置的引用計數。

<span class="filename">檔案：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

<span class="caption">示例 15-26：建立兩個彼此互相指向的 `List` 值，從而形成引用迴圈</span>

我們在變數 `a` 中建立了一個 `Rc<List>` 例項，它持有一個值為 `5, Nil` 的 `List`。接著，又在變數 `b` 中建立了另一個 `Rc<List>` 例項，它持有一個值為 `10`、並指向 `a` 中列表的 `List`。

然後，我們修改 `a`，讓它指向 `b` 而不是 `Nil`，這樣就建立了一個迴圈。為此，我們使用 `tail` 方法獲取 `a` 中 `RefCell<Rc<List>>` 的引用，並把它放到變數 `link` 中。接著，呼叫這個 `RefCell<Rc<List>>` 上的 `borrow_mut` 方法，把它內部的值從持有 `Nil` 的 `Rc<List>` 改成 `b` 中的 `Rc<List>`。

如果保持最後的 `println!` 行註釋並執行程式碼，會得到如下輸出：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

我們可以看到，當把 `a` 中的列表改為指向 `b` 之後，`a` 和 `b` 中 `Rc<List>` 例項的引用計數都變成了 2。在 `main` 結束時，Rust 會先丟棄變數 `b`，這會使 `b` 中那個 `Rc<List>` 例項的引用計數從 2 減到 1。由於引用計數不是 0，所以此時分配在堆上的記憶體不會被丟棄。然後，Rust 再丟棄 `a`，這會使 `a` 中那個 `Rc<List>` 例項的引用計數也從 2 減到 1。這個例項的記憶體同樣無法被清理，因為另一個 `Rc<List>` 例項仍然引用著它。分配給這些列表的記憶體將會永遠留在那裡而不會被回收。為了更直觀地展示這個引用迴圈，我們建立了圖 15-4 所示的示意圖：

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">圖 15-4：列表 `a` 和 `b` 彼此互相指向，從而形成引用迴圈</span>

如果取消最後 `println!` 的註釋並執行程式，Rust 會嘗試打印出 `a` 指向 `b` 指向 `a` 這樣的迴圈直到棧溢位。

相比真實世界的程式，這個例子中建立引用迴圈的結果並不可怕：建立了引用迴圈之後程式立刻就結束了。如果在更為複雜的程式中並在迴圈裡分配了很多記憶體並佔有很長時間，這個程式會使用多於它所需要的記憶體，並有可能壓垮系統並造成沒有記憶體可供使用。

建立引用迴圈並不容易，但也不是不可能。如果你有包含 `Rc<T>` 的 `RefCell<T>` 值或類似的巢狀結合了內部可變性和引用計數的型別，請務必小心確保你沒有形成一個引用迴圈；你無法指望 Rust 幫你捕獲它們。建立引用迴圈是一個程式上的邏輯 bug，你應該使用自動化測試、程式碼評審和其他軟體開發最佳實踐來使其最小化。

另一個解決方案是重新組織資料結構，使得一部分引用擁有所有權而另一部分沒有。換句話說，迴圈將由一些擁有所有權的關係和一些無所有權的關係組成，只有所有權關係才能影響值是否可以被丟棄。在示例 15-25 中，我們總是希望 `Cons` 變體擁有其列表，所以重新組織資料結構是不可能的。讓我們看看一個由父節點和子節點構成的圖的例子，觀察何時是使用無所有權的關係來避免引用迴圈的合適時機。

### 使用 `Weak<T>` 防止引用迴圈

到目前為止，我們已經展示了呼叫 `Rc::clone` 會增加 `Rc<T>` 例項的 `strong_count`，和只在其 `strong_count` 為 0 時 `Rc<T>` 例項才會被清理。你也可以透過呼叫 `Rc::downgrade` 並傳遞 `Rc<T>` 例項的引用來建立其值的**弱引用**（_weak reference_）。強引用代表如何共享 `Rc<T>` 例項的所有權；弱引用不表達所有權關係，當 `Rc<T>` 例項被清理時其計數沒有影響。它們不會造成引用迴圈，因為任何涉及弱引用的迴圈會在其相關的值的強引用計數為 0 時被打斷。

呼叫 `Rc::downgrade` 時會得到 `Weak<T>` 型別的智慧指標。不同於將 `Rc<T>` 例項的 `strong_count` 加 1，呼叫 `Rc::downgrade` 會將 `weak_count` 加 1。`Rc<T>` 型別使用 `weak_count` 來記錄其存在多少個 `Weak<T>` 引用，類似於 `strong_count`。其區別在於 `weak_count` 無需計數為 0 就能使 `Rc<T>` 例項被清理。

因為 `Weak<T>` 引用的值可能已經被丟棄了，為了使用 `Weak<T>` 所指向的值，我們必須確保其值仍然有效。為此可以呼叫 `Weak<T>` 例項的 `upgrade` 方法，這會返回 `Option<Rc<T>>`。如果 `Rc<T>` 值還未被丟棄，則結果是 `Some`；如果 `Rc<T>` 已被丟棄，則結果是 `None`。因為 `upgrade` 返回一個 `Option<Rc<T>>`，Rust 會確保處理 `Some` 和 `None` 的情況，所以它不會返回無效指標。

作為示例，我們不再使用只知道下一個元素的列表，而是建立一個既知道子節點又知道父節點的樹結構。

#### 建立樹形資料結構

首先，我們將構建一棵樹，其中節點能夠知道自己的子節點。我們會建立一個名為 `Node` 的結構體，它存放自己的 `i32` 值，以及對子 `Node` 值的引用：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

我們希望 `Node` 能擁有它的子節點，同時也希望能與變數共享這種所有權，以便能夠直接訪問樹中的每個 `Node`。為此，我們將 `Vec<T>` 中元素的型別定義為 `Rc<Node>`。我們還希望能夠修改某個節點的子節點，因此把 `children` 中的 `Vec<Rc<Node>>` 包裝進了 `RefCell<T>`。

接下來，使用此結構體定義來建立一個叫做 `leaf` 的帶有值 `3` 且沒有子節點的 `Node` 例項，和另一個帶有值 5 並以 `leaf` 作為子節點的例項 `branch`，如示例 15-27 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

<span class="caption">示例 15-27：建立沒有子節點的 `leaf` 節點和以 `leaf` 作為子節點的 `branch` 節點</span>

這裡克隆了 `leaf` 中的 `Rc<Node>` 並儲存在 `branch` 中，這意味著 `leaf` 中的 `Node` 現在有兩個所有者：`leaf` 和 `branch`。可以透過 `branch.children` 從 `branch` 中獲得 `leaf`，不過無法從 `leaf` 得到 `branch`。`leaf` 沒有到 `branch` 的引用且並不知道它們相互關聯。我們希望 `leaf` 知道 `branch` 是其父節點。接下來我們會這麼做。

#### 增加從子到父的引用

為了使子節點知道其父節點，需要在 `Node` 結構體定義中增加一個 `parent` 欄位。問題是 `parent` 的型別應該是什麼。我們知道其不能包含 `Rc<T>`，因為這樣 `leaf.parent` 將會指向 `branch` 而 `branch.children` 會包含 `leaf` 的指標，這會形成引用迴圈，會造成其 `strong_count` 永遠也不會為 0。

換一種方式來思考這種關係：父節點應該擁有它的子節點。如果父節點被丟棄了，它的子節點也應該被丟棄。然而，子節點不應該擁有它的父節點。如果我們丟棄一個子節點，父節點仍然應該存在。這正是弱引用適用的場景！

所以 `parent` 使用 `Weak<T>` 型別而不是 `Rc<T>`，具體來說是 `RefCell<Weak<Node>>`。現在 `Node` 結構體定義看起來像這樣：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

這樣，一個節點就能夠引用其父節點，但不擁有其父節點。在示例 15-28 中，我們更新 `main` 來使用新定義以便 `leaf` 節點可以透過 `branch` 引用其父節點：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

<span class="caption">示例 15-28：一個 `leaf` 節點，其擁有指向其父節點 `branch` 的弱引用</span>

建立 `leaf` 節點類似於示例 15-27，除了 `parent` 欄位有所不同：`leaf` 開始時沒有父節點，所以我們新建了一個空的 `Weak<Node>` 引用例項。

此時，當嘗試使用 `upgrade` 方法獲取 `leaf` 的父節點引用時，會得到一個 `None` 值。如第一個 `println!` 輸出所示：

```text
leaf parent = None
```

當建立 `branch` 節點時，其也會新建一個 `Weak<Node>` 引用，因為 `branch` 並沒有父節點。`leaf` 仍然作為 `branch` 的一個子節點。一旦在 `branch` 中有了 `Node` 例項，就可以修改 `leaf` 使其擁有指向父節點的 `Weak<Node>` 引用。這裡使用了 `leaf` 中 `parent` 欄位裡的 `RefCell<Weak<Node>>` 的 `borrow_mut` 方法，接著使用了 `Rc::downgrade` 函式來從 `branch` 中的 `Rc<Node>` 值建立了一個指向 `branch` 的 `Weak<Node>` 引用。

當再次打印出 `leaf` 的父節點時，這一次將會得到存放了 `branch` 的 `Some` 值：現在 `leaf` 可以訪問其父節點了！當打印出 `leaf` 時，我們也避免瞭如示例 15-26 中最終會導致棧溢位的迴圈：`Weak<Node>` 引用被列印為 `(Weak)`：

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

沒有無限的輸出表明這段程式碼並沒有造成引用迴圈。這一點也可以從觀察 `Rc::strong_count` 和 `Rc::weak_count` 呼叫的結果看出。

#### 視覺化 `strong_count` 和 `weak_count` 的變化

讓我們透過建立了一個新的內部作用域並將 `branch` 的建立放入其中，來觀察 `Rc<Node>` 例項的 `strong_count` 和 `weak_count` 值的變化。這會展示當 `branch` 建立和離開作用域被丟棄時會發生什麼。這些修改如示例 15-29 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

<span class="caption">示例 15-29：在內部作用域建立 `branch` 並檢查其強弱引用計數</span>

一旦建立了 `leaf`，其 `Rc<Node>` 的強引用計數為 1，弱引用計數為 0。在內部作用域中建立了 `branch` 並與 `leaf` 相關聯，此時 `branch` 中 `Rc<Node>` 的強引用計數為 1，弱引用計數為 1（因為 `leaf.parent` 透過 `Weak<Node>` 指向 `branch`）。這裡 `leaf` 的強引用計數為 2，因為現在 `branch` 的 `branch.children` 中儲存了 `leaf` 的 `Rc<Node>` 的複製，不過弱引用計數仍然為 0。

當內部作用域結束時，`branch` 離開作用域，`Rc<Node>` 的強引用計數減少為 0，所以其 `Node` 被丟棄。來自 `leaf.parent` 的弱引用計數 1 與 `Node` 是否被丟棄無關，所以並沒有產生任何記憶體洩漏！

如果在內部作用域結束後嘗試訪問 `leaf` 的父節點，會再次得到 `None`。在程式的結尾，`leaf` 中 `Rc<Node>` 的強引用計數為 1，弱引用計數為 0，因為現在 `leaf` 又是 `Rc<Node>` 唯一的引用了。

所有這些管理計數和值的邏輯都內建於 `Rc<T>` 和 `Weak<T>` 以及它們的 `Drop` trait 實現中。透過在 `Node` 定義中指定從子節點到父節點的關係為一個 `Weak<T>` 引用，就能夠擁有父節點和子節點之間的雙向引用而不會造成引用迴圈和記憶體洩漏。

## 總結

這一章涵蓋了如何使用智慧指標來做出不同於 Rust 常規引用預設所提供的保證與取捨。`Box<T>` 有一個已知的大小並指向分配在堆上的資料。`Rc<T>` 記錄了堆上資料的引用計數從而允許多個所有者。`RefCell<T>` 型別及其內部可變性允許我們在保持型別不可變的前提下更改其內部值；它也在執行時而非編譯時執行借用規則檢查。

我們還討論了 trait `Deref` 和 `Drop`，它們實現了智慧指標的許多功能。同時探索了會造成記憶體洩漏的引用迴圈，以及如何使用 `Weak<T>` 來避免它們。

如果本章內容引起了你的興趣並希望現在就實現你自己的智慧指標的話，請閱讀 [“The Rustonomicon”][nomicon] 來獲取更多有用的資訊。

接下來，讓我們談談 Rust 的併發。屆時甚至還會學習到一些新的對併發有幫助的智慧指標。

[nomicon]: https://doc.rust-lang.org/nomicon/index.html
