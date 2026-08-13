// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("hello world");

    // `first_word` 適用於 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也適用於 `String` 的引用，
    // 這等價於整個 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 適用於字串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因為字串字面值已經 **是** 字串 slice 了，
    // 這也是適用的，無需 slice 語法！
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
