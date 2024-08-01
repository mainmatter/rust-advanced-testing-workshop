//! Refactor the `square` function to ask for a type that implements the `Logger` trait rather than the concrete
//! `PrintlnLogger` type.\
//! Then pass a `TestLogger` to `square` in the test. `TestLogger` should implement `Logger` and do nothing
//! when `log` is called.

pub fn square(x: i32, logger: PrintlnLogger) -> i32 {
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub struct PrintlnLogger;

impl PrintlnLogger {
    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::square;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        assert_eq!(square(2, todo!()), 4);
    }
}
