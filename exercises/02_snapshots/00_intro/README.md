# Snapshot testing

In all the tests we've written so far we've always manually created the expected value.  
This is fine for simple cases, but it can quickly become cumbersome when the expected value is complex
(e.g. a large JSON document) and it needs to be updated fairly often (e.g. the responses of a downstream API
service that's under active development).

To solve this problem we can use _snapshot testing_.  
You _snapshot_ the output of an operation and compare it with a previously saved snapshot. 
You then review the changes and decide whether they are expected or not: if they are, we can _automatically_ update the snapshot.

## `insta`

[`insta`](https://docs.rs/insta/) is an established snapshot testing library for Rust.

It comes with a CLI, `cargo-insta`, which we'll use to manage our snapshots.
Install it before moving forward:

```bash
cargo install --locked cargo-insta
```

Run `cargo insta --help` to make sure everything is working properly.