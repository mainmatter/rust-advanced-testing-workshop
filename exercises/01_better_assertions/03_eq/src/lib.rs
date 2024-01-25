#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{empty, eq, gt, lt, not};

    #[googletest::test]
    fn failed_eq() {
        let x = 1;
        let y = 2;
        assert_eq!(x, y);
    }

    #[googletest::test]
    fn failed_neq() {
        let x = 1;
        let y = 1;
        assert_ne!(x, y);
    }

    #[googletest::test]
    fn failed_greater_than() {
        let x = 3;
        let y = 4;
        assert!(x > y);
    }

    #[googletest::test]
    fn failed_less_than() {
        let x = 10;
        let y = -1;
        assert!(x < y);
    }
}
