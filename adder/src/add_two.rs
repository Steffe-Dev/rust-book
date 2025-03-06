pub fn add_two(val: u64) -> u64 {
    2 + val
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn it_adds() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
