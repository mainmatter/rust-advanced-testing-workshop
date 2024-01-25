#[cfg(test)]
mod tests {
    #[test]
    fn end() {
        let msg = "I'm done with snapshot testing!";
        assert_eq!(msg, "I'm done with snapshot testing!")
    }
}
