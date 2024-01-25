fn panic_one() {
    panic!("Panic #1");
}

fn panic_two() {
    panic!("Panic #2");
}

fn hello() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macros03::test(before=panic_one)]
    fn panic_before() {}

    #[macros03::test(after=panic_two)]
    fn panic_after() {}

    #[macros03::test(before=hello, after=hello)]
    fn happy() {}
}
