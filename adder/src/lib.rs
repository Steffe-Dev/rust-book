mod add_two;
mod rect;

pub use add_two::*;
pub use rect::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn it_adds() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("fail");
    }
}
