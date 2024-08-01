# Mocking

We love to think about software as a collection of small, well-defined units that are then composed together
to create more complex behaviours.\
Real codebases are rarely that simple, though: they often contain complex interactions with external services,
tangled dependencies, and a lot of incidental complexity.

Those dependencies make testing harder.

## Example: a `login` endpoint

Let's look at an example:

```rust
async fn login(
    request: &HttpRequest,
    database_pool: &DatabasePool,
    auth0_client: &Auth0Client,
    rate_limiter: &RateLimiter,
) -> Result<LoginResponse, LoginError> {
    // [...]
}
```

The `login` function has four dependencies: the incoming HTTP request, a database connection pool, an Auth0 client, and
a rate limiter.\
To invoke `login` in your tests, you need to provide all of them.

Let's make the reasonable assumption that `login` is asking for those dependencies because it needs them to do its job.\
Therefore you can expect queries and HTTP requests to be made when you invoke it in your tests. Something
needs to handle those queries and requests, otherwise you won't be able to exercise the scenarios you care about.

## A spectrum

When it comes to testing, all approaches exist on a spectrum.

On one end, you have **full-fidelity testing**: you run your code with a setup that's as close as possible to the
production environment.
A real database, a real HTTP client, a real rate limiter.

On the other end, you have **test doubles**: you replace your dependencies with alternative
implementations that are easier to create and control in your tests.

Full-fidelity testing gives you the highest confidence in your code, but it can be expensive to set up and maintain.\
Test doubles are cheaper to create, but they can be a poor representation of the real world.

## This course

During this course, we'll cover both approaches.\
We'll see how to implement full-fidelity testing for filesystem, database, and HTTP interactions.\
We'll also explore how to use test doubles when full-fidelity testing is not feasible or convenient.

Let's start from test doubles!
