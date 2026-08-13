## 結構體的定義和例項化

[ch05-01-defining-structs.md](https://github.com/rust-lang/book/blob/8a6130451b0817ead5c2522ce641dcb0f11a8571/src/ch05-01-defining-structs.md)

結構體和我們在[“元組型別”][tuples]部分討論過的元組類似，它們都包含多個相關的值。和元組一樣，結構體的每一部分可以是不同型別。但不同於元組，結構體需要命名各部分資料以便能清楚的表明其值的意義。由於有了這些名字，結構體比元組更靈活：不需要依賴順序來指定或訪問例項中的值。

定義結構體，需要使用 `struct` 關鍵字併為整個結構體提供一個名字。結構體的名字需要描述它所組合的資料的意義。接著，在大括號中，定義每一部分資料的名字和型別，我們稱為 **欄位**（*field*）。例如，示例 5-1 展示了一個儲存使用者賬號資訊的結構體：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">示例 5-1：`User` 結構體定義</span>

一旦定義了結構體後，為了使用它，透過為每個欄位指定具體值來建立這個結構體的**例項**。建立一個例項需要以結構體的名字開頭，接著在大括號中使用 `key: value` 鍵 - 值對的形式提供欄位，其中 key 是欄位的名字，value 是需要儲存在欄位中的資料值。例項中欄位的順序不需要和它們在結構體中宣告的順序一致。換句話說，結構體的定義就像一個型別的通用模板，而例項則會在這個模板中放入特定資料來建立這個型別的值。例如，可以像示例 5-2 這樣來宣告一個特定的使用者：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">示例 5-2：建立 `User` 結構體的例項</span>

為了從結構體中獲取某個特定的值，可以使用點號。舉個例子，想要使用者的郵箱地址，可以用 `user1.email`。如果結構體的例項是可變的，我們可以使用點號併為對應的欄位賦值。示例 5-3 展示瞭如何改變一個可變的 `User` 例項中 `email` 欄位的值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">示例 5-3：改變 `User` 例項 `email` 欄位的值</span>

注意整個例項必須是可變的；Rust 並不允許只將某個欄位標記為可變。另外需要注意同其他任何表示式一樣，我們可以在函式體的最後一個表示式中構造一個結構體的新例項，來隱式地返回這個例項。

示例 5-4 顯示了一個 `build_user` 函式，它返回一個帶有給定的 email 和使用者名稱的 `User` 結構體例項。`active` 欄位的值為 `true`，並且 `sign_in_count` 的值為 `1`。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">示例 5-4：`build_user` 函式獲取 email 和使用者名稱並返回 `User` 例項</span>

為函式引數使用與結構體欄位相同的名字是很合理的，但不得不重複寫 `email` 和 `username` 這些欄位名和變數名，多少有些囉嗦。如果結構體有更多欄位，重複每個名稱就更煩人了。幸運的是，有一種方便的簡寫語法！

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### 使用欄位初始化簡寫語法

因為示例 5-4 中的引數名與欄位名都完全相同，我們可以使用 **欄位初始化簡寫語法**（*field init shorthand*）來重寫 `build_user`，這樣其行為與之前完全相同，不過無需重複 `username` 和 `email` 了，如示例 5-5 所示。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">示例 5-5：`build_user` 函式使用了欄位初始化簡寫語法，因為 `username` 和 `email` 引數與結構體欄位同名</span>

這裡我們建立了一個新的 `User` 結構體例項，它有一個叫做 `email` 的欄位。我們想要將 `email` 欄位的值設定為 `build_user` 函式 `email` 引數的值。因為 `email` 欄位與 `email` 引數有著相同的名稱，則只需編寫 `email` 而不是 `email: email`。

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

### 使用結構體更新語法建立例項

使用舊例項的大部分值但改變其部分值來建立一個新的結構體例項通常是很有用的。這可以透過 **結構體更新語法**（*struct update syntax*）實現。

首先，示例 5-6 展示了不使用更新語法時，如何在 `user2` 中建立一個新 `User` 例項。我們為 `email` 設定了新的值，其他值則使用了例項 5-2 中建立的 `user1` 中的同名值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">示例 5-6：使用 `user1` 中的一個值建立一個新的 `User` 例項</span>

使用結構體更新語法，我們可以透過更少的程式碼來達到相同的效果，如示例 5-7 所示。`..` 語法指定了剩餘未顯式設定值的欄位應有與給定例項對應欄位相同的值。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">示例 5-7：使用結構體更新語法為一個 `User` 例項設定一個新的 `email` 值，不過其餘值來自 `user1` 變數中例項的欄位</span>

示例 5-7 中的程式碼也在 `user2` 中建立了一個新例項，但該例項中 `email` 欄位的值與 `user1` 不同，而 `username`、 `active` 和 `sign_in_count` 欄位的值與 `user1` 相同。`..user1` 必須放在最後，以指定其餘的欄位應從 `user1` 的相應欄位中獲取其值，但我們可以選擇以任何順序為任意欄位指定值，而不用考慮結構體定義中欄位的順序。

請注意，結構更新語法就像帶有 `=` 的賦值，因為它移動了資料，就像我們在[“使用移動的變數與資料互動”][move]部分講到的一樣。在這個例子中，總體上說我們在建立 `user2` 後就不能再使用 `user1` 了，因為 `user1` 的 `username` 欄位中的 `String` 被移到 `user2` 中。如果我們給 `user2` 的 `email` 和 `username` 都賦予新的 `String` 值，從而只複用 `user1` 的 `active` 和 `sign_in_count` 值，那麼 `user1` 在建立 `user2` 後仍然有效。`active` 和 `sign_in_count` 的型別是實現 `Copy` trait 的型別，所以我們在[“只在棧上的資料：複製”][copy] 部分討論的行為同樣適用。在本例中我們也可以繼續使用 `user1.email`，因為它的值並未從 `user1` 中移動出去。

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

### 使用元組結構體建立不同的型別

也可以定義與元組類似的結構體，稱為 **元組結構體**（*tuple structs*）。元組結構體有著結構體名稱提供的含義，但沒有具體的欄位名，只有欄位的型別。當你想給整個元組取一個名字，並使元組成為與其他元組不同的型別時，元組結構體是很有用的，這時像常規結構體那樣為每個欄位命名就顯得多餘和形式化了。

要定義元組結構體，以 `struct` 關鍵字和結構體名開頭並後跟元組中的型別。例如，下面是兩個分別叫做 `Color` 和 `Point` 元組結構體的定義和用法：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

注意 `black` 和 `origin` 值的型別不同，因為它們是不同的元組結構體的例項。你定義的每一個結構體有其自己的型別，即使結構體中的欄位可能有著相同的型別。例如，一個獲取 `Color` 型別引數的函式不能接受 `Point` 作為引數，即便這兩個型別都由三個 `i32` 值組成。除此之外，元組結構體例項類似於元組，你可以將它們解構為單獨的部分，也可以使用 `.` 後跟索引來訪問單獨的值。與元組不同的是，解構元組結構體時必須寫明結構體的型別。例如，我們可以寫 `let Point(x, y, z) = origin;`，將 `origin` 的值解構到名為 `x`、`y` 和 `z` 的變數中。

<a id="unit-like-structs-without-any-fields"></a>

### 定義類單元結構體

你也可以定義沒有任何欄位的結構體！它們被稱為 **類單元結構體**（*unit-like structs*），因為它們的行為類似於 `()`，也就是我們在[“元組型別”][tuples]一節中提到的 unit 型別。類單元結構體在你想要在某個型別上實現 trait，但又不需要在該型別本身中儲存任何資料時會很有用。我們會在第十章討論 trait。下面是一個宣告和例項化名為 `AlwaysEqual` 的 unit 結構體的例子：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

為了定義 `AlwaysEqual`，我們使用 `struct` 關鍵字，接著是我們想要的名稱，然後是一個分號。不需要花括號或圓括號！然後，我們可以以類似的方式在 `subject` 變數中建立 `AlwaysEqual` 的例項：只需使用我們定義的名稱，無需任何花括號或圓括號。設想我們稍後將為這個型別實現某種行為，使得每個 `AlwaysEqual` 的例項始終等於任何其它型別的例項，也許是為了獲得一個已知的結果以便進行測試。我們不需要任何資料來實現這種行為！在第十章中，你會看到如何定義 trait 並在任何型別上實現它們，包括類單元結構體。

> ### 結構體資料的所有權
>
> 在示例 5-1 中的 `User` 結構體的定義中，我們使用了自身擁有所有權的 `String` 型別而不是 `&str` 字串 slice 型別。這是一個有意而為之的選擇，因為我們想要這個結構體擁有它所有的資料，為此只要整個結構體是有效的話其資料也是有效的。
>
> 可以使結構體儲存被其他物件擁有的資料的引用，不過這麼做的話需要用上 **生命週期**（*lifetimes*），這是一個第十章會討論的 Rust 特性。生命週期確保結構體引用的資料有效性跟結構體本身保持一致。如果你嘗試在結構體中儲存一個引用而不指定生命週期將是無效的，比如這樣：
>
> <span class="filename">檔名：src/main.rs</span>
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> 編譯器會抱怨它需要生命週期識別符號：
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> 第十章會討論如何修復這些錯誤，從而讓你可以在結構體中儲存引用；不過現在，我們會用像 `String` 這樣的擁有所有權的型別來替代像 `&str` 這樣的引用，以此修復這類錯誤。

[tuples]: ch03-02-data-types.html#元組型別
[move]: ch04-01-what-is-ownership.html#使用移動的變數與資料互動
[copy]: ch04-01-what-is-ownership.html#只在棧上的資料複製
