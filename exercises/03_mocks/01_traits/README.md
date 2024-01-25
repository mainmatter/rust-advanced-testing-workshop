# Refactor to an interface

Let's look again at the `login` function from the README of the previous exercise:

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

You don't want to spin up a real database, a real Auth0 client, and a real rate limiter in your tests; you want
to use test doubles instead.  
How do you proceed?

## The problem

Rust is a statically typed language.  
The `login` function expects four arguments, and each of them has a specific type. There's no way to pass
a different type to the function without running into a compiler error.

In order to use test doubles, you need to **decouple** `login` from specific implementations of its dependencies.  
Instead of asking for an `Auth0Client`, you need to ask for something that can _act like_ an `Auth0Client`.  
You need to **refactor to an interface**.

## Traits

In Rust, you use **traits** to define interfaces.  
A trait is a collection of methods that can be implemented by concrete types.

Continuing with the `Auth0Client` example, you can define a trait that describes the methods you need to
interact with Auth0:

```rust
trait Authenticator {
  async fn verify(&self, jwt: &str) -> Result<UserId, VerificationError>;
}
```

You would then implement this trait for `Auth0Client`:

```rust
impl Authenticator for Auth0Client {
  async fn verify(&self, jwt: &str) -> Result<UserId, VerificationError> {
    // [...]
  }
}
```

Finally, you would change the signature[^dispatch] of `login` to ask for an `Authenticator` instead of an `Auth0Client`:

```rust
async fn login<A>(
  request: &HttpRequest,
  database_pool: &DatabasePool,
  authenticator: &A,
  rate_limiter: &RateLimiter,
) -> Result<LoginResponse, LoginError> 
where
    A: Authenticator,
{
  // [...]
}
```

You have successfully refactored to an interface!  

## Tread carefully

Refactoring to an interface is **the** technique you need to master if you want to use test doubles.  
All the other exercises in this workshop provide conveniences to reduce the amount of boilerplate you need to write, 
but they don't fundamentally move the needle on the complexity of the problem.

This kind of refactoring might not always be easy (nor possible!).  
You need to analyze your codebase to determine if it's viable and **if it's worth the effort**.
You're introducing a layer of indirection that might not be necessary beyond your tests. That's **incidental complexity**.  
In some cases, it might be better to use a full-fidelity testing approach, like the ones you'll see later in 
this workshop.

## Exercise

Refactor the `square` function to ask for a type that implements the `Logger` trait rather than the concrete
`PrintlnLogger` type.  
Then pass a `TestLogger` to `square` in the test. `TestLogger` should implement `Logger` and do nothing
when `log` is called.

[^dispatch]: In this example we've used **static dispatch** to make `login` polymorphic with respect to the `Authenticator` type.  
    You can also use **dynamic dispatch** by changing the signature of `login` to ask for a `&dyn Authenticator` (a trait object) instead of an `&A`.  