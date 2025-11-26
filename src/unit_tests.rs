pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn guess_number(value: i32) -> bool {
    if value <= 0 {
        panic!("Value must be greater 0, got {value}!");
    }

    value == 5
}

pub fn guess_text(value: &str) -> Result<bool,String> {
    if value.is_empty() {
        Err(String::from("Value is empty"))
    }
    else {
        Ok(value.eq("Guess!"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Michael");
        assert!(result.contains("Michael"));
    }

    #[test]
    fn greeting_contains_hello() {
        let result = greeting("Hello");
        assert!(
            result.contains("Hello"),
            "Does not contain 'Hello', value was '{result}'"
        );
    }

    #[test]
    #[should_panic(expected = "must be greater 0")]
    fn smaller_than_zero() {
        let result = guess_number(-1);
        assert!(result);
    }

    #[test]
    fn guess_right() {
        let result = guess_number(5);
        assert!(result);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    #[test]
    fn guess_wrong() {
        let result = guess_text("Wrong");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        println!("Test takes a long duration")
    }
}
