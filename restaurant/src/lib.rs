//This doc shows a lot of the different path and scoping mechanisms in addition to modules, packages and libraries

use std::io::{self, Write};
pub mod front_of_house; //this is in a seperate file

mod restaurant {
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            crate::front_of_house::serving::serve_order();
        }
        fn cook_order() {
            fix_incorrect_order();
        }
    }
}
use front_of_house::hosting;
pub fn eat_at_restaurant() {
    //Absolute
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative
    crate::front_of_house::hosting::add_to_waitlist();
    //In scope with use
    hosting::add_to_waitlist();
}
