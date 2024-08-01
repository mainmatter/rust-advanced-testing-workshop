# Summary

- [Welcome](00_intro/00_welcome.md)

- [Better assertions](01_better_assertions/00_intro.md)
  - [The built-in toolkit](01_better_assertions/01_std_assertions.md)
  - [`googletest`](01_better_assertions/02_googletest.md)
  - [Basic matchers](01_better_assertions/03_eq.md)
  - [`Option` and `Result` matchers](01_better_assertions/04_options_and_results.md)
  - [Enums](01_better_assertions/05_enums.md)
  - [Collections](01_better_assertions/06_collections.md)
  - [Custom matchers](01_better_assertions/07_custom_matcher.md)
  - [`expect_that!`](01_better_assertions/08_expect_that.md)

- [Snapshot testing](02_snapshots/00_intro.md)
  - [`insta`](02_snapshots/01_snapshots.md)
  - [Storing snapshots](02_snapshots/02_storage_location.md)
  - [Redactions](02_snapshots/03_redactions.md)
  - [Outro](02_snapshots/04_outro.md)

- [Mocks](03_mocks/00_intro.md)
  - [Refactor to an interface](03_mocks/01_traits.md)
  - [`mockall`](03_mocks/02_mockall.md)
  - [Call sequences](03_mocks/03_sequences.md)
  - [Checkpoints](03_mocks/04_checkpoints.md)
  - [Mocking foreign traits](03_mocks/05_foreign_traits.md)
  - [Outro](03_mocks/06_outro.md)

- [The testing system](04_interlude/00_testing_infrastructure.md)

- [Filesystem isolation](05_filesystem_isolation/00_intro.md)
  - [Implicit or explicit?](05_filesystem_isolation/01_named_tempfile.md)
  - [Temporary files](05_filesystem_isolation/02_tempfile.md)
  - [Path coupling](05_filesystem_isolation/03_tempdir.md)
  - [Outro](05_filesystem_isolation/04_outro.md)

- [Database isolation](06_database_isolation/00_intro.md)
  - [`sqlx::test`](06_database_isolation/01_sqlx_test.md)

- [HTTP mocking](07_http_mocking/00_intro.md)
  - [`wiremock`](07_http_mocking/01_basics.md)
  - [Matchers](07_http_mocking/02_match.md)
  - [Checkpoints](07_http_mocking/03_checkpoints.md)
  - [Outro](07_http_mocking/04_outro.md)

- [Macros](08_macros/00_intro.md)
  - [Your first macro](08_macros/01_no_op_macro.md)
  - [Parsing tokens](08_macros/02_test.md)
  - [Parsing arguments](08_macros/03_hooks.md)
  - [Outro](08_macros/04_outro.md)

- [Test harnesses](09_test_harness/00_intro.md)
  - [Custom harnesses](09_test_harness/01_harness.md)
  - [`libtest_mimic`](09_test_harness/02_cli.md)
  - [Outro](09_test_harness/03_outro.md)

- [Capstone](10_capstone/00_capstone.md)
