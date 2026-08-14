## 控制流

[ch03-05-control-flow.md](https://github.com/rust-lang/book/blob/9cc190796f28505c7a9a9cacea42f50d895ff3bd/src/ch03-05-control-flow.md)

根據條件是否為 `true` 來決定是否執行某些程式碼，以及在條件為 `true` 時重複執行某些程式碼的能力，是大多數程式語言的基本構件。Rust 中最常見的控制執行流的結構是 `if` 表示式和迴圈。

### `if` 表示式

`if` 表示式允許根據條件執行不同的程式碼分支。你提供一個條件並表示 “如果條件滿足，執行這段程式碼；如果條件不滿足，不執行這段程式碼。”

在 *projects* 目錄中建立一個名為 *branches* 的新專案，來體驗 `if` 表示式。在 *src/main.rs* 檔案中輸入如下內容：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

所有 `if` 表示式都以 `if` 關鍵字開頭，後面緊跟一個條件。在這個例子中，條件會檢查變數 `number` 的值是否小於 5。如果條件為 `true`，就執行緊跟在條件後面的大括號中的程式碼塊。與 `if` 表示式中各個條件關聯的程式碼塊有時也被稱為 *arms*，就像我們在第二章[“比較猜測的數字和秘密數字”][comparing-the-guess-to-the-secret-number]一節中討論過的 `match` 表示式分支一樣。

也可以包含一個可選的 `else` 表示式來提供一個在條件為 `false` 時應當執行的程式碼塊，這裡我們就這麼做了。如果不提供 `else` 表示式並且條件為 `false` 時，程式會直接忽略 `if` 程式碼塊並繼續執行下面的程式碼。

嘗試執行程式碼，應該能看到如下輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

嘗試改變 `number` 的值使條件為 `false` 時看看會發生什麼：

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

再次執行程式並檢視輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

還值得注意的是，條件**必須**是 `bool` 值。如果條件不是 `bool`，我們就會得到一個錯誤。例如，嘗試執行下面這段程式碼：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

這裡 `if` 條件的值是 `3`，Rust 丟擲了一個錯誤：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

這個錯誤表明 Rust 期望得到的是一個 `bool`，卻收到了一個整數。不同於 Ruby 或 JavaScript 這樣的語言，Rust 不會自動嘗試把非布林型別轉換成布林型別。你必須顯式地為 `if` 提供一個布林值作為條件。例如，如果我們希望 `if` 程式碼塊只在某個數字不等於 `0` 時執行，就可以把 `if` 表示式改成下面這樣：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

執行程式碼會打印出 `number was something other than zero`。

#### 使用 `else if` 處理多重條件

可以將 `else if` 表示式與 `if` 和 `else` 組合來實現多重條件。例如：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

這個程式有四個可能的執行路徑。執行後應該能看到如下輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

當執行這個程式時，它按順序檢查每個 `if` 表示式並執行第一個條件為 `true` 的程式碼塊。注意即使 6 可以被 2 整除，也不會輸出 `number is divisible by 2`，更不會輸出 `else` 塊中的 `number is not divisible by 4, 3, or 2`。原因是 Rust 只會執行第一個條件為 `true` 的程式碼塊，並且一旦它找到一個以後，甚至都不會檢查剩下的條件了。

使用過多的 `else if` 表示式會讓程式碼顯得雜亂，所以如果你有不止一個 `else if`，可能就該考慮重構程式碼了。針對這種情況，第六章會介紹一個強大的 Rust 分支結構（branching construct），叫做 `match`。

#### 在 `let` 語句中使用 `if`

因為 `if` 是一個表示式，我們可以在 `let` 語句的右側使用它，例如在示例 3-2 中：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">示例 3-2：將 `if` 表示式的返回值賦給一個變數</span>

變數 `number` 會繫結到 `if` 表示式結果所產生的那個值。執行這段程式碼看看會發生什麼：

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

記住，程式碼塊的值就是其中最後一個表示式的值，而數字本身也是表示式。在這個例子中，整個 `if` 表示式的值取決於哪個程式碼塊被執行。這意味著 `if` 的各個分支可能產生的結果值都必須是相同型別；在示例 3-2 中，`if` 分支和 `else` 分支的結果都是 `i32` 整數。如果型別不一致，就會像下面這個例子一樣報錯：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

當編譯這段程式碼時，會得到一個錯誤。`if` 和 `else` 分支的值型別是不相容的，同時 Rust 也準確地指出在程式中的何處發現的這個問題：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` 程式碼塊中的表示式會求值為一個整數，而 `else` 程式碼塊中的表示式會求值為一個字串。這是行不通的，因為變數必須只有一個型別。Rust 需要在編譯時就明確知道 `number` 的型別，這樣它才能在編譯階段驗證每一處對 `number` 的使用是否合法。如果 `number` 的型別只能在執行時確定，Rust 就無法做到這一點；而如果編譯器必須為每個變數跟蹤多種假設型別，它也會變得更加複雜，並且對程式碼的保證會更少。

### 使用迴圈重複執行

反覆執行同一段程式碼是一件很常見的事，為此 Rust 提供了多種 **迴圈**（*loops*）。迴圈會執行迴圈體中的程式碼直到結尾，然後立即回到開頭繼續執行。為了體驗迴圈，我們來新建一個叫做 *loops* 的專案。

Rust 有三種迴圈：`loop`、`while` 和 `for`。我們每一個都試試。

#### 使用 `loop` 重複執行程式碼

`loop` 關鍵字告訴 Rust 反覆執行一段程式碼，要麼永遠執行下去，要麼直到你明確要求它停止。

作為一個例子，將 *loops* 目錄中的 *src/main.rs* 檔案修改為如下：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

執行這個程式時，我們會看到 `again!` 被不斷重複列印，直到我們手動停止程式。大多數終端都支援使用快捷鍵 <kbd>ctrl</kbd>-<kbd>C</kbd> 來中斷一個陷入無限迴圈的程式。試試看：

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

符號 `^C` 表示你在這裡按下了 <kbd>ctrl</kbd>-<kbd>C</kbd>。在 `^C` 後面，你可能會看到，也可能不會看到 `again!`，這取決於程式碼在收到中斷訊號時正執行到迴圈的哪個位置。

幸運的是，Rust 也提供了在程式碼中跳出迴圈的方法。你可以在迴圈中放置 `break` 關鍵字，告訴程式何時停止執行該迴圈。回憶一下，我們曾在第二章猜數字遊戲的[“猜測正確後退出”][quitting-after-a-correct-guess]一節中使用過它，讓程式在使用者猜中數字後退出。

我們在猜數字遊戲中也使用過 `continue`。在迴圈裡，`continue` 關鍵字會告訴程式跳過本次迴圈迭代剩餘的程式碼，並直接進入下一次迭代。

#### 從迴圈返回值

`loop` 的一個用途是重試那些你知道可能失敗的操作，比如檢查某個執行緒是否完成了任務。不過，你也可能希望把這個操作的結果傳遞給其他程式碼。為此，你可以在用於停止迴圈的 `break` 表示式後面加上想要返回的值；這個值會作為迴圈的返回值返回出來，因而你就可以使用它，如下所示：

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

在迴圈之前，我們聲明瞭一個名為 `counter` 的變數，並將其初始化為 `0`。然後，又聲明瞭一個名為 `result` 的變數，用來儲存迴圈返回的值。在迴圈的每次迭代中，我們都會給 `counter` 加 `1`，然後檢查它是否等於 `10`。當條件滿足時，就用 `break` 關鍵字返回 `counter * 2` 的值。迴圈結束後，我們用分號結束把值賦給 `result` 的那條語句。最後，打印出 `result` 的值，也就是 `20`。

如果你在迴圈內部使用 `return`，也可以從中返回。不過，`break` 只會退出當前迴圈，而 `return` 總是會退出當前函式。

<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

#### 迴圈標籤：在多個迴圈之間消除歧義

如果迴圈中又套了迴圈，那麼 `break` 和 `continue` 預設只作用於當前最內層的那個迴圈。你可以選擇給某個迴圈加上一個 **迴圈標籤**（*loop label*），然後把這個標籤和 `break` 或 `continue` 一起使用，這樣這些關鍵字就會作用於被標記的迴圈，而不是最內層迴圈。下面是一個包含兩層巢狀迴圈的例子：

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

外層迴圈帶有標籤 `'counting_up`，它會從 0 數到 2。沒有標籤的內層迴圈則從 10 倒數到 9。第一個沒有指定標籤的 `break` 只會退出內層迴圈。語句 `break 'counting_up;` 則會退出外層迴圈。這段程式碼會列印：

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<a id="conditional-loops-with-while"></a>

#### `while` 條件迴圈

程式經常需要在迴圈中計算某個條件：只要條件為 `true`，迴圈就繼續；當條件不再為 `true` 時，程式就會呼叫 `break` 來停止迴圈。這種迴圈型別可以透過組合 `loop`、`if`、`else` 和 `break` 來實現；如果你願意，現在就可以在程式裡試試看。不過，這種模式實在太常見了，所以 Rust 為它內建了一個語言結構，叫做 `while` 迴圈。在示例 3-3 中，我們使用 `while` 讓程式迴圈三次，每次計數都減一；之後，在迴圈結束後列印另一條訊息並退出。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">示例 3-3: 當條件為 `true` 時，使用 `while` 迴圈執行程式碼</span>

這種結構消除了使用 `loop`、`if`、`else` 和 `break` 時原本需要的大量巢狀，因此程式碼會更清晰。只要條件求值為 `true`，程式碼就會繼續執行；否則就退出迴圈。

#### 使用 `for` 遍歷集合

可以使用 `while` 結構來遍歷集合中的元素，比如陣列。例如，示例 3-4 中的迴圈會列印陣列 `a` 中的每一個元素。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">示例 3-4：使用 `while` 迴圈遍歷集合中的每一個元素</span>

這裡，程式碼對陣列中的元素進行計數。它從索引 `0` 開始，並接著迴圈直到遇到陣列的最後一個索引（這時，`index < 5` 不再為 `true`）。執行這段程式碼會打印出陣列中的每一個元素：

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

陣列中的所有五個元素都如期出現在終端中。儘管 `index` 在某一時刻會到達值 `5`，不過迴圈在其嘗試從陣列獲取第六個值（會越界）之前就停止了。

不過，這種方式很容易出錯；如果索引值或測試條件寫錯了，就會導致程式 panic。例如，如果你把陣列 `a` 改成只有 4 個元素，卻忘了把條件更新成 `while index < 4`，程式碼就會 panic。它也會讓程式變慢，因為編譯器會加入執行時程式碼，在每次迴圈迭代時檢查索引是否仍然位於陣列邊界之內。

作為更簡潔的替代方案，可以使用 `for` 迴圈來對一個集合的每個元素執行一些程式碼。`for` 迴圈看起來如示例 3-5 所示：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">示例 3-5：使用 `for` 迴圈遍歷集合中的元素</span>

執行這段程式碼時，你會看到和示例 3-4 相同的輸出。更重要的是，我們提高了程式碼的安全性，並消除了那種可能因為越過陣列末尾，或遍歷不夠完整而漏掉某些元素所導致的 bug。

例如，在示例 3-4 的程式碼中，如果你把陣列 `a` 改成只有 4 個元素，卻忘了把條件更新為 `while index < 4`，程式碼就會 panic。而使用 `for` 迴圈時，你就不必記著在修改陣列元素個數時還要同步修改其他程式碼了。

`for` 迴圈的安全性和簡潔性，使它成為 Rust 中最常用的迴圈結構。即使是在你只想把某段程式碼執行特定次數的情況下，比如示例 3-3 裡那個使用 `while` 的倒計時例子，大多數 Rustaceans 也會選擇使用 `for` 迴圈。實現這種寫法的方式是使用 `Range`，這是標準庫提供的一種型別，用來生成從某個數字開始、到另一個數字之前結束的所有數字序列。

下面是一個使用 `for` 迴圈來倒計時的例子，它還用到了一個我們尚未講到的方法 `rev`，用於反轉 range。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

這段程式碼是不是更好一些？

## 總結

你做到了！這是內容相當豐富的一章：你學習了變數、標量和複合資料型別、函式、註釋、`if` 表示式以及迴圈！如果你想練習本章討論的概念，可以嘗試構建下面這些程式：

- 相互轉換攝氏與華氏溫度。
- 生成第 n 個斐波那契數。
- 列印聖誕頌歌 “The Twelve Days of Christmas” 的歌詞，並利用歌曲中的重複部分（透過編寫迴圈）。

當你準備好繼續時，我們將討論一個在其他程式語言中**並不**常見的概念：所有權（ownership）。

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#比較猜測的數字和秘密數字
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#猜測正確後退出
