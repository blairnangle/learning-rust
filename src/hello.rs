pub fn hello_world() -> String {
    println!("Hello, world!");

    String::from("Hello, world!")
}

#[cfg(test)]
mod tests {
    use crate::hello::hello_world;

    #[test]
    fn test_hello_world() {
        let actual = hello_world();
        let expected = "Hello, world!";

        assert_eq!(actual, expected);
    }
}
