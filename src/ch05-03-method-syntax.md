## 方法

[ch05-03-method-syntax.md](https://github.com/rust-lang/book/blob/8a6130451b0817ead5c2522ce641dcb0f11a8571/src/ch05-03-method-syntax.md)

**方法**（method）與函式類似：它們使用 `fn` 關鍵字和名稱宣告，可以擁有引數和返回值，同時包含在某處呼叫該方法時會執行的程式碼。不過方法與函式是不同的，因為它們在結構體的上下文中被定義（或者是列舉或 trait 物件的上下文，將分別在[第六章][enums]和[第十八章][trait-objects]講解），並且它們第一個引數總是 `self`，它代表呼叫該方法的結構體例項。

<a id="defining-methods"></a>

### 方法語法

讓我們把前面實現的獲取一個 `Rectangle` 例項作為引數的 `area` 函式，改寫成一個定義於 `Rectangle` 結構體上的 `area` 方法，如示例 5-13 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">示例 5-13：在 `Rectangle` 結構體上定義 `area` 方法</span>

為了使函式定義於 `Rectangle` 的上下文中，我們開始了一個 `impl` 塊（`impl` 是 *implementation* 的縮寫），這個 `impl` 塊中的所有內容都將與 `Rectangle` 型別相關聯。接著將 `area` 函式移動到 `impl` 大括號中，並將簽名中的第一個（在這裡也是唯一一個）引數和函式體中其他地方的對應引數改成 `self`。然後在 `main` 中將我們先前呼叫 `area` 方法並傳遞 `rect1` 作為引數的地方，改成使用 **方法語法**（*method syntax*）在 `Rectangle` 例項上呼叫 `area` 方法。方法語法獲取一個例項並加上一個點號，後跟方法名、圓括號以及任何引數。

在 `area` 的簽名中，使用 `&self` 來替代 `rectangle: &Rectangle`，`&self` 實際上是 `self: &Self` 的縮寫。在一個 `impl` 塊中，`Self` 型別是 `impl` 塊的型別的別名。方法的第一個引數必須有一個名為 `self` 的`Self` 型別的引數，所以 Rust 讓你在第一個引數位置上只用 `self` 這個名字來簡化。注意，我們仍然需要在 `self` 前面使用 `&` 來表示這個方法借用了 `Self` 例項，就像我們在 `rectangle: &Rectangle` 中做的那樣。方法可以選擇獲得 `self` 的所有權，或者像我們這裡一樣不可變地借用 `self`，或者可變地借用 `self`，就跟其他引數一樣。

這裡選擇 `&self` 的理由跟在函式版本中使用 `&Rectangle` 是相同的：我們並不想獲取所有權，只希望能夠讀取結構體中的資料，而不是寫入。如果想要在方法中改變呼叫方法的例項，需要將第一個引數改為 `&mut self`。透過僅僅使用 `self` 作為第一個引數來使方法獲取例項的所有權是很少見的；這種技術通常用在當方法將 `self` 轉換成別的例項的時候，這時我們想要防止呼叫者在轉換之後使用原始的例項。

使用方法替代函式，除了可使用方法語法和不需要在每個函式簽名中重複 `self` 的型別之外，其主要好處在於組織性。我們將某個型別例項能做的所有事情都一起放入 `impl` 塊中，而不是讓將來的使用者在我們的庫中到處尋找 `Rectangle` 的功能。

請注意，我們可以選擇讓方法與結構體中的某個欄位同名。例如，我們可以在 `Rectangle` 上定義一個也叫做 `width` 的方法：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

在這裡，我們選擇讓 `width` 方法在例項的 `width` 欄位的值大於 `0` 時返回 `true`，等於 `0` 時則返回 `false`：我們可以出於任何目的，在同名的方法中使用同名的欄位。在 `main` 中，當我們在 `rect1.width` 後面加上括號時。Rust 知道我們指的是方法 `width`。當我們不使用圓括號時，Rust 知道我們指的是欄位 `width`。

通常情況下，雖然並不總是如此，與欄位同名的方法會被定義為只返回該欄位中的值，而不做其他事情。這樣的方法被稱為 *getters*。Rust 不會像其他一些語言那樣為結構體欄位自動生成 getter。Getter 很有用，因為你可以把欄位設為私有，而把方法設為公有，從而將對該欄位的只讀訪問作為該型別公有 API 的一部分。我們會在[第七章][public]討論什麼是公有和私有，以及如何把欄位或方法指定為公有或私有。

> ### `->` 運算子到哪去了？
>
> 在 C/C++ 語言中，有兩個不同的運算子來呼叫方法：`.` 直接在物件上呼叫方法，而 `->` 在一個物件的指標上呼叫方法，這時需要先解引用（dereference）指標。換句話說，如果 `object` 是一個指標，那麼 `object->something()` 就像 `(*object).something()` 一樣。
>
> Rust 並沒有一個與 `->` 等效的運算子；相反，Rust 有一個叫 **自動引用和解引用**（*automatic referencing and dereferencing*）的功能。方法呼叫是 Rust 中少數幾個擁有這種行為的地方。
>
> 它是這樣工作的：當使用 `object.something()` 呼叫方法時，Rust 會自動為 `object` 新增 `&`、`&mut` 或 `*` 以便使 `object` 與方法簽名匹配。也就是說，這些程式碼是等價的：
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 第一種寫法看起來簡潔得多。這種自動引用之所以成立，是因為方法有一個明確的接收者，也就是 `self` 的型別。給定接收者和方法名，Rust 就能明確推斷出該方法是隻讀（`&self`）、可變借用（`&mut self`）還是獲取所有權（`self`）。事實上，Rust 對方法接收者的這種隱式借用，是所有權機制在實踐中更易用的重要原因之一。

### 帶有更多引數的方法

讓我們透過實現 `Rectangle` 結構體上的另一方法來練習使用方法。這回，我們讓一個 `Rectangle` 的例項獲取另一個 `Rectangle` 例項，如果 `self` （第一個 `Rectangle`）能完全包含第二個長方形則返回 `true`；否則返回 `false`。一旦我們定義了 `can_hold` 方法，就可以編寫示例 5-14 中的程式碼。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">示例 5-14：使用還未實現的 `can_hold` 方法</span>

同時我們希望看到如下輸出，因為 `rect2` 的兩個維度都小於 `rect1`，而 `rect3` 比 `rect1` 要寬：

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

因為我們想定義一個方法，所以它應該位於 `impl Rectangle` 塊中。方法名是 `can_hold`，並且它會獲取另一個 `Rectangle` 的不可變借用作為引數。透過觀察呼叫方法的程式碼可以看出引數是什麼型別的：`rect1.can_hold(&rect2)` 傳入了 `&rect2`，它是一個 `Rectangle` 的例項 `rect2` 的不可變借用。這是可以理解的，因為我們只需要讀取 `rect2`（而不是寫入，這意味著我們需要一個不可變借用），而且希望 `main` 保持 `rect2` 的所有權，這樣就可以在呼叫這個方法後繼續使用它。`can_hold` 的返回值是一個布林值，其實現會分別檢查 `self` 的寬高是否都大於另一個 `Rectangle`。讓我們在示例 5-13 的 `impl` 塊中增加這個新的 `can_hold` 方法，如示例 5-15 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">示例 5-15：在 `Rectangle` 上實現 `can_hold` 方法，它獲取另一個 `Rectangle` 例項作為引數</span>

如果結合示例 5-14 的 `main` 函式來執行，就會看到期望的輸出。在方法簽名中，可以在 `self` 後增加多個引數，而且這些引數就像函式中的引數一樣工作。

### 關聯函式

所有在 `impl` 塊中定義的函式被稱為 **關聯函式**（*associated functions*），因為它們與 `impl` 後面命名的型別相關。我們可以定義不以 `self` 為第一引數的關聯函式（因此不是方法），因為它們並不作用於一個結構體的例項。我們已經使用了一個這樣的函式：在 `String` 型別上定義的 `String::from` 函式。

不是方法的關聯函式經常被用作返回一個結構體新例項的建構函式。這些函式的名稱通常為 `new` ，但 `new` 並不是一個關鍵字。例如我們可以提供一個叫做 `square` 關聯函式，它接受一個維度引數並且同時作為寬和高，這樣可以更輕鬆的建立一個正方形 `Rectangle` 而不必指定兩次同樣的值：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

關鍵字 `Self` 在函式的返回型別和函式體中，都是對 `impl` 關鍵字後所示型別的別名，這裡是 `Rectangle`。

要呼叫這個關聯函式，我們使用結構體名和 `::` 語法；比如 `let sq = Rectangle::square(3);`。這個函式位於結構體的名稱空間中：`::` 語法用於關聯函式和模組建立的名稱空間。[第七章][modules]會講到模組。

### 多個 `impl` 塊

每個結構體都允許擁有多個 `impl` 塊。例如，示例 5-15 中的程式碼等同於示例 5-16 中所示的程式碼，但後者每個方法有其自己的 `impl` 塊。

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">示例 5-16：使用多個 `impl` 塊重寫示例 5-15</span>

這裡沒有理由將這些方法分散在多個 `impl` 塊中，不過這是有效的語法。第十章討論泛型和 trait 時會看到實用的多 `impl` 塊的用例。

## 總結

結構體讓你可以創建出在你的領域中有意義的自定義型別。透過結構體，我們可以將相關聯的資料片段聯絡起來並命名它們，這樣可以使得程式碼更加清晰。在 `impl` 塊中，你可以定義與你的型別相關聯的函式，而方法是一種相關聯的函式，讓你指定結構體的例項所具有的行為。

但結構體並不是建立自定義型別的唯一方法：讓我們轉向 Rust 的列舉功能，為你的工具箱再添一個工具。

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.html
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#使用-pub-關鍵字暴露路徑
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
