# Foreign traits

For `#[automock]` to work, it needs to be applied to the trait definition.\
That's not an issue if you're writing the trait yourself, but what if you're using a trait from a third-party crate?

## The problem

Rust macros can only access the code of the item they're applied to.\
There's no way for macros to ask the compiler "can you give me the trait definition of `Debug`?".

## The "solution"

If you want to use `mockall` with a trait from a third-party crate, you'll need to rely on their `mock!` macro
and... inline the trait definition in your code.

The syntax is fairly custom—refer to
the [`mock!` macro documentation](https://docs.rs/mockall/0.12.1/mockall/macro.mock.html) for the specifics.
