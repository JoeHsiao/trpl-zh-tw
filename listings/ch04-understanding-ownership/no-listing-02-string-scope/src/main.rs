fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // 從此處起，s 是有效的

        // 使用 s
    }                                  // 此作用域已結束，
                                       // s 不再有效
    // ANCHOR_END: here
}
