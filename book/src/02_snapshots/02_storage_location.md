# Where do snapshots go?

## Inline

In the previous exercise we used an **inline snapshot**.\
Inline snapshots are stored in the test itself:

```rust
#[test]
fn snapshot() {
    let m = "The new value I want to save";
    assert_snapshot!(m, @"The old snapshot I want to compare against")
}
```

When you update the snapshot, the test source code is modified accordingly. Check again the `lib.rs` file
of the previous exercise to see it for yourself!

## External

Storing the snapshot inline has its pros: when you look at a test, you can immediately see what the expected value is.\
It becomes cumbersome, however, when the snapshot is large: it clutters the test and makes it harder to read.

For this reason, `insta` supports **external snapshots**.\
They are stored in a separate file and retrieved on the fly when the test is run:

```rust
#[test]
fn snapshot() {
    let m = "The new value I want to save";
    assert_snapshot!(m)
}
```

By default, file snapshots are stored in a `snapshots` folder right next to the test file where this is used.
The name of the file is `<module>__<name>.snap` where the `name` is derived automatically from the test name.
You choose to set a custom name, if you want to:

```rust
#[test]
fn snapshot() {
    let m = "The new value I want to save";
    assert_snapshot!("custom_snapshot_name", m)
}
```
