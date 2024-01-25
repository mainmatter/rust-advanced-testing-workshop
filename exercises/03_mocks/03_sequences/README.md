# Multiple calls

## The problem

Methods on your mock object might be invoked multiple times by the code under test.  
In more complex scenarios, you might need to return different values for each invocation. Let's see how to do that.

## `times`

When you add a `times` expectation to a method, `mockall` will use the return value you specified 
up until the `times`-th invocation.

```rust
#[test]
fn test_times() {
    let mut mock = MockEmailSender::new();
    let email = /* */;
    mock.expect_send()
        .times(2)
        .returning(|_| Ok(()));

    mock.send(&email);
    mock.send(&email);
    // This panics! 
    mock.send(&email);
}
```

You can leverage this feature to return different values depending on the number of times a method has been called.

```rust

#[test]
fn test_times() {
    let mut mock = MockEmailSender::new();
    let email = /* */;
    mock.expect_send()
        .times(1)
        .returning(|_| Ok(()));
    mock.expect_send()
        .times(1)
        .returning(|_| Err(/* */));

    // This returns Ok(())...
    mock.send(&email);
    // ...while this returns Err!
    mock.send(&email);
}
```

## `Sequence`

What we have seen so far works well when you need to return different values for different invocations of the same method.  
You can take this one step further by defining a sequence of calls that your mock object should expect, spanning
multiple methods.

```rust
#[test]
fn test_sequence() {
    let mut mock = MockEmailSender::new();
    let email = /* */;
    let mut sequence = Sequence::new();
    mock.expect_send()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(|_| Ok(()));
    mock.expect_get_inbox()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(|_| Ok(/* */));

    // This panics because the sequence expected `send` to be called first!
    mock.get_inbox();
}
```

When using a `Sequence`, you need to make sure that the methods are invoked in the order specified by the sequence.  
Invoking `get_inbox` before `send` will cause the test to fail, even if they are both called exactly once.

## Exercise

Configure the mocks to get tests to pass.
