fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // 沒問題
    let r2 = &s; // 沒問題
    let r3 = &mut s; // 大問題

    println!("{r1}, {r2}, and {r3}");
    // ANCHOR_END: here
}
