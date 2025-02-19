const IS_RECURSIVE: bool = false;
fn main() {
    let n = 100;
    let x = fibonacci(n);
    println!("the {n}th fibo is {x}")
}

fn fibonacci(n: usize) -> u128 {
    if IS_RECURSIVE {
        recursive(n)
    } else {
        iterative(n)
    }
}

fn iterative(n: usize) -> u128 {
    let mut memoized = vec![1; n + 1];
    for i in 3..=n {
        memoized[i] = memoized[i - 1] + memoized[i - 2];
    }
    memoized[n]
}

fn recursive(n: usize) -> u128 {
    return if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn large() {
        assert_eq!(fibonacci(40), 102334155);
        assert_eq!(fibonacci(45), 1134903170);
        assert_eq!(fibonacci(100), 354224848179261915075);
    }
}
