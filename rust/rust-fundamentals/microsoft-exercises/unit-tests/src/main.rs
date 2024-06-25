fn main() {
    println!("Hello, world!");
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(44));
    }

    #[test]
    #[should_panic]
    fn is_false_when_odd() {
        assert!(is_even(37));
    }
}