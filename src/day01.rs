use std::collections::HashSet;

pub fn p1(input: &str) -> i32 {
    let set = build_set(input);

    for x in &set {
        match set.get(&(2020 - x)) {
            Some(&y) => return x * y,
            _ => continue,
        }
    }
    unreachable!()
}

pub fn p2(input: &str) -> i32 {
    let set = build_set(input);

    for x in &set {
        for y in &set {
            match set.get(&(2020 - x - y)) {
                Some(&z) => return x * y * z,
                _ => continue,
            }
        }
    }
    unreachable!()
}

fn build_set(input: &str) -> HashSet<i32> {
    input.lines().map(|n| n.parse::<i32>().unwrap()).collect()
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day01.txt")), 996075);
}
#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day01.txt")), 51810360);
}
