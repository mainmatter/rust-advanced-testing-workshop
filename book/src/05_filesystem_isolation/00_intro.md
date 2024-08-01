# Test isolation

The code under test doesn't run in a vacuum.\
Your tests change the state of the host system, and that state, in return, affects the outcome of your tests.

Let's use the filesystem as a concrete example. It behaves as a global variable that all your tests share.\
If one test creates a file, the other tests will see it.\
If two tests try to create a file with the same name, one of them will fail.\
If a test creates a file, but doesn't clean it up, the next time the same test runs it might fail.

Those cross-test interactions can make your test suite flaky: tests might pass or fail depending on the
order in which they were run. That's a recipe for frustration and wasted time.

We want the best of both worlds: we want to be able to test the effects of our code on the outside world,
but we also want our tests to be isolated from each other.\
Each test should behave as if it is **the only test running**.

## The plan

This section (and the next two) will be dedicated to various techniques to achieve test isolation
when using high-fidelity testing.\
In particular, we'll look at what happens when your application interacts with the filesystem, databases and
other HTTP APIs.
