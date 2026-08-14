## 什麼是所有權？

[ch04-01-what-is-ownership.md](https://github.com/rust-lang/book/blob/05d114287b7d6f6c9253d5242540f00fbd6172ab/src/ch04-01-what-is-ownership.md)

**所有權**（*ownership*）是 Rust 用於如何管理記憶體的一組規則。所有程式都必須管理其執行時使用計算機記憶體的方式。一些語言中具有垃圾回收機制，在程式執行時有規律地尋找不再使用的記憶體；在另一些語言中，程式設計師必須親自分配和釋放記憶體。Rust 則選擇了第三種方式：透過所有權系統管理記憶體，編譯器在編譯時會根據一系列的規則進行檢查。如果違反了任何這些規則，程式都不能編譯。在執行時，所有權系統的任何功能都不會減慢程式的執行。

因為所有權對很多程式設計師來說都是一個新概念，需要一些時間來適應。好訊息是隨著你對 Rust 和所有權系統的規則越來越有經驗，你就越能自然地編寫出安全和高效的程式碼。持之以恆！

當你理解了所有權，你將有一個堅實的基礎來理解那些使 Rust 獨特的功能。在本章中，你將透過完成一些示例來學習所有權，這些示例基於一個常用的資料結構：字串。

> ### 棧（Stack）與堆（Heap）
>
> 在很多語言中，你並不需要經常考慮到棧與堆。不過在像 Rust 這樣的系統程式語言中，值是位於棧上還是堆上在更大程度上影響了語言的行為以及為何必須做出這樣的抉擇。我們會在本章的稍後部分描述所有權與棧和堆相關的內容，所以這裡只是一個用來預熱的簡要解釋。
>
> 棧和堆都是程式碼在執行時可供使用的記憶體，但是它們的結構不同。棧以放入值的順序儲存值並以相反順序取出值。這也被稱作 **後進先出**（*last in, first out*）。想象一下一疊盤子：當增加更多盤子時，把它們放在盤子堆的頂部，當需要盤子時，也從頂部拿走。不能從中間也不能從底部增加或拿走盤子！增加資料叫做 **入棧**（*pushing onto the stack*），而移出資料叫做 **出棧**（*popping off the stack*）。棧中的所有資料都必須佔用已知且固定的大小。在編譯時大小未知或大小可能變化的資料，要改為儲存在堆上。
>
> 堆是缺乏組織的：當向堆放入資料時，你要請求一定大小的空間。記憶體分配器（memory allocator）在堆的某處找到一塊足夠大的空位，把它標記為已使用，並返回一個表示該位置地址的 **指標**（*pointer*）。這個過程稱作 **在堆上分配記憶體**（*allocating on the heap*），有時簡稱為 “分配”（allocating）。（將資料推入棧中並不被認為是分配）。因為指向放入堆中資料的指標是已知的並且大小是固定的，你可以將該指標儲存在棧上，不過當需要實際資料時，必須訪問指標。想象一下去餐館就座吃飯。當進入時，你說明有幾個人，餐館員工會找到一個夠大的空桌子並領你們過去。如果有人來遲了，他們也可以透過詢問來找到你們坐在哪。
>
> 入棧比在堆上分配記憶體要快，因為（入棧時）分配器無需為儲存新資料去搜索記憶體空間；其位置總是在棧頂。相比之下，在堆上分配記憶體則需要更多的工作，這是因為分配器必須首先找到一塊足夠存放資料的記憶體空間，並接著做一些記錄為下一次分配做準備。
>
> 訪問堆上的資料比訪問棧上的資料慢，因為必須透過指標來訪問。現代處理器在記憶體中跳轉越少就越快。繼續類比，假設有一個服務員在餐廳裡處理多個桌子的點菜。在一個桌子報完所有菜後再移動到下一個桌子是最有效率的。從桌子 A 聽一個菜，接著桌子 B 聽一個菜，然後再桌子 A，然後再桌子 B 這樣的流程會更加緩慢。出於同樣原因，處理器在處理的資料彼此較近的時候（比如在棧上）比較遠的時候（比如可能在堆上）更高效。
>
> 當你的程式碼呼叫一個函式時，傳遞給函式的值（包括可能指向堆上資料的指標）和函式的區域性變數被壓入棧中。當函式結束時，這些值被移出棧。
>
> 跟蹤哪部分程式碼正在使用堆上的哪些資料，最大限度的減少堆上的重複資料的數量，以及清理堆上不再使用的資料確保不會耗盡空間，這些問題正是所有權系統要處理的。一旦理解了所有權，你就不需要經常考慮棧和堆了，不過明白了所有權的主要目的就是管理堆資料，能夠幫助解釋為什麼所有權要以這種方式工作。

### 所有權規則

首先，讓我們看一下所有權的規則。當我們透過舉例說明時，請謹記這些規則：

> 1. Rust 中的每一個值都有一個 **所有者**（*owner*）。
> 2. 值在任一時刻有且只有一個所有者。
> 3. 當所有者離開作用域，這個值將被丟棄。

### 變數作用域

既然我們已經掌握了基本語法，將不會在之後的例子中包含 `fn main() {` 程式碼，所以如果你是一路跟過來的，必須手動將之後例子的程式碼放入一個 `main` 函式中。這樣，例子將顯得更加簡明，使我們可以關注實際細節而不是樣板程式碼。

在所有權的第一個例子中，我們看看一些變數的 **作用域**（*scope*）。作用域是一個項（item）在程式中有效的範圍。假設有這樣一個變數：

```rust
let s = "hello";
```

變數 `s` 繫結到了一個字串字面值，這個字串值是硬編碼程序序程式碼中的。這個變數從宣告的點開始直到當前**作用域**結束時都是有效的。示例 4-1 中的註釋標明瞭變數 `s` 在何處是有效的。

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<span class="caption">示例 4-1：一個變數和其有效的作用域</span>

換句話說，這裡有兩個重要的時間點：

* 當 `s` **進入作用域**時，它就是有效的。
* 這一直持續到它**離開作用域**為止。

目前為止，變數是否有效與作用域的關係跟其他程式語言是類似的。現在我們在此基礎上介紹 `String` 型別。

### `String` 型別

為了演示所有權的規則，我們需要一個比第三章 [“資料型別”][data-types] 中講到的都要複雜的資料型別。前面介紹的型別都是已知大小的，可以儲存在棧中，並且當離開作用域時被移出棧，如果程式碼的另一部分需要在不同的作用域中使用相同的值，可以快速簡單地複製它們來建立一個新的獨立例項。不過我們需要尋找一個儲存在堆上的資料來探索 Rust 是如何知道該在何時清理資料的，而 `String` 型別就是一個很好的例子。

我們會專注於 `String` 與所有權相關的部分。這些方面也同樣適用於標準庫提供的或你自己建立的其他複雜資料型別。在[第八章][ch8]會更深入地講解 `String`。

我們已經見過字串字面值，即被硬編碼程序序裡的字串值。字串字面值是很方便的，不過它們並不適合使用文字的每一種場景。原因之一就是它們是不可變的。另一個原因是並非所有字串的值都能在編寫程式碼時就知道：例如，要是想獲取使用者輸入並存儲該怎麼辦呢？為此，Rust 有另一種字串型別，`String`。這個型別管理被分配到堆上的資料，所以能夠儲存在編譯時未知大小的文字。可以使用 `from` 函式基於字串字面值來建立 `String`，如下：

```rust
let s = String::from("hello");
```

這兩個冒號 `::` 是運算子，允許將特定的 `from` 函式置於 `String` 型別的名稱空間（namespace）下，而不需要使用類似 `string_from` 這樣的名字。在第五章的 [“方法”][methods] 部分會更詳細地講解這個語法，而且在第七章的 [“路徑用於引用模組樹中的項”][paths-module-tree] 中會講到模組的名稱空間。

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

那麼這裡有什麼區別呢？為什麼 `String` 可變而字面值卻不行呢？區別在於兩個型別對記憶體的處理上。

### 記憶體與分配

就字串字面值來說，我們在編譯時就知道其內容，所以文字被直接硬編碼進最終的可執行檔案中。這使得字串字面值快速且高效。不過這些特性都只得益於字串字面值的不可變性。不幸的是，我們不能為了每一個在編譯時大小未知的文字而將一塊記憶體放入二進位制檔案中，並且它的大小還可能隨著程式執行而改變。

對於 `String` 型別，為了支援一個可變，可增長的文字片段，需要在堆上分配一塊在編譯時未知大小的記憶體來存放內容。這意味著：

* 必須在執行時向記憶體分配器（memory allocator）請求記憶體。
* 需要一個當我們處理完 `String` 時將記憶體返回給分配器的方法。

第一部分由我們完成：當呼叫 `String::from` 時，它的實現 (*implementation*) 請求其所需的記憶體。這在程式語言中是非常通用的。

然而，第二部分實現起來就各有區別了。在有 **垃圾回收**（*garbage collector*，*GC*）的語言中，GC 記錄並清除不再使用的記憶體，而我們並不需要關心它。在大部分沒有 GC 的語言中，識別出不再使用的記憶體並呼叫程式碼顯式釋放就是我們的責任了，跟請求記憶體的時候一樣。從歷史的角度上說正確處理記憶體回收曾經是一個困難的程式設計問題。如果忘記回收了會浪費記憶體。如果過早回收了，將會出現無效變數。如果重複回收，這也是個 bug。我們需要精確的為一個 `allocate` 配對一個 `free`。

Rust 採取了一個不同的策略：記憶體在擁有它的變數離開作用域後就被自動釋放。下面是示例 4-1 中作用域例子的一個使用 `String` 而不是字串字面值的版本：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

這是一個將 `String` 需要的記憶體返回給分配器的很自然的位置：當 `s` 離開作用域的時候。當變數離開作用域，Rust 為我們呼叫一個特殊的函式。這個函式叫做 [`drop`][drop]，在這裡 `String` 的作者可以放置釋放記憶體的程式碼。Rust 在結尾的 `}` 處自動呼叫 `drop`。

> 注意：在 C++ 中，這種 item 在生命週期結束時釋放資源的模式有時被稱作 **資源獲取即初始化**（*Resource Acquisition Is Initialization (RAII)*）。如果你使用過 RAII 模式的話應該對 Rust 的 `drop` 函式並不陌生。

這個模式對編寫 Rust 程式碼的方式有著深遠的影響。現在它看起來很簡單，不過在更復雜的場景下程式碼的行為可能是不可預測的，比如當有多個變數使用在堆上分配的記憶體時。現在讓我們探索一些這樣的場景。

<a id="ways-variables-and-data-interact-move"></a>

#### 使用移動的變數與資料互動

在 Rust 中，多個變數可以採取不同的方式與同一資料進行互動。讓我們看看示例 4-2 中一個使用整型的例子。

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<span class="caption">示例 4-2：將變數 `x` 的整數值賦給 `y`</span>

我們大致可以猜到這在幹什麼：“將 `5` 繫結到 `x`；接著生成一個值 `x` 的複製並繫結到 `y`”。現在有了兩個變數，`x` 和 `y`，都等於 `5`。這也正是事實上發生了的，因為整數是有已知固定大小的簡單值，所以這兩個 `5` 被壓入了棧中。

現在看看這個 `String` 版本：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

這看起來與上面的程式碼非常類似，所以我們可能會假設它們的執行方式也是類似的：也就是說，第二行可能會生成一個 `s1` 的複製並繫結到 `s2` 上。但事實並非如此。

看看圖 4-1 以瞭解 `String` 的底層會發生什麼。`String` 由三部分組成，如圖左側所示：一個指向存放字串內容記憶體的指標，一個長度，和一個容量。這一組資料儲存在棧上。右側則是堆上存放內容的記憶體部分。

<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">圖 4-1：將值 `"hello"` 繫結給 `s1` 的 `String` 在記憶體中的表現形式</span>

長度表示 `String` 的內容當前使用了多少位元組的記憶體。容量是 `String` 從分配器總共獲取了多少位元組的記憶體。長度與容量的區別是很重要的，不過在當前上下文中並不重要，所以現在可以忽略容量。

當我們將 `s1` 賦值給 `s2`，`String` 的資料被複制了，這意味著我們從棧上複製了它的指標、長度和容量。我們並沒有複製指標指向的堆上資料。換句話說，記憶體中資料的表現如圖 4-2 所示。

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">圖 4-2：變數 `s2` 的記憶體表現，它有一份 `s1` 指標、長度和容量的複製</span>

這個表現形式看起來**並不像**圖 4-3 中的那樣，如果 Rust 也複製了堆上的資料，那麼記憶體看起來就是這樣的。如果 Rust 這麼做了，那麼操作 `s2 = s1` 在堆上資料比較大的時候會對執行時效能造成非常大的影響。

<img alt="Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">圖 4-3：另一個 `s2 = s1` 時可能的記憶體表現，如果 Rust 同時也複製了堆上的資料的話</span>

之前我們提到過當變數離開作用域後，Rust 自動呼叫 `drop` 函式並清理變數的堆記憶體。不過圖 4-2 展示了兩個資料指標指向了同一位置。這就有了一個問題：當 `s2` 和 `s1` 離開作用域，它們都會嘗試釋放相同的記憶體。這是一個叫做 **二次釋放**（*double free*）的錯誤，也是之前提到過的記憶體安全性 bug 之一。兩次釋放（相同）記憶體會導致記憶體汙染，它可能會導致潛在的安全漏洞。

為了確保記憶體安全，在 `let s2 = s1;` 之後，Rust 認為 `s1` 不再有效，因此 Rust 不需要在 `s1` 離開作用域後清理任何東西。看看在 `s2` 被建立之後嘗試使用 `s1` 會發生什麼；這段程式碼不能執行：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

你會得到一個類似如下的錯誤，因為 Rust 禁止你使用無效的引用。

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

如果你在其他語言中聽說過術語 **淺複製**（*shallow copy*）和 **深複製**（*deep copy*），那麼複製指標、長度和容量而不複製資料可能聽起來像淺複製。不過因為 Rust 同時使第一個變數無效了，這個操作被稱為 **移動**（*move*），而不是叫做淺複製。上面的例子可以解讀為 `s1` 被 **移動** 到了 `s2` 中。那麼具體發生了什麼，如圖 4-4 所示。

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">圖 4-4：`s1` 無效之後的記憶體表現</span>

這樣就解決了我們的問題！因為只有 `s2` 是有效的，當其離開作用域，它就釋放自己的記憶體，完畢。

另外，這裡還隱含了一個設計選擇：Rust 永遠也不會自動建立資料的 “深複製”。因此，任何**自動**的複製都可以被認為是對執行時效能影響較小的。

#### 作用域與賦值

作用域、所有權和透過 `drop` 函式釋放記憶體之間的關係反過來也同樣成立。當你給一個已有的變數賦一個全新的值時，Rust 將會立即呼叫 `drop` 並釋放原始值的記憶體。例如，考慮如下程式碼：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04b-replacement-drop/src/main.rs:here}}
```

起初我們聲明瞭變數 `s` 並繫結為一個 `"hello"` 值的 `String`。接著立即建立了一個值為 `"ahoy"` 的 `String` 並賦值給 `s`。在這裡，完全沒有任何內容指向了原始堆上的值。

<img alt="One table s representing the string value on the stack, pointing to
the second piece of string data (ahoy) on the heap, with the original string
data (hello) grayed out because it cannot be accessed anymore."
src="img/trpl04-05.svg"
class="center"
style="width: 50%;"
/>

<span class="caption">圖 4-5: 當初始值被整體替換後的記憶體表現</span>

因此原始的字串立刻就離開了作用域。Rust 會在其上執行 `drop` 函式同時記憶體會馬上釋放。當結尾列印其值時，將會是 `"ahoy, world!"`。

<a id="ways-variables-and-data-interact-clone"></a>

#### 使用克隆的變數與資料互動

如果我們 **確實** 需要深度複製 `String` 中堆上的資料，而不僅僅是棧上的資料，可以使用一個叫做 `clone` 的常用方法。第五章會討論方法語法，不過因為方法在很多語言中是一個常見功能，所以之前你可能已經見過了。

這是一個實際使用 `clone` 方法的例子：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

這段程式碼能正常執行，並且明確產生圖 4-3 中行為，這裡堆上的資料**確實**被複制了。

當出現 `clone` 呼叫時，你知道一些特定的程式碼被執行而且這些程式碼可能相當消耗資源。你很容易察覺到一些不尋常的事情正在發生。

#### 只在棧上的資料：複製

這裡還有一個沒有提到的細節。這些程式碼使用了整型並且是有效的，它們是示例 4-2 中的一部分：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

但這段程式碼似乎與我們剛剛學到的內容相矛盾：沒有呼叫 `clone`，不過 `x` 依然有效且沒有被移動到 `y` 中。

原因是像整型這樣的在編譯時已知大小的型別被整個儲存在棧上，所以複製其實際的值是快速的。這意味著沒有理由在建立變數 `y` 後使 `x` 無效。換句話說，這裡沒有深淺複製的區別，所以這裡呼叫 `clone` 並不會與通常的淺複製有什麼不同，我們可以不用管它。

Rust 有一個叫做 `Copy` trait 的特殊註解，可以用在類似整型這樣的儲存在棧上的型別上（[第十章][traits]將會詳細講解 trait）。如果一個型別實現了 `Copy` trait，那麼一箇舊的變數在將其賦值給其他變數後仍然有效。

Rust 不允許自身或其任何部分實現了 `Drop` trait 的型別使用 `Copy` trait。如果我們對其值離開作用域時需要特殊處理的型別使用 `Copy` 註解，將會出現一個編譯時錯誤。要學習如何為你的型別新增 `Copy` 註解以實現該 trait，請閱讀附錄 C 中的 [“可派生的 trait”][derivable-traits]。

那麼哪些型別實現了 `Copy` trait 呢？你可以檢視給定型別的文件來確認，不過作為一個通用的規則，任何一組簡單標量值的組合都可以實現 `Copy`，任何不需要分配記憶體或某種形式資源的型別都可以實現 `Copy` 。如下是一些 `Copy` 的型別：

* 所有整數型別，比如 `u32`。
* 布林型別，`bool`，它的值是 `true` 和 `false`。
* 所有浮點數型別，比如 `f64`。
* 字元型別，`char`。
* 元組，當且僅當其包含的型別也都實現 `Copy` 的時候。比如，`(i32, i32)` 實現了 `Copy`，但 `(i32, String)` 就沒有。

### 所有權與函式

將值傳遞給函式與給變數賦值的原理相似。向函式傳遞值可能會移動或者複製，就像賦值語句一樣。示例 4-3 使用註釋展示變數何時進入和離開作用域：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<span class="caption">示例 4-3：帶有所有權和作用域註釋的函式</span>

當嘗試在呼叫 `takes_ownership` 後使用 `s` 時，Rust 會丟擲一個編譯時錯誤。這些靜態檢查使我們免於犯錯。試試在 `main` 函式中新增使用 `s` 和 `x` 的程式碼來看看哪裡能使用它們，以及所有權規則會在哪裡阻止我們這麼做。

### 返回值與作用域

返回值也可以轉移所有權。示例 4-4 展示了一個返回了某些值的示例，與示例 4-3 一樣帶有類似的註釋。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<span class="caption">示例 4-4: 轉移返回值的所有權</span>

變數的所有權總是遵循相同的模式：將值賦給另一個變數時它會移動。當持有堆中資料值的變數離開作用域時，其值將透過 `drop` 被清理掉，除非資料被移動為另一個變數所有。

雖然這樣是可以的，但是在每一個函式中都獲取所有權並接著返回所有權有些囉嗦。如果我們想要函式使用一個值但不獲取所有權該怎麼辦呢？如果我們還要接著使用它的話，每次都傳進去再返回來就有點煩人了，除此之外，我們也可能想返回函式體中產生的一些資料。

我們可以使用元組來返回多個值，如示例 4-5 所示。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<span class="caption">示例 4-5: 返回引數的所有權</span>

但是這未免有些形式主義，而且這種場景應該很常見。幸運的是，Rust 對此提供了一個不用獲取所有權就可以使用值的功能，叫做 **引用**（*references*）。

[data-types]: ch03-02-data-types.html#資料型別
[ch8]: ch08-02-strings.html
[derivable-traits]: appendix-03-derivable-traits.html
[methods]: ch05-03-method-syntax.html#方法
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[traits]: ch10-02-traits.html
[drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
