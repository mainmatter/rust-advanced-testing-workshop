# Parsing arguments

Believe it or not, but you've now touched the entirety of the core macro ecosystem.\
From now onwards, it's all about exploring the crates further while learning the intricacies of the Rust language:
you're continuously faced with weird edge cases when writing macros for a broad audience.

## Arguments

But it's not over yet!\
Let's get you to exercise these muscles a bit more before moving on to the next topic.

Our `#[vanilla_test]` macro is still a bit too vanilla.\
We have now renamed it to `#[test]`, and we have higher expectations: it should support arguments!

If a `before` argument is specified, the macro should invoke it before the test function.\
If an `after` argument is specified, the macro should invoke it after the test function.\
It should be possible to specify both on the same test.

## Caution

The happy case is often not that difficult when writing macros.\
The challenge is **returning good error messages** when things go wrong.

In this exercise, a lot of things can go wrong:

- The item passed to the macro as `before` or `after` is not a function
- The item passed to the macro as `before` or `after` is a function that takes arguments
- The item passed to the macro as `before` or `after` is a function, but it's not in scope
- Etc.

You can often overlook most of these issues if you're writing a macro for your own use. But they become
important when you're writing a macro for a larger audience.
