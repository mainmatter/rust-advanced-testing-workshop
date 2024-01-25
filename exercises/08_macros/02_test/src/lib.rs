#[cfg(test)]
mod tests {
    #[macros02::vanilla_test]
    fn without_test() {
        assert!(true);
    }

    #[macros02::vanilla_test]
    #[test]
    fn with_test() {
        assert!(true);
    }

    // There's various ways of spelling `#[test]`!
    // If you want your macros to be robust, you need to handle them.
    #[macros02::vanilla_test]
    #[core::prelude::v1::test]
    fn with_fq_test() {
        assert!(true);
    }
}
