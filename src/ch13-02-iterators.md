## 使用迭代器處理元素序列

[ch13-02-iterators.md](https://github.com/rust-lang/book/blob/4b71f17f7daba738a1363862dacb818d9b12fb81/src/ch13-02-iterators.md)

迭代器模式允許你依次對一個序列中的項執行某些操作。**迭代器**（*iterator*）負責遍歷序列中的每一項並確定序列何時結束的邏輯。使用迭代器時，你無需自己重新實現這些邏輯。

在 Rust 中，迭代器是**惰性的**（*lazy*），這意味著在呼叫消費迭代器的方法之前不會執行任何操作。例如，示例 13-10 中的程式碼通過呼叫定義於 `Vec<T>` 上的 `iter` 方法在一個 vector `v1` 上建立了一個迭代器。這段程式碼本身並沒有執行任何有用的操作。

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

<span class="caption">示例 13-10：建立一個迭代器</span>

迭代器被儲存在 `v1_iter` 變數中。一旦建立迭代器之後，可以選擇用多種方式利用它。在第三章的示例 3-5 中，我們使用 `for` 迴圈來遍歷一個數組並在每一個項上執行了一些程式碼。在底層它隱式地建立並接著消費了一個迭代器，不過直到現在我們都一筆帶過了它具體是如何工作的。

示例 13-11 中的例子將迭代器的建立和 `for` 迴圈中的使用分開。當 `for` 迴圈使用 `v1_iter` 中的迭代器時，迭代器中的每一個元素都會用於迴圈的一次迭代，並打印出每個值。

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<span class="caption">示例 13-11：在一個 `for` 迴圈中使用迭代器</span>

在標準庫中沒有提供迭代器的語言中，我們可能會使用一個從 0 開始索引的變數，使用這個變數索引 vector 中的值，並迴圈增加其值直到達到 vector 中的元素總量，以實現相同的功能。

迭代器為我們處理了所有這些邏輯，這減少了重複程式碼並消除了潛在的混亂。另外，迭代器的實現方式提供了對多種不同的序列使用相同邏輯的靈活性，而不僅僅是像 vector 這樣可索引的資料結構。讓我們看看迭代器是如何做到這些的。

### `Iterator` trait 和 `next` 方法

迭代器都實現了名為 `Iterator` 的定義於標準庫的 trait。這個 trait 的定義看起來像這樣：

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此處省略了方法的預設實現
}
```

注意這裡有一個我們還未講到的新語法：`type Item` 和 `Self::Item`，它們定義了 trait 的**關聯型別**（*associated type*）。第二十章會深入講解關聯型別，不過現在只需知道這段程式碼表明實現 `Iterator` trait 要求同時定義一個 `Item` 型別，這個 `Item` 型別被用作 `next` 方法的返回值型別。換句話說，`Item` 型別將是迭代器返回元素的型別。

`Iterator` trait 僅要求實現者定義一個方法：`next` 方法，該方法每次返回迭代器中的一個項，封裝在 `Some` 中，並且當迭代完成時，返回 `None`。

可以直接呼叫迭代器的 `next` 方法；示例 13-12 展示了對由 vector 建立的迭代器重複呼叫 `next` 方法時返回的值。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

<span class="caption">示例 13-12：在迭代器上（直接）呼叫 `next` 方法</span>

注意我們需要將 `v1_iter` 宣告為可變的：在迭代器上呼叫 `next` 方法會改變迭代器內部的狀態，該狀態用於跟蹤迭代器在序列中的位置。換句話說，程式碼**消費**（consume）了，或者說用盡了迭代器。每一次 `next` 呼叫都會從迭代器中消費一個項。使用 `for` 迴圈時無需使 `v1_iter` 可變因為 `for` 迴圈會獲取 `v1_iter` 的所有權並在後臺使 `v1_iter` 可變。

還需要注意的是，從 `next` 呼叫中獲取的值是對 vector 中值的不可變引用。`iter` 方法生成一個不可變引用的迭代器。如果我們需要一個獲取 `v1` 所有權並返回擁有所有權的迭代器，則可以呼叫 `into_iter` 而不是 `iter`。類似地，如果我們希望迭代可變引用，可以呼叫 `iter_mut` 而不是 `iter`。

### 消費迭代器的方法

`Iterator` trait 有一系列不同的由標準庫提供預設實現的方法；你可以在 `Iterator` trait 的標準庫 API 文件中找到所有這些方法。一些方法在其定義中呼叫了 `next` 方法，這也就是為什麼在實現 `Iterator` trait 時要求實現 `next` 方法的原因。

這些呼叫 `next` 方法的方法被稱為**消費介面卡**（*consuming adaptors*），因為呼叫它們會消耗迭代器。一個消費介面卡的例子是 `sum` 方法，這個方法獲取迭代器的所有權並反覆呼叫 `next` 來遍歷迭代器，從而消費迭代器。在遍歷過程中，它將每個項累加到一個執行時總和中，並在迭代完成時返回這個總和。示例 13-13 有一個展示 `sum` 方法使用的測試：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

<span class="caption">示例 13-13：呼叫 `sum` 方法獲取迭代器所有項的總和</span>

呼叫 `sum` 之後不再允許使用 `v1_iter` 因為呼叫 `sum` 時它會獲取迭代器的所有權。

### 產生其他迭代器的方法

`Iterator` trait 中定義了另一類方法，被稱為**迭代器介面卡**（*iterator adaptors*），它們不會消耗當前的迭代器，而是通過改變原始迭代器的某些方面來生成不同的迭代器。

示例 13-14 展示了一個呼叫迭代器介面卡方法 `map` 的例子，該方法使用一個閉包對每個元素進行操作。`map` 方法返回一個新的迭代器，該迭代器生成經過修改的元素。這裡的閉包建立了一個新的迭代器，其中 vector 中的每個元素都被加 1。

<span class="filename">檔名：src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<span class="caption">示例 13-14：呼叫迭代器介面卡 `map` 來建立一個新迭代器</span>

不過這些程式碼會產生一個警告：

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

示例 13-14 中的程式碼實際上並沒有做任何事；所指定的閉包從未被呼叫過。警告提醒了我們原因所在：迭代器介面卡是惰性的，因此我們需要在此處消費迭代器。

為了修復這個警告並消費迭代器，我們將使用第十二章示例 12-1 結合 `env::args` 使用的 `collect` 方法。這個方法消費迭代器並將結果收集到一個集合資料型別中。

在示例 13-15 中，我們將遍歷由 `map` 呼叫生成的迭代器結果收集到一個 vector 中。這個 vector 將包含原始 vector 中每個元素加 1 的結果。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

<span class="caption">示例 13-15：呼叫 `map` 方法建立一個新迭代器，接著呼叫 `collect` 方法消費新迭代器並建立一個 vector</span>

由於 `map` 接受一個閉包，因此我們可以指定希望在每個元素上執行的任何操作。這是一個很好的例子，展示瞭如何通過閉包來自定義某些行為，同時複用 `Iterator` trait 提供的迭代行為。

可以鏈式呼叫多個迭代器介面卡來以一種易讀的方式進行復雜的操作。不過因為所有的迭代器都是惰性的，你必須呼叫一個消費介面卡方法，才能從這些迭代器介面卡的呼叫中獲取結果。

<a id="using-closures-that-capture-their-environment"></a>

### 使用捕獲其環境的閉包

很多迭代器介面卡接受閉包作為引數，而我們通常會指定捕獲其環境的閉包作為迭代器介面卡的引數。

作為一個例子，我們使用 `filter` 方法來獲取一個閉包。該閉包從迭代器中獲取一項並返回一個 `bool`。如果閉包返回 `true`，其值將會包含在 `filter` 提供的新迭代器中。如果閉包返回 `false`，其值不會被包含。

示例 13-16 中使用 `filter` 和一個捕獲環境中變數 `shoe_size` 的閉包來遍歷一個 `Shoe` 結構體集合。它只會返回指定鞋碼的鞋子。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

<span class="caption">示例 13-16：使用 `filter` 方法和一個捕獲 `shoe_size` 的閉包</span>

`shoes_in_size` 函式獲取一個鞋子 vector 的所有權和一個鞋碼作為引數。它返回一個只包含指定鞋碼的鞋子的 vector。

`shoes_in_size` 函式體中呼叫了 `into_iter` 來建立一個獲取 vector 所有權的迭代器。接著呼叫 `filter` 將這個迭代器適配成一個只含有那些閉包返回 `true` 的元素的新迭代器。

閉包從環境中捕獲了 `shoe_size` 變數並使用其值與每一隻鞋的大小作比較，只保留指定鞋碼的鞋子。最終，呼叫 `collect` 將迭代器介面卡返回的值收集進一個 vector 並返回。

這個測試展示當呼叫 `shoes_in_size` 時，返回的只會是與我們指定的鞋碼相同的鞋子。
