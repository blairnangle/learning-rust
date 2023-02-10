pub fn fizzbuzz<>() -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let fizzbuzz_str = String::from("FizzBuzz");
    let fizz_str = String::from("Fizz");
    let buzz_str = String::from("Buzz");
    for number in 1..21 {
        if number % 3 == 0 && number % 5 == 0 {
            results.push(fizzbuzz_str.clone());
        } else if number % 3 == 0 {
            results.push(fizz_str.clone());
        } else if number % 5 == 0 {
            results.push(buzz_str.clone());
        } else {
            results.push(number.to_string());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizzbuzz;

    #[test]
    fn test_fizzbuzz() {
        let actual = fizzbuzz();
        let expected = vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz"];

        assert_eq!(actual, expected);
    }
}
