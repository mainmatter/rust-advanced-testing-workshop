//! Use [`tempfile::NamedTempFile`] to fill in the blanks in the tests.
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn get_cli_path(config_path: &Path) -> PathBuf {
    let config = std::fs::File::open(config_path).expect("Failed to open config file");
    let reader = BufReader::new(config);

    let path = reader
        .lines()
        .next()
        .expect("The config file is empty")
        .expect("First line is not valid UTF-8");
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::eq;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[googletest::gtest]
    // Tip: you can use `expected` to specify a value that must be **contained** in the panic message!
    #[should_panic(expected = "Failed to open config file")]
    fn panics_if_file_does_not_exist() {
        let config_file = todo!();
        let config_path = todo!();
        super::get_cli_path(&config_path);
    }

    #[googletest::gtest]
    #[should_panic(expected = "The config file is empty")]
    fn panics_if_file_is_empty() {
        let config_file = todo!();
        super::get_cli_path(config_file.path());
    }

    #[googletest::gtest]
    #[should_panic(expected = "First line is not valid UTF-8")]
    fn panics_if_file_contains_invalid_utf8() {
        let invalid_utf8 = [0xFF];
        let mut config_file = todo!();
        todo!();
        super::get_cli_path(config_file.path());
    }

    #[googletest::gtest]
    fn happy_path() {
        let cli_path = PathBuf::from("my_cli");

        let mut config_file = todo!();
        todo!();

        let actual = super::get_cli_path(config_file.path());
        assert_that!(&actual, eq(&cli_path));
    }
}
