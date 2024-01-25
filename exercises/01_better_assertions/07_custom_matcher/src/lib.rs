//! Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.
use googletest::description::Description;
use googletest::matcher::{Matcher, MatcherResult};
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<ActualT = StatusCode> {
    RedirectMatcher
}

struct RedirectMatcher;

impl Matcher for RedirectMatcher {
    type ActualT = StatusCode;

    fn matches(&self, actual: &StatusCode) -> MatcherResult {
        if actual.is_redirection() {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => "is a redirection status code".into(),
            MatcherResult::NoMatch => "isn't a redirection status code".into(),
        }
    }
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
