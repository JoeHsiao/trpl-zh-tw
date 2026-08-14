## 實現一個面向物件設計模式

[ch18-03-oo-design-patterns.md](https://github.com/rust-lang/book/blob/eb54c18184c0555acd9f636f5ef7875236b0ff53/src/ch18-03-oo-design-patterns.md)

**狀態模式**（*state pattern*）是一個面向物件設計模式。該模式的關鍵在於定義值的一系列內含狀態。這些狀態體現為一系列的**狀態物件**（_state objects_），同時值的行為隨著其內部狀態而改變。我們將編寫一個部落格釋出結構體的例子，它擁有一個包含其狀態的欄位，該欄位可以是 "draft"、"review" 或 "published" 狀態物件之一。

狀態物件共享功能：當然，在 Rust 中使用結構體和 trait 而不是物件和繼承。每一個狀態物件負責其自身的行為，以及該狀態何時應當轉移至另一個狀態。持有一個狀態物件的值對於不同狀態的行為以及何時狀態轉移毫不知情。

使用狀態模式的優點在於，程式的業務需求改變時，無需改變值持有狀態或者使用值的程式碼。我們只需更新某個狀態物件中的程式碼來改變其規則，或者是增加更多的狀態物件。

首先我們將以一種更加傳統的面向物件的方式實現狀態模式，接著使用一種在 Rust 中更自然的方式。讓我們使用狀態模式來增量式地實現一個釋出博文的工作流以探索這個概念。

最終功能看起來像這樣：

1. 博文從空白的草稿開始。
2. 一旦草稿完成，請求稽核博文。
3. 一旦博文過審，它將被髮表。
4. 只有被髮表的博文的內容會被列印，這樣就不會意外打印出沒有被稽核的博文的文字。

任何其他對博文的修改嘗試都不會生效。例如，如果嘗試在請求稽核之前透過一個草稿博文，博文應該保持未釋出的狀態。

### 嘗試採用傳統的面向物件風格

實現同一個問題的程式碼組織方式有無窮多種，而且各自都有不同取捨。本節的實現更偏向一種傳統的面向物件風格：在 Rust 裡當然可以這樣寫，但它並沒有利用 Rust 的某些優勢。稍後我們會展示另一種仍然使用面向物件設計模式、但結構上對有面向物件經驗的程式設計師來說可能不那麼熟悉的做法。透過比較這兩種方案，我們就能體會到：以不同於其他語言的方式來設計 Rust 程式碼，會帶來怎樣的權衡。

示例 18-11 展示這個工作流的程式碼形式：這是一個我們將要在一個叫做 `blog` 的庫 crate 中實現的 API 的示例。這段程式碼還不能編譯，因為還未實現 `blog` crate。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

<span class="caption">示例 18-11: 展示了 `blog` crate 期望行為的程式碼</span>

我們希望允許使用者使用 `Post::new` 建立一個新的博文草稿。也希望能在草稿階段為博文編寫一些文字。如果在審批之前嘗試立刻獲取博文的內容，不應該獲取到任何文字因為博文仍然是草稿。出於演示目的我們在程式碼中添加了 `assert_eq!`。一個好的單元測試將是斷言草稿博文的 `content` 方法返回空字串，不過我們並不準備為這個例子編寫單元測試。

接下來，我們希望能夠請求稽核博文，而在等待稽核的階段 `content` 應該仍然返回空字串。最後當博文稽核透過，它應該被髮表，這意味著當呼叫 `content` 時博文的文字將被返回。

注意我們與 crate 互動的唯一的型別是 `Post`。這個型別會使用狀態模式並會存放處於三種博文所可能的狀態之一的值 —— 草稿，稽核和釋出。狀態上的改變由 `Post` 型別內部進行管理。狀態依庫使用者對 `Post` 例項呼叫的方法而改變，但是不能直接管理狀態變化。這也意味著使用者不會在狀態上犯錯，比如在過審前釋出博文。

#### 定義 `Post` 並建立一個新例項

讓我們開始實現這個庫吧！我們知道需要一個公有 `Post` 結構體來存放一些文字，所以讓我們從結構體的定義和一個建立 `Post` 例項的公有關聯函式 `new` 開始，如示例 18-12 所示。還需定義一個私有 trait `State` 用於定義 `Post` 的狀態物件所必須有的行為。

`Post` 將在私有欄位 `state` 中存放一個 `Option<T>` 型別的 trait 物件 `Box<dyn State>`。稍後將會看到為何 `Option<T>` 是必須的。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

<span class="caption">示例 18-12: `Post` 結構體的定義和新建 `Post` 例項的 `new` 函式，`State` trait 和結構體 `Draft`</span>

`State` trait 定義了所有不同狀態的博文所共享的行為，這個狀態物件是 `Draft`、`PendingReview` 和 `Published`，它們都會實現 `State` trait。現在這個 trait 並沒有任何方法，同時開始將只定義 `Draft` 狀態因為這是我們希望博文的初始狀態。

當建立新的 `Post` 時，我們將其 `state` 欄位設定為一個存放了 `Box` 的 `Some` 值。這個 `Box` 指向一個 `Draft` 結構體新例項。這確保了無論何時新建一個 `Post` 例項，它都會從草稿開始。因為 `Post` 的 `state` 欄位是私有的，也就無法建立任何其他狀態的 `Post` 了！`Post::new` 函式中將 `content` 設定為新建的空 `String`。

#### 儲存文章內容的文字

在示例 18-11 中，展示了我們希望能夠呼叫一個叫做 `add_text` 的方法並向其傳遞一個 `&str` 來將文字增加到博文的內容中。選擇實現為一個方法而不是將 `content` 欄位暴露為 `pub` 。這意味著之後可以實現一個方法來控制 `content` 欄位如何被讀取。`add_text` 方法是非常直觀的，讓我們在示例 18-13 的 `impl Post` 塊中增加一個實現：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

<span class="caption">示例 18-13: 實現方法 `add_text` 來向博文的 `content` 增加文字</span>

`add_text` 獲取一個 `self` 的可變引用，因為需要改變呼叫 `add_text` 的 `Post` 例項。接著呼叫 `content` 中的 `String` 的 `push_str` 並傳遞 `text` 引數來將其追加到已儲存的 `content` 中。這不是狀態模式的一部分，因為它的行為並不依賴博文所處的狀態。`add_text` 方法完全不與 `state` 欄位互動，不過這是我們希望支援的行為的一部分。

#### 確保草稿文章的內容為空

即使呼叫 `add_text` 並向博文增加一些內容之後，我們仍然希望 `content` 方法返回一個空字串 slice，因為博文仍然處於草稿狀態，如示例 18-11 的第 7 行所示。現在讓我們使用能滿足要求的最簡單的方式來實現 `content` 方法：總是返回一個空字串 slice。當實現了將博文狀態改為釋出的能力之後將改變這一做法。但是目前博文只能是草稿狀態，這意味著其內容應該總是空的。示例 18-14 展示了這個佔位符實現。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

<span class="caption">示例 18-14: 增加一個 `Post` 的 `content` 方法的佔位實現，它總是返回一個空字串 slice</span>

透過增加這個 `content` 方法，示例 18-11 中直到第 7 行的程式碼能如期執行。

#### 請求稽核，從而改變文章狀態

接下來需要增加請求稽核博文的功能，這應當將其狀態由 `Draft` 改為 `PendingReview`。示例 18-15 展示了這個程式碼：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

<span class="caption">示例 18-15: 實現 `Post` 和 `State` trait 的 `request_review` 方法</span>

這裡為 `Post` 增加一個獲取 `self` 可變引用的公有方法 `request_review`。接著在 `Post` 的當前狀態下呼叫內部的 `request_review` 方法，並且第二個 `request_review` 方法會消費當前的狀態並返回一個新狀態。

這裡給 `State` trait 增加了 `request_review` 方法；所有實現了這個 trait 的型別現在都需要實現 `request_review` 方法。注意不同於使用 `self`、 `&self` 或者 `&mut self` 作為方法的第一個引數，這裡使用了 `self: Box<Self>`。這個語法意味著該方法只可在持有這個型別的 `Box` 上被呼叫。這個語法獲取了 `Box<Self>` 的所有權使老狀態無效化，以便 `Post` 的狀態值可轉換為一個新狀態。

為了消費老狀態，`request_review` 方法需要獲取狀態值的所有權。這就是 `Post` 的 `state` 欄位中 `Option` 的來歷：呼叫 `take` 方法將 `state` 欄位中的 `Some` 值取出並留下一個 `None`，因為 Rust 不允許結構體例項中存在未初始化的欄位。這使得我們將 `state` 的值移出 `Post` 而不是借用它。接著我們將博文的 `state` 值設定為這個操作的結果。

我們需要將 `state` 臨時設定為 `None` 來獲取 `state` 值，即老狀態的所有權，而不是使用 `self.state = self.state.request_review();` 這樣的程式碼直接更新狀態值。這確保了當 `Post` 被轉換為新狀態後不能再使用老 `state` 值。

`Draft` 的 `request_review` 方法需要返回一個新的，裝箱的 `PendingReview` 結構體的例項，其用來代表博文處於等待稽核狀態。結構體 `PendingReview` 同樣也實現了 `request_review` 方法，不過它不進行任何狀態轉換。相反它返回自身，因為當我們請求稽核一個已經處於 `PendingReview` 狀態的博文，它應該繼續保持 `PendingReview` 狀態。

現在我們能看出狀態模式的優勢了：無論 `state` 是何值，`Post` 的 `request_review` 方法都是一樣的。每個狀態只負責它自己的規則。

我們將繼續保持 `Post` 的 `content` 方法實現不變，返回一個空字串 slice。現在我們可以擁有 `PendingReview` 狀態和 `Draft` 狀態的 `Post` 了，不過我們希望在 `PendingReview` 狀態下 `Post` 也有相同的行為。現在示例 18-11 中直到 10 行的程式碼是可以執行的！

#### 新增 `approve` 來改變 `content` 的行為

`approve` 方法將與 `request_review` 方法類似：它會將 `state` 設定為稽核透過時應處於的狀態，如示例 18-16 所示。

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

<span class="caption">示例 18-16: 為 `Post` 和 `State` trait 實現 `approve` 方法</span>

這裡為 `State` trait 增加了 `approve` 方法，並新增了一個實現了 `State` 的結構體，`Published` 狀態。

類似於 `PendingReview` 中 `request_review` 的工作方式，如果對 `Draft` 呼叫 `approve` 方法，並沒有任何效果，因為它會返回 `self`。當對 `PendingReview` 呼叫 `approve` 時，它返回一個新的、裝箱的 `Published` 結構體的例項。`Published` 結構體實現了 `State` trait，同時對於 `request_review` 和 `approve` 兩方法來說，它返回自身，因為在這兩種情況博文應該保持 `Published` 狀態。

現在需要更新 `Post` 的 `content` 方法。我們希望 `content` 根據 `Post` 的當前狀態返回值，所以需要 `Post` 代理一個定義於 `state` 上的 `content` 方法，如示例 18-17 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

<span class="caption">示例 18-17: 更新 `Post` 的 `content` 方法來委託呼叫 `State` 的 `content` 方法</span>

因為目標是將所有像這樣的規則保持在實現了 `State` 的結構體中，我們將呼叫 `state` 中的值的 `content` 方法並傳遞博文例項（也就是 `self`）作為引數。接著返回 `state` 值的 `content` 方法的返回值。

這裡呼叫 `Option` 的 `as_ref` 方法是因為需要 `Option` 中值的引用而不是獲取其所有權。因為 `state` 是一個 `Option<Box<dyn State>>`，呼叫 `as_ref` 會返回一個 `Option<&Box<dyn State>>`。如果不呼叫 `as_ref`，將會得到一個錯誤，因為不能將 `state` 移動出借用的 `&self` 函式引數。

接著呼叫 `unwrap` 方法，這裡我們知道它永遠也不會 panic，因為 `Post` 的所有方法都確保在它們返回時 `state` 會有一個 `Some` 值。這就是一個第十二章 [“當你比編譯器知道更多時”][more-info-than-rustc] 部分討論過的我們知道 `None` 是不可能的而編譯器卻不能理解的情況之一。

接著我們就有了一個 `&Box<dyn State>`，當呼叫其 `content` 時，解引用強制轉換會作用於 `&` 和 `Box` ，這樣最終會呼叫實現了 `State` trait 的型別的 `content` 方法。這意味著需要為 `State` trait 定義增加 `content`，這也是放置根據所處狀態返回什麼內容的邏輯的地方，如示例 18-18 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

<span class="caption">示例 18-18: 為 `State` trait 增加 `content` 方法</span>

這裡增加了一個 `content` 方法的預設實現來返回一個空字串 slice。這意味著無需為 `Draft` 和 `PendingReview` 結構體實現 `content` 了。`Published` 結構體會重寫 `content` 方法並會返回 `post.content` 的值。

注意這個方法需要生命週期註解，如第十章所討論的。這裡獲取 `post` 的引用作為引數，並返回 `post` 一部分的引用，所以返回的引用的生命週期與 `post` 引數相關。

現在示例完成了 —— 現在示例 18-11 中所有的程式碼都能工作！我們透過釋出博文工作流的規則實現了狀態模式。圍繞這些規則的邏輯都存在於狀態物件中而不是分散在 `Post` 之中。

> #### 為什麼不用列舉？
>
> 你可能會好奇為什麼不用包含不同可能的博文狀態變體的 `enum` 作為變數。這確實是一個可能的方案；嘗試實現並對比最終結果來看看哪一種更適合你！使用列舉的一個缺點是每一個檢查列舉值的地方都需要一個 `match` 表示式或類似的程式碼來處理所有可能的變體。這相比 trait 物件模式可能顯得更重複。

#### 評估狀態模式

我們展示了 Rust 是能夠實現面向物件的狀態模式的，以便能根據博文所處的狀態來封裝不同型別的行為。`Post` 的方法並不知道這些不同型別的行為。透過這種組織程式碼的方式，要找到所有已釋出博文的不同行為只需檢視一處程式碼：`Published` 的 `State` trait 的實現。

如果要建立一個不使用狀態模式的替代實現，則可能會在 `Post` 的方法中，或者甚至於在 `main` 程式碼中用到 `match` 語句，來檢查博文狀態並在這裡改變其行為。這意味著需要檢視很多位置來理解處於釋出狀態的博文的所有邏輯！這在增加更多狀態時會變得更糟：每一個 `match` 語句都會需要另一個分支。

對於狀態模式來說，`Post` 的方法和使用 `Post` 的位置無需 `match` 語句，同時增加新狀態只涉及到增加一個新 `struct` 和為其實現 trait 的方法。

這個實現易於擴充套件增加更多功能。為了體會使用此模式維護程式碼的簡潔性，請嘗試如下一些建議：

- 增加 `reject` 方法將博文的狀態從 `PendingReview` 變回 `Draft`。
- 在將狀態變為 `Published` 之前要求兩次 `approve` 呼叫。
- 只允許博文處於 `Draft` 狀態時增加文字內容。提示：讓狀態物件負責內容可能發生什麼改變，但不負責修改 `Post`。

狀態模式的一個缺點是因為狀態實現了狀態之間的轉換，一些狀態會相互聯絡。如果在 `PendingReview` 和 `Published` 之間增加另一個狀態，比如 `Scheduled`，則不得不修改 `PendingReview` 中的程式碼來轉移到 `Scheduled`。如果 `PendingReview` 無需因為新增的狀態而改變就更好了，不過這意味著切換到另一種設計模式。

另一個缺點是我們會發現一些重複的邏輯。為了消除它們，可以嘗試在 `State` trait 中為 `request_review` 和 `approve` 方法提供返回 `self` 的預設實現；然而這樣做行不通：當將 `State` 用作 trait 物件時，trait 並不知道 `self` 具體是什麼型別，因此無法在編譯時確定返回型別。（這是前面提到的 dyn 相容性規則之一。）

其他的重複邏輯還包括 `Post` 中 `request_review` 和 `approve` 這兩個類似的實現。它們都會對 `Post` 的 `state` 欄位呼叫 `Option::take`，如果 `state` 為 `Some`，就將呼叫委託給封裝值的同名方法，並將返回結果重新賦值給 `state` 欄位。如果 `Post` 中的很多方法都遵循這個模式，我們可能會考慮定義一個宏來消除重複（檢視第二十章的 [“宏”][macros] 部分）。

完全按照面向物件語言的定義實現這個模式並沒有儘可能地利用 Rust 的優勢。讓我們看看一些程式碼中可以做出的修改，來將無效的狀態和狀態轉移變為編譯時錯誤。

#### 將狀態和行為編碼為型別

我們將展示如何稍微反思狀態模式來進行一系列不同的權衡取捨。不同於完全封裝狀態和狀態轉移使得外部程式碼對其毫不知情，我們將狀態編碼進不同的型別。如此，Rust 的型別檢查就會將任何在只能使用釋出博文的地方使用草稿博文的嘗試變為編譯時錯誤。

讓我們考慮一下示例 18-11 中 `main` 的第一部分：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

我們仍然希望能夠使用 `Post::new` 建立一個新的草稿博文，並能夠增加博文的內容。不過不同於存在一個草稿博文時返回空字串的 `content` 方法，我們將使草稿博文完全沒有 `content` 方法。這樣如果嘗試獲取草稿博文的內容，將會得到一個方法不存在的編譯錯誤。這使得我們不可能在生產環境意外顯示出草稿博文的內容，因為這樣的程式碼甚至就不能編譯。示例 18-19 展示了 `Post` 結構體、`DraftPost` 結構體以及各自的方法的定義：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

<span class="caption">示例 18-19: 帶有 `content` 方法的 `Post` 和沒有 `content` 方法的 `DraftPost`</span>

`Post` 和 `DraftPost` 結構體都有一個私有的 `content` 欄位來儲存博文的文字。這些結構體不再有 `state` 欄位因為我們將狀態編碼改為結構體型別本身。`Post` 將代表釋出的博文，它有一個返回 `content` 的 `content` 方法。

仍然有一個 `Post::new` 函式，不過不同於返回 `Post` 例項，它返回 `DraftPost` 的例項。現在不可能建立一個 `Post` 例項，因為 `content` 是私有的同時沒有任何函式返回 `Post`。

`DraftPost` 上定義了一個 `add_text` 方法，這樣就可以像之前那樣向 `content` 增加文字，不過注意 `DraftPost` 並沒有定義 `content` 方法！如此現在程式確保了所有博文都從草稿開始，同時草稿博文沒有任何可供展示的內容。任何繞過這些限制的嘗試都會產生編譯錯誤。

#### 實現狀態轉移為不同型別的轉換

那麼如何得到釋出的博文呢？我們希望強制執行的規則是草稿博文在可以釋出之前必須被稽核透過。等待稽核狀態的博文應該仍然不會顯示任何內容。讓我們透過增加另一個結構體 `PendingReviewPost` 來實現這個限制，在 `DraftPost` 上定義 `request_review` 方法來返回 `PendingReviewPost`，並在 `PendingReviewPost` 上定義 `approve` 方法來返回 `Post`，如示例 18-20 所示：

<span class="filename">檔名：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

<span class="caption">示例 18-20: `PendingReviewPost` 透過呼叫 `DraftPost` 的 `request_review` 建立，`approve` 方法將 `PendingReviewPost` 變為釋出的 `Post`</span>

`request_review` 和 `approve` 方法獲取 `self` 的所有權，因此會消費 `DraftPost` 和 `PendingReviewPost` 例項，並分別轉換為 `PendingReviewPost` 和釋出的 `Post`。這樣在呼叫 `request_review` 之後就不會遺留任何 `DraftPost` 例項，後者同理。`PendingReviewPost` 並沒有定義 `content` 方法，所以嘗試讀取其內容會導致編譯錯誤，`DraftPost` 同理。因為唯一得到定義了 `content` 方法的 `Post` 例項的途徑是呼叫 `PendingReviewPost` 的 `approve` 方法，而得到 `PendingReviewPost` 的唯一辦法是呼叫 `DraftPost` 的 `request_review` 方法，現在我們就將發博文的工作流編碼進了型別系統。

這也意味著不得不對 `main` 做出一些小的修改。因為 `request_review` 和 `approve` 返回新例項而不是修改被呼叫的結構體，所以我們需要增加更多的 `let post = ` 遮蔽賦值來儲存返回的例項。也不再能斷言草稿和等待稽核的博文的內容為空字串了，我們也不再需要它們：不能編譯嘗試使用這些狀態下博文內容的程式碼。更新後的 `main` 的程式碼如示例 18-21 所示。

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

<span class="caption">示例 18-21: `main` 中使用新的博文工作流實現的修改</span>

不得不修改 `main` 來重新賦值 `post` 使得這個實現不再完全遵守面向物件的狀態模式：狀態間的轉換不再完全封裝在 `Post` 實現中。然而，得益於型別系統和編譯時型別檢查，我們得到的收穫是無效狀態是不可能的了！這確保了某些特定的 bug，比如顯示未釋出博文的內容，將在部署到生產環境之前被發現。

嘗試為示例 18-21 之後的 `blog` crate 實現這一部分開始所建議的任務來體會使用這個版本的程式碼是何感覺。注意在這個設計中一些需求可能已經完成了。

我們已經看到，雖然 Rust 能夠實現面向物件設計模式，但 Rust 還提供了諸如將狀態編碼進型別系統之類的其他模式。這些模式有著不同的權衡取捨。雖然你可能非常熟悉面向物件模式，重新思考這些問題來利用 Rust 提供的像在編譯時避免一些 bug 這樣的有益功能。在 Rust 中面向物件模式並不總是最好的解決方案，因為 Rust 擁有像所有權這樣的面嚮物件語言所沒有的特性。

## 總結

閱讀本章後，不管你是否認為 Rust 是一個面嚮物件語言，現在你都見識了 trait 物件是一個 Rust 中獲取部分面向物件功能的方法。動態分發可以透過犧牲少量執行時效能來為你的程式碼提供一些靈活性。這些靈活性可以用來實現有助於程式碼可維護性的面向物件模式。Rust 也有像所有權這樣不同於面嚮物件語言的特性。面向物件模式並不總是利用 Rust 優勢的最好方式，但也是一個可用選項。

接下來，讓我們看看另一個提供了多樣靈活性的 Rust 功能：模式。我們在全書中已多次簡要提及它們，但尚未充分領略它們的全部威力。讓我們開始探索吧！

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#當你比編譯器知道更多時
[macros]: ch20-05-macros.html
