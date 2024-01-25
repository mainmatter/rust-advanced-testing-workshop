# Test runners

In the [interlude](../../04_interlude/README.md) we had a first look under the hood 
of `cargo test`. In particular, you learned how tests are grouped into executables and
reflected on the implications.

In this chapter, we'll take things one step further: you'll write your own test harness!

## Test targets

In your past projects you might have had to set properties for your binary (`[[bin]]`) 
and library (`[lib]`) targets in your `Cargo.toml`.  
You can do the same for your **test targets**!

```toml
[[test]]
name = "integration"
```

The configuration above declares the existence of a test target named `integration`.  
By default, `cargo` expects to find it in `tests/integration.rs`. You can also customize 
the path to the test entrypoint using the `path` property.

You don't often see `[[test]]` targets in the wild because `cargo` infers them automatically—i.e.
if you have a `tests/integration.rs` file, it will automatically be compiled and run as an integration test.

When you see a `[[test]]` target in a `Cargo.toml`, it's usually because the author wants to disable
the **default test harness**:

```toml
[[test]]
name = "integration"
# 👇 That's enabled by default
harness = false
```

## Test harness

The **test harness** is the code that `cargo` invokes to run each of your test suites.  

When `harness` is set to `true`, `cargo` automatically creates an entrypoint (i.e. a `main` function)
for your test executable using [`libtest`](https://github.com/rust-lang/rust/tree/master/library/test),
the default test harness.

When `harness` is set to `false`, `cargo` expects you to provide your own entrypoint.  

## Pros and cons

With a custom test harness, you are in charge!  
You can execute logic before and after running your tests, you can customise how each test
is run (e.g. running them in separate processes), etc.

At the same time, you need to provide an entrypoint that integrates well with `cargo test`'s 
CLI interface. Listing, filtering, etc. are all features that you'll need to add support for,
they don't come for free.

## This section

We'll start by writing a simple test runner, to get familiar with the basics.
We'll then explore `libtest_mimic`, a crate that takes over most of the heavy lifting
required to write a high-quality custom test runner.

Let's get started!

