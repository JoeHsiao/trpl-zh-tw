## 不安全 Rust

[ch20-01-unsafe-rust.md](https://github.com/rust-lang/book/blob/57ff62db22b006e6b319e2e35c9364d932a8b4e5/src/ch20-01-unsafe-rust.md)

目前為止討論過的程式碼都有 Rust 在編譯時會強制執行的記憶體安全保證。然而，Rust 還隱藏有第二種語言，它不會強制執行這類記憶體安全保證：這被稱為 **不安全 Rust**（*unsafe Rust*）。它與常規 Rust 程式碼無異，但是會提供額外的超能力。

不安全 Rust 之所以存在，是因為靜態分析本質上是保守的。當編譯器嘗試確定一段程式碼是否支援某個保證時，拒絕一些合法的程式比接受無效的程式要好一些。這必然意味著有時程式碼**可能**是合法的，但如果 Rust 編譯器沒有足夠的資訊來確定，它將拒絕該程式碼。在這種情況下，可以使用不安全程式碼告訴編譯器，“相信我，我知道自己在幹什麼。” 不過千萬注意，使用不安全 Rust 風險自擔：如果不安全程式碼出錯了，比如解引用空指標，可能會導致不安全的記憶體使用。

另一個 Rust 存在不安全一面的原因是底層計算機硬體固有的不安全性。如果 Rust 不允許進行不安全操作，那麼有些任務則根本完成不了。Rust 需要能夠進行像直接與作業系統互動甚至於編寫你自己的作業系統這樣的底層系統程式設計。底層系統程式設計也是 Rust 語言的目標之一。讓我們看看不安全 Rust 能做什麼，和怎麼做。

### 執行不安全的超能力

要切換到 `unsafe Rust`，可以使用 `unsafe` 關鍵字，然後開啟一個包含不安全程式碼的新塊。這裡有五類可以在不安全 Rust 中進行而不能用於安全 Rust 的操作，它們稱之為**不安全的超能力**（**unsafe superpowers**）。這些超能力包括：

* 解引用裸指標
* 呼叫不安全的函式或方法
* 訪問或修改可變靜態變數
* 實現不安全 trait
* 訪問 `union` 的欄位

有一點很重要，`unsafe` 並不會關閉借用檢查器或停用任何其他 Rust 安全檢查：如果在不安全程式碼中使用引用，它仍會被檢查。`unsafe` 關鍵字只是提供了那五個不會被編譯器檢查記憶體安全的功能。你仍然能在不安全塊中獲得某種程度的安全。

再者，`unsafe` 不意味著塊中的程式碼就一定是危險的或者必然導致記憶體安全問題：其意圖在於作為程式設計師，你將會確保 `unsafe` 塊中的程式碼以有效的方式訪問記憶體。

人難免出錯，錯誤總會發生，不過透過要求這五類不安全操作必須位於標記為 `unsafe` 的塊中，就能夠知道任何與記憶體安全相關的錯誤必定位於 `unsafe` 塊內。保持 `unsafe` 塊儘可能小；如此當之後調查記憶體 bug 時就會感謝你自己了。

為了儘可能隔離不安全程式碼，最好將不安全程式碼封裝進一個安全的抽象並提供安全 API，當我們學習不安全函式和方法時會討論到。標準庫的一部分被實現為在被評審過的不安全程式碼之上的安全抽象。這個技術防止了 `unsafe` 洩露到所有你或者使用者希望使用由 `unsafe` 程式碼實現的功能的地方，因為使用其安全抽象是安全的。

讓我們按順序依次介紹上述五類超能力，同時我們會看到一些提供不安全程式碼的安全介面的抽象。

### 解引用裸指標

回到第四章的[“懸垂引用”][dangling-references]一節，那裡提到了編譯器會確保引用總是有效的。不安全 Rust 有兩個被稱為 **裸指標**（*raw pointers*）的類似於引用的新型別。和引用一樣，裸指標是不可變或可變的，分別寫作 `*const T` 和 `*mut T`。這裡的星號不是解引用運算子；它是型別名稱的一部分。在裸指標的上下文中，**不可變** 意味著指標解引用之後不能直接賦值。

裸指標與引用和智慧指標的區別在於

* 允許忽略借用規則，可以同時擁有不可變和可變的指標，或多個指向相同位置的可變指標
* 不保證指向有效的記憶體
* 允許為空
* 不能實現任何自動清理功能

透過去掉 Rust 強加的保證，你可以放棄安全保證以換取效能或使用另一個語言或硬體介面的能力，此時 Rust 的保證並不適用。

示例 20-1 展示瞭如何建立一個不可變裸指標和一個可變裸指標。

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

<span class="caption">示例 20-1: 透過引用建立裸指標</span>

注意這段程式碼中沒有引入 `unsafe` 關鍵字。可以在安全程式碼中建立裸指標；只是不能在不安全塊之外解引用裸指標，稍後便會看到。

我們透過使用裸指標借用運算子（raw borrow operators）建立裸指標：`&raw const num` 會建立一個 `*const i32` 的不可變裸指標。因為由於我們是直接從一個區域性變數建立它們的，因此可以確定這些特定的裸指標是有效的，但是不能對任何裸指標都做出如此假設。

為了演示這一點，接下來我們將建立一個有效性無法確定的裸指標，使用 `as` 進行型別轉換而不是使用裸指標借用運算子。示例 20-2 展示瞭如何建立一個指向任意記憶體地址的裸指標。嘗試使用任意記憶體是未定義行為：此地址可能有資料也可能沒有，編譯器可能會最佳化掉這個記憶體訪問，或者程式可能因段錯誤（segmentation fault）而終止。通常在有裸指標借用運算子可用的情況下，沒有充分的理由編寫這樣的程式碼，但這確實是可行的。

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

<span class="caption">示例 20-2: 建立指向任意記憶體地址的裸指標</span>

記得我們說過可以在安全程式碼中建立裸指標，但不能 **解引用** 裸指標和讀取其指向的資料。示例 20-3 中，我們在裸指標上使用瞭解引用運算子 `*`，該操作需要一個 `unsafe` 塊：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

<span class="caption">示例 20-3: 在 `unsafe` 塊中解引用裸指標</span>

建立一個指標不會造成任何危害；只有當訪問其指向的值時才有可能遇到無效的值。

還需注意示例 20-1 和 20-3 中建立了同時指向相同記憶體位置 `num` 的裸指標 `*const i32` 和 `*mut i32`。相反如果嘗試同時建立 `num` 的不可變和可變引用，程式碼將無法透過編譯，因為 Rust 的所有權規則不允許在擁有任何不可變引用的同時再建立可變引用。透過裸指標，就能夠同時建立同一地址的可變指標和不可變指標，若透過可變指標修改資料，則可能造成潛在資料競爭。請多加小心！

既然存在這麼多的危險，為何還要使用裸指標呢？一個主要的應用場景便是呼叫 C 程式碼介面，這在下一部分 [“呼叫不安全函式或方法”](#呼叫不安全函式或方法) 中會講到。另一個場景是構建借用檢查器無法理解的安全抽象。讓我們先介紹不安全函式，接著看一看使用不安全程式碼的安全抽象的示例。

### 呼叫不安全函式或方法

第二類可以在不安全塊中進行的操作是呼叫不安全函式。不安全函式和方法與常規函式方法十分類似，除了其開頭有一個額外的 `unsafe`。在此上下文中，關鍵字 `unsafe` 表示該函式具有呼叫時需要滿足的要求，而 Rust 不會保證滿足這些要求。透過在 `unsafe` 塊中呼叫不安全函式，表明我們已經閱讀過此函式的文件並對其是否滿足函式自身的契約負責。

如下是一個沒有做任何操作的不安全函式 `dangerous` 的例子：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

必須在一個單獨的 `unsafe` 塊中呼叫 `dangerous` 函式。如果嘗試不使用 `unsafe` 塊呼叫 `dangerous`，則會得到一個錯誤：

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

透過 `unsafe` 塊，我們向 Rust 斷言我們已經閱讀過函式的文件，理解如何正確使用它，並核實我們履行了該函式的契約。

在不安全函式的函式體內部執行不安全操作時，同樣需要使用 `unsafe` 塊，就像在普通函式中一樣，如果忘記了，編譯器會發出警告。這有助於將 `unsafe` 塊保持得儘可能小，因為不安全操作未必需要覆蓋整個函式體。

#### 建立不安全程式碼的安全抽象

僅僅因為函式包含不安全程式碼並不意味著整個函式都需要標記為不安全的。事實上，將不安全程式碼封裝進安全函式是一種常見的抽象方式。作為一個例子，瞭解一下標準庫中的函式 `split_at_mut`，它需要一些不安全程式碼，讓我們探索可以如何實現它。這個安全函式定義於可變 slice 之上：它獲取一個 slice 並從給定的索引引數開始將其分割為兩個 slice。示例 20-4 展示瞭如何使用 `split_at_mut`。

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

<span class="caption">示例 20-4: 使用安全的 `split_at_mut` 函式</span>

這個函式無法只通過安全 Rust 實現。一個嘗試可能看起來像示例 20-5，它不能編譯。出於簡單考慮，我們將 `split_at_mut` 實現為函式而不是方法，並只處理 `i32` 值而非泛型 `T` 的 slice。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

<span class="caption">示例 20-5: 嘗試只使用安全 Rust 來實現 `split_at_mut`</span>

此函式首先獲取 slice 的長度，然後透過檢查引數是否小於或等於這個長度來斷言引數所給定的索引位於 slice 當中。該斷言意味著如果傳入的索引比要分割的 slice 的索引更大，此函式在嘗試使用這個索引前 panic。

之後我們在一個元組中返回兩個可變的 slice：一個從原始 slice 的開頭直到 `mid` 索引，另一個從 `mid` 直到原 slice 的結尾。

如果嘗試編譯示例 20-5 的程式碼，會得到一個錯誤：

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust 的借用檢查器無法理解我們要借用這個 slice 的兩個不同部分：它只知道我們借用了同一個 slice 兩次。本質上借用 slice 的不同部分是可以的，因為這兩段 slice 不會重疊，不過 Rust 還沒有智慧到能夠理解這些。當我們知道某些事是可以的而 Rust 不知道的時候，就是觸及不安全程式碼的時候了

示例 20-6 展示瞭如何使用 `unsafe` 塊，裸指標和一些不安全函式呼叫來實現 `split_at_mut`：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

<span class="caption">示例 20-6: 在 `split_at_mut` 函式的實現中使用不安全程式碼</span>

回憶第四章的[“Slice 型別” ][the-slice-type]部分，slice 是一個指向一些資料的指標，並帶有該 slice 的長度。可以使用 `len` 方法獲取 slice 的長度，使用 `as_mut_ptr` 方法訪問 slice 的裸指標。在這個例子中，因為有一個 `i32` 值的可變 slice，`as_mut_ptr` 返回一個 `*mut i32` 型別的裸指標，並將其儲存在 `ptr` 變數中。

我們保持索引 `mid` 位於 slice 中的斷言。接著是不安全程式碼：`slice::from_raw_parts_mut` 函式獲取一個裸指標和一個長度來建立一個 slice。這裡使用此函式從 `ptr` 中建立了一個有 `mid` 個項的 slice。之後在 `ptr` 上呼叫 `add` 方法並使用 `mid` 作為引數來獲取一個從 `mid` 開始的裸指標，使用這個裸指標並以 `mid` 之後項的數量為長度建立另一個 slice。

`slice::from_raw_parts_mut` 函式是不安全的因為它獲取一個裸指標，並必須確信這個指標是有效的。裸指標上的 `add` 方法也是不安全的，因為其必須確信此地址偏移量也是有效的指標。因此必須將 `slice::from_raw_parts_mut` 和 `add` 放入 `unsafe` 塊中以便能呼叫它們。透過觀察程式碼，和增加 `mid` 必然小於等於 `len` 的斷言，我們可以說 `unsafe` 塊中所有的裸指標將是有效的 slice 中資料的指標。這是一個可以接受的 `unsafe` 的恰當用法。

注意無需將 `split_at_mut` 函式的結果標記為 `unsafe`，並可以在安全 Rust 中呼叫此函式。我們建立了一個不安全程式碼的安全抽象，其程式碼以一種安全的方式使用了 `unsafe` 程式碼，因為其只從這個函式訪問的資料中建立了有效的指標。

與此相對，示例 20-7 中的 `slice::from_raw_parts_mut` 在使用 slice 時很有可能會崩潰。這段程式碼獲取任意記憶體地址並建立了一個長度為一萬的 slice：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

<span class="caption">示例 20-7: 透過任意記憶體地址建立 slice</span>

我們並不擁有這個任意地址的記憶體，也不能保證這段程式碼建立的 slice 包含有效的 `i32` 值。試圖使用臆測為有效的 `values` 會導致未定義的行為。

#### 使用 `extern` 函式呼叫外部程式碼

有時你的 Rust 程式碼可能需要與其他語言編寫的程式碼互動。為此 Rust 有一個關鍵字，`extern`，有助於建立和使用 **外部函式介面**（*Foreign Function Interface*，FFI）。外部函式介面是一個程式語言用以定義函式的方式，其允許不同（外部）程式語言呼叫這些函式。

示例 20-8 展示瞭如何整合 C 標準庫中的 `abs` 函式。`extern` 塊中宣告的函式在 Rust 程式碼中通常是不安全的因此 `extern` 塊本身也必須標註 `unsafe`。之所以如此，是因為其他語言不會強制執行 Rust 的規則，Rust 也無法檢查這些約束，因此程式設計師有責任確保呼叫的安全性。

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

<span class="caption">示例 20-8: 宣告並呼叫另一個語言中定義的 `extern` 函式</span>

在 `unsafe extern "C"` 塊中，我們列出了希望能夠呼叫的另一個語言中的外部函式的簽名和名稱。`"C"` 部分定義了外部函式所使用的 **應用二進位制介面**（*application binary interface*，ABI） —— ABI 定義瞭如何在組合語言層面呼叫此函式。`"C"` ABI 是最常見的，並遵循 C 程式語言的 ABI。有關 Rust 支援的所有 ABI 的資訊請參見 [the Rust Reference][ABI]。

`unsafe extern` 中宣告的任何項都隱式地是 `unsafe` 的。然而，一些 FFI 函式**可以**安全地呼叫。例如，C 標準庫中的 `abs` 函式沒有任何記憶體安全方面的考量並且我們知道它可以使用任何 `i32` 呼叫。在類似這樣的例子中，我們可以使用 `safe` 關鍵字來表明這個特定的函式即便是在 `unsafe extern` 塊中也是可以安全呼叫的。一旦我們做出這個修改，呼叫它不再需要 `unsafe` 塊，如示例 20-9 所示。

<figure class="listing">

<span class="file-name">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

<figcaption>示例 20-9：在 `unsafe extern` 塊中顯式地標記一個函式為 `safe` 並安全地呼叫它</figcaption>

</figure>

將一個函式標記為 `safe` 並不會固有地使其變得安全！相反，這像是一個對 Rust 的承諾表明它**是**安全的。確保履行這個承諾仍然是你的責任！

> #### 從其它語言呼叫 Rust 函式
>
> 也可以使用 `extern` 來建立一個允許其它語言呼叫 Rust 函式的介面。不同於建立整個 `extern` 塊，就在 `fn` 關鍵字之前增加 `extern` 關鍵字併為相關函式指定所用到的 ABI。還需增加 `#[no_mangle]` 註解來告訴 Rust 編譯器不要 mangle 此函式的名稱。*Mangling* 指編譯器將我們命名的函式名更改為包含更多供其他編譯過程使用的資訊的名稱，不過可讀性較差。每一個程式語言的編譯器都會以稍微不同的方式 mangle 函式名，所以為了使 Rust 函式能在其他語言中指定，必須停用 Rust 編譯器的 name mangling。這是不安全的因為在沒有內建 mangling 的時候在庫之間可能有命名衝突，所以確保所選的名稱可以不用 mangling 地安全匯出是我們的責任。
>
> 在如下的例子中，一旦其編譯為動態庫並從 C 語言中連結，`call_from_c` 函式就能夠在 C 程式碼中訪問：
>
> ```rust
> #[unsafe(no_mangle)]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> 這種 `extern` 用法只在屬性中需要 `unsafe`，而不需要在 `extern` 塊本身使用 `unsafe`。

### 訪問或修改可變靜態變數

在本書中，我們尚未討論過 **全域性變數**（*global variables*），Rust 確實支援它們，不過這對於 Rust 的所有權規則來說是有問題的。如果有兩個執行緒訪問相同的可變全域性變數，則可能會造成資料競爭。

全域性變數在 Rust 中被稱為 **靜態**（*static*）變數。示例 20-10 展示了一個擁有字串 slice 值的靜態變數的宣告和使用：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

<span class="caption">示例 20-10: 定義和使用一個不可變靜態變數</span>

靜態（`static`）變數類似於第三章 [“常量”][constants] 部分討論的常量。通常靜態變數的名稱採用 `SCREAMING_SNAKE_CASE` 寫法。靜態變數只能儲存擁有 `'static` 生命週期的引用，這意味著 Rust 編譯器可以自己計算出其生命週期而無需顯式標註。訪問不可變靜態變數是安全的。

常量與不可變靜態變數的一個微妙的區別是靜態變數中的值有一個固定的記憶體地址。使用這個值總是會訪問相同的地址。另一方面，常量則允許在任何被用到的時候複製其資料。另一個區別在於靜態變數可以是可變的。訪問和修改可變靜態變數都是 **不安全** 的。示例 20-11 展示瞭如何宣告、訪問和修改名為 `COUNTER` 的可變靜態變數：

<span class="filename">檔名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

<span class="caption">示例 20-11: 讀取或修改一個可變靜態變數是不安全的</span>

就像常規變數一樣，我們使用 `mut` 關鍵字來指定可變性。任何讀寫 `COUNTER` 的程式碼都必須位於 `unsafe` 塊中。這段程式碼可以編譯並如期打印出 `COUNTER: 3`，因為這是單執行緒的。擁有多個執行緒訪問 `COUNTER` 則可能導致資料競爭，所以這是未定義行為。因此，我們需要將整個函式標記為 `unsafe`，並在文件註釋中說明其安全性限制，以便呼叫者明確哪些操作是安全的、哪些是不安全的。

每當我們編寫一個不安全函式，慣常做法是編寫一個以 `SAFETY` 開頭的註釋並解釋呼叫者需要做什麼才可以安全地呼叫該方法。同理，當我們進行不安全操作時，慣常做法是編寫一個以 `SAFETY` 開頭並解釋安全性規則是如何維護的。

另外，編譯器不會允許你建立一個可變靜態變數的引用。你只能透過用裸指標解引用運算子建立的裸指標訪問它。這包括引用的建立是不可見的情況，例如這個程式碼示例中用於 `println!` 的情況。可變靜態變數只能透過裸指標建立的要求有助於確保使用它們的安全要求更為明確。

擁有可以全域性訪問的可變資料，難以保證不存在資料競爭，這就是為何 Rust 認為可變靜態變數是不安全的。在任何可能的情況下，請優先使用第十六章討論的併發技術和執行緒安全智慧指標，這樣編譯器就能檢測不同執行緒間的資料訪問是否是安全的。

### 實現不安全 trait

我們可以使用 `unsafe` 來實現一個不安全 trait。當 trait 中至少有一個方法中包含編譯器無法驗證的不變式（invariant）時該 trait 就是不安全的。可以在 `trait` 之前增加 `unsafe` 關鍵字將 trait 宣告為 `unsafe`，同時 trait 的實現也必須標記為 `unsafe`，如示例 20-12 所示：

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

<span class="caption">示例 20-12: 定義並實現不安全 trait</span>

透過 `unsafe impl`，我們承諾將保證編譯器所不能驗證的不變式。

作為一個例子，回憶第十六章 [“使用 `Sync` 和 `Send` trait 的可擴充套件併發”][send-and-sync] 部分中的 `Sync` 和 `Send` 標記 trait：如果我們的型別完全由實現了 `Send` 與 `Sync` 的其他型別組成，編譯器會自動為其實現這些 trait。如果我們定義的型別包含某些未實現 `Send` 或 `Sync` 的型別，例如裸指標，但又想將該型別標記為 `Send` 或 `Sync`，就必須使用 `unsafe`。Rust 不能驗證我們的型別保證可以安全地跨執行緒傳送或在多執行緒間訪問，所以需要我們自己進行檢查，並透過 `unsafe` 表明這一點。

### 訪問聯合體中的欄位

最後一個只能在 `unsafe` 塊中執行的操作是訪問（union）中的欄位。`union` 和 `struct` 類似，但是在一個例項中同時只能使用一個已宣告的欄位。聯合體主要用於和 C 程式碼中的聯合體進行互動。訪問聯合體的欄位是不安全的，因為 Rust 無法保證當前儲存在聯合體例項中資料的型別。可以檢視 [the Rust Reference][unions] 瞭解有關聯合體的更多資訊。

### 使用 Miri 檢查不安全程式碼

當編寫不安全程式碼時，你可能會想要檢查編寫的程式碼是否真的安全正確。最好的方式之一是使用 Miri，一個用來檢測未定義行為的 Rust 官方工具。鑑於借用檢查器是一個在編譯時工作的**靜態**工具，Miri 是一個在執行時工作的**動態**工具。它透過執行程式，或者測試集來檢查程式碼，並檢測你是否違反了它理解的 Rust 應該如何工作的規則。

使用 Miri 要求使用 nightly 版本的 Rust（我們在[附錄 G：Rust 是如何開發的與 “Nightly Rust”][nightly]中有更多討論）。你可以透過輸入 `rustup +nightly component add miri` 來同時安裝 nightly 版本的 Rust 和 Miri。這並不會改變你專案正在使用的 Rust 版本；它只是為你的系統增加了這個工具所以你可以在需要的時候使用它。你可以透過輸入 `cargo +nightly miri run` or `cargo +nightly miri test` 在專案中使用 Miri。

作為一個它是如何有用的例子，考慮一下對示例 20-11 執行它時會發生什麼。

```console
{{#include ../listings/ch20-advanced-features/listing-20-11/output.txt}}
```

Miri 正確地警告我們：我們正在把一個整數轉換成指標，這可能是個問題。不過，Miri 無法判斷這裡是否一定有問題，因為它不知道這個指標最初是如何產生的。隨後，Miri 又在我們取消引用該值的地方返回了一個錯誤。藉助 Miri，我們現在至少知道這裡存在未定義行為的風險，於是就可以進一步思考如何讓這段程式碼變得安全。在某些情況下，Miri 甚至能夠檢測出真正的錯誤，也就是**確定有問題**的程式碼模式，並給出如何修復這些錯誤的建議。

Miri 並不能捕獲編寫不安全程式碼時可能出現的所有錯誤。Miri 是一個動態分析工具，因此它只能捕獲程式碼實際執行時出現的問題。這意味著需要將其與良好的測試技術相結合以增強你對所編寫的不安全程式碼的信心。Miri 也不能覆蓋程式碼所有的不可靠的地方。

換句話說：如果 Miri **捕獲到了**一個問題，那你就知道這裡確實有個 bug；但僅僅因為 Miri **沒有**捕獲某個 bug，並不意味著這裡就沒有問題。不過，它確實能發現很多問題。你可以試著對本章中的其他不安全程式碼示例執行 Miri，看看它會給出什麼結果！

你可以在 [Miri 的 GitHub 倉庫][miri]瞭解更多資訊。

### 正確使用不安全程式碼

使用 `unsafe` 來進行這五個操作（超能力）之一是沒有問題的，甚至是不需要深思熟慮的，不過使得 `unsafe` 程式碼正確也實屬不易，因為編譯器不能幫助保證記憶體安全。當有理由使用 `unsafe` 程式碼時，是可以這麼做的，透過使用顯式的 `unsafe` 標註可以更容易地在錯誤發生時追蹤問題的源頭。每當編寫不安全程式碼時，都可以藉助 Miri 來更加自信地驗證所寫程式碼是否遵循 Rust 的規則。

若想更深入地瞭解如何高效使用不安全 Rust，請閱讀 Rust 關於該主題的官方指南 [Rustonomicon][nomicon]。

[dangling-references]: ch04-02-references-and-borrowing.html#懸垂引用
[ABI]: https://doc.rust-lang.org/reference/items/external-blocks.html#abi
[constants]: ch03-01-variables-and-mutability.html#常量
[send-and-sync]: ch16-04-extensible-concurrency-sync-and-send.html
[the-slice-type]: ch04-03-slices.html#slice-型別
[unions]: https://doc.rust-lang.org/reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[editions]: appendix-05-editions.html
[nightly]: appendix-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
