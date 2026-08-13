fn main() {
    // ANCHOR: here
    {                      // s 在這裡無效，它尚未宣告
        let s = "hello";   // 從此處起，s 是有效的

        // 使用 s
    }                      // 此作用域已結束，s 不再有效
    // ANCHOR_END: here
}
