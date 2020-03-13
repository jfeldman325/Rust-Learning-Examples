mod rectangles;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    let mut n1: u64 = 0;
    let mut n2: u64 = 1;
    match n {
        0 => n2,
        _ => {
            for _ in 0..n {
                n2 = n1 + n2;
                n1 = n2 - n1;
            }
            n2
        }
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Value out of range");
        } else {
            Guess { value }
        }
    }
}

fn read_from_file() -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;
    let mut s = String::new();
    File::open("hello_world.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another_test() {
        panic!("This test will fail")
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = rectangles::Rectangle::new(10, 5);
        let smaller = rectangles::Rectangle::new(3, 4);
        assert!(larger.can_hold(&smaller))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = rectangles::Rectangle::new(10, 5);
        let smaller = rectangles::Rectangle::new(3, 4);
        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(5, add_two(3))
    }

    #[test]
    #[should_panic(expected = "Value out of range")]
    fn invalid_guess_panics() {
        Guess::new(-5);
    }

    #[test]
    fn testing_with_results() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Trump: two plus two does not equal four!"))
        }
    }

    #[test]
    fn read_file_to_string() -> Result<(), std::io::Error> {
        match read_from_file() {
            Ok(x) => Ok(()),
            Err(x) => Err(x),
        }
    }

    #[test]
    #[ignore]
    fn stupid_test() -> Result<(), String> {
        let mut sum: i64 = 0;
        for i in 0..50 {
            sum = sum + (sum + 1);
        }

        if sum > 1000000 {
            Ok(())
        } else {
            Err("Not big enough!".to_string())
        }
    }

    #[test]
    fn fib_correct() {
        assert_eq!(fibonacci(3), 1);
    }
}
