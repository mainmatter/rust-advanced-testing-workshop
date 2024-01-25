#[cfg(test)]
mod tests {
    #[test]
    fn end() {
        let msg = "I'm done with __!";
        assert_eq!(msg, "I'm done with custom test runners!")
    }
}
