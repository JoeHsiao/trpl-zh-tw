## 列舉的定義

[ch06-01-defining-an-enum.md](https://github.com/rust-lang/book/blob/602a0d2e898f5e4ff030eac0b457755a10e0be1e/src/ch06-01-defining-an-enum.md)

結構體給予你將欄位和資料聚合在一起的方法，像 `Rectangle` 結構體有 `width` 和 `height` 兩個欄位。而列舉給予你一個途徑去宣告某個值是一個集合中的一員。比如，我們想讓 `Rectangle` 是一些形狀的集合，包含 `Circle` 和 `Triangle` 。為此，Rust 允許我們將這些可能性編碼為一個列舉型別。

讓我們看看一個需要訴諸於程式碼的場景，來考慮為何此時使用列舉更為合適且實用。假設我們要處理 IP 地址。目前被廣泛使用的兩個主要 IP 標準：IPv4（version four）和 IPv6（version six）。這是我們的程式可能會遇到的所有可能的 IP 地址型別：所以可以**列舉**出所有可能的值，這也正是列舉一詞的由來。

任何一個 IP 地址要麼是 IPv4 的要麼是 IPv6 的，而且不能兩者都是。IP 地址的這個特性使得列舉資料結構非常適合這個場景，因為列舉值只可能是其中一個變體。IPv4 和 IPv6 從根本上講仍是 IP 地址，所以當代碼在處理適用於任何型別的 IP 地址的場景時應該把它們當作相同的型別。

可以透過在程式碼中定義一個 `IpAddrKind` 列舉來表現這個概念並列出可能的 IP 地址型別，`V4` 和 `V6`。這被稱為列舉的**變體**（*variants*）：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

現在 `IpAddrKind` 就是一個可以在程式碼中使用的自定義資料型別了。

### 列舉值

可以像這樣建立 `IpAddrKind` 兩個不同變體的例項：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

注意列舉的變體位於其識別符號的名稱空間中，並使用兩個冒號分開。這麼設計的益處是現在 `IpAddrKind::V4` 和 `IpAddrKind::V6` 都是 `IpAddrKind` 型別的。例如，接著可以定義一個函式來接收任何 `IpAddrKind`型別的引數：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

現在可以使用任一變體來呼叫這個函式：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

使用列舉甚至還有更多優勢。進一步考慮一下我們的 IP 地址型別，目前沒有一個儲存實際 IP 地址**資料**的方法；只知道它是什麼**型別**的。考慮到已經在第五章學習過結構體了，你可能會像示例 6-1 那樣嘗試用結構體來解決這個問題：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">示例 6-1：將 IP 地址的資料和 `IpAddrKind` 變體儲存在一個 `struct` 中</span>

這裡我們定義了一個有兩個欄位的結構體 `IpAddr`：`IpAddrKind`（之前定義的列舉）型別的 `kind` 欄位和 `String` 型別 `address` 欄位。我們有這個結構體的兩個例項。第一個，`home`，它的 `kind` 的值是 `IpAddrKind::V4` 與之相關聯的地址資料是 `127.0.0.1`。第二個例項，`loopback`，`kind` 的值是 `IpAddrKind` 的另一個變體，`V6`，關聯的地址是 `::1`。我們使用了一個結構體來將 `kind` 和 `address` 打包在一起，現在列舉變體就與值相關聯了。

我們可以使用一種更簡潔的方式來表達相同的概念，僅僅使用列舉並將資料直接放進每一個列舉變體而不是將列舉作為結構體的一部分。`IpAddr` 列舉的新定義表明了 `V4` 和 `V6` 變體都關聯了 `String` 值：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

我們直接將資料附加到列舉的每個變體上，這樣就不需要一個額外的結構體了。這裡也很容易看出列舉工作的另一個細節：每一個我們定義的列舉變體的名字也變成了一個構建列舉的例項的函式。也就是說，`IpAddr::V4()` 是一個獲取 `String` 引數並返回 `IpAddr` 型別例項的函式呼叫。作為定義列舉的結果，這些建構函式會自動被定義。

用列舉替代結構體還有另一個優勢：每個變體可以處理不同型別和數量的資料。IPv4 版本的 IP 地址總是含有四個值在 0 和 255 之間的數字部分。如果我們想要將 `V4` 地址儲存為四個 `u8` 值而 `V6` 地址仍然表現為一個 `String`，這就不能使用結構體了。列舉則可以輕易的處理這個情況：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

這些程式碼展示了使用列舉來儲存兩種不同 IP 地址的幾種可能的選擇。然而，事實證明儲存和編碼 IP 地址實在是太常見了[以致標準庫提供了一個開箱即用的定義！][IpAddr]<!-- ignore -->讓我們看看標準庫是如何定義 `IpAddr` 的：它正有著跟我們定義和使用的一樣的列舉和變體，不過它將變體中的地址資料嵌入到了兩個不同形式的結構體中，它們對不同的變體的定義是不同的：

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

這些程式碼展示了可以將任意型別的資料放入列舉變體中：例如字串、數字型別或者結構體。甚至可以包含另一個列舉！另外，標準庫中的型別通常並不比你設想出來的要複雜多少。

注意雖然標準庫中包含一個 `IpAddr` 的定義，仍然可以建立和使用我們自己的定義而不會有衝突，因為我們並沒有將標準庫中的定義引入作用域。第七章會講到如何匯入型別。

來看看示例 6-2 中的另一個列舉的例子：它的變體中內嵌了多種多樣的型別：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">示例 6-2：一個 `Message` 列舉，其每個變體都儲存了不同數量和型別的值</span>

這個列舉有四個含有不同型別的變體：

* `Quit` 沒有關聯任何資料。
* `Move` 類似結構體包含命名欄位。
* `Write` 包含單獨一個 `String`。
* `ChangeColor` 包含三個 `i32`。

定義一個如示例 6-2 中所示那樣的有關聯值的列舉的方式和定義多個不同型別的結構體的方式很相像，除了列舉不使用 `struct` 關鍵字以及其所有變體都被組合在一起位於 `Message` 型別下。如下這些結構體可以包含與之前列舉變體中相同的資料：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

不過，如果我們使用不同的結構體，由於它們都有不同的型別，我們將不能像使用示例 6-2 中定義的 `Message` 列舉那樣，輕易的定義一個能夠處理這些不同型別的結構體的函式，因為列舉是單獨一個型別。

結構體和列舉還有另一個相似點：就像可以使用 `impl` 來為結構體定義方法那樣，也可以在列舉上定義方法。這是一個定義於我們 `Message` 列舉上的叫做 `call` 的方法：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

方法體使用了 `self` 來獲取呼叫方法的值。這個例子中，建立了一個值為 `Message::Write(String::from("hello"))` 的變數 `m`，而且這就是當 `m.call()` 執行時 `call` 方法中的 `self` 的值。

讓我們看看標準庫中的另一個非常常見且實用的列舉：`Option`。

<a id="the-option-enum-and-its-advantages-over-null-values"></a>

### `Option` 列舉

這一部分會分析一個 `Option` 的案例，`Option` 是標準庫定義的另一個列舉。`Option` 型別應用廣泛因為它編碼了一個非常普遍的場景，即一個值要麼有值要麼沒值。

例如，如果請求一個非空列表的第一項，會得到一個值，如果請求一個空的列表，就什麼也不會得到。從型別系統的角度來表達這個概念就意味著編譯器需要檢查是否處理了所有應該處理的情況，這樣就可以避免在其他程式語言中非常常見的 bug。

程式語言的設計經常要考慮包含哪些功能，但考慮排除哪些功能也很重要。Rust 並沒有很多其他語言中有的空值功能。**空值**（*Null* ）是一個值，它代表沒有值。在有空值的語言中，變數總是這兩種狀態之一：空值和非空值。

Tony Hoare，null 的發明者，在他 2009 年的演講 “Null References: The Billion Dollar Mistake” 中曾經說到：

> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn't resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.
>
> 我稱之為我十億美元的錯誤。當時，我在為一個面嚮物件語言設計第一個綜合性的面向引用的型別系統。我的目標是透過編譯器的自動檢查來保證所有引用的使用都應該是絕對安全的。不過我未能抵抗住引入一個空引用的誘惑，僅僅是因為它是這麼的容易實現。這引發了無數錯誤、漏洞和系統崩潰，在過去四十年裡可能造成了價值十億美元的痛苦和損失。

空值的問題在於當你嘗試像一個非空值那樣使用一個空值，會出現某種形式的錯誤。因為空和非空的屬性無處不在，非常容易出現這類錯誤。

然而，空值嘗試表達的概念仍然是有意義的：空值是一個因為某種原因目前無效或缺失的值。

問題不在於概念而在於具體的實現。為此，Rust 並沒有空值，不過它確實擁有一個可以編碼存在或不存在概念的列舉。這個列舉是 `Option<T>`，而且它[定義於標準庫中][option]<!-- ignore -->，如下：

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` 列舉是如此有用以至於它甚至被包含在了 prelude 之中，無需將其顯式引入作用域。另外，它的變體也是如此：可以不需要 `Option::` 字首來直接使用 `Some` 和 `None`。即便如此 `Option<T>` 也仍是常規的列舉，`Some(T)` 和 `None` 仍是 `Option<T>` 的變體。

`<T>` 語法是一個我們還未講到的 Rust 功能。它是一個泛型型別引數，第十章會更詳細的講解泛型。目前，所有你需要知道的就是 `<T>` 意味著 `Option` 列舉的 `Some` 變體可以包含任意型別的資料，同時每一個用於 `T` 位置的具體型別使得 `Option<T>` 整體作為不同的型別。這裡是一些包含數字型別和字元型別 `Option` 值的例子：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

`some_number` 的型別是 `Option<i32>`。`some_char` 的型別是 `Option<char>`，是不同於 `some_number` 的型別。因為我們在 `Some` 變體中指定了值，Rust 可以推斷其型別。對於 `absent_number`，Rust 需要我們指定 `Option` 整體的型別，因為編譯器只通過 `None` 值無法推斷出 `Some` 變體儲存的值的型別。這裡我們告訴 Rust 希望 `absent_number` 是 `Option<i32>` 型別的。

當有一個 `Some` 值時，我們就知道存在一個值，而這個值儲存在 `Some` 中。當有個 `None` 值時，在某種意義上，它跟空值具有相同的意義：並沒有一個有效的值。那麼，`Option<T>` 為什麼就比空值要好呢？

簡而言之，因為 `Option<T>` 和 `T`（這裡的 `T` 可以是任何型別）是不同的型別，所以編譯器不允許我們把 `Option<T>` 當成一個肯定有效的值來使用。例如，這段程式碼不能編譯，因為它試圖把 `Option<i8>` 和 `i8` 相加：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

如果執行這些程式碼，將得到類似這樣的錯誤資訊：

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

很好！事實上，這條錯誤資訊的意思是，Rust 不知道該如何把 `Option<i8>` 和 `i8` 相加，因為它們是不同的型別。當我們在 Rust 中擁有一個像 `i8` 這樣的值時，編譯器會確保它總是有效的。我們可以放心使用它，而無需先做空值檢查。只有當我們使用 `Option<i8>`（或者任何別的 `Option<T>`）時，才需要考慮值可能不存在，而編譯器會確保我們在使用這個值之前處理了這種情況。

換句話說，在對 `Option<T>` 進行運算之前必須將其轉換為 `T`。通常這能幫助我們捕獲到空值最常見的問題之一：假設某值不為空但實際上為空的情況。

消除了錯誤地假設一個非空值的風險，會讓你對程式碼更加有信心。為了擁有一個可能為空的值，你必須要顯式的將其放入對應型別的 `Option<T>` 中。接著，當使用這個值時，必須明確的處理值為空的情況。只要一個值不是 `Option<T>` 型別，你就**可以**安全的認定它的值不為空。這是 Rust 的一個經過深思熟慮的設計決策，來限制空值的泛濫以增加 Rust 程式碼的安全性。

那麼當有一個 `Option<T>` 的值時，如何從 `Some` 變體中取出 `T` 的值來使用它呢？`Option<T>` 列舉擁有大量用於各種情況的方法：你可以檢視[它的文件][docs]<!-- ignore -->。熟悉 `Option<T>` 的方法將對你的 Rust 之旅非常有用。

總的來說，為了使用 `Option<T>` 值，需要編寫處理每個變體的程式碼。你想要一些程式碼只當擁有 `Some(T)` 值時執行，允許這些程式碼使用其中的 `T`。也希望一些程式碼只在值為 `None` 時執行，這些程式碼並沒有一個可用的 `T` 值。`match` 表示式就是這麼一個處理列舉的控制流結構：它會根據列舉的變體執行不同的程式碼，這些程式碼可以使用匹配到的值中的資料。

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[docs]: https://doc.rust-lang.org/std/option/enum.Option.html
