use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 將 Rust 程式碼構建成我們可以操作的語法樹。
    let ast = syn::parse(input).unwrap();

    // 生成 trait 的實現。
    impl_hello_macro(&ast)
}
