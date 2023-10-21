pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(to_greet: &str) -> String {
    format!("Hello {}", to_greet)
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess was not in range, got {}", value)
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // UNCOMMENT TO SEE TEST FAIL
    // #[test]
    // fn another() {
    //     panic!("This test is always failing");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            width: 3,
            height: 3,
        };
        let r2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(r2.can_hold(&r1));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let r1 = Rectangle {
            width: 3,
            height: 3,
        };
        let r2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!r1.can_hold(&r2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(250);
    }

    #[test]
    #[should_panic(expected = "not in range")]
    fn greater_than_100_better_panic() {
        Guess::new(200);
    }

    #[test]
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
