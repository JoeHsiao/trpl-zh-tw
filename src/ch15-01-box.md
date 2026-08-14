## 使用 `Box<T>` 指向堆上的資料

[ch15-01-box.md](https://github.com/rust-lang/book/blob/ecef81cbc6f0c2d1c8a67409329b0641258c04c2/src/ch15-01-box.md)

最簡單直接的智慧指標是 box，其型別寫作 `Box<T>`。Box 允許你將資料儲存在堆上而不是棧上。留在棧上的則是指向堆資料的指標。如果你想回顧一下棧和堆之間的區別，可以參考第四章。

除了把資料儲存在堆上而不是棧上之外，box 沒有效能開銷。不過，它們也沒有太多額外能力。你最常在以下這些場景中使用它們：

- 當有一個在編譯時未知大小的型別，而又想要在需要確切大小的上下文中使用這個型別值的時候
- 當有大量資料並希望在確保資料不被複製的情況下轉移所有權的時候
- 當希望擁有一個值並只關心它的型別是否實現了特定 trait 而不是其具體型別的時候

我們會在[“Box 允許建立遞迴型別”](#box-允許建立遞迴型別)一節中展示第一種場景。在第二種情況下，轉移大量資料的所有權可能會花費很長時間，因為資料會在棧上被複制。為了改善這種場景下的效能，我們可以把大量資料放進 box 中儲存到堆上。這樣，只有少量指標資料會在棧上被複制，而它所指向的資料則會一直留在堆上的同一位置。第三種情況被稱為**trait 物件**（*trait object*），第十八章中的[“使用 trait 物件來抽象共享行為”][trait-objects]專門討論了這個主題。所以你在這裡學到的內容，還會在那一節中再次用到！

### 在堆上儲存資料

在討論 `Box<T>` 的堆儲存用例之前，讓我們熟悉一下語法以及如何與儲存在 `Box<T>` 中的值進行互動。

示例 15-1 展示瞭如何使用 box 在堆上儲存一個 `i32` 值。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

<span class="caption">示例 15-1：使用 box 在堆上儲存一個 `i32` 值</span>

我們將變數 `b` 定義為一個 `Box`，它指向值 `5`，而這個值被分配在堆上。這個程式會列印 `b = 5`；在這個例子裡，我們訪問 box 中資料的方式，和資料位於棧上時的方式類似。和任何擁有所有權的值一樣，當 box 離開作用域時，就像 `b` 在 `main` 結束時那樣，它會被釋放。釋放時既會清理 box 本身（儲存在棧上），也會清理它指向的資料（儲存在堆上）。

把單個值放到堆上並沒有太大意義，所以你不會經常像示例 15-1 那樣單獨使用 box。對於像單個 `i32` 這樣的值來說，把它們放在預設儲存位置棧上，在大多數情況下更合適。接下來，我們來看一個如果沒有 box 就無法定義的型別。

### Box 允許建立遞迴型別

**遞迴型別**（_recursive type_）的值可以擁有另一個同類型的值作為其自身的一部分。但是這會產生一個問題，因為 Rust 需要在編譯時知道型別佔用多少空間。遞迴型別的值巢狀理論上可以無限地進行下去，所以 Rust 不知道遞迴型別需要多少空間。因為 box 有一個已知的大小，所以通過在遞迴型別定義中插入 box，就可以建立遞迴型別了。

作為遞迴型別的例子，讓我們來看看 cons list。這是一種在函數語言程式設計語言中常見的資料型別。我們將定義的 cons list 除了遞迴之外都很簡單，因此這個例子裡的概念，在你遇到更復雜的遞迴型別場景時也會很有用。

#### 理解 cons list

cons list 是一種來自 Lisp 程式語言及其方言的資料結構，由巢狀的 pair 組成，也是 Lisp 版本的連結串列。它的名字來源於 Lisp 中的 `cons` 函式（即 *construct function* 的縮寫），這個函式用它的兩個引數構造一個新的 pair。通過對一個由某個值和另一個 pair 組成的 pair 呼叫 `cons`，我們就能構造出由遞迴 pair 組成的 cons list。

例如這裡有一個包含列表 `1, 2, 3` 的 cons list 的虛擬碼表示，其每個對在一個括號中：

```text
(1, (2, (3, Nil)))
```

cons list 中的每一項都包含兩個元素：當前項的值，以及下一項。列表中的最後一項只包含一個名為 `Nil` 的值，而沒有下一項。cons list 是通過遞迴呼叫 `cons` 函式構造出來的。用來表示遞迴基例的規範名稱是 `Nil`。注意，這和第六章討論過的 “null” 或 “nil” 概念並不相同，後者表示無效或缺失的值。

cons list 並不是一個 Rust 中常見的型別。大部分在 Rust 中需要列表的時候，`Vec<T>` 是一個更好的選擇。其他更為複雜的遞迴資料型別**確實**在 Rust 的很多場景中很有用，不過通過以 cons list 作為開始，我們可以探索如何使用 box 毫不費力地定義一個遞迴資料型別。

示例 15-2 包含一個 cons list 的列舉定義。注意這還不能編譯因為這個型別沒有已知的大小，之後我們會展示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

<span class="caption">示例 15-2：第一次嘗試定義一個代表 `i32` 值的 cons list 資料結構的列舉</span>

> 注意：出於示例的需要我們選擇實現一個只存放 `i32` 值的 cons list。也可以用泛型，正如第十章講到的，來定義一個可以存放任何型別值的 cons list 型別。

使用這個 cons list 來儲存列表 `1, 2, 3` 將看起來如示例 15-3 所示：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

<span class="caption">示例 15-3：使用 `List` 列舉儲存列表 `1, 2, 3`</span>

第一個 `Cons` 儲存了 `1` 和另一個 `List` 值。這個 `List` 是另一個包含 `2` 的 `Cons` 值和下一個 `List` 值。接著又有另一個存放了 `3` 的 `Cons` 值和最後一個值為 `Nil` 的 `List`，非遞迴變體代表了列表的結尾。

如果嘗試編譯示例 15-3 的程式碼，會得到如示例 15-4 所示的錯誤：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

<span class="caption">示例 15-4：嘗試定義一個遞迴列舉時得到的錯誤</span>

這個錯誤表明，這個型別“有無限大小”。原因在於，我們把 `List` 的一個變體定義成了遞迴的：它直接持有另一個同類型的值。因此，Rust 無法判斷儲存一個 `List` 值到底需要多少空間。讓我們拆開看看為什麼會出現這個錯誤。首先，先來看看 Rust 是如何決定儲存非遞迴型別的值需要多少空間的。

#### 計算非遞迴型別的大小

回憶一下第六章討論列舉定義時示例 6-2 中定義的 `Message` 列舉：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

當 Rust 需要知道要為 `Message` 值分配多少空間時，它可以檢查每一個變體並發現 `Message::Quit` 並不需要任何空間，`Message::Move` 需要足夠儲存兩個 `i32` 值的空間，依此類推。因為 enum 實際上只會使用其中的一個變體，所以 `Message` 值所需的空間等於儲存其最大變體的空間大小。

與之相對的是，當 Rust 試圖確定像示例 15-2 中 `List` 列舉這樣的遞迴型別需要多少空間時，會發生什麼。編譯器先檢視 `Cons` 變體，它持有一個 `i32` 型別的值和一個 `List` 型別的值。因此，`Cons` 所需的空間等於一個 `i32` 的大小再加上一個 `List` 的大小。為了算出 `List` 型別需要多少記憶體，編譯器又要繼續檢視它的變體，並再次從 `Cons` 開始。`Cons` 又持有一個 `i32` 和一個 `List`，這個過程就會無限持續下去，如圖 15-1 所示：

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">圖 15-1：一個包含無限個 `Cons` 變體的無限 `List`</span>

#### 獲取一個已知大小的給遞迴型別

因為 Rust 無法計算出要為定義為遞迴的型別分配多少空間，所以編譯器給出了一個包括了有用建議的錯誤：

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

在這裡，_indirection_ 的意思是：不要直接儲存一個值，而是通過儲存一個指向該值的指標，間接地儲存它。

因為 `Box<T>` 是一個指標，Rust 總是知道 `Box<T>` 需要多少空間：指標的大小並不會隨它指向的資料量而變化。這意味著我們可以在 `Cons` 變體中放一個 `Box<T>`，而不是直接再放一個 `List` 值。這個 `Box<T>` 會指向下一個位於堆上的 `List` 值，而不是把這個 `List` 值直接放在 `Cons` 變體內部。從概念上說，我們仍然有一個“由列表組成的列表”，但現在這種實現更像是把這些項彼此相連，而不是一層層相互包含。

我們可以修改示例 15-2 中 `List` 列舉的定義和示例 15-3 中對 `List` 的應用，如示例 15-5 所示，這是可以編譯的：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

<span class="caption">示例 15-5：為了擁有已知大小而使用 `Box<T>` 的 `List` 定義</span>

`Cons` 變體將會需要一個 `i32` 的大小加上儲存 box 指標資料的空間。`Nil` 變體不儲存值，所以它比 `Cons` 變體需要更少的空間。現在我們知道了任何 `List` 值最多需要一個 `i32` 加上 box 指標資料的大小。通過使用 box，打破了這無限遞迴的連鎖，這樣編譯器就能夠計算出儲存 `List` 值需要的大小了。圖 15-2 展示了現在 `Cons` 變體看起來像什麼：

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

<span class="caption">圖 15-2：因為 `Cons` 存放一個 `Box` 所以 `List` 不是無限大小的了</span>

Box 只提供間接儲存和堆分配；它沒有我們將在其他智慧指標型別中看到的那些額外特殊能力。它也沒有那些特殊能力帶來的效能開銷，因此在像 cons list 這樣我們只需要“間接儲存”這一特性的場景裡，Box 就很有用。我們還會在第十八章看到更多 Box 的用例。

`Box<T>` 型別之所以是智慧指標，是因為它實現了 `Deref` trait，這讓 `Box<T>` 的值可以像引用一樣被處理。當 `Box<T>` 的值離開作用域時，由於 `Drop` trait 的實現，box 指向的堆資料也會被清理掉。這兩個 trait 對本章餘下將討論的其他智慧指標型別所提供的功能會更加重要。接下來，我們更詳細地看看這兩個 trait。

[trait-objects]: ch18-02-trait-objects.html#使用-trait-object-來抽象出共享行為
