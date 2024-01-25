use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn vanilla_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
