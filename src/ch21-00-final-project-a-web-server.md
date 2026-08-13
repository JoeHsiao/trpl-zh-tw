# 最後的專案：構建多執行緒 web server

[ch21-00-final-project-a-web-server.md](https://github.com/rust-lang/book/blob/8aa0d003e6499d733d639de32d70f590efa48657/src/ch21-00-final-project-a-web-server.md)

這是一次漫長的旅途，不過我們已經抵達了本書的結尾。在本章中，我們將一同構建另一個專案，來展示最後幾章所學，同時複習更早的章節。

作為最後一個專案，我們要編寫一個會說 “Hello!” 的 web server，它在瀏覽器中看起來會像圖 21-1 那樣：

<img alt="瀏覽器訪問 127.0.0.1:8080 時顯示一個網頁，頁面文字內容為“Hello! Hi from Rust”" src="img/trpl21-01.png" class="center" style="width: 50%;" />

<span class="caption">圖 21-1：我們最後一個共同完成的專案</span>

下面是構建這個 web server 的計劃：

1. 學一點 TCP 和 HTTP 相關知識。
2. 在套接字（socket）上監聽 TCP 連線。
3. 解析少量 HTTP 請求。
4. 建立正確的 HTTP 響應。
5. 用執行緒池改善 server 的吞吐量。

在開始之前，我們先提兩點說明。首先，這裡使用的方法並不是使用 Rust 構建 web server 的最佳方式。[crates.io](https://crates.io/) 上有很多可用於生產環境的 crate，它們提供了比我們所要編寫的更為完整的 web server 和執行緒池實現。然而，本章的目的在於學習，而不是走捷徑。因為 Rust 是一個系統程式語言，我們能夠選擇處理什麼層次的抽象，並能夠選擇比其他語言可能或可用的層次更低的層次。

其次，這裡不會使用 async 和 await。僅僅構建一個執行緒池本身就已經足夠有挑戰性了，無需再把構建非同步執行時的複雜度加進來！不過，我們會指出 async 和 await 可能如何適用於本章中遇到的一些相同問題。歸根結底，正如我們在第十七章提到的那樣，許多非同步執行時本身也是藉助執行緒池來管理工作的。

因此，我們將手動編寫一個基礎的 HTTP server 和執行緒池，這樣你就能學到未來可能會用到的那些 crate 背後的通用思想和技術。
