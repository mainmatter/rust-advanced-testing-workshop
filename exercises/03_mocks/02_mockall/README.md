# `mockall`

In the previous exercise you've manually implemented a do-nothing logger for your tests.  
It can get tedious to do that for every dependency you want to mock. Let's bring some automation to the party!

## Mocking with `mockall`

[`mockall`](https://docs.rs/mockall/) is the most popular auto-mocking library for Rust.  
It's built around the `#[automock]` attribute, which generates a mock implementation of a trait for you.

Let's look at an example:

```rust
pub trait EmailSender {
  fn send(&self, email: &Email) -> Result<(), EmailError>;
}
```

To generate a mock implementation of `EmailSender`, you need to add the `#[automock]` attribute to the trait:

```rust
use mockall::automock;

#[automock]
pub trait EmailSender {
  fn send(&self, email: &Email) -> Result<(), EmailError>;
}
```

`mockall` will generate a struct named `MockEmailSender` with an implementation of the `EmailSender` trait.  
Each of the methods in the trait will have a counterpart in the mock struct, prefixed with `expect_`.  
By calling `expect_send`, you can configure how `MockEmailSender` will behave when the `send` method is called.  
In particular, you can define:

- **Preconditions** (e.g. assertions on the arguments passed to the method)
- **Expectations** (e.g. how many times you expect the method to be called)
- **Return values** (e.g. what the method should return)

In an example test:

```rust
#[test]
fn test_email_sender() {
    let mut mock = MockEmailSender::new();
    mock.expect_send()
        // Precondition: do what follows only if the email subject is "Hello"
        .withf(|email| email.subject == "Hello")
        // Expectation: panic if the method is not called exactly once
        .times(1)
        // Return value
        .returning(|_| Ok(()));

    // [...]
}
```

## A word on expectations

Expectations such as `times` are a powerful feature of `mockall`. 
They allow you to test how your code interacts with the dependency that's being mocked.  

At the same time, they should be used sparingly.  
Expectations **couple your test to the implementation of the code under test**.

Only use expectations when you explicitly want to test how your code interacts with the dependency—e.g.
you are testing a retry mechanism and you want to make sure that it retries according to the configured policy.  
Avoid settings `times` expectations on every mock method in your tests just because you can.

## Exercise

Use `mockall` to mock the `Logger` trait in the `square` function.  
Use the generated mock in the test.