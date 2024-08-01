# Combining everything together

We've covered a lot of ground together: a new assertion framework, snapshot testing,
(auto)mocking, full-fidelity testing for various resources as well as tooling to build
custom test macros and harnesses.\
I've tried to break each topic down into small bites, empowering you to build up your
knowledge incrementally.

It's time to put everything together!

## The challenge

You have to design a custom test harness that's going to do the following:

- Start a (named) Docker container for PostgreSQL before running any tests.
- Before each test:
  - Create a separate logical database in the container
  - Run migrations on the database
- Run all tests in parallel, while injecting a `PgPool` instance for each test
- After each test:
  - Drop the logical database
- Stop the Docker container after all tests have completed

I don't have a suite of tests for you here, but please call me in when you're done—I want to
see what you come with!
