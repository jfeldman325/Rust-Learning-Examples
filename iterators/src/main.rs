fn main() {
    let mut v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    for item in v1.iter_mut() {
        //must use iter_mut() to iterate through mutable refrences
        *item = 5;
    }

    for item in v1.iter() {
        println!("{}", *item)
    }
    let sum: i32 = v1.iter().sum();
    println!("{}", sum);

    let v2: Vec<_> = v1.iter().map(|item| item + 1).collect();
    println!("{:?}", v2);

    let counter = Counter::new();
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    let counter_iter = counter.iter_mut();
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count == 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
