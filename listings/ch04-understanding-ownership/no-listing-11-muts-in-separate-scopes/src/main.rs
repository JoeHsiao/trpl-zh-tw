fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在這裡離開了作用域，所以我們完全可以建立一個新的引用

    let r2 = &mut s;
    // ANCHOR_END: here
}
