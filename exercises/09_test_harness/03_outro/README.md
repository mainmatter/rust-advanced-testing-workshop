# Outro

A custom test harness gives you a great deal of flexibility, but there are some limitations.

## No `#[test]` attribute

The most obvious one is that you can't use the `#[test]` attribute.  
There is no built-in mechanism to automatically collect all annotated tests, as `cargo test` does with `#[test]`.  
You either have to manually register your tests (e.g. as you did in the previous exercise with that vector)
or find a way to automatically collect them (e.g. by establishing a file naming convention).

You can try to emulate distributed registration using some third-party crates (e.g. [`linkme`](https://crates.io/crates/linkme)
or [`inventory`](https://crates.io/crates/inventory)).

## Suite-scoped

Using a custom test harness you can customise how a _single_ test suite is run.  
If you need to perform some setup or teardown actions before or after _all_ test suites, you're out of luck. 
You still need to design some cross-process communication mechanism to coordinate across different test binaries.  
Alternatively, you need to replace `cargo test` with a different command that takes charge of 
collecting and running all your test binaries (e.g. like [`cargo-nextest`](https://crates.io/crates/cargo-nextest)).