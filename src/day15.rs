use std::collections::hash_map::Entry;
use std::collections::HashMap;

// Memo maps the number with the last turn it was spoken.
type Memo = HashMap<usize, usize>;

pub fn p1(input: &str) -> usize {
    game(parse(input)).nth(2019).unwrap()
}

pub fn p2(input: &str) -> usize {
    game(parse(input)).nth(29999999).unwrap()
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn game<'a>(starting_numbers: Vec<usize>) -> impl Iterator<Item = usize> + 'a {
    let mut memo: Memo = starting_numbers
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i + 1))
        .collect();

    let mut turn = starting_numbers.len();
    let mut last = *starting_numbers.last().unwrap();

    starting_numbers
        .into_iter()
        .chain(std::iter::from_fn(move || {
            last = match memo.entry(last) {
                Entry::Occupied(mut entry) => {
                    let age = turn - entry.get();
                    *entry.get_mut() = turn;
                    age
                }
                Entry::Vacant(entry) => {
                    entry.insert(turn);
                    0
                }
            };

            turn += 1;
            Some(last)
        }))
}

#[test]
fn test_p1() {
    assert_eq!(p1(include_str!("../inputs/day15.txt")), 1238);
}

#[test]
fn test_p2() {
    assert_eq!(p2(include_str!("../inputs/day15.txt")), 3745954);
}
