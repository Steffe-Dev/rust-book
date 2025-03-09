mod my_box;

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use List::{Cons, Nil};

    #[test]
    fn it_works() {
        let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
        assert!(true);
    }
}
