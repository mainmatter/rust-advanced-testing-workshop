# `wiremock`

The [`wiremock` crate](https://crates.io/crates/wiremock) is a loose port of the well-known
[WireMock](http://wiremock.org/) library from Java.

## How does it work?

The core idea in `wiremock` is simple: you start a server that listens for HTTP requests and returns pre-determined
responses. The rest is just sugar to make it easy to define matching rules and expected responses.

## `MockServer`

`MockServer` is the interface to the test server.\
When you call `MockServer::start()`, a new server is launched on a random port.
You can retrieve the base URL of the server with `MockServer::uri()`.

```rust
#[tokio::test]
async fn test() {
    let mock_server = MockServer::start().await;
    let base_url = mock_server.uri();
    // ...
}
```

`wiremock` uses a random port in `MockServer` in order to allow you to run tests in parallel.\
If you specify the same port across multiple tests, you're then forced to run them sequentially, which can be a
significant performance hit.

## Writing testable code, HTTP client edition

Let's assume that we have a function that sends a request to GitHub's API to retrieve the tag of the latest release
for a given repository:

```rust
use reqwest::Client;

async fn get_latest_release(client: &Client, repo: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let release = response.json::<serde_json::Value>().await?;
    let tag = release["tag_name"].as_str().unwrap();
    Ok(tag.into())
}
```

As it stands, this function cannot be tested using `wiremock`.

### 1. Take base URLs as arguments

We want the code under the test to send requests to the `MockServer` we created in the test.\
We can't make that happen if the base URL of the external service is hard-coded in the function.

Base URLs must be passed as arguments to the code under test:

```rust
use reqwest::Client;

async fn get_latest_release(client: &Client, github_base_uri: http::Uri, repo: &str) -> Result<String, reqwest::Error> {
    let endpoint = format!("{github_base_uri}/repos/{repo}/releases/latest");
    let response = client.get(&endpoint).send().await?;
    let release = response.json::<serde_json::Value>().await?;
    let tag = release["tag_name"].as_str().unwrap();
    Ok(tag.into())
}
```

### 2. If you need to hard-code a base URL, do it close to the binary entrypoint

If we need to hard-code a base URL, it is better to do it in the `main` function, or as close to the binary entrypoint
as possible.
This limits the scope of difficult-to-test code. In particular, the binary becomes a very thin (and boring) layer
around a library that can be tested in isolation.

Even better: take the base URL as part of your application configuration.

## `Mock`

You have a `MockServer` and the code under test has been refactored to make the base URL configurable. What now?
You need to configure `MockServer` to respond to incoming requests using one or more `Mock`s.

A `Mock` lets you define:

- **Preconditions** (e.g. assertions on the requests received by the server)
- **Expectations** (e.g. how many times you expect the method to be called)
- **Response values** (e.g. what response should be returned to the caller)

> Yes, this is very similar to `mockall`!

In an example test:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::method;

#[tokio::test]
async fn test() {
    let mock_server = MockServer::start().await;

    // Precondition: do what follows only if the request method is "GET"
    Mock::given(method("GET"))
        // Response value: return a 200 OK
        .respond_with(ResponseTemplate::new(200))
        // Expectation: panic if this mock doesn't match at least once
        .expect(1..)
        .mount(&mock_server)
        .await;

    // [...]
}
```

A `Mock` doesn't take effect until it's registered with a `MockServer`.
You do that by calling `Mock::mount` and passing the `MockServer` as an argument, as in the example
above.

## Expectations

Setting expectations on a `Mock` is optional: use them when you want to test how your code interacts with the
dependency that's being mocked, but don't overdo it.\
Expectations, by default, are verified when the `MockServer` is dropped. We'll look at other verification
strategies in a later section.
