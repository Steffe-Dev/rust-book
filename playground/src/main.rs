fn main() {}

pub fn remove_char(s: &str) -> String {
    s.get(1..s.len() - 1).unwrap_or_default().to_string()
}

fn sel_number(n: u32, d: u8) -> u32 {
    (10..=n).filter(|num| n % 2 == 0).count().into();
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sel_number;

    #[test]
    fn sample_tests() {
        assert_eq!(sel_number(0, 1), 0);
        assert_eq!(sel_number(3, 1), 0);
        assert_eq!(sel_number(13, 1), 1);
        assert_eq!(sel_number(15, 1), 1);
        assert_eq!(sel_number(20, 2), 2);
        assert_eq!(sel_number(47, 3), 12);
    }
}
