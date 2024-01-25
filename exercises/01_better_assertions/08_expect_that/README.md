# `googletest` - `expect_that!`

All your `googletest` tests so far have used the `assert_that!` macro.  
If the assertion fails, it panics and the test fails immediately. No code after the assertion is executed.

## `expect_that!`

`googletest` provides another macro, called `expect_that!`.  
It uses the same matchers as `assert_that!`, but **it doesn't panic if the test fails**.  
When the test ends (either because the entire test function has been executed or because it later panicked), `googletest`
will check if any `expect_that!` assertions failed and report them as test failures.  

This allows you to write tests that check multiple things and report all the failures at once.  
A good use case is verifying multiple properties on the same object.

## Exercise

Use `expect_that!` to simplify `assert_that!` invocations and to check multiple properties on the same object.