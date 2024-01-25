# `googletest` - Basic matchers

To truly leverage a testing library like `googletest` you need to get familiar with their built-in matchers. 
They're the building blocks of your assertions and they need to roll off your fingers as easily as `assert_eq!` does.

We'll spend this exercise and a few more to get familiar with the most common matchers, starting with the most basic ones.

> Tooling helps: coding assistants like GitHub Copilot or Cody will start suggesting the right matchers as you type
if you've already used them in a few tests in the same project.

## Exercise

Replace the `assert_eq!`/`assert_ne!` calls with the corresponding `googletest` matchers.

Take a moment to read the error messages and compare them with the ones you got from `assert_eq!`/`assert_ne!`. 
Notice how `googletest` sees through variable names and prints the actual values that were compared!
