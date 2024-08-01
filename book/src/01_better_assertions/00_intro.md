# Exercise expectations

By this point you should have all the tools installed and ready to go.
Let's discuss how automated verification works in this course.

This is a testing workshop, therefore we need to check that the _tests_ you write behave as expected. It's a bit meta!\
It's not enough to know that a test failed, we also need to know _why_ it failed and what message it produced.
We do this by using `ctr`, the custom tool you just installed. It runs the tests in each exercise and compares
the outcome with a set of expectations.

You can find those expectations in
the [`expectations.yml`](https://github.com/mainmatter/rust-advanced-testing-workshop/blob/main/exercises/01_better_assertions/00_intro/expectations.yml)
file.\
You should **never** modify this file. Refer to it in order to understand what the tests are supposed to do,
but don't change it.
