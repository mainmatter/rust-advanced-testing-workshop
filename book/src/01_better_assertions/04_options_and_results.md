# `Option` and `Result` matchers

`googletest` comes with a few special matchers for `Option` and `Result` that return good error messages
when something that should be `Some` or `Ok` is actually `None` or `Err`, and vice-versa.
