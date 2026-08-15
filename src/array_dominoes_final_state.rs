#[allow(dead_code)]
pub fn dominoes_final_state(mut array: Vec<char>) -> Vec<char> {
    let mut slow: usize = 0;
    let mut fast: usize = 1;

    while fast < array.len() {
        while fast < array.len() - 1 && *array.get(fast).expect("Index out of range.") == '.' {
            fast += 1;
        }

        let left = *array.get(slow).expect("Index out of range.");
        let right = *array.get(fast).expect("Index out of range.");

        if left == 'R' && right == 'L' {
            let mut s = slow + 1;
            let mut f = fast - 1;

            while s < f {
                *array.get_mut(s).expect("Index out of range.") = 'R';
                *array.get_mut(f).expect("Index out of range.") = 'L';
                s += 1;
                f -= 1;
            }
        } else if { left == '.' && right == 'L' } || { left == 'L' && right == 'L' } {
            while slow <= fast {
                *array.get_mut(slow).expect("Index out of range.") = 'L';
                slow += 1;
            }
        } else if { left == 'R' && right == '.' } || { left == 'R' && right == 'R' } {
            while slow <= fast {
                *array.get_mut(slow).expect("Index out of range.") = 'R';
                slow += 1;
            }
        }

        slow = fast;
        fast += 1;
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let array = vec![];
        assert_eq!(dominoes_final_state(array.clone()), array);
    }

    #[test]
    fn test_stable_array() {
        let array = vec!['.'];
        assert_eq!(dominoes_final_state(array.clone()), array);
        let array = vec!['L'];
        assert_eq!(dominoes_final_state(array.clone()), array);
        let array = vec!['R'];
        assert_eq!(dominoes_final_state(array.clone()), array);
        let array = vec!['R', 'R', 'R'];
        assert_eq!(dominoes_final_state(array.clone()), array);
        let array = vec!['L', 'L', 'L'];
        assert_eq!(dominoes_final_state(array.clone()), array);
        let array = vec!['.', '.', '.'];
        assert_eq!(dominoes_final_state(array.clone()), array);
    }

    #[test]
    fn test_unstable_array() {
        let array = vec!['R', '.', '.', 'L'];
        let result = vec!['R', 'R', 'L', 'L'];
        assert_eq!(dominoes_final_state(array.clone()), result);
        let array = vec!['.', 'L', '.', 'R', '.'];
        let result = vec!['L', 'L', '.', 'R', 'R'];
        assert_eq!(dominoes_final_state(array.clone()), result);
        let array = vec!['.', 'R', '.', '.', 'L', '.'];
        let result = vec!['.', 'R', 'R', 'L', 'L', '.'];
        assert_eq!(dominoes_final_state(array.clone()), result);
        let array = vec!['.', 'R', '.', '.', '.', 'L', '.'];
        let result = vec!['.', 'R', 'R', '.', 'L', 'L', '.'];
        assert_eq!(dominoes_final_state(array.clone()), result);
        let array = vec!['.', 'R', '.', 'L', '.', 'L', '.', 'R', '.'];
        let result = vec!['.', 'R', '.', 'L', 'L', 'L', '.', 'R', 'R'];
        assert_eq!(dominoes_final_state(array.clone()), result);
    }
}
