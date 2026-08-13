fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 這裡 field_name 和 field_value 不再有效，
    // 嘗試使用它們看看會出現什麼編譯錯誤！
    // ANCHOR_END: here
}
