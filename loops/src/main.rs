fn main() {
    let a = [10, 34, 23, 3, 234];
    for item in a.iter().rev() {
        if item == &334 {
            println!("found it!");
            break;
        }
        if item == &a[a.len() - 1] {
            println!("couldn't find it!");
        }
    }
}
