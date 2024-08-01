//! Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.
use googletest::matcher::Matcher;
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<ActualT = StatusCode> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
