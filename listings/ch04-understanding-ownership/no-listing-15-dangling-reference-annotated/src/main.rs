fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle 返回一個字串的引用

    let s = String::from("hello"); // s 是一個新字串

    &s // 返回字串 s 的引用
} // 這裡 s 離開作用域並被丟棄。其記憶體被釋放。
  // 危險！
// ANCHOR_END: here
