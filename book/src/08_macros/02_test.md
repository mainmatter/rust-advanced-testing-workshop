# Parsing tokens

In the previous exercise, both `#[vanilla_test]` and the default `#[test]` macro had to be specified on top of
the test function. Without adding `#[test]`, the annotated function is not picked up by the test runner.

## Detecting existing attributes

You'll augment `#[vanilla_test]`:

- If the annotated function has been annotated with `#[test]`, it should emit the code unchanged.
- If the annotated function has not been annotated with `#[test]`, it should add `#[test]` to the function.

This is how `#[googletest::gtest]` works, for example.

## The toolkit

When the macro game is serious, you can't get by using the built-in `proc_macro` crate.\
Almost all macros written in Rust are built on top of three ecosystem crates:

- [`syn`](https://docs.rs/syn/) for parsing tokens into abstract syntax tree nodes (AST node)
- [`quote`](https://docs.rs/quote/) for expressing the generated code with a `println!`-style syntax
- [`proc-macro2`](https://docs.rs/proc-macro2/), a wrapper around `proc_macro`'s types
