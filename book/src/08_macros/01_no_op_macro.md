# Your first macro

Let's start from the basics: you'll write a macro that does nothing. It just re-emits the code that's been annotated
with
the macro, unchanged.\
This will give you a chance to get familiar with the overall setup before moving on to more complex endeavors.

## `proc-macro = true`

You can't define a procedural macro in a "normal" library crate.\
They need to be in a separate crate, with a `Cargo.toml` that includes this key:

```toml
[lib]
proc-macro = true
```

That key tells `cargo` that this crate contains procedural macros and it should be compiled accordingly.

## `#[proc_macro_attribute]`

There are various kinds of procedural macros:

- **Function-like macros**. Their invocation looks like a function call (e.g. `println!`).
- **Derive macros**. They're specified inside a `derive` attribute (e.g. `#[derive(Debug)]`).
- **Attribute procedural macros**. They're applied to items as attributes (e.g. `#[tokio::test]`).

For a test macro, we need an attribute procedural macro.\
As you've learned in the intro, it's a function that's annotated with `#[proc_macro_attribute]`:

```rust
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_attribute_macro(args: TokenStream, item: TokenStream) -> TokenStream {
    // [...]
}
```

The `proc_macro` crate is distributed as part of the Rust toolchain, just like the standard library, `std`.
