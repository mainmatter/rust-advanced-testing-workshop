# Temporary files

[`NamedTempFile`](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html) solved our issues 
in the previous exercise. But how does it work?

## Temporary file directory

Most operating systems provide a [temporary file directory](https://en.wikipedia.org/wiki/Temporary_folder).  
You can retrieve the path to the temporary directory using [`std::env::temp_dir`](https://doc.rust-lang.org/std/env/fn.temp_dir.html).
Files created in that directory will be automatically deleted _at a later time_.

## When are temporary files deleted?

When using `NamedTempFile`, there are two deletion mechanisms at play:

- `NamedTempFile` will delete the file when it is dropped.  
  This is robust in the face of panics (if they don't abort!) and is the main mechanism `tempfile` relies on.
- If destructor-based deletion fails, the OS will eventually delete the file since it's in the temporary directory.

The latter mechanism is not guaranteed to run at any specific time, therefore `NamedTempFile` tries to generate
a unique filename to minimise the risk of collision with a leaked file.

## Security

There are a fair number of OS-specific details to take into account when working with temporary files, but
`tempfile` takes care of all of them for us.  
In particular, there are some [security considerations](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html#security-1)
when working with `NamedTempFile`. When it comes to usage in test suites, you're in the clear.

## `tempfile()`

`tempfile` also provides a [`tempfile()`](https://docs.rs/tempfile/latest/tempfile/fn.tempfile.html) function
that returns a special kind of `File`: the OS is guaranteed to delete the file when the last handle to it is dropped.

There's a caveat though: **you can't access the path to the file**.

## Refactoring

We could choose to refactor `get_cli_path` to make `tempfile()` viable for our testing needs.

```rust
use std::io::BufRead;
use std::path::PathBuf;

fn get_cli_path<R>(config: R) -> PathBuf
where
    R: BufRead,
{
    let path = config
        .lines()
        .next()
        .expect("The config is empty")
        .expect("First line is not valid UTF-8");
    PathBuf::from(path)
}
```

We are no longer performing any filesystem operation in `get_cli_path`: the configuration "source" is now abstracted
behind the `BufRead` trait.  
We could now use `get_cli_path` to process a `File`, a `String` (using `std::io::Cursor`), or other types that implement
`BufRead`.  

This is a valuable refactoring to have in your toolkit, but it's not a panacea.  
You'll still need to deal with that filesystem access at some point. You could move it to the binary entrypoint, but
does it really count as "thin" and "boring"?  
You'll probably have logic to handle failures, with different code paths depending on the error. You should test that!  

Evaluate on a case-by-case basis whether it's worth it to refactor your code to make it easier to test with 
something like `tempfile()`.

## Exercise

Use `tempfile()` to fill in the blanks in the tests.