fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字串後追加字面值

    println!("{s}"); // 將列印 `hello, world!`
                     // ANCHOR_END: here
}
