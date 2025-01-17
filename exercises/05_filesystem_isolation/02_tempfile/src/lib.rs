//! Use `tempfile()` to fill in the blanks in the tests.
use std::io::BufRead;
use std::path::PathBuf;

fn get_cli_path<R>(config: R) -> PathBuf
where
    R: BufRead,
{
    let path = config
        .lines()
        .next()
        .expect("The config is empty")
        .expect("First line is not valid UTF-8");
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::eq;
    use std::io::{BufReader, Seek, SeekFrom, Write};
    use std::path::PathBuf;
    use tempfile::tempfile;

    #[googletest::gtest]
    #[should_panic(expected = "The config is empty")]
    fn panics_if_config_is_empty() {
        let mut config = BufReader::new(todo!());
        super::get_cli_path(&mut config);
    }

    #[googletest::gtest]
    #[should_panic(expected = "First line is not valid UTF-8")]
    fn panics_if_config_contains_invalid_utf8() {
        let invalid_utf8 = [0xFF];
        let mut config = todo!();

        super::get_cli_path(BufReader::new(config));
    }

    #[googletest::gtest]
    fn happy_path() {
        let cli_path = PathBuf::from("my_cli");

        let mut config = todo!();

        let actual = super::get_cli_path(BufReader::new(config));
        assert_that!(&actual, eq(&cli_path));
    }
}
