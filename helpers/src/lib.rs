use std::path::PathBuf;

/// Execute a binary with the given `name`, under the assumption that it was defined
/// in the current workspace.
///
/// It looks for a build using the same profile (usually `debug`) in the `target` folder.
pub fn run_workspace_test_binary(name: &'static str) -> Result<TestOutput, anyhow::Error> {
    // Cargo doesn't give us the target directory as an environment variable,
    // so we have to compute it ourselves.
    // We rely on the fact that all exercise crates are nested 2 levels deep
    // under the `exercise` folder.
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let target_dir = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target");
    let command_path = target_dir.join("debug").join(name);
    let mut command = std::process::Command::new(command_path);
    Ok(TestOutput::new(command.output()?))
}

pub struct TestOutput {
    pub stdout: String,
    pub stderr: String,
}

impl TestOutput {
    pub fn new(raw: std::process::Output) -> Self {
        let stdout = String::from_utf8(raw.stdout).expect("stdout is not valid UTF-8");
        let stderr = String::from_utf8(raw.stderr).expect("stderr is not valid UTF-8");
        Self { stdout, stderr }
    }

    pub fn stripped_stderr(&self) -> String {
        self.stderr
            .lines()
            .filter(|line| {
                !(line.starts_with("thread 'main' panicked")
                    || line.starts_with("note: run with `RUST_BACKTRACE=1`"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
