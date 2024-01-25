#[cfg(test)]
mod tests {
    #[test]
    fn intro() {
        let msg = "I've installed __!";
        assert_eq!(msg, "I've installed `insta`!")
    }
}
