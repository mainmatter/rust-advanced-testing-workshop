#[cfg(test)]
mod tests {
    #[test]
    fn ctr_is_installed_and_on_path() {
        std::process::Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg("ctr")
            .arg("--bin")
            .arg("ctr")
            .output()
            .expect("Failed to invoke `ctr` through `cargo`");
    }

    #[test]
    fn nightly_is_installed() {
        let output = std::process::Command::new("rustup")
            .arg("run")
            .arg("nightly")
            .arg("cargo")
            .arg("--version")
            .output()
            .expect("Failed to run rustup run nightly cargo --version");

        assert!(
            output.status.success(),
            "`rustup run nightly cargo --version` failed:\n\t{}\n\nTo fix the error, install the nightly toolchain on your machine:\n\n\n\trustup toolchain install nightly\n",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
