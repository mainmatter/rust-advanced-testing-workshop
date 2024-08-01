//! Run the test in this exercise with `wr`: it should fail.
//! Then use `cargo insta review` to review the changes and update the snapshots.
#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    #[test]
    fn snapshot() {
        let m = "The new value I want to save";
        assert_snapshot!(m, @"The old snapshot I want to compare against")
    }
}
