//! Add a `vanilla_test` attribute macro to the `macros01` crate.\
//! It should do nothing: it should just re-emit the code that's been annotated with the macro, unchanged.
#[cfg(test)]
mod tests {
    #[macros01::vanilla_test]
    #[test]
    fn unchanged() {
        assert!(true);
    }
}
