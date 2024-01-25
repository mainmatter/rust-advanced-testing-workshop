#[cfg(test)]
mod tests {
    #[test]
    fn interlude() {
        let msg = "I understand what happens when I run `cargo test`!";
        assert_eq!(msg, "I understand what happens when I run `cargo test`!")
    }
}
