# Welcome

This is the first exercise of the course.\
It is meant to be a warm-up exercise, to get you familiar with the tools and the workflow.

## Exercise structure

All exercises are structured in the same way.

### Intro

You have a `README.md` file (the one you're reading!), which contains the exercise description and an explanation
of the concepts you are about to learn.

### Expectations

This is a testing workshop, therefore we need to check that the _tests_ you write behave as expected. It's a bit meta!\
It's not enough to know that a test failed, we also need to know _why_ it failed and what message it produced.
We do this by using a custom tool ([`ctr`](../../../ctr)), which runs the tests in each exercise and compares
the outcome with a set of expectations.

You can find those expectations in the [`expectations.yml`](expectations.yml) file.\
You should **never** modify this file. Refer to it in order to understand what the tests are supposed to do,
but don't change it.

### Source

In the `src` folder you'll find the source code of the exercise. Since this is a testing workshop, they'll be mostly
tests.\
You're supposed to modify the tests in order to get them to match what's described in the expectations file.

## Get started

Jump into [`src/lib.rs`](src/lib.rs) and try to solve the first exercise.\
Use `wr` to check your progress!
