# The testing system: a look under the hood

We won't move on to the next big topic (filesystem testing) just yet.  
Instead, we'll take a moment to understand what we've been using so far: the testing system. What 
happens when you run `cargo test`?

## Different kinds of tests

There are three types of tests in Rust:

- Unit tests
- Integration tests
- Doc tests

### Unit tests

Unit tests are the tests you write alongside your non-test code, _inside_ your Rust library or binary.  
They're the ones we've been writing so far: an inline module annotated with `#[cfg(test)]` and a bunch of
`#[test]` functions.

### Integration tests

Integration tests are tests that live _outside_ your Rust library or binary, in the special `tests/` directory.  
You don't need to annotate any module with `#[cfg(test)]` here: the compiler automatically assumes that
everything in `tests/` is under `#[cfg(test)]`.

### Doc tests

Doc tests are tests that live inside your documentation comments.  
They're a great way to make sure your examples are always up-to-date and working.

## Compilation units

Depending on the type of test, `cargo test` will compile and run your tests in different ways:

- All unit tests defined in the **same package** are compiled into a single binary and run together (i.e. in a single process).
- All the tests defined under the same **top-level item** under `tests/` (e.g. a single file `tests/foo.rs` 
  or a single directory `tests/foo/`) are compiled into a single binary and run together in the same process.
  Different top-level items are compiled into different binaries and run in different processes. 
- Each doc test is compiled into a separate binary and run in its own process.

This has a number of consequences:

- Any global in-memory state (e.g. variables behind a `lazy_static!` or `once_cell::Lazy`) is only shared between tests
  that are compiled into the same binary and run in the same process.
  If you want to synchronize access to a shared resource across **the entire test suite** (e.g. a database), 
  you need to use a synchronization primitive that works across processes.
- The more tests you have, the more binaries `cargo test` will need to compile and run. 
  Make sure you're using [a good linker](https://github.com/LukeMathWalker/zero-to-production/blob/main/.cargo/config.toml) 
  to minimize the time spent linking your tests.
- Any process-specific state (e.g. the current working directory) is shared between all the tests that are compiled
  into the same binary and run in the same process.  
  This means that if you change the current working directory in one test, it will affect other tests that share the same process!

The last point will turn out to be quite relevant in the next section: isolating tests that rely 
on the filesystem from each other.
  
> All the details above apply specifically to `cargo test`.  
> If you use a **different test runner**, you might get different behavior.
> We'll explore this later in the workshop with `cargo-nextest`.