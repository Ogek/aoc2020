use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(HashSet::new(), fold_lines_to_answers_set)
                .len()
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(line_to_answer_set)
                .fold_first(|acc, hs| acc.intersection(&hs).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}

fn line_to_answer_set(line: &str) -> HashSet<char> {
    line.chars().collect()
}

fn fold_lines_to_answers_set(set: HashSet<char>, line: &str) -> HashSet<char> {
    set.union(&line_to_answer_set(line)).cloned().collect()
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day06.txt")), 6416);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day06.txt")), 3050);
}
