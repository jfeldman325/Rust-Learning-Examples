fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['a', 'b', 'n', 'd', 'z'];

    println!("The largest number was {}", largest(&number_list));
    println!("The largest char was {}", largest(&char_list));

    let p1 = point { x: 3.1, y: 3.4 };
    let p2 = point { x: 3, y: 4 };

    match p1.symetric(0.4) {
        true => println!("The corrdinates of p1 are symetric"),
        false => println!("The coordinates of p1 are not symetric"),
    }

    match p2.symetric() {
        true => println!("The coordinates of p2 are symetric"),
        false => println!("The coordinates of p2 are not symetric"),
    }

    println!("The coordinates  ")
}

struct point<U, V> {
    x: U,
    y: V,
}

impl point<f32, f32> {
    fn symetric(&self, thresh: f32) -> bool {
        if self.y - thresh < self.x && self.x < self.y + thresh {
            true
        } else {
            false
        }
    }
}
impl point<i32, i32> {
    fn symetric(&self) -> bool {
        if self.x == self.y {
            return true;
        } else {
            return false;
        }
    }
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
}
