#[cfg(test)]
mod tests {
    #[test]
    fn assertion_with_message() {
        assert_eq!(
            2 + 2,
            5,
            "The Rust compiler hasn't read 1984 by George Orwell."
        )
    }
}
