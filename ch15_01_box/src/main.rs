use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Cons(2, Cons(3, Nil)));
    println!("list = {:?}", list);
    // fixme - infinite size of list
}
