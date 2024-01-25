use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Attribute, ItemFn};

#[proc_macro_attribute]
pub fn vanilla_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut test_fn: ItemFn = todo!("Use syn");
    if test_fn
        .attrs
        .iter()
        .find(|a| is_test_attribute(a))
        .is_some()
    {
        test_fn.to_token_stream()
    } else {
        todo!("Use quote");
    }
    .into()
}

fn is_test_attribute(attr: &Attribute) -> bool {
    todo!()
}
