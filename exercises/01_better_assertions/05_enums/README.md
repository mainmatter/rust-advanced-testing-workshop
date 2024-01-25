# `googletest` - Matching patterns

The matchers we've seen in the previous exercise are specialised for `Option` and `Result`, but `googletest` also has
a more generic matcher to match variants of arbitrary enums (and other patterns).

## Exercise

Replace the `assert!` calls with the corresponding `googletest` matchers.  

Take a moment to read the error messages and compare them with the ones you got from `assert!`.
Notice how `googletest` sees through variable names and prints the actual values if the variant wasn't what the assertion expected!
