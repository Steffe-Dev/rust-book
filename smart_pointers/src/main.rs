use smart_pointers::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    if let Cons(_val, ls) = list {
        println!("{:?}", *ls);
    }
}
