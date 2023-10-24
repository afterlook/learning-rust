use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
    println!("count after 'a' declaration: {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after 'b' declaration: {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after 'c' declaration: {}", Rc::strong_count(&a));
    }
    println!("count after 'c' leaving scope: {}", Rc::strong_count(&a));
}
