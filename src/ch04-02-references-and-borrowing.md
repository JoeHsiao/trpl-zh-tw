## 引用與借用

[ch04-02-references-and-borrowing.md](https://github.com/rust-lang/book/blob/bb86b1763bdfb823e3e1d52c57020543b0fc7c4a/src/ch04-02-references-and-borrowing.md)

示例 4-5 中的元組程式碼有這樣一個問題：我們必須把 `String` 返回給呼叫函式，這樣在呼叫 `calculate_length` 之後仍然能使用它，因為 `String` 已經被移動進了 `calculate_length`。另一種做法是提供 `String` 值的引用（reference）。**引用**（*reference*）有點像指標，因為它是一個地址，我們可以沿著它訪問儲存在該地址中的資料，而這些資料歸其他變數所有。與指標不同，引用在其生命週期內保證會指向某個特定型別的有效值。

下面是如何定義並使用一個（新的）`calculate_length` 函式，它以一個物件的引用作為引數而不是獲取值的所有權：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

首先，注意變數宣告和函式返回值中的元組程式碼都消失了。其次，注意我們把 `&s1` 傳給 `calculate_length`，而在函式定義中，我們接收的是 `&String` 而不是 `String`。這些 `&` 符號表示 **引用**；它們讓你引用某個值而不取得它的所有權。圖 4-6 展示了這一概念。

<img alt="Three tables: the table for s contains only a pointer to the table
for s1. The table for s1 contains the stack data for s1 and points to the
string data on the heap." src="img/trpl04-06.svg" class="center" />

<span class="caption">圖 4-6：`&String s` 指向 `String s1` 示意圖</span>

> 注意：與使用 `&` 引用相反的操作是 **解引用**（*dereferencing*），它使用解引用運算子 `*` 實現。我們將會在第八章遇到一些解引用運算子，並在第十五章詳細討論解引用。

仔細看看這個函式呼叫：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&s1` 語法讓我們建立一個**指向**值 `s1` 的引用，但並不擁有它。因為這個引用並不擁有該值，所以當引用停止使用時，它所指向的值也不會被丟棄。

同理，函式簽名使用 `&` 來表明引數 `s` 的型別是一個引用。讓我們增加一些解釋性的註釋：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

變數 `s` 的有效作用域與其他函式引數相同，不過當 `s` 停止使用時，它所指向的值不會被丟棄，因為 `s` 並不擁有它。當函式把引用而不是實際值作為引數時，就不需要通過返回值來交還所有權，因為函式從未擁有過它。

我們將建立一個引用的行為稱為 **借用**（*borrowing*）。正如現實生活中，如果一個人擁有某樣東西，你可以從他那裡借來。當你使用完後，必須還回去。因為我們並不擁有它的所有權。

那如果我們嘗試修改借用的變數呢？嘗試示例 4-6 中的程式碼。劇透：這行不通！

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">示例 4-6：嘗試修改借用的值</span>

這裡是錯誤：

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

正如變數預設是不可變的，引用預設也是不可變的。我們不允許通過引用修改它指向的值。

### 可變引用

我們通過一個小調整就能修復示例 4-6 程式碼中的錯誤，允許我們修改一個借用的值，這就是 **可變引用**（*mutable reference*）：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

首先，我們必須把 `s` 改成 `mut`。然後在呼叫 `change` 函式時建立一個可變引用 `&mut s`，並更新函式簽名，讓它接收一個可變引用 `some_string: &mut String`。這樣就很清楚地表明，`change` 函式會修改它所借用的值。

可變引用有一個很大的限制：如果你有一個對該變數的可變引用，你就不能再建立對該變數的引用。這些嘗試建立兩個 `s` 的可變引用的程式碼會失敗：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

錯誤如下：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

這個報錯說明這段程式碼無效，因為我們不能在同一時間多次以可變方式借用 `s`。第一個可變借用在 `r1` 中，並且必須持續到它在 `println!` 中被使用；但在這個可變引用被建立和被使用之間，我們又嘗試在 `r2` 中建立另一個可變引用，它借用的是和 `r1` 相同的資料。

這一限制讓可變性以一種受到嚴格控制的方式出現，從而防止在同一時間對同一資料存在多個可變引用。剛接觸 Rust 的人往往不太適應這一點，因為大多數語言都允許你隨時修改變數。這個限制的好處是 Rust 可以在編譯時防止資料競爭。**資料競爭**（*data race*）類似於競態條件，它會在以下三種行為同時發生時出現：

- 兩個或更多指標同時訪問同一資料。
- 至少有一個指標被用來寫入資料。
- 沒有同步資料訪問的機制。

資料競爭會導致未定義行為，難以在執行時追蹤，並且難以診斷和修復；Rust 通過拒絕編譯存在資料競爭的程式碼來避免此問題！

一如既往，可以使用大括號來建立一個新的作用域，以允許擁有多個可變引用，只是不能**同時**擁有：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust 在同時使用可變與不可變引用時也強制採用類似的規則。這些程式碼會導致一個錯誤：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

錯誤如下：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

呼！我們**也**不能在擁有不可變引用的同時擁有可變引用。

不可變引用的借用者可不希望在借用時值會突然發生改變！然而，多個不可變引用是可以的，因為沒有哪個只能讀取資料的引用者能夠影響其他引用者讀取到的資料。

注意一個引用的作用域從宣告的地方開始一直持續到最後一次使用為止。例如，因為最後一次使用不可變引用的位置在 `println!`，它發生在宣告可變引用之前，所以如下程式碼是可以編譯的：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

不可變引用 `r1` 和 `r2` 的作用域在 `println!` 最後一次使用之後結束，這發生在可變引用 `r3` 被建立之前。因為它們的作用域沒有重疊，所以程式碼是可以編譯的。編譯器可以在作用域結束之前判斷不再使用的引用。

儘管借用錯誤有時令人沮喪，但請記住，這是 Rust 編譯器在提前指出一個潛在 bug，並且精確告訴你問題出在哪裡，而且這一切發生在編譯時而不是執行時。這樣你就不必再去追查，為什麼資料的狀態和你原先想的不一樣。

### 懸垂引用

在帶有指標的語言中，如果釋放了一塊記憶體，卻保留了指向它的指標，就很容易錯誤地製造出一個**懸垂指標**（*dangling pointer*）：這個指標指向的記憶體位置可能已經被分配作其他用途。相比之下，在 Rust 中，編譯器保證引用永遠不會變成懸垂引用：如果你持有某些資料的引用，編譯器會確保這些資料不會在它們的引用之前離開作用域。

讓我們嘗試建立一個懸垂引用，看看 Rust 如何通過一個編譯時錯誤來防止它：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

這裡是錯誤：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

這條錯誤資訊提到了一個我們還沒有講到的特性：生命週期（lifetimes）。第十章會詳細討論生命週期。不過，即使先不理會和生命週期相關的部分，這條錯誤資訊裡也已經包含了說明這段程式碼為何有問題的關鍵資訊：

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

讓我們仔細看看我們的 `dangle` 程式碼的每個階段到底發生了什麼：

<span class="filename">檔名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

因為 `s` 是在 `dangle` 函式內部建立的，所以當 `dangle` 的程式碼執行完畢後，`s` 就會被釋放。但我們卻嘗試返回對它的引用。這意味著這個引用將指向一個無效的 `String`，這顯然不對！Rust 不允許我們這麼做。

這裡的解決方法是直接返回 `String`：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

這樣就沒有任何錯誤了。所有權被移動出去，所以沒有值被釋放。

### 引用的規則

讓我們概括一下之前對引用的討論：

* 在任意給定時間，**要麼**只能有一個可變引用，**要麼**只能有多個不可變引用。
* 引用必須總是有效的。

接下來，我們來看看另一種不同型別的引用：slice。
