# Quaking like `cargo test`

As you have seen in the previous exercise, there are no requirements on your test
entrypoint beyond... _existing_.\
You can execute arbitrary logic, print in whatever format, etc.\
The only thing that `cargo test` cares about is the exit code of your test executable,
which must be `0` if all tests passed, and `1` otherwise.

## Integration brings benefits

Your test harness might be custom, but it's still being invoked via `cargo test`.\
As a CLI command, `cargo test` exposes quite a few knobs: you can list tests, filter them,
control the number of threads used to run them, etc.

All those features become **demands** on your custom test harness: are you going to
honor them? Or are you going to ignore them?

The latter is less work, but the resulting behaviour will surprise your user.
If I run `cargo test <test_name>`, I expect only `<test_name>` to be run, not all tests.\
But if your custom test harness ignores CLI arguments, that's exactly what will happen.

The same applies when interacting with other tools—e.g. CI systems. If your test
report format is not compatible with `cargo test`'s, you'll have to write a custom
adapter to make it work.

## `libtest_mimic`

Matching `cargo test`'s behaviour is a lot of work.\
Luckily, you don't have to do it yourself: [`libtest_mimic`](https://docs.rs/libtest-mimic)
can take over most of the heavy lifting.

It provides an [`Arguments`](https://docs.rs/libtest-mimic/latest/libtest_mimic/struct.Arguments.html)
struct that can be used to parse `cargo test`'s CLI arguments.\
`Arguments` is one of the two inputs to their [`run`](https://docs.rs/libtest-mimic/latest/libtest_mimic/fn.run.html)
function, the other being all the tests in your test suite.
`run` interprets the parsed arguments and runs the tests accordingly (e.g. listing them,
filtering them, etc.). It's a testing framework, so to speak.
