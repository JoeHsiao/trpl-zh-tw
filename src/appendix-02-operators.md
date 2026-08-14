## 附錄 B：運算子與符號

[appendix-02-operators.md](https://github.com/rust-lang/book/blob/799aa759e03d36dff11e920cf35ac38f496815ea/src/appendix-02-operators.md)

該附錄包含了 Rust 語法的詞彙表，包括運算子以及其它符號，這些符號單獨出現或出現在路徑、泛型、trait 約束、宏、屬性、註釋、元組以及大括號上下文中。

### 運算子

表 B-1 包含了 Rust 中的運算子、運算子如何出現在上下文中的示例、簡短解釋以及該運算子是否可過載。如果一個運算子是可過載的，則該運算子上用於過載的相關 trait 也會列出。

<span class="caption">表 B-1: 運算子</span>

| 運算子 | 示例 | 解釋 | 是否可過載 |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | 巨集展開 |  |
| `!` | `!expr` | 按位非或邏輯非 | `Not` |
| `!=` | `expr != expr` | 不等比較 | `PartialEq` |
| `%` | `expr % expr` | 算術取餘 | `Rem` |
| `%=` | `var %= expr` | 算術取餘與賦值 | `RemAssign` |
| `&` | `&expr`, `&mut expr` | 借用 | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | 借用指標型別 |  |
| `&` | `expr & expr` | 按位與 | `BitAnd` |
| `&=` | `var &= expr` | 按位與及賦值 | `BitAndAssign` |
| `&&` | `expr && expr` | 短路（Short-circuiting）邏輯與 |  |
| `*` | `expr * expr` | 算術乘法 | `Mul` |
| `*=` | `var *= expr` | 算術乘法與賦值 | `MulAssign` |
| `*` | `*expr` | 解引用 | `Deref` |
| `*` | `*const type`, `*mut type` | 裸指標 | |
| `+` | `trait + trait`, `'a + trait` | 複合型別限制 | |
| `+` | `expr + expr` | 算術加法 | `Add` |
| `+=` | `var += expr` | 算術加法與賦值 | `AddAssign` |
| `,` | `expr, expr` | 引數以及元素分隔符 | |
| `-` | `- expr` | 算術取負 | `Neg` |
| `-` | `expr - expr` | 算術減法 | `Sub` |
| `-=` | `var -= expr` | 算術減法與賦值 | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | 函式與閉包的返回型別 | |
| `.` | `expr.ident` | 欄位訪問 | |
| `.` | `expr.ident(expr, ...)` | 方法呼叫 | |
| `.` | `expr.0`, `expr.1`, etc. | 元組索引 | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | 右開區間範圍字面值 | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | 右閉區間範圍字面值 | `PartialOrd` |
| `..` | `..expr` | 結構體更新語法 | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | “與剩餘部分” 的模式繫結 | |
| `...` | `expr...expr` | （Deprecated，請使用 `..=`）在模式中：閉區間範圍模式 | |
| `/` | `expr / expr` | 算術除法 | `Div` |
| `/=` | `var /= expr` | 算術除法與賦值 | `DivAssign` |
| `:` | `pat: type`, `ident: type` | 約束 | |
| `:` | `ident: expr` | 結構體欄位初始化 | |
| `:` | `'a: loop {...}` | 迴圈標誌 | |
| `;` | `expr;` | 語句和語句結束符 | |
| `;` | `[...; len]` | 固定大小陣列語法的部分 | |
| `<<` | `expr << expr` |左移 | `Shl` |
| `<<=` | `var <<= expr` | 左移與賦值 | `ShlAssign` |
| `<` | `expr < expr` | 小於比較 | `PartialOrd` |
| `<=` | `expr <= expr` | 小於等於比較 | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | 賦值/等值 | |
| `==` | `expr == expr` | 等於比較 | `PartialEq` |
| `=>` | `pat => expr` | 匹配分支語法的部分 | |
| `>` | `expr > expr` | 大於比較 | `PartialOrd` |
| `>=` | `expr >= expr` | 大於或等於比較 | `PartialOrd` |
| `>>` | `expr >> expr` | 右移 | `Shr` |
| `>>=` | `var >>= expr` | 右移與賦值 | `ShrAssign` |
| `@` | `ident @ pat` | 模式繫結 | |
| `^` | `expr ^ expr` | 按位異或 | `BitXor` |
| `^=` | `var ^= expr` | 按位異或與賦值 | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | 模式替代 | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | 按位或 | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | 按位或與賦值 | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | 短路（Short-circuiting）邏輯或 | |
| `?` | `expr?` | 錯誤傳播 | |

### 非運算子符號

下面的列表中包含了所有和運算子不一樣功能的符號；也就是說，它們不表現為函式或方法呼叫。

表 B-2 展示了以其自身出現以及出現在合法其他各個地方的符號。

<span class="caption">表 B-2：獨立語法</span>

| 符號 | 解釋 |
|--------|-------------|
| `'ident` | 命名生命週期或迴圈標籤 |
| `...u8`, `...i32`, `...f64`, `...usize` 等 | 指定型別的數值常量 |
| `"..."` | 字串字面值 |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | 原始字串字面值，未處理的跳脫字元 |
| `b"..."` | 位元組字串字面值; 構造一個位元組陣列型別而非字串 |
| `br"..."`, `br#"..."#`, `br##"..."##` 等 | 原始位元組字串字面值，原始和位元組字串字面值的結合 |
| `'...'` | 字元字面值 |
| `b'...'` | ASCII 碼位元組字面值 |
| <code>&vert;...&vert; expr</code> | 閉包 |
| `!` | 發散函式使用的始終為空的底型別 |
| `_` | “忽略” 模式繫結；也用於增強整型字面值的可讀性 |

表 B-3 展示了出現在從模組結構到項的路徑上下文中的符號

<span class="caption">表 B-3：路徑相關語法</span>

| 符號 | 解釋 |
|--------|-------------|
| `ident::ident` | 名稱空間路徑 |
| `::path` | 相對於 crate 根的路徑（也就是說，一個顯式的絕對路徑） |
| `self::path` | 與當前模組相對的路徑（即一個顯式相對路徑）|
| `super::path` | 與當前模組的父模組相對的路徑 |
| `type::ident`, `<type as trait>::ident` | 關聯常量、函式以及型別 |
| `<type>::...` | 不可以被直接命名的關聯項型別（如 `<&T>::...`，`<[T]>::...`，等） |
| `trait::method(...)` | 通過命名定義的 trait 來消除方法呼叫的二義性 |
| `type::method(...)` | 通過命名定義的型別來消除方法呼叫的二義性 |
| `<type as trait>::method(...)` | 通過命名 trait 和型別來消除方法呼叫的二義性 |


表 B-4 展示了用於泛型型別引數上下文中的符號。

<span class="caption">表 B-4：泛型</span>

| 符號 | 解釋 |
|--------|-------------|
| `path<...>` | 為一個型別中的泛型指定具體引數（如 `Vec<u8>`） |
| `path::<...>`, `method::<...>` | 為一個泛型、函式或表示式中的方法指定具體引數，通常指 turbofish（如 `"42".parse::<i32>()`）|
| `fn ident<...> ...` | 泛型函式定義 |
| `struct ident<...> ...` | 泛型結構體定義 |
| `enum ident<...> ...` | 泛型列舉定義 |
| `impl<...> ...` | 定義泛型實現 |
| `for<...> type` | 高階生命週期限制 |
| `type<ident=type>` | 泛型，其一個或多個相關型別必須被指定為特定型別（如 `Iterator<Item=T>`）|

表 B-5 展示了出現在使用 trait 約束泛型引數上下文中的符號。

<span class="caption">表 B-5: Trait 約束</span>

| 符號 | 解釋 |
|--------|-------------|
| `T: U` | 泛型引數 `T` 約束於實現了 `U` 的型別 |
| `T: 'a` | 泛型 `T` 的生命週期必須長於 `'a`（意味著該型別不能傳遞包含生命週期短於 `'a` 的任何引用）|
| `T: 'static` | 泛型 T 不包含除 `'static` 之外的借用引用 |
| `'b: 'a` | 泛型 `'b` 生命週期必須長於泛型 `'a` |
| `T: ?Sized` | 使用一個不定大小的泛型型別 |
| `'a + trait`, `trait + trait` | 複合型別限制 |

表 B-6 展示了在呼叫或定義宏以及在項上指定屬性的上下文中出現的符號。

<span class="caption">表 B-6: 宏與屬性</span>

| 符號 | 解釋 |
|--------|-------------|
| `#[meta]` | 外部屬性 |
| `#![meta]` | 內部屬性 |
| `$ident` | 巨集替換 |
| `$ident:kind` | 宏捕獲 |
| `$(…)…` | 宏重複 |
| `ident!(...)`, `ident!{...}`, `ident![...]` | 巨集呼叫 |

表 B-7 展示了建立註釋的符號。

<span class="caption">表 B-7: 註釋</span>

| 符號 | 註釋 |
|--------|-------------|
| `//` | 行註釋 |
| `//!` | 內部行文件註釋 |
| `///` | 外部行文件註釋 |
| `/*...*/` | 塊註釋 |
| `/*!...*/` | 內部塊文件註釋 |
| `/**...*/` | 外部塊文件註釋 |

表 B-8 展示了出現在使用圓括號上下文中的符號。

<span class="caption">表 B-8: 圓括號</span>

| 符號 | 解釋 |
|--------|-------------|
| `()` | 空元組（亦稱單元），即是字面值也是型別 |
| `(expr)` | 括號表示式 |
| `(expr,)` | 單一元素元組表示式 |
| `(type,)` | 單一元素元組型別 |
| `(expr, ...)` | 元組表示式 |
| `(type, ...)` | 元組型別 |
| `expr(expr, ...)` | 函式呼叫表示式；也用於初始化元組結構體 `struct` 以及元組列舉 `enum` 變體 |

表 B-9 展示了使用大括號的上下文。

<span class="caption">表 B-9: 大括號</span>

| 符號 | 解釋 |
|---------|-------------|
| `{...}` | 塊表示式 |
| `Type {...}` | `struct` 字面值  |

表 B-10 展示了使用方括號的上下文。

<span class="caption">表 B-10: 方括號</span>

| 符號 | 解釋 |
|---------|-------------|
| `[...]` | 陣列字面值 |
| `[expr; len]` | 複製了 `len` 個 `expr` 的陣列 |
| `[type; len]` | 包含 `len` 個 `type` 型別的陣列|
| `expr[expr]` | 集合索引。過載（`Index`, `IndexMut`） |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | 集合索引，使用 `Range`，`RangeFrom`，`RangeTo` 或 `RangeFull` 作為索引來代替集合 slice |
