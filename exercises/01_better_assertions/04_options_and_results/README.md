# `googletest` - `Option` and `Result` matchers

`googletest` comes with a few special matchers for `Option` and `Result` that return good error messages 
when something that should be `Some` or `Ok` is actually `None` or `Err`, and vice-versa.

## Exercise

Replace the `assert!` calls with the corresponding `googletest` matchers.  

Take a moment to read the error messages and compare them with the ones you got from `assert!`.
Notice how `googletest` sees through variable names and prints the actual values if the variant wasn't what the assertion expected!
