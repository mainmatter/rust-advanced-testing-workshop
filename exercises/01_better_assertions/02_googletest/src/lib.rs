#[cfg(test)]
mod tests {
    use googletest::assert_that;

    // The default `#[test]` attribute is not enough if you want to use some of
    // `googletest`'s macros (e.g. `expect_that!`). You need to use `#[googletest::test]` instead.
    // Our recommendation: if you're using `googletest`, default to using `#[googletest::test]`
    // rather than `#[test]`.
    //
    // You'll learn how to write a custom test macro later in the workshop!
    #[googletest::test]
    fn is_empty() {
        let v: Vec<i32> = vec![];
        // The `assert_that!` macro is the equivalent of `assert!` from the standard library.
        // It takes two arguments: the value you want to assert on, and the **matcher** you want to use.
        // You can find all the built-in matchers in the [`matchers`](https://docs.rs/googletest/0.11.0/googletest/index.html#available-matchers)
        // module of the `googletest` crate. Find the right one!
        assert_that!(v, todo!());
        // ☝️Assert it's empty
    }

    #[googletest::test]
    fn one_value() {
        let v: Vec<i32> = vec![-1];
        // Assert it's empty
    }

    #[googletest::test]
    fn two_values() {
        let v: Vec<i32> = vec![-1, 1];
        // Assert it's empty
    }
}
