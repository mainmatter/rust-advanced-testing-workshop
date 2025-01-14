//! Before digging any deeper, let's get familiar with the basics of `googletest` by rewriting the tests
//! from the previous exercise.
#[cfg(test)]
mod tests {
    use googletest::assert_that;

    // The default `#[test]` attribute is not enough if you want to use some of
    // `googletest`'s macros (e.g. `expect_that!`). You need to use `#[googletest::gtest]` instead.
    // Our recommendation: if you're using `googletest`, default to using `#[googletest::gtest]`
    // rather than `#[test]`.
    //
    // You'll learn how to write a custom test macro later in the workshop!
    #[googletest::gtest]
    fn is_empty() {
        let v: Vec<i32> = vec![];
        // The `assert_that!` macro is the equivalent of `assert!` from the standard library.
        // It takes two arguments: the value you want to assert on, and the **matcher** you want to use.
        // You can find all the built-in matchers in the [`matchers`](https://docs.rs/googletest/0.13.0/googletest/index.html#available-matchers)
        // module of the `googletest` crate. Find the right one!
        assert_that!(v, todo!());
        // ☝️Assert it's empty
    }

    #[googletest::gtest]
    fn one_value() {
        let v: Vec<i32> = vec![-1];
        // Assert it's empty
    }

    #[googletest::gtest]
    fn two_values() {
        let v: Vec<i32> = vec![-1, 1];
        // Assert it's empty
    }
}
