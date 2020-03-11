mod rectangles;

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

fn main() {
    rectangles::main();
    let user1 = new_user(
        "jacobffeldman".to_string(),
        "vitruvius".to_string(),
        true,
        1,
    );
}

fn new_user(email: String, username: String, active: bool, sign_in_count: u32) -> User {
    User {
        email,
        username,
        active,
        sign_in_count,
    }
}
