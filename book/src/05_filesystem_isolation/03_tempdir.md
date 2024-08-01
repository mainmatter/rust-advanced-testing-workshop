# Path coupling

Our testing, so far, has been limited to cases where the code interacts with a single isolated file.\
Real-world codebases are rarely that simple.

More often than not, you'll have to deal with multiple files and there'll be assumptions as to where
they are located relative to each other.

Think of `cargo` as an example: it might load the `Cargo.toml` manifest for a workspace and then go looking
for the `Cargo.toml` files of each member crate based on the relative paths specified in the workspace manifest.\
If you just create a bunch of `NamedTempFile`s, it won't work: the paths will be completely random
and the code will fail to find the files where it expects them.

## `tempdir`

The `tempfile` crate provides a solution for this
scenario: [`TempDir`](https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html).\
With the default configuration, it will create a temporary directory inside the system's temporary directory.\
You can then create files inside of that directory using the usual `std::fs` APIs, therefore controlling the
(relative) paths of the files you create.

When `TempDir` is dropped, it will delete the directory and all its contents.

## Working directory

To work with `TempDir` effectively, it helps to structure your code in a way that minimises the number of
assumptions it makes about the **current working directory**.

Every time you're using a relative path, you're relying on the current working directory: you're
reading `{working_directory}/{relative_path}`.

The current working directory is set on a per-process basis.\
As you learned in the interlude, that implies that it is shared between tests,
since multiple tests can be compiled into the same binary and run in the same process.\
Running `std::env::set_current_dir` in one test will affect the outcome of the other tests,
which is not what we want.

The solution is to make all paths relative to **a configurable root directory**.\
The root directory is set by the binary entrypoint (e.g. `main`), and it's then passed down to the rest of the
codebase.\
You can then set the root directory to the path of a `TempDir` in your tests, and you're good to go!
