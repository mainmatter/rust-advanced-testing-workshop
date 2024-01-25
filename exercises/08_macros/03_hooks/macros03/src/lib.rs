use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, ItemFn, Token};

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut test_fn: ItemFn = syn::parse_macro_input!(input as ItemFn);
    let Args { before, after } = syn::parse_macro_input!(args as RawArgs).validate();

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = test_fn;
    let mut output = quote! {
        #vis #sig {
            #(#before();)*
            #block
            #(#after();)*
        }
    };

    if !attrs.iter().any(|a| is_test_attribute(a)) {
        output = {
            quote! {
                #[::core::prelude::v1::test]
                #output
            }
        };
    }
    output.into()
}

struct RawArgs {
    vars: Vec<RawHook>,
}

/// Argument parsing goes through two phases:
///
/// 1. Parse the raw arguments into a struct, which is syntactically what we expect
/// 2. Validate the arguments and convert them into the form we want (semantic validation)
struct Args {
    before: Vec<syn::Path>,
    after: Vec<syn::Path>,
}

impl RawArgs {
    pub fn validate(self) -> Args {
        let mut before = Vec::new();
        let mut after = Vec::new();
        for RawHook {
            type_,
            equals: _,
            fn_path,
        } in self.vars
        {
            if type_.to_string() == "before" {
                before.push(fn_path);
            } else if type_.to_string() == "after" {
                after.push(fn_path);
            } else {
                panic!("Unexpected hook type: {}", type_);
            }
        }
        Args { before, after }
    }
}

struct RawHook {
    type_: syn::Ident,
    equals: Token![=],
    fn_path: syn::Path,
}

impl Parse for RawArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<RawHook, Token![,]>::parse_terminated(input)?;
        Ok(RawArgs {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Parse for RawHook {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_ = input.parse()?;
        let equals = input.parse()?;
        let fn_path = input.parse()?;
        Ok(RawHook {
            type_,
            equals,
            fn_path,
        })
    }
}

fn is_test_attribute(attr: &Attribute) -> bool {
    let last_segment = match attr.path().segments.last() {
        Some(last_segment) => last_segment,
        None => return false,
    };
    last_segment.ident == "test"
}
