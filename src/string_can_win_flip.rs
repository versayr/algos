#[allow(dead_code)]
pub fn can_win(string: &str) -> bool {
    if !string.contains("++") {
        return false;
    }

    let chars: Vec<char> = string.chars().collect();

    for i in 0..string.len() - 1 {
        if *chars.get(i).expect("Index out of range.") == '+'
            && *chars.get(i).expect("Index out of range.") == '+'
        {
            let mut new_str: String = string.to_string();
            new_str.replace_range(i..i + 2, "--");
            if !can_win(&new_str) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn tests() {
        assert!(can_win("").not());
        assert!(can_win("+").not());
        assert!(can_win("-").not());
        assert!(can_win("--").not());
        assert!(can_win("++-++").not());
        assert!(can_win("+-+-").not());
        assert!(can_win("++"));
        assert!(can_win("-++"));
        assert!(can_win("-+++"));
        assert!(can_win("++++"));
    }
}
