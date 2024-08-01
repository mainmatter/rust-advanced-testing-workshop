# Handling non-reproducible data

Sometimes the data you want to snapshot cannot be reproduced deterministically in different runs of the test.\
For example, it might contain the current timestamp or a random value.

In these cases, you can use **redactions** to remove the non-reproducible parts of the data
before taking the snapshot (and before comparing it with the saved one).

## Redactions

Redactions are specified as an additional argument of the assertion macro you're using.\
They only work for _structured formats_ (e.g. JSON, XML, etc.). If you're snapshotting a string, you can use
[regex filters](https://insta.rs/docs/filters/) instead.

Redactions use a `jq`-style syntax to specify the parts of the data you want to remove:
refer to the [documentation](https://insta.rs/docs/redactions/#selectors) for an exhaustive reference.
