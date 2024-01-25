# Testing with `sqlx`

Let's try to implement the "one database per test" approach with `sqlx`.  

## Spinning up a "physical" database

We need to have a "physical" database instance to create a dedicated logical database for each test.  
We recommend using an ephemeral Docker container for this purpose. Containers are portable, easy to spin up and tear down.

> If you don't have Docker installed, go get it! You can find instructions [here](https://docs.docker.com/get-docker/).

In [`db_launcher`](../../../db_launcher) you'll find a small Rust script that does just that: it launches a new 
named container with a PostgreSQL database, unless there is one already running.  

In our ideal setup, you'd just execute `cargo test` and the required setup (i.e. spinning up the container) would be 
executed automatically. We are not quite there yet, though, so for now you'll have to run it manually:

```bash
# Make sure Docker is running first!
cargo r --bin db_launcher
```

## Configuring `sqlx`

For this section, we'll be using [`sqlx`](https://crates.io/crates/sqlx) to interact with PostgreSQL.  
One of the key features provided by `sqlx` is compile-time query validation: when you compile your project,
`sqlx` will check that all your queries are valid SQL and that they are compatible with your database schema.  
This is done via their custom macros: at compile-time, they issue a statement against a live database to carry
out the validation.  

For that reason, we need to provide `sqlx` with a connection string to said database.  
The common approach is to define a `.env` file in the root of the project: `sqlx` will automatically read it and
use the value of the `DATABASE_URL` variable as the connection string. We'll stick to [this approach](../../../.env).

> `sqlx` exposes a few different macro variants, but we'll mostly be using [`sqlx::query!`](https://docs.rs/sqlx/0.7.3/sqlx/macro.query.html).

## `#[sqlx::test]`

`sqlx` itself embraces the "one database per test" approach and provides a custom test attribute, `#[sqlx::test]`, to do
the heavy lifting for you.  
You add an input parameter to your test function (e.g. `pool: sqlx::PgPool`) and `sqlx` will automatically create a new
database and pass a connection pool to your test.

> You can find the list of injectable parameters in the [`sqlx::test` documentation](https://docs.rs/sqlx/0.7.3/sqlx/attr.test.html).

Under the hood, this is [what `sqlx` does](https://github.com/launchbadge/sqlx/blob/982c014f54843cb5661e667f66937d59bd09b512/sqlx-core/src/testing/mod.rs#L171):

- It connects to the database specified in the `DATABASE_URL` environment variable.
- It creates a new database with a random name.
- (Optional) It runs all the migrations in the `migrations` directory.
- It creates a connection pool to the new database.
- It passes the connection pool to your test function.
- It waits for the test to complete.
- It deletes the database.

## Exercise

The `insert` test should succeed the first time you run it, but fail the second time since the `users` table already exists
and it already contains a row with the same `id`, thus violating the `PRIMARY KEY` constraint.  

Rewrite the test using the `#[sqlx::test]` attribute.