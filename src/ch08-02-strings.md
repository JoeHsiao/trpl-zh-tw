## 使用字串儲存 UTF-8 編碼的文本

[ch08-02-strings.md](https://github.com/rust-lang/book/blob/2581c23b669eff30c26e036a13475ec5cf70c1b8/src/ch08-02-strings.md)

第四章已經講過一些字串的內容，不過現在讓我們更深入地瞭解它。字串是新晉 Rustacean 們通常會被困住的領域，這是由於三方面理由的結合：Rust 傾向於確保暴露出可能的錯誤，字串是比很多程式設計師所想像的要更為複雜的資料結構，以及 UTF-8。所有這些要素結合起來對於來自其他語言背景的程式設計師就可能顯得很困難了。

在集合章節中討論字串的原因是，字串就是作為位元組的集合外加一些方法實現的，當這些位元組被解釋為文本時，這些方法提供了實用的功能。在本小節中，我們會講到 `String` 中那些任何集合型別都有的操作，比如建立、更新和讀取。也會討論 `String` 與其他集合不一樣的地方，例如索引 `String` 是很複雜的，由於人和計算機理解 `String` 資料方式的不同。

<a id="what-is-a-string"></a>

### 定義字串

我們先定義一下**字串**這一術語的具體意義。Rust 的核心語言中只有一種字串型別，字串 slice `str`，它通常以被借用的形式出現，`&str`。第四章講到了**字串 slices**：它們是一些對儲存在別處的 UTF-8 編碼字串資料的引用。舉例來說，由於字串字面值被儲存在程式的二進位制輸出中，因此它們也是字串 slices。

字串（`String`）型別由 Rust 標準庫提供，而不是編入核心語言，它是一種可增長、可變、可擁有、UTF-8 編碼的字串型別。當 Rustaceans 提及 Rust 中的 "字串 "時，他們可能指的是 `String` 或 string slice `&str` 型別，而不僅僅是其中一種型別。雖然本節主要討論 `String`，但這兩種型別在 Rust 的標準庫中都有大量使用，而且 `String` 和 字串 slices 都是 UTF-8 編碼的。

### 新建字串

很多 `Vec<T>` 上可用的操作在 `String` 中同樣可用，事實上 `String` 被實現為一個帶有一些額外保證、限制和功能的位元組 vector 的封裝。其中一個同樣作用於 `Vec<T>` 和 `String` 函式的例子是用來新建一個例項的 `new` 函式，如示例 8-11 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">示例 8-11：新建一個空的 `String`</span>

這新建了一個叫做 `s` 的空的字串，接著我們可以向其中載入資料。通常字串會有初始資料，因為我們希望一開始就有這個字串。為此，可以使用 `to_string` 方法，它能用於任何實現了 `Display` trait 的型別，比如字串字面值。示例 8-12 展示了兩個例子。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<span class="caption">示例 8-12：使用 `to_string` 方法從字串字面值建立 `String`</span>

這些程式碼會建立包含 `initial contents` 的字串。

也可以使用 `String::from` 函式來從字串字面值建立 `String`。示例 8-13 中的程式碼等同於使用 `to_string`。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<span class="caption">示例 8-13：使用 `String::from` 函式從字串字面值建立 `String`</span>

因為字串應用廣泛，這裡有很多不同的用於字串的通用 API 可供選擇。其中一些可能看起來多餘，不過都有其用武之地！在這個例子中，`String::from` 和 `.to_string` 最終做了完全相同的工作，所以如何選擇就是程式碼風格與可讀性的問題了。

記住字串是 UTF-8 編碼的，所以可以包含任何經過正確編碼的資料，如示例 8-14 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<span class="caption">示例 8-14：在字串中儲存不同語言的問候語</span>

所有這些都是有效的 `String` 值。

### 更新字串

`String` 的大小可以增加，其內容也可以改變，就像可以放入更多資料來改變 `Vec` 的內容一樣。另外，可以方便的使用 `+` 運算子或 `format!` 宏來拼接 `String` 值。

<a id="appending-to-a-string-with-push_str-and-push"></a>

#### 使用 `push_str` 和 `push` 追加字串

可以通過 `push_str` 方法來附加字串 slice，從而使 `String` 變長，如示例 8-15 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<span class="caption">示例 8-15：使用 `push_str` 方法向 `String` 附加字串 slice</span>

執行這兩行程式碼之後，`s` 將會包含 `foobar`。`push_str` 方法採用字串 slice，因為我們並不需要獲取引數的所有權。例如，示例 8-16 中我們希望在將 `s2` 的內容附加到 `s1` 之後還能使用它。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<span class="caption">示例 8-16：將字串 slice 的內容附加到 `String` 後使用它</span>

如果 `push_str` 方法獲取了 `s2` 的所有權，就不能在最後一行打印出其值了。好在程式碼如我們期望那樣工作！

`push` 方法被定義為獲取一個單獨的字元作為引數，並附加到 `String` 中。示例 8-17 展示了使用 `push` 方法將字母 `l` 加入 `String` 的程式碼。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">示例 8-17：使用 `push` 將一個字元加入 `String` 值中</span>

執行這些程式碼之後，`s` 將會包含 `lol`。

<a id="concatenation-with-the--operator-or-the-format-macro"></a>

#### 使用 `+` 運算子或 `format!` 宏拼接字串

通常你會希望將兩個已知的字串合併在一起。一種辦法是像這樣使用 `+` 運算子，如示例 8-18 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">示例 8-18：使用 `+` 運算子將兩個 `String` 值合併到一個新的 `String` 值中</span>

執行完這些程式碼之後，字串 `s3` 將會包含 `Hello, world!`。`s1` 在相加後不再有效的原因，和使用 `s2` 的引用的原因，與使用 `+` 運算子時呼叫的函式簽名有關。`+` 運算子使用了 `add` 函式，這個函式簽名看起來像這樣：

```rust,ignore
fn add(self, s: &str) -> String {
```

在標準庫中你會發現，`add` 的定義使用了泛型和關聯型別。在這裡我們替換為了具體型別，這也正是當使用 `String` 值呼叫這個方法會發生的。第十章會討論泛型。這個簽名提供了理解 `+` 運算那微妙部分的線索。

首先，`s2` 使用了 `&`，意味著我們使用第二個字串的**引用**與第一個字串相加。這是因為 `add` 函式的 `s` 引數：只能將 `&str` 和 `String` 相加，不能將兩個 `String` 值相加。不過等一下 —— `&s2` 的型別是 `&String`, 而不是 `add` 第二個引數所指定的 `&str`。那麼為什麼示例 8-18 還能編譯呢？

之所以能夠在 `add` 呼叫中使用 `&s2`，是因為編譯器可以把 `&String` 引數強制轉換成 `&str`。當呼叫 `add` 方法時，Rust 會使用一種叫做 **deref 強制轉換**（*deref coercion*）的機制，這裡會把 `&s2` 轉換成 `&s2[..]`。第十五章會更深入地討論 deref 強制轉換。因為 `add` 不會獲取 `s` 引數的所有權，所以在這個操作之後，`s2` 仍然是一個有效的 `String`。

其次，可以發現簽名中 `add` 獲取了 `self` 的所有權，因為 `self` **沒有**使用 `&`。這意味著示例 8-18 中的 `s1` 的所有權將被移動到 `add` 呼叫中，之後就不再有效。所以雖然 `let s3 = s1 + &s2;` 看起來就像它會複製兩個字串並建立一個新的字串，而實際上這個語句會獲取 `s1` 的所有權，附加上從 `s2` 中複製的內容，並返回結果的所有權。換句話說，它看起來好像生成了很多複製，不過實際上並沒有：這個實現比複製要更高效。

如果想要級聯多個字串，`+` 運算子的行為就顯得笨重了：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

這時 `s` 的內容會是 `tic-tac-toe`。在有這麼多 `+` 和 `"` 字元的情況下，很難理解具體發生了什麼。對於更為複雜的字串連結，可以使用 `format!` 宏：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

這些程式碼也會將 `s` 設定為 `tic-tac-toe`。`format!` 與 `println!` 的工作原理相同，不過不同於將輸出列印到螢幕上，它返回一個帶有結果內容的 `String`。這個版本就好理解的多，宏 `format!` 生成的程式碼使用引用因此不會獲取任何引數的所有權。

### 索引字串

在很多語言中，通過索引來引用字串中的單獨字元是有效且常見的操作。然而在 Rust 中，如果你嘗試使用索引語法訪問 `String` 的一部分，會出現一個錯誤。考慮一下如示例 8-19 中所示的無效程式碼。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">示例 8-19：嘗試對字串使用索引語法</span>

這段程式碼會導致如下錯誤：

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

錯誤和提示說明了全部問題：Rust 的字串不支援索引。那麼，為什麼會這樣呢？為了回答這個問題，我們必須先聊一聊 Rust 是如何在記憶體中儲存字串的。

#### 內部表現

`String` 是一個 `Vec<u8>` 的封裝。讓我們看看示例 8-14 中一些正確編碼的字串的例子。首先是這一例：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

在這裡，`len` 的值是 `4`，這意味著儲存字串 `"Hola"` 的 vector 的長度是四個位元組：這裡每一個字母的 UTF-8 編碼都佔用一個位元組。下面這一行可能會讓你感到意外（注意這個字串中的首字母是西里爾字母的 *Ze* 而不是數字 3。）：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

如果有人問及該字串的長度，你可能會回答 12。然而，Rust 的回答是 24：這是使用 UTF-8 編碼 “Здравствуйте” 所需要的位元組數，這是因為在這個字串中每個 Unicode 標量值需要兩個位元組儲存。因此一個字串位元組值的索引並不總是對應一個有效的 Unicode 標量值。作為演示，考慮如下無效的 Rust 程式碼：

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

我們已經知道 `answer` 不是第一個字元 `З`。當使用 UTF-8 編碼時，`З` 的第一個位元組是 `208`，第二個是 `151`，所以 `answer` 實際上應該是 `208`，不過 `208` 自身並不是一個有效的字母。返回 `208` 可不是一個請求字串第一個字母的人所希望看到的，不過它是 Rust 在位元組索引 0 位置所能提供的唯一資料。使用者通常不會想要一個位元組值被返回，即使這個字串只有拉丁字母，如果 `&"hi"[0]` 是返回位元組值的有效程式碼，它也會返回 `104` 而不是 `h`。

為了避免返回意外的值並造成不能立刻發現的 bug，Rust 根本不會編譯這些程式碼，並在開發過程中及早杜絕了誤會的發生。

<a id="bytes-and-scalar-values-and-grapheme-clusters-oh-my"></a>

#### 位元組、標量值和字形簇

這引起了關於 UTF-8 的另外一個問題：從 Rust 的角度來講，事實上有三種相關方式可以檢視字串：位元組、標量值和字形簇（最接近人們眼中 **字母**（*letters*）的概念）。

比如這個用梵文書寫的印度語單詞 “नमस्ते”，最終它儲存在 vector 中的 `u8` 值看起來像這樣：

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

這裡有 18 個位元組，也就是計算機最終會儲存的資料。如果從 Unicode 標量值的角度理解它們，也就像 Rust 的 `char` 型別那樣，這些位元組看起來像這樣：

```text
['न', 'म', 'स', '्', 'त', 'े']
```

這裡有六個 `char`，不過第四個和第六個都不是字母，它們是發音符號本身並沒有任何意義。最後，如果以字形簇的角度理解，就會得到人們所說的構成這個單詞的四個字母：

```text
["न", "म", "स्", "ते"]
```

Rust 提供了多種不同的方式來解釋計算機儲存的原始字串資料，這樣程式就可以選擇它需要的表現方式，而無所謂是何種人類語言。

最後一個 Rust 不允許使用索引獲取 `String` 字元的原因是，索引操作預期總是需要常數時間（O(1)）。但是對於 `String` 不可能保證這樣的效能，因為 Rust 必須從開頭到索引位置遍歷來確定有多少有效的字元。

### 字串 slice

索引字串通常是一個壞點子，因為字串索引應該返回的型別是不明確的：位元組值、字元、字形簇或者字串 slice。因此，如果你真的希望使用索引建立字串 slice 時，Rust 會要求你更明確一些。為了更明確索引並表明你需要一個字串 slice，相比使用 `[]` 和單個值的索引，可以使用 `[]` 和一個 range 來建立含特定位元組的字串 slice：

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

這裡，`s` 會是一個 `&str`，它包含字串的頭四個位元組。早些時候，我們提到了這些字母都是兩個位元組長的，所以這意味著 `s` 將會是 `Зд`。

如果嘗試用類似 `&hello[0..1]` 的方式對字元的部分位元組進行 slice，Rust 會在執行時 panic，就跟訪問 vector 中的無效索引時一樣：

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

在使用 range 來建立字串 slice 時要格外小心，因為這麼做可能會使你的程式崩潰。

<a id="methods-for-iterating-over-strings"></a>

### 遍歷字串

操作字串每一部分的最好的方法是明確表示需要字元還是位元組。對於單獨的 Unicode 標量值使用 `chars` 方法。對 “Зд” 呼叫 `chars` 方法會將其分開並返回兩個 `char` 型別的值，接著就可以遍歷其結果來訪問每一個元素了：

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

這些程式碼會打印出如下內容：

```text
З
д
```

另外 `bytes` 方法返回每一個原始位元組，這可能會適合你的使用場景：

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

這些程式碼會打印出組成字串的四個位元組：

```text
208
151
208
180
```

不過請務必記住有效的 Unicode 標量值可能會由不止一個位元組組成。

從字串中獲取如同天城文這樣的字形簇是很複雜的，所以標準庫並沒有提供這個功能。[crates.io](https://crates.io/)<!-- ignore --> 上有些提供這樣功能的 crate。

<a id="strings-are-not-so-simple"></a>

### 處理字串的複雜性

總而言之，字串是複雜的。不同的程式語言會選擇不同的方式，把這種複雜性呈現給程式設計師。Rust 選擇把正確處理 `String` 資料作為所有 Rust 程式的預設行為，這意味著程式設計師必須在一開始就更多地思考如何處理 UTF-8 資料。這種權衡比其他程式語言更直接地暴露了字串的複雜性，但它能避免你在開發週期的後期再去處理那些涉及非 ASCII 字元的錯誤。

好訊息是，標準庫圍繞 `String` 和 `&str` 構建了很多功能，來幫助我們正確處理這些複雜場景。請務必檢視相關文件，瞭解一些有用的方法，例如用 `contains` 搜尋字串，或用 `replace` 把字串的一部分替換成另一段字串。

現在讓我們轉向一種稍微沒那麼複雜的集合：雜湊對映！
