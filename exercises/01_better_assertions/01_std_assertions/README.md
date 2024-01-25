# The built-in testing toolkit

The standard library provides three macros for test assertions: 
[`assert!`](https://doc.rust-lang.org/std/macro.assert.html), 
[`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html) and 
[`assert_ne!`](https://doc.rust-lang.org/std/macro.assert_ne.html).

They're used to check that a condition is true, or that two values are equal or not equal, respectively.

```rust
#[test]
fn t() {
    assert!(true);
    assert_eq!(1, 1);
    assert_ne!(1, 2);
}
```

## Panic messages

If the assertion fails, the macro will panic and it'll try to print a useful message for you to understand what went wrong.
In the case of `assert_eq!` and `assert_ne!`, the message will include the values that were compared.

```rust
#[test]
fn t() {
    assert_eq!(1, 2);
}
```

```bash
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs:2:5
```

In the case of `assert!`, the message will include the condition that was checked, stringified.

```rust
#[test]
fn t() {
    let v = vec![1];
    assert!(v.is_empty());
}
```

```text
thread 'main' panicked at 'assertion failed: v.is_empty()', src/main.rs:3:5
```

## Custom panic messages

The default panic messages are useful for simple cases, but they don't keep up with more complex scenarios.  
Going back to our `Vec` example, we might want to know what values were in the vector when the assertion failed, or 
how many elements it actually contained.

That's why all three macros accept an additional (optional) argument: a custom message to print when the assertion fails.  
You've seen this in the previous exercise:

```rust
#[test]
fn assertion_with_message() {
    assert_eq!(2 + 2, 5, "The Rust compiler hasn't read 1984 by George Orwell.")
}
```

The custom message will be printed _in addition_ to the default message for `assert_eq!` and `assert_ne!`.  
For `assert!`, it will replace the default message.

## Exercise

Before we move on to more advanced assertion libraries, we want to make sure that you know how to get the most out of
the built-in testing toolkit.  
In this exercise, you'll need to write a custom _dynamic_ panic message: the message must be different depending on the
values of the variables you're comparing.
