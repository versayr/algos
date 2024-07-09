#[allow(dead_code)]
pub fn justify(words: Vec<&str>, target: usize) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let mut iter = words.into_iter().peekable();

    while iter.peek().is_some() {
        let mut line: Vec<&str> = vec![];
        let mut chars: usize = 0;

        while let Some(word) = iter.next_if(|&w| w.len() + chars < target) {
            line.push(word);
            if line.len() != 1 {
                chars += 1
            };
            chars += word.len();
        }

        if iter.peek().is_none() || line.len() == 1 {
            output.push(format!("{:target$}", line.join(" ")));
        } else {
            output.push(normal_justify(line, chars, target));
        }
    }

    output
}

fn normal_justify(line: Vec<&str>, chars: usize, max: usize) -> String {
    let gaps: usize = line.len() - 1;
    let baseline: usize = 1 + (max - chars) / gaps;
    let baseline_spaces: String = format!("{:baseline$}", " ");
    let mut remainder: usize = (max - chars) % gaps;
    let mut words: Vec<&str> = vec![];
    let mut iter = line.into_iter().peekable();

    while let Some(word) = iter.next() {
        words.push(word);
        if iter.peek().is_some() {
            words.push(&baseline_spaces)
        }
        if remainder > 0 {
            words.push(" ");
            remainder -= 1;
        }
    }

    words.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(justify(vec!["singleton"], 15), vec!["singleton      "]);
        assert_eq!(
            justify(vec!["xx", "x", "x", "xx", "x", "x"], 5),
            vec!["xx  x", "x  xx", "x x  "]
        );
        assert_eq!(
            justify(
                vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"],
                9
            ),
            vec![
                "the quick",
                "brown fox",
                "jumps    ",
                "over  the",
                "lazy dog "
            ]
        );
        assert_eq!(
            justify(
                vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"],
                16
            ),
            vec!["the  quick brown", "fox  jumps  over", "the lazy dog    "]
        );
    }
}

//  OLD
//  let mut idx: usize = 0;

//  while idx < words.len() {
//      let mut line: Vec<&str> = vec![];
//      let mut chars: usize = 0;

//      while idx < words.len() && chars + words[idx].len() < target {
//          let word: &str = words[idx];
//          line.push(word);

//          if line.len() != 1 { chars += 1 };
//          chars += word.len();

//          idx += 1;
//      }

//      if idx == words.len() || line.len() == 1 {
//          output.push(format!("{:width$}", line.join(" "), width = target));
//      } else {
//          output.push(normal_justify(line, chars, target));
//      }
//  }
