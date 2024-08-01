# Outro

Custom test macros can get you a long way, but they're not a silver bullet.

## Complexity

Writing macros is its own skill: you can work with Rust successfully for years without ever having to
go beyond a `macro_rules!` definition.\
The next time you get the impulse to write a macro, ask yourself: if a colleague opens this file in 6 months,
will they be able to understand what's going on?

## Test-scoped

Furthermore, there's a limit to what you can do with custom test macros.\
Their action is scoped to a single test case and it's cumbersome to customise the way **the whole test suite** is run.

## Next

In the next chapter, we'll look at one more way to customise your tests: **custom test harnesses**.
