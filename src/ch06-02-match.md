<a id="the-match-control-flow-operator"></a>

## `match` 控制流結構

[ch06-02-match.md](https://github.com/rust-lang/book/blob/602a0d2e898f5e4ff030eac0b457755a10e0be1e/src/ch06-02-match.md)

Rust 有一個叫做 `match` 的極為強大的控制流運算子，它允許我們將一個值與一系列的模式相比較，並根據相匹配的模式執行相應程式碼。模式可由字面值、變數、萬用字元和許多其他內容構成；[第十九章][ch19-00-patterns]會涉及到所有不同種類的模式以及它們的作用。`match` 的力量來源於模式的表現力，以及編譯器能夠確認所有可能情況均已被覆蓋。

可以把 `match` 表示式想象成某種硬幣分類器：硬幣滑入有著不同大小孔洞的軌道，每一個硬幣都會掉入符合它大小的孔洞。同樣地，值也會透過 `match` 的每一個模式，並且在遇到第一個 “符合” 的模式時，值會進入相關聯的程式碼塊並在執行中被使用。

因為剛剛提到了硬幣，讓我們用它們來作為一個使用 `match` 的例子！我們可以編寫一個函式來獲取一個未知的美國硬幣，並以一種類似驗鈔機的方式，確定它是何種硬幣並返回它的美分值，如示例 6-3 中所示。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">示例 6-3：一個列舉和一個以列舉變體作為模式的 `match` 表示式</span>

拆開 `value_in_cents` 函式中的 `match` 來看。首先，我們列出 `match` 關鍵字後跟一個表示式，在這個例子中是 `coin` 的值。這看起來非常像 `if` 所使用的條件表示式，不過這裡有一個非常大的區別：對於 `if`，表示式必須返回一個布林值，而這裡它可以是任何型別的。例子中的 `coin` 的型別是示例 6-3 中定義的 `Coin` 列舉。

接下來是 `match` 的分支。一個分支有兩個部分：一個模式和一些程式碼。第一個分支的模式是值 `Coin::Penny` 而之後的 `=>` 運算子將模式和將要執行的程式碼分開。這裡的程式碼就僅僅是值 `1`。每一個分支之間使用逗號分隔。

當 `match` 表示式執行時，它將結果值按順序與每一個分支的模式相比較。如果模式匹配了這個值，這個模式相關聯的程式碼將被執行。如果模式並不匹配這個值，將繼續執行下一個分支，非常類似一個硬幣分類器。可以擁有任意多的分支：示例 6-3 中的 `match` 有四個分支。

每個分支相關聯的程式碼是一個表示式，而表示式的結果值將作為整個 `match` 表示式的返回值。

如果分支程式碼較短的話通常不使用大括號，正如示例 6-3 中的每個分支都只是返回一個值。如果想要在分支中執行多行程式碼，可以使用大括號，而分支後的逗號是可選的。例如，如下程式碼在每次使用`Coin::Penny` 呼叫時都會打印出 “Lucky penny!”，同時仍然返回程式碼塊最後的值，`1`：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### 繫結值的模式

匹配分支的另一個有用的功能是可以繫結匹配的模式的部分值。這也就是如何從列舉變體中提取值的。

作為一個例子，讓我們修改列舉的一個變體來存放資料。1999 年到 2008 年間，美國在 25 美分的硬幣的一側為 50 個州的每一個都印刷了不同的設計。其他的硬幣都沒有這種區分州的設計，所以只有這些 25 美分硬幣有特殊的價值。可以將這些資訊加入我們的 `enum`，透過改變 `Quarter` 變體來包含一個 `State` 值，示例 6-4 中完成了這些修改：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">示例 6-4：`Quarter` 變體也存放了一個 `UsState` 值的 `Coin` 列舉</span>

想象一下我們的一個朋友嘗試收集所有 50 個州的 25 美分硬幣。在根據硬幣型別分類零錢的同時，也可以報告出每個 25 美分硬幣所對應的州名稱，這樣如果我們的朋友沒有的話，他可以將其加入收藏。

在這些程式碼的匹配表示式中，我們在匹配 `Coin::Quarter` 變體的分支的模式中增加了一個叫做 `state` 的變數。當匹配到 `Coin::Quarter` 時，變數 `state` 將會繫結 25 美分硬幣所對應州的值。接著在那個分支的程式碼中使用 `state`，如下：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

如果呼叫 `value_in_cents(Coin::Quarter(UsState::Alaska))`，`coin` 將是 `Coin::Quarter(UsState::Alaska)`。當將值與每個分支相比較時，沒有分支會匹配，直到遇到 `Coin::Quarter(state)`。這時，`state` 繫結的將會是值 `UsState::Alaska`。接著就可以在 `println!` 表示式中使用這個綁定了，像這樣就可以獲取 `Coin` 列舉的 `Quarter` 變體中內部的州的值。

<a id="matching-with-optiont"></a>

### 匹配 `Option<T>`

我們在之前的部分中使用 `Option<T>` 時，是為了從 `Some` 中取出其內部的 `T` 值；我們還可以像處理 `Coin` 列舉那樣使用 `match` 處理 `Option<T>`！只不過這回比較的不再是硬幣，而是 `Option<T>` 的變體，但 `match` 表示式的工作方式保持不變。

比如我們想要編寫一個函式，它獲取一個 `Option<i32>` ，如果其中含有一個值，將其加一。如果其中沒有值，函式應該返回 `None` 值，而不嘗試執行任何操作。

得益於 `match`，編寫這個函式非常簡單，它將看起來像示例 6-5 中這樣：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">示例 6-5：一個在 `Option<i32>` 上使用 `match` 表示式的函式</span>

讓我們更仔細地檢查 `plus_one` 的第一行操作。當呼叫 `plus_one(five)` 時，`plus_one` 函式體中的 `x` 將會是值 `Some(5)`。接著將其與每個分支比較。

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

值 `Some(5)` 並不匹配模式 `None`，所以繼續進行下一個分支。

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` 與 `Some(i)` 匹配嗎？當然匹配！它們是相同的變體。`i` 綁定了 `Some` 中包含的值，所以 `i` 的值是 `5`。接著匹配分支的程式碼被執行，所以我們將 `i` 的值加一併返回一個含有值 `6` 的新 `Some`。

接著考慮下示例 6-5 中 `plus_one` 的第二個呼叫，這裡 `x` 是 `None`。我們進入 `match` 並與第一個分支相比較。

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

匹配成功！這裡沒有值來加一，所以程式結束並返回 `=>` 右側的值 `None`，因為第一個分支就匹配到了，其他的分支將不再比較。

將 `match` 與列舉相結合在很多場景中都是有用的。你會在 Rust 程式碼中看到很多這樣的模式：`match` 一個列舉，繫結其中的值到一個變數，接著根據其值執行程式碼。這在一開始有點複雜，不過一旦習慣了，你會希望所有語言都擁有它！這一直是使用者的最愛。

### 匹配是窮盡的

`match` 還有另一方面需要討論：這些分支必須覆蓋了所有的可能性。考慮一下 `plus_one` 函式的這個版本，它有一個 bug 並不能編譯：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

我們沒有處理 `None` 的情況，所以這些程式碼會造成一個 bug。幸運的是，這是一個 Rust 知道如何處理的 bug。如果嘗試編譯這段程式碼，會得到這個錯誤：

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust 知道我們沒有覆蓋所有可能的情況甚至知道哪些模式被忘記了！Rust 中的匹配是 **窮盡的**（*exhaustive*）：必須窮舉到最後的可能性來使程式碼有效。特別的在這個 `Option<T>` 的例子中，Rust 防止我們忘記明確的處理 `None` 的情況，這讓我們免於假設擁有一個實際上為空的值，從而使之前提到的價值億萬的錯誤不可能發生。

### 通配模式和 `_` 佔位符

使用列舉，我們也可以針對少數幾個特定值執行特殊操作，而對其他所有值採取預設操作。想象我們正在玩一個遊戲，如果你擲出骰子的值為 3，角色不會移動，而是會得到一頂新奇的帽子。如果你擲出了 7，你的角色將失去一頂新奇的帽子。對於其他的數值，你的角色會在棋盤上移動相應的格子。這是一個實現了上述邏輯的 `match`，骰子的結果是硬編碼而不是一個隨機值，其他的邏輯部分使用了沒有函式體的函式來表示，實現它們超出了本例的範圍：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

對於前兩個分支，匹配模式是字面值 `3` 和 `7`，最後一個分支則涵蓋了所有其他可能的值，模式是我們命名為 `other` 的一個變數。`other` 分支的程式碼透過將其傳遞給 `move_player` 函式來使用這個變數。

即使我們沒有列出 `u8` 所有可能的值，這段程式碼依然能夠編譯，因為最後一個模式將匹配所有未被特殊列出的值。這種通配模式滿足了 `match` 必須被窮盡的要求。請注意，我們必須將通配分支放在最後，因為模式是按順序匹配的。如果我們在通配分支後新增其他分支，Rust 將會警告我們，因為此後的分支永遠不會被匹配到。

Rust 還提供了一個模式，當我們不想使用通配模式獲取的值時，請使用 `_` ，這是一個特殊的模式，可以匹配任意值而不繫結到該值。這告訴 Rust 我們不會使用這個值，所以 Rust 也不會警告我們存在未使用的變數。

讓我們改變遊戲規則：現在，當你擲出的值不是 3 或 7 的時候，你必須再次擲出。這種情況下我們不需要使用這個值，所以我們改動程式碼使用 `_` 來替代變數 `other` ：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

這個例子也滿足窮盡性要求，因為我們在最後一個分支中顯式地忽略了其它值。我們沒有忘記處理任何東西。

最後，讓我們再次改變遊戲規則，如果你擲出 3 或 7 以外的值，你的回合將無事發生。我們可以使用單元值（在[“元組型別”][tuples]<!-- ignore -->一節中提到的空元組）作為 `_` 分支的程式碼：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

在這裡，我們明確告訴 Rust 我們不會使用與前面模式不匹配的值，並且這種情況下我們不想執行任何程式碼。

我們將在[第十九章][ch19-00-patterns]<!-- ignore -->中介紹更多關於模式和匹配的內容。現在，讓我們繼續討論 `if let` 語法，這在 `match` 表示式顯得有些冗長時非常有用。

[tuples]: ch03-02-data-types.html#元組型別
[ch19-00-patterns]: ch19-00-patterns.html
