fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
} // 這裡，s 離開了作用域。但因為它並不擁有引用值的所有權，
  // 所以什麼也不會發生
// ANCHOR_END: here
