## `RefCell<T>` 和內部可變性模式

[ch15-05-interior-mutability.md](https://github.com/rust-lang/book/blob/ecef81cbc6f0c2d1c8a67409329b0641258c04c2/src/ch15-05-interior-mutability.md)

**內部可變性**（_Interior mutability_）是 Rust 中的一個設計模式，它允許你即使在有不可變引用時也可以改變資料，這通常是借用規則所不允許的。為了改變資料，該模式在資料結構中使用 `unsafe` 程式碼來模糊 Rust 通常的可變性和借用規則。不安全程式碼表明我們在手動檢查這些規則而不是讓編譯器替我們檢查。第二十章會更詳細地介紹不安全程式碼。

當可以確保程式碼在執行時會遵守借用規則，即使編譯器不能保證的情況，可以選擇使用那些運用內部可變性模式的型別。所涉及的 `unsafe` 程式碼將被封裝進安全的 API 中，而外部型別仍然是不可變的。

讓我們通過遵循內部可變性模式的 `RefCell<T>` 型別來探索這個概念。

### 在執行時強制借用規則

不同於 `Rc<T>`，`RefCell<T>` 代表其資料的唯一的所有權。那麼是什麼讓 `RefCell<T>` 不同於像 `Box<T>` 這樣的型別呢？回憶一下第四章所學的借用規則：

- 在任意給定時刻，只能擁有一個可變引用或任意數量的不可變引用**之一**（而不是兩者）。
- 引用必須始終有效。

對於引用和 `Box<T>`，借用規則的不可變性（invariants）在編譯時就會被強制執行。對於 `RefCell<T>`，這些不可變性作用於**執行時**。對於引用，如果違反這些規則，會得到一個編譯錯誤。而對於 `RefCell<T>`，如果違反這些規則程式會 panic 並退出。

在編譯時檢查借用規則的優勢是這些錯誤將在開發過程的早期被捕獲，同時對執行時沒有效能影響，因為所有的分析都提前完成了。為此，在編譯時檢查借用規則是大部分情況的最佳選擇，這也正是其為何是 Rust 的預設行為。

相反在執行時檢查借用規則的好處則是允許出現特定記憶體安全的場景，而它們在編譯時檢查中是不允許的。靜態分析，正如 Rust 編譯器，是天生保守的。但程式碼的一些屬性不可能通過分析程式碼發現：其中最著名的就是[停機問題（Halting Problem）](https://zh.wikipedia.org/wiki/%E5%81%9C%E6%9C%BA%E9%97%AE%E9%A2%98)，這超出了本書的範疇，不過如果你感興趣的話這是一個值得研究的有趣主題。

因為一些分析是不可能的，如果 Rust 編譯器不能通過所有權規則編譯，它可能會拒絕一個正確的程式；從這種角度考慮它是保守的。如果 Rust 接受不正確的程式，那麼使用者也就不會相信 Rust 所做的保證了。然而，如果 Rust 拒絕正確的程式，雖然會給程式設計師帶來不便，但不會帶來災難。`RefCell<T>` 正是用於當你確信程式碼遵守借用規則，而編譯器不能理解和確定的時候。

類似於 `Rc<T>`，`RefCell<T>` 只能用於單執行緒場景。如果嘗試在多執行緒上下文中使用 `RefCell<T>`，會得到一個編譯錯誤。第十六章會介紹如何在多執行緒程式中實現 `RefCell<T>` 的功能。

如下為選擇 `Box<T>`，`Rc<T>` 或 `RefCell<T>` 的理由：

- `Rc<T>` 允許相同資料有多個所有者；`Box<T>` 和 `RefCell<T>` 則只有單一所有者。
- `Box<T>` 允許在編譯時執行不可變或可變借用檢查；`Rc<T>` 僅允許在編譯時執行不可變借用檢查；`RefCell<T>` 允許在執行時執行不可變或可變借用檢查。
- 因為 `RefCell<T>` 允許在執行時執行可變借用檢查，所以我們可以在即便 `RefCell<T>` 自身是不可變的情況下修改其內部的值。

在不可變值內部改變值就是**內部可變性**（_interior mutability_）模式。讓我們看看何時內部可變性是有用的，並討論這是如何成為可能的。

### 使用內部可變性

借用規則的一個推論是當有一個不可變值時，不能可變地借用它。例如，如下程式碼不能編譯：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

如果嘗試編譯，會得到如下錯誤：

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

然而，特定情況下，令一個值在其方法內部能夠修改自身，而在其他程式碼中仍視為不可變，是很有用的。值方法外部的程式碼就不能修改其值了。`RefCell<T>` 是一個獲得內部可變性的方法。`RefCell<T>` 並沒有完全繞開借用規則，編譯器中的借用檢查器允許內部可變性並相應地在執行時檢查借用規則。如果違反了這些規則，會出現 panic 而不是編譯錯誤。

讓我們通過一個實際的例子來探索何處可以使用 `RefCell<T>` 來修改不可變值並看看為何這麼做是有意義的。

#### 使用 mock 物件測試

有時在測試中程式設計師會用某個型別替換另一個型別，以便觀察特定的行為並斷言它是被正確實現的。這個佔位符型別被稱為 **測試替身**（_test double_）。就像電影製作中的替身演員（_stunt double_）一樣，替代演員完成高難度的場景。測試替身在執行測試時替代某個型別。**mock 物件** 是特定型別的測試替身，它們記錄測試過程中發生了什麼以便可以斷言操作是正確的。

Rust 並不像其他語言那樣在標準庫中提供內建的物件模型，Rust 也沒有像其他語言那樣在標準庫中內建 mock 物件功能，不過我們確實可以建立一個與 mock 物件有著相同功能的結構體。

如下是一個我們想要測試的場景：我們在編寫一個記錄某個值與最大值的差距的庫，並根據當前值與最大值的差距來發送訊息。例如，這個庫可以用於記錄使用者所允許的 API 呼叫數量限額。

該庫只提供記錄與最大值的差距，以及何種情況傳送什麼訊息的功能。使用此庫的程式則期望提供實際傳送訊息的機制：程式可以選擇記錄一條訊息、傳送 email、傳送簡訊等等。庫本身無需知道這些細節；只需實現其提供的 `Messenger` trait 即可。示例 15-20 展示了庫程式碼：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

<span class="caption">示例 15-20：一個記錄某個值與最大值差距的庫，並根據此值的特定級別發出警告</span>

這些程式碼中一個重要部分是擁有一個方法 `send` 的 `Messenger` trait，其獲取一個 `self` 的不可變引用和文本資訊。這個 trait 是 mock 物件所需要實現的介面庫，這樣 mock 就能像一個真正的物件那樣使用了。另一個重要的部分是我們需要測試 `LimitTracker` 的 `set_value` 方法的行為。可以改變傳遞的 `value` 引數的值，不過 `set_value` 並沒有返回任何可供斷言的值。我們希望能夠說，如果我們建立一個實現了 `Messenger` trait 和具有特定 `max` 值的 `LimitTracker` 時，當傳遞不同 `value` 值時，訊息傳送者應被告知傳送合適的訊息。

我們所需的 mock 物件是，呼叫 `send` 並不實際傳送 email 或訊息，而是隻記錄資訊被通知要傳送了。可以新建一個 mock 物件例項，用其建立 `LimitTracker`，呼叫 `LimitTracker` 的 `set_value` 方法，然後檢查 mock 物件是否有我們期望的訊息。示例 15-21 展示了一個如此嘗試的 mock 物件實現，不過借用檢查器並不允許：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

<span class="caption">示例 15-21：嘗試實現 `MockMessenger`，借用檢查器不允許這麼做</span>

測試程式碼定義了一個 `MockMessenger` 結構體，其 `sent_messages` 欄位為一個 `String` 值的 `Vec` 用來記錄被告知傳送的訊息。我們還定義了一個關聯函式 `new` 以便於新建從空訊息列表開始的 `MockMessenger` 值。接著為 `MockMessenger` 實現 `Messenger` trait 這樣就可以為 `LimitTracker` 提供一個 `MockMessenger`。在 `send` 方法的定義中，獲取傳入的訊息作為引數並儲存在 `MockMessenger` 的 `sent_messages` 列表中。

在測試中，我們測試了當 `LimitTracker` 被告知將 `value` 設定為超過 `max` 值 75% 的某個值。首先新建一個 `MockMessenger`，其從空訊息列表開始。接著新建一個 `LimitTracker` 並傳遞新建 `MockMessenger` 的引用和 `max` 值 `100`。我們使用值 `80` 呼叫 `LimitTracker` 的 `set_value` 方法，這超過了 100 的 75%。接著斷言 `MockMessenger` 中記錄的訊息列表應該有一條訊息。

然而，這個測試存在一個問題，如下所示：


```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

不能修改 `MockMessenger` 來記錄訊息，因為 `send` 方法接收的是對 `self` 的不可變引用。我們也不能採納錯誤提示中將 `&self` 改為 `&mut self` 的建議，因為那樣既要在 `impl` 方法中修改簽名，也要在 `Messenger` trait 定義中修改簽名。我們並不希望僅為了測試而改變 `Messenger` trait。相反，我們需要想辦法讓測試程式碼與現有設計相容，正常工作。

這正是內部可變性可以派上用場的地方！我們會把 `sent_messages` 儲存在一個 `RefCell<T>` 裡，然後 `send` 方法就能修改 `sent_messages` 來儲存我們見到的訊息。示例 15-22 展示了它的寫法：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

<span class="caption">示例 15-22：使用 `RefCell<T>` 能夠在外部值被認為是不可變的情況下修改內部值</span>

現在 `sent_messages` 欄位的型別是 `RefCell<Vec<String>>` 而不是 `Vec<String>`。在 `new` 函式中在空 vector 外層建立了一個 `RefCell<Vec<String>>` 例項。

對於 `send` 方法的實現，第一個引數仍為 `self` 的不可變借用，這是符合 trait 定義的。我們呼叫 `self.sent_messages` 中 `RefCell<Vec<String>>` 的 `borrow_mut` 方法來獲取 `RefCell<Vec<String>>` 中值的可變引用，這是一個 vector。接著可以對 vector 的可變引用呼叫 `push` 以便記錄測試過程中看到的訊息。

最後必須做出的修改位於斷言中：為了看到其內部 vector 中有多少個項，需要呼叫 `RefCell<Vec<String>>` 的 `borrow` 以獲取 vector 的不可變引用。

現在我們見識瞭如何使用 `RefCell<T>`，讓我們研究一下它怎樣工作的！

### 在執行時記錄借用

當建立不可變和可變引用時，我們分別使用 `&` 和 `&mut` 語法。對於 `RefCell<T>` 來說，則是 `borrow` 和 `borrow_mut` 方法，這屬於 `RefCell<T>` 安全 API 的一部分。`borrow` 方法返回 `Ref<T>` 型別的智慧指標，`borrow_mut` 方法返回 `RefMut<T>` 型別的智慧指標。這兩個型別都實現了 `Deref`，所以可以當作常規引用對待。

`RefCell<T>` 記錄當前有多少個活動的 `Ref<T>` 和 `RefMut<T>` 智慧指標。每次呼叫 `borrow`，`RefCell<T>` 將活動的不可變借用計數加一。當 `Ref<T>` 值離開作用域時，不可變借用計數減一。就像編譯時借用規則一樣，`RefCell<T>` 在任何時候只允許有多個不可變借用或一個可變借用。

如果我們嘗試違反這些規則，相比引用時的編譯時錯誤，`RefCell<T>` 的實現會在執行時出現 panic。示例 15-23 展示了對示例 15-22 中 `send` 實現的修改，這裡我們故意嘗試在相同作用域建立兩個可變借用以便演示 `RefCell<T>` 不允許我們在執行時這麼做。

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

<span class="caption">示例 15-23：在同一作用域中建立兩個可變引用並觀察 `RefCell<T>` 將會 panic</span>

這裡為 `borrow_mut` 返回的 `RefMut` 智慧指標建立了 `one_borrow` 變數。接著用相同的方式在變數 `two_borrow` 建立了另一個可變借用。這會在相同作用域中建立兩個可變引用，這是不允許的。當執行庫的測試時，示例 15-23 編譯時不會有任何錯誤，不過測試會失敗：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

注意程式碼 panic 和資訊 `already borrowed: BorrowMutError`。這也就是 `RefCell<T>` 如何在執行時處理違反借用規則的情況。

像這裡這樣選擇在執行時而不是編譯時捕獲借用錯誤，意味著你可能會在開發過程的更後期才發現錯誤，甚至直到程式碼部署到生產環境後才暴露出來。與此同時，你的程式碼還會因為在執行時而不是編譯時跟蹤借用而承擔一點執行時效能開銷。不過，使用 `RefCell<T>` 能讓我們在只允許不可變值的上下文中，寫出一個可以修改自身來記錄訊息的 mock 物件。儘管存在這些權衡，`RefCell<T>` 仍能提供比常規引用更多的能力。

### 允許多個可變資料所有者

`RefCell<T>` 的一個常見用法是與 `Rc<T>` 結合。回憶一下 `Rc<T>` 允許對相同資料有多個所有者，不過只能提供資料的不可變訪問。如果有一個儲存了 `RefCell<T>` 的 `Rc<T>` 的話，就可以得到有多個所有者**並且**可以修改的值了！

例如，回憶示例 15-18 中的 cons list 例子，我們使用 `Rc<T>` 讓多個列表共享另一個列表的所有權。由於 `Rc<T>` 只持有不可變值，所以一旦建立這些列表之後，就無法再修改其中的值。現在讓我們加入 `RefCell<T>`，藉助它來修改列表中的值。示例 15-24 展示了：通過在 `Cons` 定義中使用 `RefCell<T>`，我們就能修改所有列表中儲存的值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

<span class="caption">示例 15-24：使用 `Rc<RefCell<i32>>` 建立可以修改的 `List`</span>

這裡建立了一個 `Rc<RefCell<i32>>` 例項並儲存在變數 `value` 中以便之後直接訪問。接著在 `a` 中用包含 `value` 的 `Cons` 變體建立了一個 `List`。需要克隆 `value` 以便 `a` 和 `value` 都能擁有其內部值 `5` 的所有權，而不是將所有權從 `value` 移動到 `a` 或者讓 `a` 借用 `value`。

我們將列表 `a` 封裝進了 `Rc<T>` 這樣當建立列表 `b` 和 `c` 時，它們都可以引用 `a`，正如示例 15-18 一樣。

一旦建立了列表 `a`、`b` 和 `c`，我們將 `value` 的值加 10。為此對 `value` 呼叫了 `borrow_mut`，這裡使用了第五章討論的自動解引用功能（[“`->` 運算子到哪去了？”][wheres-the---operator] 部分）來解引用 `Rc<T>` 以獲取其內部的 `RefCell<T>` 值。`borrow_mut` 方法返回 `RefMut<T>` 智慧指標，可以對其使用解引用運算子並修改其內部值。

當我們打印出 `a`、`b` 和 `c` 時，可以看到它們都擁有修改後的值 `15` 而不是 `5`：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

這個技巧相當巧妙！通過使用 `RefCell<T>`，我們可以擁有一個對外看起來不可變的 `List` 值，但在需要時仍然能夠使用 `RefCell<T>` 提供的內部可變性方法來修改資料。借用規則在執行時進行檢查，這也確實保護了我們免受資料競爭的影響；有時候，為了換取資料結構上的這種靈活性，付出一點效能代價是值得的。注意，`RefCell<T>` 不適用於多執行緒程式碼！`Mutex<T>` 是 `RefCell<T>` 的執行緒安全版本，我們會在第十六章討論 `Mutex<T>`。

[wheres-the---operator]: ch05-03-method-syntax.html#--運算子到哪去了
