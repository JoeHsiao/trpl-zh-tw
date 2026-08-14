## 使用 Hash Map 儲存鍵值對

[ch08-03-hash-maps.md](https://github.com/rust-lang/book/blob/2581c23b669eff30c26e036a13475ec5cf70c1b8/src/ch08-03-hash-maps.md)

最後介紹的常用集合型別是**雜湊 map**（*hash map*）。`HashMap<K, V>` 型別儲存了一個鍵型別 `K` 對應一個值型別 `V` 的對映。它通過一個**雜湊函式**（*hashing function*）來實現對映，決定如何將鍵和值放入記憶體中。很多程式語言支援這種資料結構，不過通常有不同的名字：**雜湊**、**map**、**物件**、**雜湊表**、**字典**或者**關聯陣列**，僅舉幾例。

雜湊 map 可以用於需要任何型別作為鍵來尋找資料的情況，而不是像 vector 那樣通過索引。例如，在一個遊戲中，你可以將每個團隊的分數記錄到雜湊 map 中，其中鍵是隊伍的名字而值是每個隊伍的分數。給出一個隊名，就能檢索到該隊的得分。

本章我們會介紹雜湊 map 的基本 API，不過還有更多吸引人的功能隱藏於標準庫在 `HashMap<K, V>` 上定義的函式中。一如既往請檢視標準庫文件來了解更多資訊。

### 新建一個雜湊 map

可以使用 `new` 建立一個空的 `HashMap`，並使用 `insert` 增加元素。在示例 8-20 中我們記錄兩支隊伍的分數，分別是**藍隊**和**黃隊**。藍隊開始有 10 分而黃隊開始有 50 分：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">示例 8-20：新建一個雜湊 map 並插入一些鍵值對</span>

注意必須首先 `use` 標準庫中集合部分的 `HashMap`。在這三個常用集合中，`HashMap` 是最不常用的，所以並沒有被 prelude 自動引用。標準庫中對 `HashMap` 的支援也相對較少，例如，並沒有內建的構建宏。

像 vector 一樣，雜湊 map 將它們的資料儲存在堆上，這個 `HashMap` 的鍵型別是 `String` 而值型別是 `i32`。類似於 vector，雜湊 map 是同質的：所有的鍵必須是相同型別，值也必須都是相同型別。

### 訪問雜湊 map 中的值

可以通過 `get` 方法並提供對應的鍵來從雜湊 map 中獲取值，如示例 8-21 所示：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">示例 8-21：訪問雜湊 map 中儲存的藍隊分數</span>

這裡，`score` 是與藍隊分數相關的值，應為 `10`。`get` 方法返回 `Option<&V>`，如果某個鍵在雜湊 map 中沒有對應的值，`get` 會返回 `None`。程式中通過呼叫 `copied` 方法來獲取一個 `Option<i32>` 而不是 `Option<&i32>`，接著呼叫 `unwrap_or` 在  `scores` 中沒有該鍵所對應的項時將其設定為零。

可以使用與 vector 類似的方式來遍歷雜湊 map 中的每一個鍵值對，也就是 `for` 迴圈：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

這會以任意順序打印出每一個鍵值對：

```text
Yellow: 50
Blue: 10
```

<a id="hash-maps-and-ownership"></a>

### 在雜湊 map 中管理所有權

對於像 `i32` 這樣的實現了 `Copy` trait 的型別，其值可以複製進雜湊 map。對於像 `String` 這樣擁有所有權的值，其值將被移動而雜湊 map 會成為這些值的所有者，如示例 8-22 所示：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">示例 8-22：展示一旦鍵值對被插入後就為雜湊 map 所擁有</span>

當 `insert` 呼叫將 `field_name` 和 `field_value` 移動到雜湊 map 中後，將不能使用這兩個繫結。

如果我們把對值的引用插入雜湊 map，這些值本身並不會被移動進雜湊 map。引用所指向的值必須至少在雜湊 map 有效的那段時間裡一直有效。第十章的[“生命週期確保引用有效”][validating-references-with-lifetimes]部分會更詳細地討論這個問題。

### 更新雜湊 map

儘管鍵值對的數量是可以增長的，每個唯一的鍵只能同時關聯一個值（反之不一定成立：比如藍隊和黃隊的 `scores` 雜湊 map 中都可能儲存有 10 這個值）。

當我們想要改變雜湊 map 中的資料時，必須決定如何處理一個鍵已經有值了的情況。可以選擇完全無視舊值並用新值代替舊值。可以選擇保留舊值而忽略新值，並只在鍵**沒有**對應值時增加新值。或者可以結合新舊兩值。讓我們看看這分別該如何實現！

#### 覆蓋一個值

如果我們插入了一個鍵值對，接著用相同的鍵插入一個不同的值，與這個鍵相關聯的舊值將被替換。即便示例 8-23 中的程式碼呼叫了兩次 `insert`，雜湊 map 也只會包含一個鍵值對，因為兩次都是對藍隊的鍵插入的值：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">示例 8-23：替換以特定鍵儲存的值</span>

這會打印出 `{"Blue": 25}`。原始的值 `10` 則被覆蓋了。

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### 只在鍵尚不存在時插入鍵值對

我們經常會檢查某個特定的鍵是否已經在雜湊 map 中有對應的值，然後執行如下操作：如果這個鍵已經存在，就讓原來的值保持不變；如果這個鍵不存在，就插入它和它對應的值。

Hash map 為這種場景提供了一個特殊的 API，叫做 `entry`，它接收你想檢查的鍵作為引數。`entry` 方法的返回值是一個名為 `Entry` 的列舉，它表示一個可能存在、也可能不存在的值。假設我們想檢查黃隊這個鍵是否已經有關聯的值。如果沒有，就插入值 `50`；藍隊也是同樣的處理方式。使用 `entry` API 的程式碼如示例 8-24 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">示例 8-24：使用 `entry` 方法只在鍵沒有對應一個值時插入</span>

`Entry` 上的 `or_insert` 方法被定義為：如果對應 `Entry` 的鍵已經存在，就返回該值的可變引用；如果不存在，就把引數作為這個鍵的新值插入，並返回這個新值的可變引用。這比我們自己手寫邏輯要清晰得多，而且和借用檢查器的配合也更好。

執行示例 8-24 的程式碼會打印出 `{"Yellow": 50, "Blue": 10}`。第一個 `entry` 呼叫會插入黃隊的鍵和值 `50`，因為黃隊並沒有一個值。第二個 `entry` 呼叫不會改變雜湊 map 因為藍隊已經有了值 `10`。

#### 根據舊值更新一個值

另一個常見的雜湊 map 的應用場景是找到一個鍵對應的值並根據舊的值更新它。例如，示例 8-25 中的程式碼計數一些文本中每一個單詞分別出現了多少次。我們使用雜湊 map 以單詞作為鍵並遞增其值來記錄我們遇到過幾次這個單詞。如果是第一次看到某個單詞，就先插入值 `0`。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">示例 8-25：通過雜湊 map 儲存單詞和計數來統計出現次數</span>

這會打印出 `{"world": 2, "hello": 1, "wonderful": 1}`。你可能會看到相同的鍵值對以不同的順序列印：回憶一下[“訪問雜湊 map 中的值”][access]部分中提到遍歷雜湊 map 會以任意順序進行。

`split_whitespace` 方法返回一個由空格分隔 `text` 值子 slice 的迭代器。`or_insert` 方法返回這個鍵的值的一個可變引用（`&mut V`）。這裡我們將這個可變引用儲存在 `count` 變數中，所以為了賦值必須首先使用星號（`*`）解引用 `count`。這個可變引用在 `for` 迴圈的結尾離開作用域，這樣所有這些改變都是安全的並符合借用規則。

### 雜湊函式

`HashMap` 預設使用一種叫做 SipHash 的雜湊函式，它可以提供對涉及雜湊表[^siphash]的拒絕服務（Denial of Service, DoS）攻擊的抵抗能力。不過這不是目前可用的最快雜湊演算法，但為了更好的安全性而接受一些效能下降，是值得的權衡。如果你分析程式碼後發現預設雜湊函式對你的用途來說太慢，就可以通過指定不同的 hasher 來切換到其他函式。*hasher* 是一種實現了 `BuildHasher` trait 的型別。[第十章][traits]會討論 trait 以及如何實現它們。你不一定要從零開始自己實現 hasher；[crates.io](https://crates.io/) 上有其他 Rust 使用者共享的庫，它們提供了許多常見雜湊演算法的 hasher 實現。

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## 總結

vector、字串和雜湊 map 會在你的程式需要儲存、訪問和修改資料時幫助你。這裡有一些你應該能夠解決的練習問題：

1. 給定一組整數，使用 vector 並返回這個列表的中位數（排列陣列後位於中間的值）和眾數（出現次數最多的值；在這裡雜湊 map 會很有幫助）。
2. 將字串轉換為 pig latin。也就是每一個單詞的第一個子音字母被移動到單詞的結尾並增加 *ay*，所以 *first* 會變成 *irst-fay*。母音字母開頭的單詞則在結尾增加 *hay*（*apple* 會變成 *apple-hay*）。請注意 UTF-8 編碼的細節！
3. 使用雜湊 map 和 vector，建立一個文本介面來允許使用者向公司的部門中增加員工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接著讓使用者獲取一個部門的所有員工的列表，或者公司每個部門的所有員工按照字典序排列的列表。

標準庫 API 文件中描述的這些型別的方法將有助於你進行這些練習！

我們已經開始接觸可能會有失敗操作的複雜程式了，這也意味著接下來是一個瞭解錯誤處理的絕佳時機！接下來我們將討論這一部分！

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#生命週期確保引用有效
[access]: #訪問雜湊-map-中的值
[traits]: ch10-02-traits.html
