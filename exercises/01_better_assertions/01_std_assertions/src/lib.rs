#[cfg(test)]
mod tests {
    #[test]
    fn empty() {
        let v: Vec<i32> = vec![];
        assert_empty(v)
    }

    #[test]
    fn one_value() {
        let v: Vec<i32> = vec![-1];
        assert_empty(v)
    }

    #[test]
    fn two_values() {
        let v: Vec<i32> = vec![-1, 1];
        assert_empty(v)
    }

    fn assert_empty<T>(v: Vec<T>) {
        // You should compose the message _directly_ in the assertion.
        // Don't do a `let msg = format!(...)` before the assertion!
        assert!(v.is_empty(), "TODO")
    }
}
