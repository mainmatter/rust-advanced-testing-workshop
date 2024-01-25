//! Replace the `assert_eq!`/`assert_ne!` calls with the corresponding `googletest` matchers.
//!
//! Take a moment to read the error messages and compare them with the ones you got from `assert_eq!`/`assert_ne!`.
//! Notice how `googletest` sees through variable names and prints the actual values that were compared!
#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{empty, eq, gt, lt, not};

    #[googletest::gtest]
    fn failed_eq() {
        let x = 1;
        let y = 2;
        assert_that!(x, eq(y));
    }

    #[googletest::gtest]
    fn failed_neq() {
        let x = 1;
        let y = 1;
        assert_that!(x, not(eq(y)));
    }

    #[googletest::gtest]
    fn failed_greater_than() {
        let x = 3;
        let y = 4;
        assert_that!(x, gt(y));
    }

    #[googletest::gtest]
    fn failed_less_than() {
        let x = 10;
        let y = -1;
        assert_that!(x, lt(y));
    }
}
