#[cfg(test)]
mod tests {
    use googletest::matchers::{each, empty, eq, ge, gt, lt, not};
    use googletest::{all, assert_that, expect_that, verify_current_test_outcome};

    #[googletest::test]
    fn multi_matcher() {
        let v: Vec<i32> = vec![1, 2, 3];
        // Convert the assertion below into two invocations of `expect_that!`.
        assert_that!(v, all!(each(lt(3)), each(gt(1))))
    }

    #[googletest::test]
    fn multi_property() {
        struct Person {
            name: String,
            surname: String,
            age: u8,
        }

        let person = Person {
            name: "John".to_string(),
            surname: "".to_string(),
            age: 16,
        };

        // Check that name and surname are not empty and that age is greater or equal than 18.
    }

    #[googletest::test]
    // Notice that we are returning a `Result` from the test function.
    fn barrier() -> googletest::Result<()> {
        let v: Vec<i32> = vec![1, 2, 3];
        expect_that!(v, each(lt(3)));
        expect_that!(v, each(gt(1)));

        // TODO: if any of the assertions above fails, we want to abort the test
        //   and return an error.
        //   Do it without changing the assertions above!

        expect_that!(v, empty());
        Ok(())
    }
}
