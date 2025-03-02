use std::collections::HashMap;

pub struct Stats<T> {
    pub median: T,
    pub mode: T,
}

pub fn get_stats(list: &Vec<i32>) -> Option<Stats<i32>> {
    if list.is_empty() {
        return None;
    }
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for i in list {
        let current = occurrences.entry(*i).or_insert(0);
        *current += 1;
    }
    let mut mode = list[0];
    let mut current_max = 1;
    for (key, value) in &occurrences {
        if *value > current_max {
            mode = *key;
            current_max = *value;
        }
    }
    let mut cloned = list.into_iter().cloned().collect::<Vec<i32>>();
    cloned.sort();
    let median = cloned[list.len() / 2];

    Some(Stats { median, mode })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_none_when_passed_an_empty_list() {
        let list: Vec<i32> = vec![];
        assert!(get_stats(&list).is_none());
    }

    #[test]
    fn it_returns_the_same_for_both_when_only_one_element() {
        let list = vec![1];
        assert_eq!(get_stats(&list).unwrap().median, 1);
        assert_eq!(get_stats(&list).unwrap().mode, 1);
    }

    #[test]
    fn it_returns_the_correct_mode_1() {
        let list = vec![3, 1, 5, 1, 2];
        assert_eq!(get_stats(&list).unwrap().mode, 1);
    }

    #[test]
    fn it_returns_the_correct_median_1() {
        let list = vec![3, 1, 5, 1, 2];
        assert_eq!(get_stats(&list).unwrap().median, 2);
    }

    #[test]
    fn it_returns_the_correct_median_2() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(get_stats(&list).unwrap().median, 3);
    }

    #[test]
    fn it_returns_the_correct_median_3() {
        let list = vec![5, 3, 1, 4, 2];
        assert_eq!(get_stats(&list).unwrap().median, 3);
    }

    #[test]
    fn it_returns_the_correct_median_4() {
        let list = vec![10, 20, 30, 40, 50];
        assert_eq!(get_stats(&list).unwrap().median, 30);
    }

    #[test]
    fn it_returns_the_correct_median_5() {
        let list = vec![7, 8, 9, 10, 11];
        assert_eq!(get_stats(&list).unwrap().median, 9);
    }

    #[test]
    fn it_returns_the_correct_median_6() {
        let list = vec![1, 1, 1, 1, 1];
        assert_eq!(get_stats(&list).unwrap().median, 1);
    }

    #[test]
    fn it_returns_the_correct_mode_2() {
        let list = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(get_stats(&list).unwrap().mode, 3);
    }

    #[test]
    fn it_returns_the_correct_mode_3() {
        let list = vec![4, 4, 4, 4, 4];
        assert_eq!(get_stats(&list).unwrap().mode, 4);
    }

    #[test]
    fn it_returns_the_correct_mode_4() {
        let list = vec![1, 2, 3, 4, 5, 5, 5, 5];
        assert_eq!(get_stats(&list).unwrap().mode, 5);
    }

    #[test]
    fn it_returns_the_correct_mode_5() {
        let list = vec![10, 20, 20, 30, 30, 30];
        assert_eq!(get_stats(&list).unwrap().mode, 30);
    }

    #[test]
    fn it_returns_the_correct_mode_6() {
        let list = vec![7, 8, 9, 9, 9, 10, 11];
        assert_eq!(get_stats(&list).unwrap().mode, 9);
    }
}
