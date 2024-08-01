# Test macros

In the previous sections you've had a chance to see quite a few "custom" test macros in action:
`#[googletest::test]`, `#[tokio::test]`, `#[sqlx::test]`. Sometimes you even combined them, stacking them
on top of each other!

In this section, you'll learn _why_ these macros exist and _how_ to build your own.

## The default toolkit is limited

`cargo test` and `#[test]` are the two building blocks of the Rust testing ecosystem, the ones
available to you out of the box.\
They are powerful, but they lack a few advanced features that you might be familiar with from
testing frameworks in other ecosystems:

- No **lifecycle hooks**. You can't easily execute code before or after a test case.
  That's a requirement if you want to set up and tear down external resources (e.g. a database, like in `#[sqlx::test]`).
- No **fixtures**. You can't inject types into the signature of a test function and expect the test framework
  to instantiate them for you (e.g. like `PgPool` with `#[sqlx::test]`).
- No **parameterised tests**. You can't run the same test with different inputs and have each input
  show up as a separate test case in the final test report (e.g. see [`rstest`](https://docs.rs/rstest/latest/rstest/)).
- No **first-class async tests**. Rust doesn't ship with a default executor, so you can't write async tests
  without pulling in a third-party crate. Macros like `#[tokio::test]`, under the hood, rewrite your async test function
  as a sync function with a call to `block_on` (see [here](https://docs.rs/tokio/latest/tokio/attr.test.html#using-the-multi-thread-runtime)).

## Macros to the rescue

Custom test macros are a way to augment the default toolkit with the features you need.\
All the macros we mentioned so far are **attribute procedural macros**.\
Procedural macros are **token transformers**. As input, they receive:

- A stream of tokens, representing the Rust code that's been annotated with the macro;
- A stream of tokens, representing the arguments passed to the macro.

As output, they return another stream of tokens, the Rust code that will actually be compiled as part of the crate
that used the macro.

### Example: `#[tokio::test]`

Let's look at an example to make things concrete: `#[tokio::test]`.\
The `#[tokio::test]` macro definition looks like this:

```rust
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    // [...]
}
```

If you use `#[tokio::test]` on a test function, we can see the two streams of tokens in action:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn it_works() {
    assert!(true);
}
```

- The first stream of tokens (`args`) contains the arguments passed to the macro: `flavor = "multi_thread"`.
- The second stream of tokens (`item`) contains the Rust code that's been annotated with the macro:
  `async fn it_works() { assert!(true); }`.
- The output stream, instead, will look like this:

```rust
#[test]
fn it_works() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            assert!(true);
        })
}
```

## Objectives

This is not a workshop on procedural macros, so we won't be exploring advanced macro-writing techniques.\
Nonetheless, a basic understanding of how macros work and a few exercises can go a long way: you don't need to
know _that much_ about macros to write your own test macro!

That's the goal of this section.
