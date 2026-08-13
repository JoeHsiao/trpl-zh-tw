fn main() {
    let s1 = gives_ownership();        // gives_ownership 將它的返回值傳遞給 s1

    let s2 = String::from("hello");    // s2 進入作用域

    let s3 = takes_and_gives_back(s2); // s2 被傳入 takes_and_gives_back, 
                                       // 它的返回值又傳遞給 s3
} // 此處，s3 移出作用域並被丟棄。s2 被 move，所以無事發生
  // s1 移出作用域並被丟棄

fn gives_ownership() -> String {       // gives_ownership 將會把返回值傳入
                                       // 呼叫它的函式

    let some_string = String::from("yours"); // some_string 進入作用域

    some_string                        // 返回 some_string 並將其移至呼叫函式
}

// 該函式將傳入字串並返回該值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 進入作用域

    a_string  // 返回 a_string 並移出給呼叫的函式
}
