static mut COUNTER: u32 = 0;

/// SAFETY: 同時在多個執行緒呼叫這個方法是未定義的行為，所以你*必須*保證同一時間只
/// 有一個執行緒在呼叫它。
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: 它只在 `main` 這一個執行緒被呼叫。
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
