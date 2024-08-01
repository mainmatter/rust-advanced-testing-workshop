# Advanced testing for Rust applications

No application is an island: you need to interact with third-party APIs, databases and who knows what else.
Testing those interactions is tricky, to say the least! This workshop will focus on expanding your Rust testing toolkit,
going beyond the basic techniques you're already familiar with.
At the end of this workshop, you'll have a strategy to test most of the scenarios that are relevant for a complex Rust
application.

The course assumes you have a good understanding of Rust's basic concepts and want to
move beyond the built-in testing toolkit.
If you run into any issue with the assumed level of Rust knowledge, please ping us and we'll sort it together!

> [!NOTE]
> This workshop has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [our portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or
> training!

## Getting started

Open [the companion book for this course](https://rust-exercises.com/advanced-testing/) in your browser.
Follow the instructions there to get started.

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).\
  If Rust is already installed on your system, make sure you are running on the latest compiler
  version (`cargo --version`).\
  If not, update using `rustup update` (or another appropriate command depending on how you installed Rust on your
  system).
  You **must** have `rustup` installed (check with `rustup --version`).
- _(Optional)_ An IDE with Rust autocompletion support.
  We recommend one of the following:
    - [RustRover](https://www.jetbrains.com/rust/);
    - [Visual Studio Code](https://code.visualstudio.com) with
      the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Solutions

You can find the solutions to the exercises in
the [`solutions` branch](https://github.com/mainmatter/rust-advanced-testing-workshop/tree/solutions) of this
repository.

## References

Throughout the workshop, the following resources might turn out to be useful:

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust documentation](https://doc.rust-lang.org/std/) (you can also open the documentation offline with `rustup doc`!)
- [`googletest` documentation](https://docs.rs/googletest/)
- [`insta` documentation](https://insta.rs/docs/)
- [`tempfile` documentation](https://docs.rs/tempfile/)
- [`sqlx` documentation](https://docs.rs/sqlx/)
- [`wiremock` documentation](https://docs.rs/wiremock/)
- [`syn` documentation](https://docs.rs/syn/)
- [`quote` documentation](https://docs.rs/quote/)
- [`proc-macro2` documentation](https://docs.rs/proc-macro2/)
- [`libtest_mimic` documentation](https://docs.rs/libtest-mimic)

# License

Copyright © 2023- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).
