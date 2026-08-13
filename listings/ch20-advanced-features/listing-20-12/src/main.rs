// ANCHOR: here
unsafe trait Foo {
    // 方法在這裡
}

unsafe impl Foo for i32 {
    // 方法實現在這裡
}
// ANCHOR_END: here

fn main() {}
