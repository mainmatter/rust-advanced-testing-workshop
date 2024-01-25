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

    #[test]
    #[should_panic(expected = "The config is empty")]
    fn panics_if_config_is_empty() {
        let mut config = BufReader::new(tempfile().unwrap());
        super::get_cli_path(&mut config);
    }

    #[test]
    #[should_panic(expected = "First line is not valid UTF-8")]
    fn panics_if_config_contains_invalid_utf8() {
        let invalid_utf8 = [0xFF];
        let mut config = tempfile().unwrap();
        config.write_all(&invalid_utf8).unwrap();
        config.seek(SeekFrom::Start(0)).unwrap();

        super::get_cli_path(BufReader::new(config));
    }

    #[googletest::test]
    fn happy_path() {
        let cli_path = PathBuf::from("my_cli");

        let mut config = tempfile().unwrap();
        writeln!(config, "{}", cli_path.to_str().unwrap()).unwrap();
        config.seek(SeekFrom::Start(0)).unwrap();

        let actual = super::get_cli_path(BufReader::new(config));
        assert_that!(&actual, eq(&cli_path));
    }
}
