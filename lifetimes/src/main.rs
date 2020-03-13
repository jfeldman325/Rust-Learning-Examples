use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }

    println!("The longest string is {}", result)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn longest_with_an_announcement<T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display + Copy,
    {
        println!("Announcement: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
