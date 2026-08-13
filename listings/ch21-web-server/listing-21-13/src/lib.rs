pub struct ThreadPool;

// ANCHOR: here
impl ThreadPool {
    /// 建立一個新的執行緒池。
    ///
    /// size 是池中執行緒的數量。
    ///
    /// # Panics
    ///
    /// 如果 size 為 0，`new` 方法會 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // --snip--
    // ANCHOR_END: here
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}
// ANCHOR_END: here
