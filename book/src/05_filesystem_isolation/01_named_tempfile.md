# Implicit or explicit?

Testability is a property of software systems.\
Given a set of requirements, you can look at implementations with very different levels of testability.

This is especially true when we look at the interactions between the system under test and the host.

## Filesystem as a dependency

In Rust, any piece of code can choose to interact with the filesystem. You can create files, read files, delete files,
etc.\
It doesn't _necessarily_ show up in the function signature. The dependency can be **implicit**.

```rust
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

fn get_cli_path() -> PathBuf {
    let config = std::fs::File::open("config.txt").unwrap();
    let reader = BufReader::new(config);

    let path = reader.lines().next().unwrap().unwrap();
    PathBuf::from(path)
}
```

It is suspicious that `get_cli_path` is able to conjure a `PathBuf` out of thin air.
But it's not immediately obvious that it's interacting with the filesystem. It might also be
more obfuscated in a real-world codebase (e.g. there might be other inputs).

This is an issue when we want to test `get_cli_path`.\
We can create a file called `config.txt` where `get_cli_path` expects it to be, but things quickly become
complicated:

- We can't run tests in parallel if they all invoke `get_cli_path` _and_ if we need `get_cli_path` to return
  different values in different tests, since they would all be reading from the same file.
- We need to make sure that the file is deleted after each test, regardless of its outcome, otherwise there might be
  side-effects that affect the outcome of other tests (either in the same run or in a future run).

Let's see how we can refactor `get_cli_path` to mitigate both issues.

## Writing testable code, filesystem edition

### 1. Take paths as arguments

Instead of hard-coding the path to the config file in `get_cli_path`, we can take it as an argument.

```rust
use std::io::{BufReader, BufRead};
use std::path::{PathBuf, Path};

fn get_cli_path(config_path: &Path) -> PathBuf {
    let config = std::fs::File::open(config_path).unwrap();
    let reader = BufReader::new(config);

    let path = reader.lines().next().unwrap().unwrap();
    PathBuf::from(path)
}
```

### 2. If you need to hard-code a path, do it close to the binary entrypoint

If we need to hard-code a path, it is better to do it in the `main` function, or as close to the binary entrypoint as
possible.

```rust
use std::path::PathBuf;
use crate::get_cli_path;

fn main() {
    let config_path = PathBuf::from("config.txt");
    let cli_path = get_cli_path(&config_path);
}
```

This limits the scope of difficult-to-test code. In particular, the binary becomes a very thin (and boring) layer
around a library that can be tested in isolation.

> Having a thin binary layer around a library is a common pattern in Rust.
> It is a good pattern to adopt for testability, beyond the specifics of the filesystem.
> You'll see more examples of this pattern in action later in the workshop!

## `tempfile`

We've refactored `get_cli_path` to make it easier to test.\
But we still need to write those tests!

We have two problems to solve:

- Each test should use a different file, so that they don't interfere with each other and we can run them in parallel.
- We need to make sure that the file is deleted after each test, regardless of its outcome.

This is where the [`tempfile`](https://docs.rs/tempfile/) crate comes in handy!\
It provides tools to work with temporary files and directories. In this exercise (and the next)
we'll focus on how to leverage it!
