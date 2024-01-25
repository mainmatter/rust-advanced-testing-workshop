# `googletest` - Custom matchers

Built-in matchers can only take you so far. Sometimes you need to write your own!

## The `Matcher` trait

All matchers must implement the [`Matcher`](https://docs.rs/googletest/0.10.0/googletest/matcher/trait.Matcher.html) 
trait. There are two key methods you need to implement:

- `matches`. It returns [`MatcherResult::Match`](https://docs.rs/googletest/0.10.0/googletest/matcher/enum.MatcherResult.html#variant.Match)
  if it matched, [`MatcherResult::NoMatch`](https://docs.rs/googletest/0.10.0/googletest/matcher/enum.MatcherResult.html#variant.NoMatch) otherwise.
- `describe`. It returns a description of the outcome of the match. This is shown to the user when the match fails.

Optionally, you can also implement the `explain_match` method if you want to include further information
derived from the actual and expected values in the failure message shown to the user.

## Patterns

Most matchers in `googletest` follow the same pattern.  
You define two items:

- A struct which implements the `Matcher` trait (e.g. [`EqMatcher`](https://docs.rs/googletest/0.10.0/googletest/matchers/struct.EqMatcher.html))
- A free function that returns an instance of the struct (e.g. [`eq`](https://docs.rs/googletest/0.10.0/googletest/matchers/fn.eq.html))

The free function is a convenience for the user since it results in terser assertions.  
You can also choose to make the struct type private, returning `impl Matcher` from the free function instead
(see [`anything`](https://docs.rs/googletest/0.10.0/googletest/matchers/fn.anything.html) as an example).

## Exercise

Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.