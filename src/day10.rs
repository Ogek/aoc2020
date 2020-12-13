use std::collections::HashMap;
use std::iter;

type Adaptors = Vec<usize>;

pub fn p1(input: &str) -> usize {
    let adaptors = parse(input);
    let (a, b): (Vec<usize>, Vec<usize>) = adaptors
        .windows(2)
        .map(|w| w[1] - w[0])
        .partition(|&n| n == 1);
    a.len() * b.len()
}

pub fn p2(input: &str) -> usize {
    let adaptors = parse(input);
    let mut options: HashMap<usize, usize> = HashMap::new();
    options.insert(0, 1);

    for a in &adaptors[1..] {
        let diff_range = match a {
            1 | 2 => (1..=*a),
            _ => (1..=3),
        };
        options.insert(
            *a,
            diff_range.filter_map(|diff| options.get(&(a - diff))).sum(),
        );
    }

    *options.get(&(adaptors[adaptors.len() - 1])).unwrap()
}

fn parse(input: &str) -> Adaptors {
    let mut parsed = iter::once("0")
        .chain(input.lines())
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    parsed.sort();
    parsed.push(parsed[parsed.len() - 1] + 3);
    parsed
}

#[test]
fn test_dummy_data() {
    let data = "16
10
15
5
1
11
7
19
6
12
4";
    assert_eq!(p1(data), 7 * 5);
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day10.txt")), 2046);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day10.txt")), 1157018619904);
}
