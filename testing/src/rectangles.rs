#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn ratio(&self) -> String {
        let width_f = self.width as f32;
        let height_f = self.height as f32;
        return format!("The ratio is {}:1", width_f / height_f);
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        //does it fit in width_wise
        if self.width > other.width && self.height > other.height {
            return true;
        } else if self.width > other.height && self.height > other.width
        //how about height_wise?
        {
            return true;
        } else {
            return false;
        }
    }
    pub fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
    fn square(side: i32) -> Rectangle {
        Rectangle::new(side, side)
    }
}

pub fn main() {
    let rect = Rectangle::new(50, 32);
    println!("{}", rect.can_hold(&Rectangle::new(20, 60)));
    println!("{:#?}", rect);
    println!(
        "There are of the rectangle is {} square pixles.",
        rect.area()
    );
    println!("{}", rect.ratio());
}
