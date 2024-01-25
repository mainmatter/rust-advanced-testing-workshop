# Database isolation

Let's move on to another external dependency: the database.  
We'll use PostgreSQL as our reference database, but the same principles apply to other databases.

## The challenge

The database is often the most complex external dependency in an application, especially if it is distributed.  
The database is in charge of storing your data, ensuring its integrity in the face of various kinds of 
failures and complex concurrent access patterns.

If you want to write a black-box test for your application, you'll have to deal with the database.  
The challenge is, in many ways, similar to what we discussed in the previous section about the filesystem:
if we just point our tests to a shared database, we'll end up with spurious failures and a slow test suite, 
since the tests will interfere with each other and we'll be forced to execute them sequentially.

## The dream

Our goal is to run our tests in parallel, with minimal overhead, and without having to worry about cross-test
interference.  
Each test should be able to assume that it is the only test running, and that it can safely modify the database
as it sees fit.

## Approaches

### 1. Use an in-memory database

Instead of using an actual database instance, we replace it with an in-memory database.  
Each test creates a separate in-memory database, and we don't have to worry about interference between tests.

It isn't all roses, though:

- You'll have to structure your code so that you can easily swap the database implementation.  
  This will inevitably increase the complexity of your application without adding much value to the production code. 
- You're not really testing the system as a whole, since you're not using the same database of your production environment.  
  This is especially problematic if you're using database-specific features. An in-memory database will not behave
  exactly like your production database, especially when it comes to concurrency and locking. 
  Subtle (but potentially serious) bugs will slip through your test suite.

In-memory databases used to be a popular approach, but they have fallen out of favor in recent years since
it has become significantly easier to run instances of real databases on laptops and in CI environments. Thanks Docker!
  
### 2. Use uncommitted transactions

Many databases (relational and otherwise) support transactions: a way to group multiple operations into a single
unit of work that either succeeds or fails as a whole.  
In particular, you can use transactions to create a "private" view of the database for each test:
what happens in a transaction is not visible to other transactions until it is committed[^isolation-levels], but it is visible to the
client that created it.  
You can leverage this fact to run your tests in parallel, as long as you make sure that each test runs in a separate
transaction that's rolled back at the end of the test.

There are some complexities to this approach: 

- When the code under test needs to perform multiple transactions, you end up with _nested transactions_.  
  In a SQL database, that requires (implicitly) converting your `COMMIT` statements into `SAVEPOINT` statements. 
  Other databases may not support nested transactions at all.
- Rust is a statically typed language. Writing code that can accept both an open transaction and a "simple" connection
  as the object that represents the database can be... complicated. 

### 3. Use a separate database for each test

Since our goal is to isolate each test, the most straightforward approach is to use a separate database for each test!  
Today's laptops, combined with Docker, make this approach feasible even for large test suites.

Our recommendation is to use a different _logical_ database for each test, rather than a _physical_ database (e.g.
a separate Docker container for each test). It lowers the overhead, resulting in faster tests.  

## Our recommendation

We recommend approach #3: a separate database for each test.  
It has the lowest impact on your production code and it gives you the highest level of confidence in your tests.  
We'll see how to implement it with `sqlx` in the next section.

[^isolation-levels]: The exact semantics of transactions depend on the isolation level of the database.  
  What we describe here is the behavior of the [`READ COMMITTED` isolation level](https://www.postgresql.org/docs/current/transaction-iso.html), 
  which is the default in PostgreSQL. You need to use an isolation level that doesn't allow **dirty reads**.