//! Write a custom matcher that matches if:
//!
//! - The method is `POST`
//! - The `Content-Type` header is present and set to `application/json`
//! - The request body is a valid JSON object
//! - The `Content-Length` header is set and its value matches the length of the request body (in bytes)
use wiremock::{Match, Request};

struct WellFormedJson;

impl Match for WellFormedJson {
    fn matches(&self, request: &Request) -> bool {
        todo!("Implement me!")
    }
}

#[cfg(test)]
mod tests {
    use crate::WellFormedJson;
    use googletest::assert_that;
    use googletest::matchers::eq;
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn test_server() -> MockServer {
        let server = MockServer::start().await;
        server
            .register(Mock::given(WellFormedJson).respond_with(ResponseTemplate::new(200)))
            .await;
        server
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_invalid_json() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        // Trailing comma is not valid in JSON
        let body = r#"{"hi": 2,"#;
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .header("Content-Type", "application/json")
            .body(r#"{"hi": 2,"#)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_missing_content_type() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&json!({"hi": 2})).unwrap();
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_invalid_content_length() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&json!({"hi": 2})).unwrap();
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_non_post() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = json!({"hi": 2});

        let outcome = client
            .patch(&server.uri())
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn happy_path() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = json!({"hi": 2});

        let outcome = client.post(&server.uri()).json(&body).send().await.unwrap();
        assert_that!(outcome.status().as_u16(), eq(200));
    }
}
