#[cfg(test)]
mod tests {
    #[test]
    fn intro() {
        let msg = "I'm ready to learn about snapshot testing!";
        assert_eq!(msg, "I'm ready to learn about snapshot testing!")
    }

    #[test]
    fn insta_is_installed() {
        let output = std::process::Command::new("cargo")
            .arg("insta")
            .arg("--help")
            .output()
            .expect("Failed to run `cargo insta --version`");

        assert!(
            output.status.success(),
            "`cargo insta --help` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
