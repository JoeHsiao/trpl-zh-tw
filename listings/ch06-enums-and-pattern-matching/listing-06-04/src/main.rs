// ANCHOR: here
#[derive(Debug)] // 這樣可以立刻看到州的名稱
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// ANCHOR_END: here

fn main() {}
