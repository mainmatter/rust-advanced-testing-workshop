//! Use `libtest_mimic::run` to implement a test runner that matches `cargo test`'s behaviour.
//! Make sure to register all tests in your test suite, and to return the correct exit code.
use libtest_mimic::{Arguments, Failed, Trial};

mod tests;

fn main() {
    let args = Arguments::from_args();
    let tests = vec![
        Trial::test("happy_test", runner(tests::happy_test)),
        Trial::test("sad_test", runner(tests::sad_test)),
    ];
    libtest_mimic::run(&args, tests).exit()
}

// A test, for `libtest_mimic`, is a function with no arguments that returns a `Result<(), Failed>`.
// We adapt the signature of our tests to this format: if a test doesn't panic, it's a success.
fn runner(test: fn() -> ()) -> impl FnOnce() -> Result<(), Failed> + Send + 'static {
    move || {
        test();
        Ok(())
    }
}
