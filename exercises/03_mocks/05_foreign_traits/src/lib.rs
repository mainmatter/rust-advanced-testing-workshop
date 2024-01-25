//! Use `mock!` to generate a mock type named `MockParsed` that implements the `FromStr` trait from the standard library.
use std::str::FromStr;

// Mock the `FromStr` trait from the standard library.
mockall::mock! {
    Parsed {}
    impl FromStr for Parsed {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<Self, <MockParsed as std::str::FromStr>::Err>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements() {
        static_assertions::assert_impl_one!(MockParsed: FromStr);
    }
}
