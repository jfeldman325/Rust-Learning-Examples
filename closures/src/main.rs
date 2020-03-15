use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 10;
    let random_num = 3;

    generate_workout(intensity, random_num);

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; //here the move closure moves ownership of x to the closure and leaves scope afterwards.

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

struct Cacher<T, E>
where
    E: Copy,
    T: Fn(E) -> E,
{
    calculation: T,
    result: HashMap<E, E>,
}

impl<T, E> Cacher<T, E>
where
    E: Copy + std::cmp::Eq + std::hash::Hash,
    T: Fn(E) -> E,
{
    fn new(calculation: T) -> Cacher<T, E> {
        Cacher {
            calculation,
            result: HashMap::new(),
        }
    }

    fn value(&mut self, arg: E) -> E {
        *self.result.entry(arg).or_insert((self.calculation)(arg))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut expensive_operation = Cacher::new(expensive_closure);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_operation.value(intensity)
        );
        println!("Next, do {} situps!", expensive_operation.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_operation.value(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
#[cfg(test)]
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
