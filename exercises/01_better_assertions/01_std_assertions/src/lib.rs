//! Before we move on to more advanced assertion libraries, we want to make sure that you know
//! how to get the most out of the built-in testing toolkit.
//! In this exercise, you'll need to write a custom _dynamic_ panic message: the message must be
//! different depending on the values of the variables you're comparing.
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
        assert!(
            v.is_empty(),
            "The vector is not empty, it has {} elements",
            v.len()
        )
    }
}
