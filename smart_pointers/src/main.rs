// use crate::List::{Cons, Nil};

use core::ops::Deref;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

fn main() {
    // let b = Box::new(5);
    // println!("b={}", b); basic example

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(5);
    assert_eq!(5, *y); //will return true because we are derefrencing with *

    let z = MyBox::new(5);
    assert_eq!(5, *z); //Smart pointer syntax through custom type
    let s = MyBox::new("Jacob".to_string());
    hello(&s); //This works because MyBox implements the Deref trait and thus the compiler can check that this MyBox can be converted into a string refrence
    println!("end of main");

    use crate::List::{Cons, Nil};
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); Not allowed because Box owns the refrence and cannot share the pointer to a

    use crate::Rc_List::{Rc_Cons, Rc_Nil};
    let a_rc = Rc::new(Rc_Cons(
        RefCell::new(5),
        Rc::new(Rc_Cons(RefCell::new(10), Rc::new(Rc_Nil))),
    ));
    let b_rc = Rc_Cons(RefCell::new(3), Rc::clone(&a_rc));
    println!(
        "There are {} refrences to a_rc at line {}",
        Rc::strong_count(&a_rc),
        line!()
    );
    {
        let c_rc = Rc_Cons(RefCell::new(4), Rc::clone(&a_rc));
        println!(
            "There are {} refrences to a_rc at line {}",
            Rc::strong_count(&a_rc),
            line!()
        );
    }
    println!(
        "There are {} refrences to a_rc at line {}",
        Rc::strong_count(&a_rc),
        line!()
    );

    match &*a_rc {
        Rc_Cons(value, list) => *value.borrow_mut() += 10,
        Rc_Nil => (),
    };

    println!("{:?}", a_rc);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum Rc_List {
    Rc_Cons(RefCell<i32>, Rc<Rc_List>),
    Rc_Nil,
}

struct MyBox<T: Debug>(T);

impl<T: std::fmt::Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: std::fmt::Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: std::fmt::Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("He's dead Jim. Well at least the value {:?} is.", &self.0) //this will be aclled twice after line 22 is executed
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}
