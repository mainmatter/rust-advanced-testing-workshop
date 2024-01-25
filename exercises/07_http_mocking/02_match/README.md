# Matchers

When configuring a `Mock`, you can specify one or more _matchers_ for incoming requests.  
The `Mock` is only triggered if the incoming request satisfies all the matchers attached to it.

## Common matchers

The `wiremock` crate provides an extensive collection of matchers out of the box.  
Check out the documentation of the [`matchers`](https://docs.rs/wiremock/latest/wiremock/matchers/index.html) 
module for the full list.

## Writing your own matchers

Occasionally, you'll need to write your own matchers, either because you need to match on a property that's not
supported by the built-in matchers, or because you want to build a higher-level matcher out of existing ones.

To write a custom matcher, you need to implement the [`Match`](https://docs.rs/wiremock/latest/wiremock/trait.Match.html) trait:

```rust
pub trait Match: Send + Sync {
    // Required method
    fn matches(&self, request: &Request) -> bool;
}
```

The trait is quite straight-forward. It has a single method, `matches`, that takes a reference to the incoming `Request` 
and returns a `bool`: `true` if the request matches, `false` otherwise.

## Exercise

Write a custom matcher that matches if:

- The method is `POST`
- The `Content-Type` header is present and set to `application/json`
- The request body is a valid JSON object
- The `Content-Length` header is set and its value matches the length of the request body (in bytes)