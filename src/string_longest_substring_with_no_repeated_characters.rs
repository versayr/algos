use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: &str) -> i32 {
    let mut result: i32 = 0;
    let mut slow: usize = 0;
    let mut map: HashMap<char, usize> = HashMap::new();

    for (fast, cha) in s.chars().enumerate() {
        if let Some(index) = map.get(&cha) && index >= &slow {
            slow = index + 1;
        } else {
            result = result.max(i32::try_from(fast - slow).expect("reason") + 1);
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
        let s = "";
        let result: i32 = 0;
        assert_eq!(length_of_longest_substring(s), result);
    }

    #[test]
    fn my_cases() {
        let input = "abcdefg";
        let output = 7;
        assert_eq!(length_of_longest_substring(input), output);
    }

    #[test]
    fn leetcode_cases() {
        let input = "abcabcbb";
        let output = 3;
        assert_eq!(length_of_longest_substring(input), output);

        let input = "bbbbb";
        let output = 1;
        assert_eq!(length_of_longest_substring(input), output);

        let input = "pwwkew";
        let output = 3;
        assert_eq!(length_of_longest_substring(input), output);

        let input = " ";
        let output = 1;
        assert_eq!(length_of_longest_substring(input), output);

        let input = "tmmzuxt";
        let output = 5;
        assert_eq!(length_of_longest_substring(input), output);

        let input = "aab";
        let output = 2;
        assert_eq!(length_of_longest_substring(input), output);
    }
}
