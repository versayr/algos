#[allow(dead_code)]
pub fn dominoes_final_state(mut array: Vec<char>) -> Vec<char> {
    let mut slow: usize = 0;
    let mut fast: usize = 1;

    while fast < array.len() {
        while fast < array.len() - 1 && array[fast] == '.' {
            fast += 1;
        }

        let left = array[slow];
        let right = array[fast];

        if left == 'R' && right == 'L' {
            let mut slo = slow + 1;
            let mut fst = fast - 1;

            while slo < fst {
                array[slo] = 'R';
                array[fst] = 'L';
                slo += 1;
                fst -= 1;
            }
        } else if { left == '.' && right == 'L' } || { left == 'L' && right == 'L' } {
            while slow <= fast {
                array[slow] = 'L';
                slow += 1;
            }
        } else if { left == 'R' && right == '.' } || { left == 'R' && right == 'R' } {
            while slow <= fast {
                array[slow] = 'R';
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
