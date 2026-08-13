fn main() {
    let s = String::from("hello");  // s 進入作用域

    takes_ownership(s);             // s 的值移動到函數里 ...
                                    // ... 所以到這裡不再有效

    let x = 5;                      // x 進入作用域

    makes_copy(x);                  // x 應該移動函數里，
                                    // 但 i32 是 Copy 的，
    println!("{}", x);              // 所以在後面可繼續使用 x

} // 這裡，x 先移出了作用域，然後是 s。但因為 s 的值已被移走，
  // 沒有特殊之處

fn takes_ownership(some_string: String) { // some_string 進入作用域
    println!("{some_string}");
} // 這裡，some_string 移出作用域並呼叫 `drop` 方法。
  // 佔用的記憶體被釋放

fn makes_copy(some_integer: i32) { // some_integer 進入作用域
    println!("{some_integer}");
} // 這裡，some_integer 移出作用域。沒有特殊之處
