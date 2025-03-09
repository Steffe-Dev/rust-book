use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) -> String {
    format!("Hello, {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_hello() {
        let slice = MyBox::new(String::from("boons"));
        assert_eq!("Hello, boons", hello(&slice));
    }
}
