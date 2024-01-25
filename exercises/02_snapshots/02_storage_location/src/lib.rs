#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    #[test]
    fn snapshot() {
        let m = "The new value I want to save";
        assert_snapshot!(m)
    }
}
