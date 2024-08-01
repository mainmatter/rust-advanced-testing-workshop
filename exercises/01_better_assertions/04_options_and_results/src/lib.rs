//! Replace the `assert!` calls with the corresponding `googletest` matchers.
//!
//! Take a moment to read the error messages and compare them with the ones you got from `assert!`.
//! Notice how `googletest` sees through variable names and prints the actual values if the variant wasn't what the assertion expected!
#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{anything, err, none, ok, some};

    #[googletest::test]
    fn failed_is_none() {
        let x = Some(1);
        assert!(x.is_none());
    }

    #[googletest::test]
    fn failed_is_some() {
        let x: Option<usize> = None;
        assert!(x.is_some());
    }

    #[googletest::test]
    fn failed_is_ok() {
        let x: Result<u32, &str> = Err("Something went wrong");
        assert!(x.is_ok());
    }

    #[googletest::test]
    fn failed_is_err() {
        let x: Result<u32, &str> = Ok(42);
        assert!(x.is_err());
    }
}
