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

fn greeting(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn i_fail() {
        panic!("I failed!");
    }

    #[test]
    fn can_fit() {
        let big = Rectangle {
            width: 100,
            height: 100,
        };
        let small = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(big.can_hold(&small));
    }

    #[test]
    fn can_not_fit() {
        let big = Rectangle {
            width: 100,
            height: 100,
        };
        let small = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(!small.can_hold(&big));
    }

    #[test]
    fn not_equal() {
        assert_ne!(5, 10);
    }

    // we can provide custom failure messages for tests
    #[test]
    fn greeting_should_have_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "This custom failure message. result: {}",
            result
        );
    }

    // we can also annotate that  test is expected to panic
    // use expected = to only pass on a specific panic message substring
    #[test]
    #[should_panic(expected = "im panicking!")]
    fn should_panic() {
        panic!("im panicking!");
    }

    // can also write tests using Result type
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
