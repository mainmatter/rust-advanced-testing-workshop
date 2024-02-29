#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::matches_pattern;

    #[derive(Debug)]
    enum MyCustomEnum {
        A,
        B(u32),
        C { a: &'static str },
    }

    #[googletest::test]
    fn failed_is_b() {
        let x = MyCustomEnum::A;
        // This will become `assert_matches!` once it stabilises!
        assert!(matches!(x, MyCustomEnum::B(_)));
    }

    #[googletest::test]
    fn failed_is_c() {
        let x = MyCustomEnum::B(10);
        assert!(matches!(x, MyCustomEnum::C { .. }));
    }

    #[googletest::test]
    fn failed_is_c_with_value() {
        let x = MyCustomEnum::B(10);
        assert!(matches!(x, MyCustomEnum::C { a: "hello" }));
    }
}
