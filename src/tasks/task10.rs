use core::panic;

use crate::{task_handler::get_task, tasks::quickselect::nth_element};

#[derive(Clone, Copy)]
enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '{' | '[' | '<' | '(' => Some(Self::Open(c)),
            '}' => Some(Self::Close('{')),
            ']' => Some(Self::Close('[')),
            '>' => Some(Self::Close('<')),
            ')' => Some(Self::Close('(')),
            _ => None,
        }
    }

    pub fn score(&self) -> usize {
        let char_bracket = match self {
            Bracket::Open(char_bracket) => char_bracket,
            Bracket::Close(char_bracket) => char_bracket,
        };
        match char_bracket {
            '(' => 3,
            '[' => 57,
            '{' => 1197,
            '<' => 25137,
            _ => panic!("Non bracket found."),
        }
    }

    pub fn score_auto_complete(&self) -> usize {
        let char_bracket = match self {
            Bracket::Open(char_bracket) => char_bracket,
            Bracket::Close(char_bracket) => char_bracket,
        };
        match char_bracket {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Non bracket found."),
        }
    }
}

// Returns none if all brackets match, otherwise returns the unmatched bracket
fn balanced_brackets_corrupted(string: &str) -> Option<Bracket> {
    let mut bracket_stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match &Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => bracket_stack.push(*char_bracket),
            Some(l @ Bracket::Close(char_bracket)) => {
                if bracket_stack.pop() != Some(*char_bracket) {
                    return Some(*l);
                }
            }
            _ => {}
        }
    }
    None
}

fn complete_line(string: &str) -> Option<Vec<Bracket>> {
    let mut bracket_stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match &Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => bracket_stack.push(*char_bracket),
            Some(Bracket::Close(char_bracket)) if bracket_stack.pop() != Some(*char_bracket) => {
                return None
            }
            _ => {}
        }
    }

    Some(
        bracket_stack
            .iter()
            .filter_map(|&x| Bracket::from_char(x))
            .collect(),
    )
}

pub fn task10_1() -> String {
    get_task(10)
        .lines()
        .filter_map(balanced_brackets_corrupted)
        .map(|bracket| bracket.score())
        .sum::<usize>()
        .to_string()
}

pub fn task10_2() -> String {
    let mut sums = get_task(10)
        .lines()
        .filter_map(complete_line)
        .map(|line| {
            line.iter()
                .map(|char_bracket| char_bracket.score_auto_complete())
                .rev()
                .fold(0, |acc, elem| acc * 5 + elem)
        })
        .collect::<Vec<_>>();

    let i = sums.len() / 2;
    // get the median
    nth_element(&mut sums, i).to_string()
}

#[cfg(test)]
mod test {
    use crate::tasks::task10::task10_2;

    #[test]
    fn test_task_2() {
        assert_eq!(task10_2(), "2995077699")
    }
}
