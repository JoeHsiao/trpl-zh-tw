fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值為 5

    s.clear(); // 這清空了字串，使其等於 ""

    // word 在此處的值仍然是 5，
    // 但是沒有更多的字串讓我們可以有效地應用數值 5。word 的值現在完全無效！
}
// ANCHOR_END: here
