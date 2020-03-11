use std::io;

fn main() {
    loop {
        println!("What is the number you would like to check?");
        let mut num_to_check = String::new();
        io::stdin()
            .read_line(&mut num_to_check)
            .expect("Could not read input stream");

        let num_to_check: f64 = match num_to_check.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number try again");
                continue;
            }
        };
        match perfect_root(num_to_check) {
            true => println!("That is perfect!"),
            false => println!("Nope!"),
        }
    }
}

fn perfect_root(x: f64) -> bool {
    if x.sqrt() != (x.sqrt().floor()) {
        return false;
    } else {
        return true;
    }
}
