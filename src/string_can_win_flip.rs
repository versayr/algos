#[allow(dead_code)]
pub fn can_win(string: String) -> bool {
    if !string.contains("++") {
        return false;
    }

    for i in 0..string.len() - 1 {
        let sub: &str = &string[i..i + 2];
        if sub == "++" {
            let mut new_str: String = string.clone();
            new_str.replace_range(i..i + 2, "--");
            if !can_win(new_str) {
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
        assert!(can_win(String::from("")).not());
        assert!(can_win(String::from("+")).not());
        assert!(can_win(String::from("-")).not());
        assert!(can_win(String::from("--")).not());
        assert!(can_win(String::from("++-++")).not());
        assert!(can_win(String::from("+-+-")).not());
        assert!(can_win(String::from("++")));
        assert!(can_win(String::from("-++")));
        assert!(can_win(String::from("-+++")));
        assert!(can_win(String::from("++++")));
    }
}
