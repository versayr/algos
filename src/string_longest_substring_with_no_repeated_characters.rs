use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut slow: usize = 0;
    let mut map: HashMap<char, usize> = HashMap::new();

    for (fast, cha) in s.chars().enumerate() {
        let mut updated = false;

        if let Some(index) = map.get(&cha) {
            if *index >= slow {
                slow = index + 1;
                updated = true;
            }
        } 

        if !updated {
            result = result.max((fast - slow) as i32 + 1);
        }

        map.insert(cha, fast);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_case() {
        let s: String = "".into();
        let result: i32 = 0;
        assert_eq!(length_of_longest_substring(s), result);
    }

    #[test]
    fn my_cases() {
        let input = String::from("abcdefg");
        let output = 7;
        assert_eq!(length_of_longest_substring(input), output);
    }

    #[test]
    fn leetcode_cases() {
        let input = String::from("abcabcbb");
        let output = 3;
        assert_eq!(length_of_longest_substring(input), output);

        let input = String::from("bbbbb");
        let output = 1;
        assert_eq!(length_of_longest_substring(input), output);

        let input = String::from("pwwkew");
        let output = 3;
        assert_eq!(length_of_longest_substring(input), output);

        let input = String::from(" ");
        let output = 1;
        assert_eq!(length_of_longest_substring(input), output);

        let input = String::from("tmmzuxt");
        let output = 5;
        assert_eq!(length_of_longest_substring(input), output);

        let input = String::from("aab");
        let output = 2;
        assert_eq!(length_of_longest_substring(input), output);
    }
}
