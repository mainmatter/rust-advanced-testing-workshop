use anyhow::Context;
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use pretty_assertions::StrComparison;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
struct Expectations {
    tests: Vec<TestExpectation>,
}

#[derive(Debug, serde::Deserialize)]
struct TestExpectation {
    name: String,
    #[serde(flatten)]
    outcome: ExpectedOutcome,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "expected_outcome")]
#[serde(rename_all = "snake_case")]
enum ExpectedOutcome {
    Success,
    Failure { expected_output: String },
}

fn main() {
    if let Err(e) = entrypoint() {
        eprintln!("Failed to verify expectations.\n{:?}", e);
        std::process::exit(1);
    } else {
        println!("✅ All good!");
    }
}

fn entrypoint() -> Result<(), anyhow::Error> {
    let expectations_filepath = PathBuf::from("expectations.yml");
    let raw_expectations = fs_err::read_to_string(expectations_filepath)
        .expect("Failed to read `expectations.yml` file in the current directory");
    let expectations: Expectations =
        serde_yaml::from_str(&raw_expectations).expect("Failed to parse `expectations.yml` file");
    let outcomes = run_tests().context("Failed to run tests")?;

    // Exhaustiveness check: the list of tests in `expectations.yml` should match the list of tests
    // that `cargo test` ran.
    let discovered_tests: HashSet<_> = outcomes.keys().collect();
    for test_name in discovered_tests {
        if !expectations
            .tests
            .iter()
            .any(|test| &test.name == test_name)
        {
            panic!(
                "Test `{}` was run by `cargo test`, but it is not listed in `expectations.yml`.\n\
                This is a bug in the workshop, please report it to the instructor!",
                test_name
            )
        }
    }

    let mut failed = false;

    for test in expectations.tests {
        let outcome = outcomes.get(&test.name).with_context(|| {
            format!(
                "There is no entry in `cargo test` output for a test named `{}`",
                &test.name
            )
        })?;
        let intro_msg = format!("🔘 Checking test `{}` against expectations", test.name);
        println!("{}", intro_msg.bold());
        match outcome {
            TestOutcome::Ok { .. } => {
                if let ExpectedOutcome::Failure { .. } = test.outcome {
                    println!(
                        "{}",
                        format!("Test `{}` succeeded, but was expected to fail", test.name)
                            .bold()
                            .red()
                    );
                    failed = true;
                }
            }
            TestOutcome::Failed {
                clean_stdout,
                raw_stdout,
            } => match test.outcome {
                ExpectedOutcome::Success => {
                    println!(
                        "{}\n{}:\n{}",
                        format!("Test `{}` failed, but was expected to succeed.", test.name)
                            .bold()
                            .red(),
                        "Raw test output:".bold(),
                        textwrap::indent(raw_stdout, "    ")
                    );
                    failed = true;
                }
                ExpectedOutcome::Failure { expected_output } => {
                    if clean_stdout != &expected_output {
                        let failure_msg_1 = format!("❌ `{}`", test.name);
                        let failure_msg_2 = "The test failed as expected, but the failure output doesn't match the expected output from `expectations.yml`.".to_string();
                        println!("{}\n{}\n", failure_msg_1.red().bold(), failure_msg_2.red());
                        println!("{}", StrComparison::new(&clean_stdout, &expected_output));
                        println!(
                            "{}\n{}",
                            "Raw test output:".bold(),
                            textwrap::indent(raw_stdout, "    ")
                        );
                        failed = true;
                    }
                }
            },
            TestOutcome::Timeout => {
                println!("{}", format!("Test `{}` timed out", test.name).bold().red());
                failed = true;
            }
        }
    }

    if failed {
        anyhow::bail!("One or more tests didn't behave as expected")
    }
    Ok(())
}

/// Execute `cargo test` in the current directory.
///
/// We use the unstable `--format` option to get the output in JSON format.
/// The output is going to be a mix of JSON and non-JSON lines, so we need to
/// filter out the non-JSON lines.
/// For the JSON lines, we parse them and keep the ones with `type` set to `test`,
/// indicating the outcome of a particular test.
/// We then return a `test name -> test outcome` mapping.
fn run_tests() -> Result<HashMap<String, TestOutcome>, anyhow::Error> {
    static THREAD_PANIC: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r#"thread \'[a-zA-Z0-9\:\-\_]+\' panicked at [a-zA-Z0-9\-\\\_\/\.]+\:(?<row>\d+)\:(?<column>\d+)"#)
            .expect("Failed to compile regex")
    });
    static MISSING_NIGHTLY: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r#"error: toolchain 'nightly-[a-zA-Z0-9\-]+' is not installed"#)
            .expect("Failed to compile regex")
    });
    static GOOGLETEST_PANIC: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r#"\s*at [a-zA-Z0-9\-\_\/\.]+\:(?<row>\d+)\:(?<column>\d+)"#)
            .expect("Failed to compile regex")
    });

    let mut command = std::process::Command::new("rustup");
    command
        .arg("run")
        .arg("nightly")
        .arg("cargo")
        .arg("test")
        .arg("--quiet")
        .arg("--no-fail-fast")
        .arg("--")
        .arg("-Z")
        .arg("unstable-options")
        .arg("--format")
        .arg("json");
    let raw_output = command
        .output()
        .context("Failed to run `rustup run nightly cargo test`")?;
    let stdout = String::from_utf8(raw_output.stdout).context("stdout contains invalid UTF-8")?;
    let stderr = String::from_utf8(raw_output.stderr).context("stdout contains invalid UTF-8")?;
    if MISSING_NIGHTLY.is_match(&stderr) {
        anyhow::bail!(
            "You need to install the nightly toolchain: `rustup toolchain install nightly`"
        )
    }
    let mut test_outcomes = HashMap::new();
    for line in stdout.lines() {
        let Ok(libtest_msg) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if !is_test_event(&libtest_msg) {
            continue;
        }
        let libtest_msg = serde_json::from_value::<LibtestMessage>(libtest_msg)
            .context("Failed to parse libtest message")?;

        let test_name = libtest_msg
            .name
            .split("::")
            .last()
            .expect("Failed to extract test name from libtest message")
            .to_owned();
        let test_outcome = match libtest_msg.event_data {
            TestEventData::Started => continue,
            TestEventData::Failed { stdout } => {
                let mut clean_stdout = Vec::new();
                let mut found_non_empty_line = false;
                for line in stdout.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() && !found_non_empty_line {
                        continue;
                    }
                    if THREAD_PANIC.is_match(line)
                        || GOOGLETEST_PANIC.is_match(trimmed)
                        || trimmed.starts_with("note: run with `RUST_BACKTRACE=1`")
                        || trimmed == "Test failed"
                        || trimmed == "Error: See failure output above"
                    {
                        continue;
                    }
                    found_non_empty_line = true;
                    clean_stdout.push(line);
                }
                // Remove trailing empty lines
                while clean_stdout
                    .last()
                    .map(|l| l.trim().is_empty())
                    .unwrap_or(false)
                {
                    clean_stdout.pop();
                }

                let clean_stdout = clean_stdout.join("\n");
                TestOutcome::Failed {
                    clean_stdout,
                    raw_stdout: stdout,
                }
            }
            TestEventData::Ok => TestOutcome::Ok,
            TestEventData::Timeout => TestOutcome::Timeout,
        };
        test_outcomes.insert(test_name, test_outcome);
    }
    Ok(test_outcomes)
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "event")]
enum TestOutcome {
    Failed {
        clean_stdout: String,
        raw_stdout: String,
    },
    Ok,
    Timeout,
}

fn is_test_event(libtest_msg: &serde_json::Value) -> bool {
    let Some(type_value) = libtest_msg.get("type") else {
        return false;
    };
    let Some(type_value) = type_value.as_str() else {
        return false;
    };
    type_value == "test"
}

/// Reference for the JSON output of `libtest`: https://github.com/rust-lang/rust/blob/master/library/test/src/formatters/json.rs
///
/// We only model events with `type` set to `test`. We ignore everything else.
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct LibtestMessage {
    name: String,
    #[serde(flatten)]
    event_data: TestEventData,
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "event")]
#[serde(rename_all = "snake_case")]
enum TestEventData {
    Started,
    Failed { stdout: String },
    Ok,
    Timeout,
}
