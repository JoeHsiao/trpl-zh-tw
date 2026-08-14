## 變數和可變性

[ch03-01-variables-and-mutability.md](https://github.com/rust-lang/book/blob/9cc190796f28505c7a9a9cacea42f50d895ff3bd/src/ch03-01-variables-and-mutability.md)

正如第二章中[“使用變數儲存值”][storing-values-with-variables] 一節提到的那樣，變數預設是不可變的（immutable）。這是 Rust 給你的眾多提醒之一，促使你以能充分利用 Rust 所提供的安全性和易於併發的方式來編寫程式碼。不過，你仍然可以把變數設為可變。讓我們來探討 Rust 為何以及如何鼓勵你偏向使用不可變性，以及為什麼有時你會想要選擇不這麼做。

當變數不可變時，一旦值被繫結一個名稱上，你就不能改變這個值。為了對此進行說明，使用 `cargo new variables` 命令在 *projects* 目錄生成一個叫做 *variables* 的新專案。

接著，在新建的 *variables* 目錄中，開啟 *src/main.rs* 並將其中的程式碼替換為下面這段程式碼。它現在還不能編譯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

儲存並使用 `cargo run` 執行程式。應該會看到一條與不可變性有關的錯誤資訊，如下輸出所示：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

這個例子展示了編譯器如何幫助你發現程式中的錯誤。編譯錯誤可能令人沮喪，但它們其實只意味著你的程式還沒有以安全的方式完成你希望它完成的工作；這**並不**意味著你不是一個好程式設計師！經驗豐富的 Rustaceans 也一樣會遇到編譯錯誤。

你收到的錯誤資訊 ``cannot assign twice to immutable variable `x` ``，是因為你試圖給不可變變數 `x` 賦第二個值。

當我們嘗試修改一個被指定為不可變的值時，能夠得到編譯時錯誤是很重要的，因為這種情況可能會導致 bug。如果程式碼的一部分假設某個值永遠不會改變，而另一部分程式碼卻改變了這個值，那麼前一部分程式碼就可能無法按設計那樣執行。事後要追蹤這類 bug 的根源會非常困難，尤其是當第二段程式碼只是**有時**才會修改這個值的時候。

Rust 編譯器保證，如果宣告一個值不會變，它就真的不會變，所以你不必自己跟蹤它。這意味著你的程式碼更易於推導。

不過，可變性也非常有用，能讓程式碼寫起來更方便。儘管變數預設是不可變的，你仍然可以像[第二章][storing-values-with-variables]中那樣，在變數名前加上 `mut` 使其變為可變。新增 `mut` 也能向未來閱讀程式碼的人傳達一種意圖：這個變數的值將會被程式碼的其他部分改變。

例如，讓我們將 *src/main.rs* 修改為如下程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

現在執行這個程式，會出現如下內容：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

使用 `mut` 後，我們就可以把繫結到 `x` 的值從 `5` 改成 `6`。歸根結底，是否使用可變性由你自己決定，這取決於在特定場景下你覺得怎樣會讓程式碼更清晰。

<a id="constants"></a>

### 常量

和不可變變數類似，*常量（constants）* 也是繫結到某個名稱且不允許改變的值，不過常量和變數之間還是有一些區別。

首先，你不能對常量使用 `mut`。常量不只是預設不可變，它們永遠都是不可變的。宣告常量時要用 `const` 關鍵字，而不是 `let`，並且 *必須* 標註值的型別。在下一節[“資料型別”][data-types]中，我們會介紹型別和型別註解，所以現在先不用擔心細節；你只要記住：宣告常量時必須總是標註型別。

常量可以在任何作用域中宣告，包括全域性作用域，這在一個值需要被很多部分的程式碼用到時很有用。

最後一個區別是，常量只能被設定為常量表達式，而不能是那些只能在執行時計算出來的值。

下面是一個宣告常量的例子：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

這個常量的名字是 `THREE_HOURS_IN_SECONDS`，它的值透過 60（一分鐘中的秒數）乘以 60（一小時中的分鐘數）再乘以 3（我們這個程式中要計算的小時數）得出。Rust 對常量的命名約定是全部大寫，並用下劃線分隔單詞。編譯器可以在編譯時對一組有限的操作進行求值，因此我們可以選擇用更易於理解和驗證的方式來寫出這個值，而不是將此常量設定為值 10,800。關於宣告常量時可以使用哪些操作的更多資訊，請參閱 [Rust Reference 中關於常量求值的部分][const-eval]。

在其宣告所在的作用域內，常量在程式執行的整個過程中都有效。這一特性使常量非常適合作為應用領域中的全域性值，比如遊戲中所有玩家能夠獲得的最高分，或者光速這樣的值。

把散落在應用程式中的硬編碼值提取為常量，有助於讓以後維護程式碼的人理解這個值的含義。如果未來需要更新這個硬編碼值，也只需要修改一個地方。

### 遮蔽

正如我們在[第二章][comparing-the-guess-to-the-secret-number]的猜數字遊戲中看到的，我們可以定義一個與之前變數同名的新變數。Rustaceans 把這種情況稱為第一個變數被第二個變數 **遮蔽（shadowed）** 了。這意味著，當你使用這個變數名時，編譯器看到的是第二個變數。實際上，第二個變數會遮住第一個變數，使得後續所有對該名稱的使用都指向第二個變數，直到它自己又被遮蔽，或者它的作用域結束。我們可以透過重複使用同一個變數名並再次寫出 `let` 關鍵字來遮蔽一個變數，如下所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

這個程式首先把 `x` 繫結到值 `5`。接著，它再次寫出 `let x =`，建立了一個新的變數 `x`，取原來的值並加上 `1`，於是 `x` 的值變成了 `6`。然後，在由花括號建立的內部作用域中，第三個 `let` 語句再次遮蔽了 `x`，並建立了一個新變數，把之前的值乘以 `2`，因此 `x` 的值變成了 `12`。當這個作用域結束時，內部的遮蔽也隨之結束，`x` 又回到 `6`。執行這個程式時，會得到如下輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

遮蔽和把變數標記為 `mut` 是不同的。如果你不小心嘗試在沒有使用 `let` 關鍵字的情況下重新給變數賦值，就會得到編譯時錯誤。而透過再次使用 `let`，我們可以對這個值做一些變換，同時又能讓變數在變換完成後繼續保持不可變。

`mut` 和遮蔽之間的另一個區別是：當我們再次使用 `let` 時，實際上是在建立一個新變數，因此我們可以改變值的型別，同時繼續複用相同的名字。例如，假設程式要求使用者輸入若干空格，以表示他們希望在某段文字之間顯示多少空格，隨後我們想把這個輸入儲存成一個數字：

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

第一個 `spaces` 變數是字串型別，而第二個 `spaces` 變數是數字型別。遮蔽讓我們不必想出不同的名字，比如 `spaces_str` 和 `spaces_num`；相反，我們可以繼續複用更簡單的 `spaces`。不過，如果像下面這樣嘗試使用 `mut`，就會得到一個編譯時錯誤：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

這個錯誤說明，我們不能修改變數的型別：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

現在我們已經瞭解了變數如何工作，讓我們看看變數可以擁有的更多資料型別。

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#比較猜測的數字和秘密數字
[data-types]: ch03-02-data-types.html#資料型別
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#使用變數儲存值
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
