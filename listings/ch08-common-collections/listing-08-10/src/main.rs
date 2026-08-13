fn main() {
    // ANCHOR: here
    {
        let v = vec![1, 2, 3, 4];

        // 使用 v
    } // <- 在這裡 v 離開作用域並被釋放
    // ANCHOR_END: here
}
