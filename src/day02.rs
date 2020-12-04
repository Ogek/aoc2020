use serde_scan::scan;

struct ParsedLine {
    pos1: usize,
    pos2: usize,
    c: char,
    pwd: String,
}

pub fn p1(input: &str) -> usize {
    input.lines().map(parse_line).filter(is_valid_pwd_1).count()
}

pub fn p2(input: &str) -> usize {
    input.lines().map(parse_line).filter(is_valid_pwd_2).count()
}

fn parse_line(line: &str) -> ParsedLine {
    let (pos1, pos2, c, pwd) = scan!("{}-{} {}: {}" <- line).unwrap();

    ParsedLine { pos1, pos2, c, pwd }
}

fn is_valid_pwd_1(pwd_val: &ParsedLine) -> bool {
    (pwd_val.pos1..=pwd_val.pos2).contains(&pwd_val.pwd.matches(pwd_val.c).count())
}

fn is_valid_pwd_2(pwd_val: &ParsedLine) -> bool {
    pwd_val
        .pwd
        .match_indices(pwd_val.c)
        .filter(move |(i, _)| i + 1 == pwd_val.pos1 || i + 1 == pwd_val.pos2)
        .count()
        == 1
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day02.txt")), 524);
}
#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day02.txt")), 485);
}
