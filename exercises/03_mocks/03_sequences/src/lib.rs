//! Configure the mocks to get tests to pass.
use mockall::automock;
use std::error::Error;

/// Retry a request until it succeeds or the maximum number of retries is reached.
/// It always returns the number of retries that have been attempted.
pub fn with_retries<C>(
    request: Request,
    client: C,
    max_n_retries: usize,
) -> (Result<Response, Box<dyn Error>>, usize)
where
    C: Client,
{
    let mut n_retries = 0;
    loop {
        match client.call(&request) {
            Ok(r) => {
                return (Ok(r), n_retries);
            }
            Err(e) => {
                if n_retries == max_n_retries {
                    return (Err(e), n_retries);
                } else {
                    n_retries += 1;
                }
            }
        }
    }
}

#[automock]
pub trait Client {
    fn call(&self, request: &Request) -> Result<Response, Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub struct Request;
#[derive(Debug, Clone)]
pub struct Response;

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::expect_that;
    use googletest::matchers::{anything, eq, err, ok};

    static MAX_N_RETRIES: usize = 3;

    #[googletest::gtest]
    fn it_retries_if_first_call_fails() {
        let mut mock_client = MockClient::new();
        // TODO: setup mock_client to fail the first call and succeed the second

        let (outcome, n_retries) = with_retries(Request, mock_client, MAX_N_RETRIES);

        expect_that!(outcome, ok(anything()));
        expect_that!(n_retries, eq(1));
    }

    #[googletest::gtest]
    fn it_does_max_retries_if_all_calls_fail() {
        let mut mock_client = MockClient::new();
        // TODO: setup mock_client to fail all calls

        let (outcome, n_retries) = with_retries(Request, mock_client, MAX_N_RETRIES);

        expect_that!(outcome, err(anything()));
        expect_that!(n_retries, eq(MAX_N_RETRIES));
    }
}
