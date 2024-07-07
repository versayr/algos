#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    let mut reversed: i32 = 0;
    let mut tmp: i32 = x;

    while tmp > 0 {
        reversed *= 10;
        reversed += tmp % 10;
        tmp /= 10;
    }

    x == reversed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn tests() {
        assert!(is_palindrome(-121).not());
        assert!(is_palindrome(10).not());
        assert!(is_palindrome(121));
        assert!(is_palindrome(1));
        assert!(is_palindrome(221122));
    }
}
