pub struct Guess {
    value: i32
}

impl Guess {
    /// Create a `Guess` struct. Note that this function still panics for
    /// backwards compatibility.
    /// For a `Result` variant use `new_without_panicking()`.
    pub fn new(value: i32) -> Guess {
        match Guess::new_without_panicking(value) {
            Ok(value)   => value,
            Err(msg)    => panic!(msg)
        }
    }

    pub fn new_without_panicking(value: i32) -> Result<Guess, String> {
        if value < 1 {
            return Err(String::from(format!(
                "Guess value must be greater than or equal to 1, got {}.", value)));
        }
        if value > 100 {
            return Err(String::from(format!(
                "Guess value must be less than or equal to 100, got {}.", value)));
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_100() {
        Guess::new(0);
    }
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn negative_guess() {
        Guess::new(-1);
    }

    #[test]
    fn non_panic_big_value() {
        assert!(Guess::new_without_panicking(121).is_err());
    }
    #[test]
    fn non_panic_small_value() {
        assert!(Guess::new_without_panicking(-1).is_err());
    }

    #[test]
    fn non_panic_ok_value() {
        let g = Guess::new_without_panicking(5);
        assert!(g.is_ok());
        assert_eq!(g.unwrap().value(), 5);
    }
}
