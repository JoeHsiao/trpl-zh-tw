## 模式語法

[ch19-03-pattern-syntax.md](https://github.com/rust-lang/book/blob/3e7aac1d7bc4de520b7040d4ef0f50c937468bd0/src/ch19-03-pattern-syntax.md)

在本節中，我們收集了模式中所有有效的語法，並討論為什麼以及何時你可能要使用這些語法。

### 匹配字面值

如第六章所示，可以直接匹配字面值模式。如下程式碼給出了一些例子：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

這段程式碼會列印 `one` 因為 `x` 的值是 1。如果希望程式碼獲得特定的具體值，則該語法很有用。

### 匹配命名變數

命名變數（Named variables）是匹配任何值的不可反駁模式，這在之前已經使用過數次。然而，當在 `match`、`if let` 或 `while let` 表示式中使用命名變數時，會出現一些複雜情況。由於這些表示式會開始一個新作用域，作為模式一部分在表示式內部宣告的變數會遮蔽外部同名變數，這與所有變數的遮蔽規則一致。在示例 19-11 中，聲明瞭一個值為 `Some(5)` 的變數 `x` 和一個值為 `10` 的變數 `y`。接著在值 `x` 上建立了一個 `match` 表示式。觀察匹配分支中的模式和結尾的 `println!`，並在執行此程式碼或進一步閱讀之前推斷這段程式碼會列印什麼。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

<span class="caption">示例 19-11: 一個 `match` 語句其中一個分支引入了遮蔽變數 `y`</span>

讓我們看看當 `match` 語句執行的時候發生了什麼。第一個匹配分支的模式並不匹配 `x` 中定義的值，所以程式碼繼續執行。

第二個匹配分支中的模式引入了一個新變數 `y`，它會匹配任何 `Some` 中的值。因為我們在 `match` 表示式的新作用域中，這是一個新變數，而不是開頭宣告為值 10 的那個 `y`。這個新的 `y` 繫結會匹配任何 `Some` 中的值，在這裡是 `x` 中的值。因此這個 `y` 綁定了 `x` 中 `Some` 內部的值。這個值是 5，所以這個分支的表示式將會執行並打印出 `Matched, y = 5`。

如果 `x` 的值是 `None` 而不是 `Some(5)`，頭兩個分支的模式不會匹配，所以會匹配下劃線。這個分支的模式中沒有引入變數 `x`，所以此時表示式中的 `x` 會是外部沒有被遮蔽的 `x`。在這個假想的例子中，`match` 將會列印 `Default case, x = None`。

一旦 `match` 表示式執行完畢，其作用域也就結束了，同理內部 `y` 的作用域也結束了。最後的 `println!` 會列印 `at the end: x = Some(5), y = 10`。

為了建立能夠比較外部 `x` 和 `y` 的值，又不引入新的變數去遮蔽已有 `y` 的 `match` 表示式，我們需要相應地使用帶有條件的匹配守衛（match guard）。我們稍後將在 [“使用匹配守衛新增額外條件”](#使用匹配守衛新增額外條件) 這一小節討論匹配守衛。

### 匹配多個模式

在 `match` 表示式中，可以使用 `|` 語法匹配多個模式，它代表 **或**（_or_）運算子模式。例如，如下程式碼將 `x` 的值與匹配分支相比較，第一個分支有**或**選項，意味著如果 `x` 的值匹配此分支的任一個值，它就會執行：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

上面的程式碼會列印 `one or two`。

### 透過 `..=` 匹配值範圍

`..=` 語法允許你匹配一個閉區間範圍（range）內的值。在如下程式碼中，當模式匹配任何在給定範圍內的值時，該分支會執行：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

如果 `x` 是 1、2、3、4 或 5，第一個分支就會匹配。這個語法在匹配多個值時相比使用 `|` 運算子來表達相同的意思更為方便；如果使用 `|` 則不得不指定 `1 | 2 | 3 | 4 | 5`。相反指定範圍就簡短的多，特別是在希望匹配比如從 1 到 1000 的數字的時候！

編譯器會在編譯時檢查範圍不為空，而 `char` 和數字值是 Rust 僅有的可以判斷範圍是否為空的型別，所以範圍只允許用於數字或 `char` 值。

如下是一個使用 `char` 型別值範圍的例子：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust 知道 `'c'` 位於第一個模式的範圍內，並會打印出 `early ASCII letter`。

### 解構並分解值

也可以使用模式來解構結構體、列舉和元組，以便使用這些值的不同部分。讓我們來分別看一看。

#### 結構體

示例 19-12 展示帶有兩個欄位 `x` 和 `y` 的結構體 `Point`，可以透過帶有模式的 `let` 語句將其分解：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

<span class="caption">示例 19-12: 解構一個結構體的欄位為單獨的變數</span>

這段程式碼建立了變數 `a` 和 `b` 來匹配結構體 `p` 中的 `x` 和 `y` 欄位。這個例子展示了模式中的變數名不必與結構體中的欄位名一致。不過通常希望變數名與欄位名一致以便於理解變數來自於哪些欄位。因為變數名匹配欄位名是常見的，同時因為 `let Point { x: x, y: y } = p;` 包含了很多重複，所以對於匹配結構體欄位的模式存在簡寫：只需列出結構體欄位的名稱，則模式建立的變數會有相同的名稱。示例 19-13 展示了與示例 19-12 有著相同行為的程式碼，不過 `let` 模式建立的變數為 `x` 和 `y` 而不是 `a` 和 `b`：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

<span class="caption">示例 19-13: 使用結構體欄位簡寫來解構結構體欄位</span>

這段程式碼建立了變數 `x` 和 `y`，與變數 `p` 中的 `x` 和 `y` 相匹配。其結果是變數 `x` 和 `y` 包含結構體 `p` 中的值。

也可以使用字面值作為結構體模式的一部分進行解構，而不是為所有的欄位建立變數。這允許我們測試一些欄位為特定值的同時建立其他欄位的變數。

示例 19-14 展示了一個 `match` 語句將 `Point` 值分成了三種情況：直接位於 `x` 軸上（此時 `y = 0` 為真）、位於 `y` 軸上（`x = 0`）或不在任何軸上的點。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

<span class="caption">示例 19-14: 解構和匹配模式中的字面值</span>

第一個分支透過指定欄位 `y` 匹配字面值 `0` 來匹配任何位於 `x` 軸上的點。此模式仍然建立了變數 `x` 以便在分支的程式碼中使用。

類似的，第二個分支透過指定欄位 `x` 匹配字面值 `0` 來匹配任何位於 `y` 軸上的點，併為欄位 `y` 建立了變數 `y`。第三個分支沒有指定任何字面值，所以其會匹配任何其他的 `Point` 併為 `x` 和 `y` 兩個欄位建立變數。

在這個例子中，值 `p` 因為其 `x` 包含 `0` 而匹配第二個分支，因此會打印出 `On the y axis at 7`。

記住 `match` 表示式一旦找到一個匹配的模式就會停止檢查其它分支，所以即使 `Point { x: 0, y: 0}` 在 `x` 軸上也在 `y` 軸上，這些程式碼也只會列印 `On the x axis at 0`。

#### 列舉

本書之前曾經解構過列舉（例如第六章示例 6-5），不過當時沒有明確提到解構列舉的模式需要對應列舉所定義的儲存資料的方式。讓我們以示例 6-2 中的 `Message` 列舉為例，編寫一個 `match` 使用模式解構每一個內部值，如示例 19-15 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

<span class="caption">示例 19-15: 解構包含不同型別值變體的列舉</span>

這段程式碼會打印出 `Change the color to red 0, green 160, and blue 255`。嘗試改變 `msg` 的值來觀察其他分支程式碼的執行。

對於像 `Message::Quit` 這樣沒有任何資料的列舉變體，不能進一步解構其值。只能匹配其字面值 `Message::Quit`，因此模式中沒有任何變數。

對於像 `Message::Move` 這樣的類結構體列舉變體，可以採用類似於匹配結構體的模式。在變體名稱後，使用大括號並列出欄位變數以便將其分解以供此分支的程式碼使用。這裡使用了示例 19-13 所展示的簡寫。

對於像 `Message::Write` 這樣的包含一個元素，以及像 `Message::ChangeColor` 這樣包含三個元素的類元組列舉變體，其模式則類似於用於解構元組的模式。模式中變數的數量必須與變體中元素的數量完全一致。

#### 巢狀的結構體和列舉

目前為止，所有的例子都只匹配了深度為一級的結構體或列舉，不過當然也可以匹配巢狀的項！例如，我們可以重構示例 19-15 的程式碼在 `ChangeColor` 訊息中同時支援 RGB 和 HSV 色彩模式，如示例 19-16 所示：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

<span class="caption">示例 19-16: 匹配巢狀的列舉</span>

`match` 表示式第一個分支的模式匹配一個包含 `Color::Rgb` 列舉變體的 `Message::ChangeColor` 列舉變體，然後模式綁定了三個內部的 `i32` 值。第二個分支的模式也匹配一個 `Message::ChangeColor` 列舉變體，但是其內部的列舉會匹配 `Color::Hsv` 列舉變體。我們可以在一個 `match` 表示式中指定這些複雜條件，即使會涉及到兩個列舉。

#### 結構體和元組

甚至可以用複雜的方式來混合、匹配和巢狀解構模式。如下是一個複雜結構體的例子，其中結構體和元組巢狀在元組中，並將所有的原始型別解構出來：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

這將複雜的型別分解成部分元件以便可以單獨使用我們感興趣的值。

透過模式解構是一個方便將值的各個片段分離開來單獨使用的方式，比如結構體中每個單獨欄位的值。

### 忽略模式中的值

有時忽略模式中的一些值是有用的，比如 `match` 中最後捕獲全部情況的分支實際上沒有做任何事，但是它確實負責匹配了所有剩餘的可能值。有一些方法可以忽略模式中全部或部分值：使用 `_` 模式（我們已經見過了），在另一個模式中使用 `_` 模式，使用一個以下劃線開始的名稱，或者使用 `..` 忽略所剩部分的值。讓我們來分別探索如何以及為什麼要這麼做。

#### 用 `_` 忽略整個值

我們已經使用過下劃線作為匹配但不繫結任何值的萬用字元模式了。雖然這作為 `match` 表示式最後的分支特別有用，也可以將其用於任意模式，包括函式引數中，如示例 19-17 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

<span class="caption">示例 19-17: 在函式簽名中使用 `_`</span>

這段程式碼會完全忽略作為第一個引數傳遞的值 `3`，並會打印出 `This code only uses the y parameter: 4`。

大部分情況當你不再需要特定函式引數時，最好修改簽名不再包含無用的引數。在一些情況下忽略函式引數會變得特別有用，比如實現 trait 時，當你需要特定型別簽名但是函式實現並不需要某個引數時。這樣可以避免一個存在未使用的函式引數的編譯警告，就跟使用命名引數一樣。

#### 用巢狀的 `_` 忽略部分值

也可以在一個模式內部使用`_` 忽略部分值，例如，當只需要測試部分值但在期望執行的程式碼中沒有用到其他部分時。示例 19-18 展示了負責管理設定值的程式碼。業務需求是使用者不允許覆蓋現有的自定義設定，但是可以取消設定，也可以在當前未設定時為其提供一個值。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

<span class="caption">示例 19-18: 當不需要 `Some` 中的值時在模式內使用下劃線來匹配 `Some` 變體</span>

這段程式碼會打印出 `Can't overwrite an existing customized value` 接著是 `setting is Some(5)`。在第一個匹配分支，我們不需要匹配或使用任一個 `Some` 變體中的值，但需要檢測 `setting_value` 和 `new_setting_value` 是否均為 `Some` 變體。在這種情況下，我們打印出為何不改變 `setting_value`，並且不會改變它。

對於所有其他情況（`setting_value` 或 `new_setting_value` 任一為 `None`），這由第二個分支的 `_` 模式體現，這時確實希望允許 `new_setting_value` 變為 `setting_value`。

也可以在一個模式中的多處使用下劃線來忽略特定值，如示例 19-19 所示，這裡忽略了一個五元元組中的第二和第四個值：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

<span class="caption">示例 19-19: 忽略元組的多個部分</span>

這會打印出 `Some numbers: 2, 8, 32`，值 `4` 和 `16` 會被忽略。

#### 透過在變數名開頭加 `_` 來忽略未使用變數

如果你建立了一個變數卻不在任何地方使用它，Rust 通常會給你一個警告，因為未使用的變數可能會是個 bug。但是有時建立一個還未使用的變數是有用的，比如你正在設計原型或剛剛開始一個專案。這時你希望告訴 Rust 不要警告未使用的變數，為此可以用下劃線作為變數名的開頭。示例 19-20 中建立了兩個未使用變數，不過當編譯程式碼時只會得到其中一個的警告：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

<span class="caption">示例 19-20: 以下劃線開始變數名以便去掉未使用變數警告</span>

這裡得到了警告說未使用變數 `y`，不過沒有警告說未使用 `_x`。

注意，只使用 `_` 和使用以下劃線開頭的名稱有些微妙的不同：比如 `_x` 仍會將值繫結到變數，而 `_` 則完全不會繫結。為了展示這個區別的意義，示例 19-21 會產生一個錯誤。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

<span class="caption">示例 19-21: 以下劃線開頭的未使用變數仍然會繫結值，它可能會獲取值的所有權</span>

我們會得到一個錯誤，因為 `s` 的值仍然會移動進 `_s`，並阻止我們再次使用 `s`。然而只使用下劃線本身，並不會繫結值。示例 19-22 能夠無錯編譯，因為 `s` 沒有被移動進 `_`：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

<span class="caption">示例 19-22: 單獨使用下劃線不會繫結值</span>

上面的程式碼能很好的執行；因為沒有把 `s` 繫結到任何變數；它沒有被移動。

#### 用 `..` 忽略值的剩餘部分

對於有多個部分的值，可以使用 `..` 語法來只使用特定部分並忽略其它值，從而避免不得不每一個忽略值列出下劃線。`..` 模式會忽略模式中剩餘的任何沒有顯式匹配的值部分。在示例 19-23 中，有一個 `Point` 結構體存放了三維空間中的座標。在 `match` 表示式中，我們希望只操作 `x` 座標並忽略 `y` 和 `z` 欄位的值：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

<span class="caption">示例 19-23: 透過使用 `..` 來忽略 `Point` 中除 `x` 以外的欄位</span>

這裡列出了 `x` 值，接著僅僅包含了 `..` 模式。這比不得不列出 `y: _` 和 `z: _` 要來得簡單，特別是在處理有很多欄位的結構體，但只涉及一到兩個欄位時的情形。

`..` 會擴充套件為所需要的值的數量。示例 19-24 展示瞭如何在元組中使用 `..`：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

<span class="caption">示例 19-24: 只匹配元組中的第一個和最後一個值並忽略掉所有其它值</span>

這裡用 `first` 和 `last` 來匹配第一個和最後一個值。`..` 將匹配並忽略中間的所有值。

然而使用 `..` 必須是無歧義的。如果期望匹配和忽略的值是不明確的，Rust 會報錯。示例 19-25 展示了一個帶有歧義的 `..` 例子，因此其不能編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

<span class="caption">示例 19-25: 嘗試以有歧義的方式運用 `..`</span>

當編譯這個示例時，會得到如下錯誤：

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

Rust 不可能決定在元組中匹配 `second` 值之前應該忽略多少個值，以及在之後忽略多少個值。這段程式碼可能表明我們意在忽略 `2`，繫結 `second` 為 `4`，接著忽略 `8`、`16` 和 `32`；抑或是意在忽略 `2` 和 `4`，繫結 `second` 為 `8`，接著忽略 `16` 和 `32`，以此類推。變數名 `second` 對於 Rust 來說並沒有任何特殊意義，所以會得到編譯錯誤，因為在這兩個地方使用 `..` 是有歧義的。

### 使用匹配守衛新增額外條件

**匹配守衛**（_match guard_）是一個指定於 `match` 分支模式之後的額外 `if` 條件，它也必須被滿足才能選擇此分支。匹配守衛用於表達比單獨的模式所能允許的更為複雜的情況。但是注意，它們僅在 `match` 表示式中可用，不能用於 `if let` 或 `while let` 表示式。

這個條件可以使用模式中建立的變數。示例 19-26 展示了一個 `match`，其中第一個分支有模式 `Some(x)` 還有匹配守衛 `if x % 2 == 0` (當 `x` 是偶數時為真)：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

<span class="caption">示例 19-26: 在模式中加入匹配守衛</span>

上例會打印出 `The number 4 is even`。當 `num` 與模式中第一個分支比較時，因為 `Some(4)` 匹配 `Some(x)` 所以可以匹配。接著匹配守衛檢查 `x` 除以 `2` 的餘數是否等於 `0`，因為它等於 `0`，所以第一個分支被選擇。

相反如果 `num` 為 `Some(5)`，因為 `5` 除以 `2` 的餘數是 `1` 不等於 `0` 所以第一個分支的匹配守衛為 `false`。接著 Rust 會前往第二個分支，這次匹配因為它沒有匹配守衛所以會匹配任何 `Some` 變體。

無法在模式中表達類似 `if x % 2 == 0` 的條件，所以透過匹配守衛提供了表達類似邏輯的能力。這種替代表達方式的缺點是，編譯器不會嘗試為包含匹配守衛的模式檢查窮盡性。

在示例 19-11 中，我們提到可以使用匹配守衛來解決模式中變數遮蔽的問題，那裡 `match` 表示式的模式中新建了一個變數而不是使用 `match` 之外的同名變數。新變數意味著不能夠測試外部變數的值。示例 19-27 展示瞭如何使用匹配守衛修復這個問題。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

<span class="caption">示例 19-27: 使用匹配守衛來測試與外部變數的相等性</span>

現在這會打印出 `Default case, x = Some(5)`。現在第二個匹配分支中的模式不會引入一個遮蔽外部 `y` 的新變數 `y`，這意味著可以在匹配守衛中使用外部的 `y`。相比指定會遮蔽外部 `y` 的模式 `Some(y)`，這裡指定為 `Some(n)`。此新建的變數 `n` 並沒有覆蓋任何值，因為 `match` 外部沒有變數 `n`。

匹配守衛 `if n == y` 並不是一個模式所以沒有引入新變數。這個 `y` **正是**外部的 `y` 而不是新的遮蔽變數 `y`，這樣就可以透過比較 `n` 和 `y` 來表達尋找一個與外部 `y` 相同的值了。

也可以在匹配守衛中使用**或**運算子 `|` 來指定多個模式，同時匹配守衛的條件會作用於所有的模式。示例 19-28 展示了結合匹配守衛與使用了 `|` 的模式的優先順序。這個例子中重要的部分是匹配守衛 `if y` 作用於 `4`、`5` **和** `6`，即使這看起來好像 `if y` 只作用於 `6`：

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

<span class="caption">示例 19-28: 結合多個模式與匹配守衛</span>

這個匹配條件表明此分支值匹配 `x` 值為 `4`、`5` 或 `6` **同時** `y` 為 `true` 的情況。執行這段程式碼時會發生的是第一個分支的模式因 `x` 為 `4` 而匹配，不過匹配守衛 `if y` 為 `false`，所以第一個分支不會被選擇。程式碼移動到第二個分支，這會匹配，此程式會打印出 `no`。這是因為 `if` 條件作用於整個 `4 | 5 | 6` 模式，而不僅是最後的值 `6`。換句話說，匹配守衛與模式的優先順序關係看起來像這樣：

```text
(4 | 5 | 6) if y => ...
```

而不是：

```text
4 | 5 | (6 if y) => ...
```

執行程式碼後，優先順序行為就很明顯了：如果匹配守衛只作用於由 `|` 運算子指定的值列表的最後一個值，這個分支就會匹配且程式會打印出 `yes`。

### 使用 `@` 繫結

_at_ 運算子（`@`）允許我們在建立一個存放值的變數的同時測試其值是否匹配模式。示例 19-29 展示了一個例子，這裡我們希望測試 `Message::Hello` 的 `id` 欄位是否位於 `3..=7` 範圍內，同時也希望能將其值繫結到 `id_variable` 變數中以便此分支相關聯的程式碼可以使用它。可以將 `id_variable` 命名為 `id`，與欄位同名，不過出於示例的目的這裡選擇了不同的名稱。

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

<span class="caption">示例 19-29: 使用 `@` 在模式中繫結值的同時測試它</span>

上例會打印出 `Found an id in range: 5`。透過在 `3..=7` 之前指定 `id_variable @`，我們捕獲了任何匹配此範圍的值並同時測試其值匹配這個範圍模式。

第二個分支只在模式中指定了一個範圍，分支相關程式碼沒有一個包含 `id` 欄位實際值的變數。`id` 欄位的值可以是 10、11 或 12，不過這個模式的程式碼並不知情也不能使用 `id` 欄位中的值，因為沒有將 `id` 值儲存進一個變數。

最後一個分支指定了一個沒有範圍的變數，此時確實擁有可以用於分支程式碼的變數 `id`。因為這裡使用了結構體欄位簡寫語法。不過此分支中沒有像頭兩個分支那樣對 `id` 欄位的值進行測試：任何值都會匹配該模式。

使用 `@` 可以在一個模式中同時測試和儲存變數值。

## 總結

模式是 Rust 中一個很有用的功能，它有助於我們區分不同型別的資料。當用於 `match` 語句時，Rust 確保模式會包含每一個可能的值，否則程式將不能編譯。`let` 語句和函式引數的模式使得這些結構更強大，可以在將值解構為更小部分的同時為變數賦值。可以建立簡單或複雜的模式來滿足我們的要求。

接下來，在本書倒數第二章中，我們將介紹一些 Rust 眾多功能中較為高階的部分。
